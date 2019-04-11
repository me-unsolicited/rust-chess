use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::engine::{EngineState, eval, gen, GoParams, Transposition};
use crate::engine::board::{Board, Color};
use crate::engine::mov::Move;
use std::collections::HashMap;

// min/max values that won't overflow on negation
const MIN_EVAL: i32 = -std::i32::MAX;
const MAX_EVAL: i32 = -MIN_EVAL;

// compile-time configuration of search algorithm
fn new_searcher(table: Arc<Mutex<HashMap<u64, Transposition>>>) -> impl Searcher { NegamaxAb::new(table) }

pub fn search(state: Arc<Mutex<EngineState>>, p: GoParams) {
    let root_position = state.lock().unwrap().position.clone();

    let mut position = root_position;
    for mov in p.search_moves {
        position = position.push(mov);
    }

    let table = state.lock().unwrap().table.clone();
    let mut searcher = new_searcher(table);
    let mov = searcher.search(&position);

    let stats = searcher.get_stats();
    eprintln!("---- {}", mov.uci());
    eprintln!("nodes_visited: {}", stats.nodes_visited);
    eprintln!("tt_hits: {}", stats.tt_hits);
    eprintln!("time_elapsed (ms): {}", stats.time_elapsed.as_millis());
    eprintln!("max_depth: {}", stats.max_depth);
    eprintln!("nps: {}", stats.nps());

    (state.lock().unwrap().callbacks.best_move_fn)(&mov);
}

#[derive(Copy, Clone)]
struct SearchStats {
    nodes_visited: u64,
    tt_hits: i32,
    time_elapsed: Duration,
    max_depth: i32,
}

