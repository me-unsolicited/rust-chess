use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::engine::{EngineState, eval, gen, GoParams, Transposition};
use crate::engine::board::{Board, CastleRights, Color, Placement};
use crate::engine::mov::Move;
use crate::engine::piece::PieceType;
use std::sync::mpsc::Sender;


const DEPTH: i32 = 4;

// min/max values that won't overflow on negation
const MIN_EVAL: i32 = -std::i32::MAX;
const MAX_EVAL: i32 = -MIN_EVAL;

pub fn search(state: Arc<Mutex<EngineState>>, p: GoParams, thread_index: usize, tx_stats: Sender<SearchStats>) {
    let root_position = state.lock().unwrap().position.clone();

    let mut position = root_position;
    for mov in p.search_moves {
        position.push(mov);
    }

    let table = state.lock().unwrap().table.clone();
    let mut searcher = NegamaxAb::new(table, thread_index);
    searcher.search(&position);

    if let Err(_) = tx_stats.send(searcher.get_stats()) {
        panic!("failed to send search statistics");
    }
}

#[derive(Copy, Clone)]
pub struct SearchStats {
    pub nodes_visited: u64,
    pub tt_hits: i32,
    pub tt_waste: i32,
    pub time_elapsed: Duration,
    pub max_depth: i32,
}

impl SearchStats {
    pub fn new() -> Self {
        Self {
            nodes_visited: 0,
            tt_hits: 0,
            tt_waste: 0,
            time_elapsed: Duration::from_secs(0),
            max_depth: 0,
        }
    }

    pub fn nps(&self) -> u64 {

        // get elapsed milliseconds and avoid divide by zero
        let millis = self.time_elapsed.as_millis() as u64;
        let millis = millis.max(1);

        1000 * self.nodes_visited / millis
    }

    pub fn combine(&mut self, other: &Self) {
        self.nodes_visited += other.nodes_visited;
        self.tt_hits += other.tt_hits;
        self.tt_waste += other.tt_waste;
        self.time_elapsed = self.time_elapsed.max(other.time_elapsed);
        self.max_depth = self.max_depth.max(other.max_depth);
    }
}

trait Searcher {
    fn search(&mut self, position: &Board);
    fn get_stats(&self) -> SearchStats;
}

struct NegamaxAb {
    stats: SearchStats,
    table: Arc<Mutex<HashMap<u64, Transposition>>>,
    rng: rand::rngs::ThreadRng,
    ab_depth: i32,
    thread_index: usize,
}

impl Searcher for NegamaxAb {
    fn search(&mut self, position: &Board) {

        // re-initialize thread local random, maybe for the first time
        self.rng = rand::thread_rng();

        // begin timing the search routine
        let start = Instant::now();

        // iterative deepening
        for i in 1..=DEPTH {
            let mut position = position.clone();
            let sign = if position.turn == Color::WHITE { 1 } else { -1 };
            self.ab_depth = i;
            self.negamax(&mut position, i, MIN_EVAL, MAX_EVAL, sign);
        }

        self.stats.time_elapsed = start.elapsed();
    }

    fn get_stats(&self) -> SearchStats {
        self.stats
    }
}

impl NegamaxAb {
    pub fn new(table: Arc<Mutex<HashMap<u64, Transposition>>>, thread_index: usize) -> Self {
        Self {
            stats: SearchStats::new(),
            table,
            rng: rand::thread_rng(),
            ab_depth: DEPTH,
            thread_index,
        }
    }

