use std::sync::{Arc, Mutex};
use crate::engine::{EngineState, GoParams, gen, eval};
use crate::engine::board::Color;

pub fn search(state: Arc<Mutex<EngineState>>, p: GoParams) {
    let root_position = state.lock().unwrap().position;

    let mut position = root_position;
    for mov in p.search_moves {
        position = position.update(mov);
    }

    let moves = gen::gen_moves(&position);

    // pick the best evaluated move
    let sign = if position.turn == Color::WHITE { 1 } else { -1 };
    let mut best_eval = std::i32::MIN;
    let mut best_move = None;
    for mov in moves.iter() {
        let eval = sign * eval::evaluate(&position.update(*mov));
        if eval > best_eval {
            best_eval = eval;
            best_move = Some(mov);
        }
    }

    (state.lock().unwrap().callbacks.best_move_fn)(best_move.expect("no move"));
}
