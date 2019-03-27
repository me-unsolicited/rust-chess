use crate::engine::board::Board;
use crate::engine::mov::Move;

pub mod mov;
mod board;
mod square;

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

    pub fn go(
        &self,
        search_moves: Vec<Move>,
        ponder: bool,
        wtime: i32,
        btime: i32,
        winc: i32,
        binc: i32,
        movestogo: i32,
        depth: i32,
        nodes: i32,
        mate: i32,
        movetime: i32,
        infinite: bool) {

        // TODO actually go
        println!("info string search_moves {:#?} ponder {} wtime {} btime {} winc {} binc {} \
          movestogo {} depth {} nodes {} mate {} movetime {} infinite {}",
                 search_moves, ponder, wtime, btime, winc, binc,
                 movestogo, depth, nodes, mate, movetime, infinite)
    }
}