    fn negamax(&mut self, position: &mut Board, depth: i32, mut alpha: i32, beta: i32, sign: i32) -> i32 {

        // switch to quiescence search at max alpha-beta depth
        if depth == 0 {
            return sign * self.quiesce(position, depth - 1, alpha, beta);
        }

        // track search statistics
        self.stats.nodes_visited += 1;
        self.stats.max_depth = self.stats.max_depth.max(self.ab_depth - depth);

        // find transposition
        let transposition = self.read_transposition(&position);
        if let Some(t) = transposition.as_ref() {

            // repeating the same position toward a draw?
            if t.eval_depth > depth {
                return 0;
            }

            // already evaluated at depth?
            if t.eval_depth == depth {
                return t.eval;
            }
        }

        // fifty-move rule
        if position.halfmove_clock >= 50 {
            return 0;
        }

        // three-fold repetition
        if is_three_fold(&position) {
            return 0;
        }

        // generate moves to test for checkmate/stalemate
        let mut moves = gen::gen_moves(&position);

        // no available moves? the game is over
        if moves.is_empty() {
            let is_mate = position.is_check();
            return if is_mate { MIN_EVAL + position.fullmove_number as i32 } else { 0 };
        }

        // order moves to improve alpha-beta pruning
        order_moves(&mut moves, position, transposition.as_ref());

        // Lazy-SMP: at the root node, reorder the moves according to the current thread
        if depth == self.ab_depth {
            let swap_index = (moves.len() - 1).min(self.thread_index);
            moves.swap(0, swap_index);
        }

        // choose the best variation
        let mut best_eval = MIN_EVAL;
        let mut best_move = None;

        // go deeper for each move
        for mov in moves {
            position.push(mov);
            let eval = -self.negamax(position, depth - 1, -beta, -alpha, -sign);
            position.pop();

            if best_move.is_none() || eval > best_eval {
                best_eval = eval;
                best_move = Some(mov);
            }

            // alpha-beta pruning
            alpha = alpha.max(best_eval);
            if alpha >= beta {
                break;
            }
        }

        // update transposition table with result
        self.write_transposition(position, Transposition {
            eval: best_eval,
            eval_depth: depth,
            best_move,
        });

        best_eval
    }

    fn quiesce(&mut self, position: &mut Board, depth: i32, mut alpha: i32, beta: i32) -> i32 {

        // track search statistics
        self.stats.nodes_visited += 1;
        self.stats.max_depth = self.stats.max_depth.max(self.ab_depth - depth);

        // fifty-move rule
        if position.halfmove_clock >= 50 {
            return 0;
        }

        // three-fold repetition
        if is_three_fold(&position) {
            return 0;
        }

        let stand_pat = eval::evaluate(position);
        if stand_pat >= beta {
            return beta;
        } else if alpha < stand_pat {
            alpha = stand_pat;
        }

        // generate moves
        let mut moves = gen::gen_moves(position);

        // no available moves? the game is over
        if moves.is_empty() {
            let is_mate = position.is_check();
            return if is_mate { MIN_EVAL + position.fullmove_number as i32 } else { 0 };
        }

        // remove quiet moves
        moves.retain(|m| is_loud(position, m));

        for mov in moves {
            position.push(mov);
            let score = -self.quiesce(position, depth - 1, -beta, -alpha);
            position.pop();

            if score >= beta {
                return beta;
            } else if score > alpha {
                alpha = score;
            }
        }

        alpha
    }

    fn read_transposition(&mut self, position: &Board) -> Option<Transposition> {
        let transposition = self.table.lock().unwrap().get(&position.hash).cloned();

        if transposition.is_some() {
            self.stats.tt_hits += 1;
        }

        transposition
    }

    fn write_transposition(&mut self, position: &Board, t: Transposition) {
        let mut table = self.table.lock().unwrap();

        // do not overwrite a more valuable record; this can happen in parallel searches
        if let Some(entry) = table.get(&position.hash) {
            if t.eval_depth <= entry.eval_depth {
                self.stats.tt_waste += 1;
                return;
            }
        }

        table.insert(position.hash, t);
    }
}

fn is_three_fold(position: &Board) -> bool {
    if position.halfmove_clock > 4 {
        let hash = position.hash;

        let mut n = 0;
        let mut previous = position.previous.as_ref();
        while previous.is_some() {
            let current = previous.unwrap();

            // find a repetition
            if hash == current.hash {
                n += 1;
                if n >= 2 {
                    return true;
                }
            }

            // look deeper unless halfmove_clock indicates irreversible move
            previous = if current.halfmove_clock > 0 {
                current.previous.as_ref()
            } else {
                None
            }
        }
    }

    false
}

