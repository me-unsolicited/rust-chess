use crate::engine::board::Board;
use crate::engine::mov::Move;
use crate::engine::square::Square;

pub mod mov;
mod board;
mod piece;
mod square;

pub enum LogLevel {
    INFO,
    DEBUG,
}


pub struct GoParams {
    pub search_moves: Vec<Move>,
    pub ponder: bool,
    pub wtime: i32,
    pub btime: i32,
    pub winc: i32,
    pub binc: i32,
    pub movestogo: i32,
    pub depth: i32,
    pub nodes: i32,
    pub mate: i32,
    pub movetime: i32,
    pub infinite: bool,
}


pub struct Engine {
    pub callbacks: Callbacks,
    position: Board,
}


pub struct Callbacks {
    pub log_fn: fn(LogLevel, &str),
    pub best_move_fn: fn(Move),
}


impl Engine {
    pub fn new(callbacks: Callbacks) -> Engine {
        Engine {
            callbacks,
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

    pub fn go(&mut self, p: GoParams) {

        // TODO actually go
        self.log(LogLevel::INFO, &format!(
            "search_moves {:#?} ponder {} wtime {} btime {} winc {} binc {} \
             movestogo {} depth {} nodes {} mate {} movetime {} infinite {}",
            p.search_moves, p.ponder, p.wtime, p.btime, p.winc, p.binc,
            p.movestogo, p.depth, p.nodes, p.mate, p.movetime, p.infinite));

        let mov = Move {
            from: &Square::E2,
            to: &Square::E4,
            promotion: None,
        };

        self.send_move(mov);
    }

    pub fn stop(&self) {
        self.log(LogLevel::DEBUG, "stopping");
        // TODO stop searching
    }

    fn send_move(&self, mov: Move) {
        (self.callbacks.best_move_fn)(mov);
    }

    fn log(&self, level: LogLevel, msg: &str) {
        (self.callbacks.log_fn)(level, msg);
    }
}