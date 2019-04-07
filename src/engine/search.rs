use std::sync::{Arc, Mutex};

use crate::engine::{EngineState, eval, gen, GoParams};
use crate::engine::board::{Board, Color};
use crate::engine::mov::Move;

// min value that won't overflow
const MIN_EVAL: i32 = -std::i32::MAX;

const DEPTH: i32 = 4;

pub fn search(state: Arc<Mutex<EngineState>>, p: GoParams) {
    let root_position = state.lock().unwrap().position;

    let mut position = root_position;
    for mov in p.search_moves {
        position = position.update(mov);
    }

    let sign = if position.turn == Color::WHITE { 1 } else { -1 };
    let (_, mov) = negamax(position, DEPTH, sign);

    (state.lock().unwrap().callbacks.best_move_fn)(&mov.expect("no move"));
}

fn negamax(position: Board, depth: i32, sign: i32) -> (i32, Option<Move>) {

    // have we reached max depth?
    if depth <= 0 {
        return (sign * eval::evaluate(&position), None);
    }

    let moves = gen::gen_moves(&position);

    // no available moves? the game is over
    if moves.is_empty() {
        let perspective = if position.turn == Color::WHITE { position } else { position.mirror() };
        let is_mate = 0 != gen::get_check_restriction(&perspective);
        return (sign * if is_mate { MIN_EVAL } else { 0 }, None);
    }

    // choose the best variation
    let mut best_eval = MIN_EVAL;
    let mut best_move = None;

    // go deeper for each move
    for mov in moves {
        let moved = position.update(mov);
        let (eval, _) = negamax(moved, depth - 1, -sign);
        let eval = -eval;

        if eval >= best_eval {
            best_eval = eval;
            best_move = Some(mov);
        }
    }

    (best_eval, best_move)
}