fn order_moves(moves: &mut Vec<Move>, board: &Board, transposition: Option<&Transposition>) {
    let mut pv = None;
    if let Some(t) = transposition {
        pv = t.best_move;
    }

    let mut orders = Vec::with_capacity(moves.len());
    while !moves.is_empty() {
        let mov = moves.pop().unwrap();
        let order = if pv.is_some() && mov == pv.unwrap() {
            MAX_EVAL
        } else {
            eval::evaluate_exchange(board, &mov)
        };

        orders.push((mov, order));
    }

    orders.sort_by_key(|o| o.1);
    while !orders.is_empty() {
        moves.push(orders.pop().unwrap().0);
    }
}

fn is_loud(position: &Board, mov: &Move) -> bool {

    // queen promotions
    if let Some(&PieceType::QUEEN) = mov.promotion {
        return true;
    }

    // captures
    let mut capture_sq = mov.to.idx as i32;
    if let Some(ep_target) = position.en_passant_target {
        let moving_pawn = 0 != position.placement.pawns & (1 << mov.from.idx);
        if moving_pawn && ep_target.idx == mov.to.idx {
            match position.turn {
                Color::WHITE => capture_sq -= 8,
                Color::BLACK => capture_sq += 8,
            }
        }
    }
    if 0 != (position.placement.white | position.placement.black) & (1 << capture_sq) {
        return true;
    }

    // checks; fake a new position after a non-capture move and test it
    let from_bit = 1 << mov.from.idx;
    let pawn_bit = position.placement.pawns & from_bit;
    let knight_bit = position.placement.knights & from_bit;
    let bishop_bit = position.placement.bishops & from_bit;
    let rook_bit = position.placement.rooks & from_bit;
    let queen_bit = position.placement.queens & from_bit;
    let king_bit = position.placement.kings & from_bit;
    let white_bit = position.placement.white & from_bit;
    let black_bit = position.placement.black & from_bit;

    let mut pawns = position.placement.pawns & !from_bit;
    let mut knights = position.placement.knights & !from_bit;
    let mut bishops = position.placement.bishops & !from_bit;
    let mut rooks = position.placement.rooks & !from_bit;
    let mut queens = position.placement.queens & !from_bit;
    let mut kings = position.placement.kings & !from_bit;
    let mut white = position.placement.white & !from_bit;
    let mut black = position.placement.black & !from_bit;

    let mut diff_sq = mov.to.idx as i32 - mov.from.idx as i32;
    if diff_sq < 0 { diff_sq += 64; }
    let diff_sq = diff_sq as u32;

    match mov.promotion {
        Some(&PieceType::KNIGHT) => knights |= 1 << mov.to.idx,
        Some(&PieceType::BISHOP) => bishops |= 1 << mov.to.idx,
        Some(&PieceType::ROOK) => rooks |= 1 << mov.to.idx,
        Some(&PieceType::QUEEN) => queens |= 1 << mov.to.idx,
        _ => {
            pawns |= pawn_bit.rotate_left(diff_sq);
            knights |= knight_bit.rotate_left(diff_sq);
            bishops |= bishop_bit.rotate_left(diff_sq);
            rooks |= rook_bit.rotate_left(diff_sq);
            queens |= queen_bit.rotate_left(diff_sq);
            kings |= king_bit.rotate_left(diff_sq);
        }
    }
    white |= white_bit.rotate_left(diff_sq);
    black |= black_bit.rotate_left(diff_sq);

    let mut spoof = Board {
        placement: Placement {
            pawns,
            knights,
            bishops,
            rooks,
            queens,
            kings,
            white,
            black,
        },
        turn: position.turn,
        castle_rights: CastleRights {
            kingside_w: false,
            queenside_w: false,
            kingside_b: false,
            queenside_b: false,
        },
        en_passant_target: None,
        halfmove_clock: 0,
        fullmove_number: 0,
        previous: None,
        hash: 0,
    };
    if spoof.turn == Color::BLACK {
        spoof = spoof.mirror();
    }
    if !0 != gen::get_check_restriction(&spoof) {
        return true;
    }

    false
}