impl SearchStats {
    pub fn new() -> Self {
        Self {
            nodes_visited: 0,
            tt_hits: 0,
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
}

trait Searcher {
    fn search(&mut self, position: &Board) -> Move;
    fn get_stats(&self) -> SearchStats;
}

struct Negamax {
    stats: SearchStats,
    table: Arc<Mutex<HashMap<u64, Transposition>>>,
}

impl Searcher for Negamax {
    fn search(&mut self, position: &Board) -> Move {
        let start = Instant::now();

        let position = position.clone();
        let sign = if position.turn == Color::WHITE { 1 } else { -1 };
        let (_, mov, _) = self.negamax(position, Self::DEPTH, sign);

        self.stats.time_elapsed = start.elapsed();

        mov.expect("no move")
    }

    fn get_stats(&self) -> SearchStats {
        self.stats
    }
}

impl Negamax {
    const DEPTH: i32 = 5;

    #[allow(dead_code)]
    pub fn new(table: Arc<Mutex<HashMap<u64, Transposition>>>) -> Self {
        Self {
            stats: SearchStats::new(),
            table,
        }
    }

    fn negamax(&mut self, mut position: Board, depth: i32, sign: i32) -> (i32, Option<Move>, Board) {

        // track search statistics
        self.stats.nodes_visited += 1;
        self.stats.max_depth = self.stats.max_depth.max(Self::DEPTH - depth);

        // fifty-move rule
        if position.halfmove_clock >= 50 {
            return (0, None, position);
        }

        // three-fold repetition
        if is_three_fold(&position) {
            return (0, None, position);
        }

        // find transposition and exit early if already evaluated at depth
        // must at least 2-ply or won't find draws
        if depth < Self::DEPTH - 1 {
            let mut table = self.table.lock().unwrap();
            let transposition = table.get_mut(&position.hash);
            if let Some(transposition) = transposition {
                self.stats.tt_hits += 1;
                if transposition.eval_depth >= depth {
                    return (transposition.eval, transposition.best_move, position);
                }
            }
        }

        // have we reached max depth?
        if depth <= 0 {
            return (sign * eval::evaluate(&position), None, position);
        }

        // generate moves to test for checkmate/stalemate
        let moves = gen::gen_moves(&position);

        // no available moves? the game is over
        if moves.is_empty() {

            // check restriction is calculated from white pieces perspective
            let check_restriction = if position.turn == Color::WHITE {
                gen::get_check_restriction(&position)
            } else {
                let mirror = position.mirror();
                gen::get_check_restriction(&mirror)
            };

            let is_mate = !0 != check_restriction;
            return (if is_mate { MIN_EVAL + position.fullmove_number as i32} else { 0 }, None, position);
        }

        // choose the best variation
        let mut best_eval = MIN_EVAL;
        let mut best_move = None;

        // go deeper for each move
        for mov in moves {
            position = position.push(mov);
            let (eval, _, stack) = self.negamax(position, depth - 1, -sign);
            let eval = -eval;

            position = *stack.pop();

            if best_move.is_none() || eval > best_eval {
                best_eval = eval;
                best_move = Some(mov);
            }
        }

        // update transposition table
        {
            let mut table = self.table.lock().unwrap();
            table.insert(position.hash, Transposition {
                eval: best_eval,
                eval_depth: depth,
                best_move,
            });
        }

        (best_eval, best_move, position)
    }
}

struct NegamaxAb {
    stats: SearchStats,
    table: Arc<Mutex<HashMap<u64, Transposition>>>,
    rng: rand::rngs::ThreadRng,
}

impl Searcher for NegamaxAb {
    fn search(&mut self, position: &Board) -> Move {

        // re-initialize thread local random
        self.rng = rand::thread_rng();

        // begin timing the search routine
        let start = Instant::now();

        let position = position.clone();
        let sign = if position.turn == Color::WHITE { 1 } else { -1 };
        let (_, mov, _) = self.negamax(position, Self::DEPTH, MIN_EVAL, MAX_EVAL, sign);

        self.stats.time_elapsed = start.elapsed();

        mov.expect("no move")
    }

    fn get_stats(&self) -> SearchStats {
        self.stats
    }
}

impl NegamaxAb {
    const DEPTH: i32 = 7;

    #[allow(dead_code)]
    pub fn new(table: Arc<Mutex<HashMap<u64, Transposition>>>) -> Self {
        Self {
            stats: SearchStats::new(),
            table,
            rng: rand::thread_rng(),
        }
    }

    fn negamax(&mut self, mut position: Board, depth: i32, mut alpha: i32, beta: i32, sign: i32) -> (i32, Option<Move>, Board) {

        // track search statistics
        self.stats.nodes_visited += 1;
        self.stats.max_depth = self.stats.max_depth.max(Self::DEPTH - depth);

        // fifty-move rule
        if position.halfmove_clock >= 50 {
            return (0, None, position);
        }

        // three-fold repetition
        if is_three_fold(&position) {
            return (0, None, position);
        }

        // find transposition and exit early if already evaluated at depth
        // must at least 2-ply or won't find draws
        let mut known_move = None;
        if depth < Self::DEPTH - 1 {
            let mut table = self.table.lock().unwrap();
            let transposition = table.get_mut(&position.hash);
            if let Some(transposition) = transposition {
                self.stats.tt_hits += 1;
                if transposition.eval_depth >= depth {
                    return (transposition.eval, transposition.best_move, position);
                }
                known_move = transposition.best_move;
            }
        }

        // have we reached max depth?
        if depth <= 0 {
            return (sign * eval::evaluate(&position), None, position);
        }

        // generate moves to test for checkmate/stalemate
        let mut moves = gen::gen_moves(&position);

        // no available moves? the game is over
        if moves.is_empty() {

            // check restriction is calculated from white pieces perspective
            let check_restriction = if position.turn == Color::WHITE {
                gen::get_check_restriction(&position)
            } else {
                let mirror = position.mirror();
                gen::get_check_restriction(&mirror)
            };

            let is_mate = !0 != check_restriction;
            return (if is_mate { MIN_EVAL + position.fullmove_number as i32} else { 0 }, None, position);
        }

        // order moves to improve alpha-beta pruning
        order_moves(&mut moves, known_move);

        // choose the best variation
        let mut best_eval = MIN_EVAL;
        let mut best_move = None;

        // go deeper for each move
        for mov in moves {
            position = position.push(mov);
            let (eval, _, stack) = self.negamax(position, depth - 1, -beta, -alpha, -sign);
            let eval = -eval;

            position = *stack.pop();

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

        // update transposition table
        {
            let mut table = self.table.lock().unwrap();
            table.insert(position.hash, Transposition {
                eval: best_eval,
                eval_depth: depth,
                best_move,
            });
        }

        (best_eval, best_move, position)
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

fn order_moves(moves: &mut Vec<Move>, pv: Option<Move>) {

    let mut index = None;
    if let Some(mov) = pv {
        index = moves.iter().position(|m| mov == *m);
    }

    // put known best move at the top
    if let Some(i) = index {
        moves.swap(0, i);
    }
}
