use std::cell::Cell;

use crate::engine::board::Board;

mod board;

pub struct Engine {
    position: Cell<Board>
}


impl Engine {
    pub fn new() -> Engine {
        Engine {
            position: Cell::new(Board::start_pos()),
        }
    }
}