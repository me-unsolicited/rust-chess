use crate::engine::board::Board;
use crate::engine::mov::Move;

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
    log_fn: fn(LogLevel, &str),
    position: Board,
}


impl Engine {
    pub fn new(log_fn: fn(LogLevel, &str)) -> Engine {
        Engine {
            log_fn,
            position: Board::start_pos(),
        }
    }

    pub fn set_log_fn(&mut self, log_fn: fn(LogLevel, &str)) {
        self.log_fn = log_fn;
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
    }

    pub fn stop(&self) {
        self.log(LogLevel::DEBUG, "stopping");
        // TODO stop searching
    }

    fn log(&self, level: LogLevel, msg: &str) {
        (self.log_fn)(level, msg);
    }
}