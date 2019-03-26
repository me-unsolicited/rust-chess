use crate::engine::board::Board;

mod board;

pub struct Engine {
    position: Board
}


impl Engine {
    pub fn new() -> Engine {
        Engine {
            position: Board::start_pos(),
        }
    }

    pub fn reset(&mut self) {
        self.position = Board::start_pos();
    }
}