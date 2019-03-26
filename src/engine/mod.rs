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
        self.set_start_pos();
    }

    pub fn set_start_pos(&mut self) {
        self.position = Board::start_pos();
    }

    pub fn set_position(&mut self, fen: &str) {
        self.position = Board::new(fen);
    }
}