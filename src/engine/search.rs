use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use rand::prelude::*;

use crate::engine::{EngineState, eval, gen, GoParams};
use crate::engine::board::{Board, Color};
use crate::engine::mov::Move;

// min/max values that won't overflow on negation
const MIN_EVAL: i32 = -std::i32::MAX;
const MAX_EVAL: i32 = -MIN_EVAL;

// compile-time configuration of search algorithm
fn new_searcher() -> impl Searcher { NegamaxAb::new() }

pub fn search(state: Arc<Mutex<EngineState>>, p: GoParams) {
    let root_position = state.lock().unwrap().position.clone();

    let mut position = root_position;
    for mov in p.search_moves {
        position = position.update(mov);
    }

    let mut searcher = new_searcher();
    let mov = searcher.search(position);

    let stats = searcher.get_stats();
    eprintln!("---- {}", mov.uci());
    eprintln!("nodes_visited: {}", stats.nodes_visited);
    eprintln!("time_elapsed (ms): {}", stats.time_elapsed.as_millis());
    eprintln!("max_depth: {}", stats.max_depth);
    eprintln!("nps: {}", stats.nps());

    (state.lock().unwrap().callbacks.best_move_fn)(&mov);
}

#[derive(Copy, Clone)]
struct SearchStats {
    nodes_visited: u64,
    time_elapsed: Duration,
    max_depth: i32,
}

impl SearchStats {
    pub fn new() -> Self {
        Self {
            nodes_visited: 0,
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
    fn search(&mut self, position: Board) -> Move;
    fn get_stats(&self) -> SearchStats;
}

struct Negamax {
    stats: SearchStats,
}

impl Searcher for Negamax {
    fn search(&mut self, position: Board) -> Move {
        let start = Instant::now();

        let sign = if position.turn == Color::WHITE { 1 } else { -1 };
        let (_, mov) = self.negamax(position, Self::DEPTH, sign);

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
    pub fn new() -> Self {
        Self {
            stats: SearchStats::new()
        }
    }

    fn negamax(&mut self, position: Board, depth: i32, sign: i32) -> (i32, Option<Move>) {

        // track search statistics
        self.stats.nodes_visited += 1;
        self.stats.max_depth = self.stats.max_depth.max(Self::DEPTH - depth);

        // generate moves first to test for checkmate/stalemate
        let moves = gen::gen_moves(&position);

        // no available moves? the game is over
        if moves.is_empty() {

            let position = if position.turn == Color::WHITE {
                position
            } else {
                position.mirror()
            };

            let is_mate = !0 != gen::get_check_restriction(&position);
            return (if is_mate { MIN_EVAL + position.fullmove_number as i32} else { 0 }, None);
        }

        // fifty-move rule
        if position.halfmove_clock >= 50 {
            return (0, None);
        }

        // three-fold repetition
        if position.halfmove_clock > 4 {
            let hash = position.hash;
            let hist = &position.history;
            let limit = position.halfmove_clock.min(hist.len() as u16);

            let mut n = 0;
            for i in 0..limit {
                let ply = hist.len() - i as usize - 1;
                if hash == hist[ply] {
                    n += 1;
                    if n >= 2 { return (0, None); }
                }
            }
        }

        // have we reached max depth?
        if depth <= 0 {
            return (sign * eval::evaluate(&position), None);
        }

        // choose the best variation
        let mut best_eval = MIN_EVAL;
        let mut best_move = None;

        // go deeper for each move
        for mov in moves {
            let moved = position.update(mov);
            let (eval, _) = self.negamax(moved, depth - 1, -sign);
            let eval = -eval;

            if best_move.is_none() || eval > best_eval {
                best_eval = eval;
                best_move = Some(mov);
            }
        }

        (best_eval, best_move)
    }
}

struct NegamaxAb {
    stats: SearchStats,
    rng: rand::rngs::ThreadRng,
}

impl Searcher for NegamaxAb {
    fn search(&mut self, position: Board) -> Move {

        // re-initialize thread local random
        self.rng = rand::thread_rng();

        // begin timing the search routine
        let start = Instant::now();

        let sign = if position.turn == Color::WHITE { 1 } else { -1 };
        let (_, mov) = self.negamax(position, Self::DEPTH, MIN_EVAL, MAX_EVAL, sign);

        self.stats.time_elapsed = start.elapsed();

        mov.expect("no move")
    }

    fn get_stats(&self) -> SearchStats {
        self.stats
    }
}

impl NegamaxAb {
    const DEPTH: i32 = 6;

    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            stats: SearchStats::new(),
            rng: rand::thread_rng(),
        }
    }

    fn negamax(&mut self, position: Board, depth: i32, mut alpha: i32, beta: i32, sign: i32) -> (i32, Option<Move>) {

        // track search statistics
        self.stats.nodes_visited += 1;
        self.stats.max_depth = self.stats.max_depth.max(Self::DEPTH - depth);

        // generate moves first to test for checkmate/stalemate
        let mut moves = gen::gen_moves(&position);

        // no available moves? the game is over
        if moves.is_empty() {

            let position = if position.turn == Color::WHITE {
                position
            } else {
                position.mirror()
            };

            let is_mate = !0 != gen::get_check_restriction(&position);
            return (if is_mate { MIN_EVAL + position.fullmove_number as i32} else { 0 }, None);
        }

        // fifty-move rule
        if position.halfmove_clock >= 50 {
            return (0, None);
        }

        // three-fold repetition
        if position.halfmove_clock > 4 {
            let hash = position.hash;
            let hist = &position.history;
            let limit = position.halfmove_clock.min(hist.len() as u16);

            let mut n = 0;
            for i in 0..limit {
                let ply = hist.len() - i as usize - 1;
                if hash == hist[ply] {
                    n += 1;
                    if n >= 2 { return (0, None); }
                }
            }
        }

        // have we reached max depth?
        if depth <= 0 {
            return (sign * eval::evaluate(&position), None);
        }

        // shuffle to improve alpha-beta pruning
        moves.shuffle(&mut self.rng);

        // choose the best variation
        let mut best_eval = MIN_EVAL;
        let mut best_move = None;

        // go deeper for each move
        for mov in moves {
            let moved = position.update(mov);
            let (eval, _) = self.negamax(moved, depth - 1, -beta, -alpha, -sign);
            let eval = -eval;

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

        (best_eval, best_move)
    }
}
