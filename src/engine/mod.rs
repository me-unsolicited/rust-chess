use crate::engine::board::Board;
use crate::engine::mov::Move;

pub mod mov;
mod board;
mod square;

pub enum LogLevel {
    INFO,
    DEBUG,
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

    pub fn go(
        &mut self,
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
        self.log(LogLevel::INFO, &format!(
            "search_moves {:#?} ponder {} wtime {} btime {} winc {} binc {} \
             movestogo {} depth {} nodes {} mate {} movetime {} infinite {}",
            search_moves, ponder, wtime, btime, winc, binc,
            movestogo, depth, nodes, mate, movetime, infinite));
    }

    pub fn stop(&self) {

        self.log(LogLevel::DEBUG, "stopping");
        // TODO stop searching
    }

    fn log(&self, level: LogLevel, msg: &str) {
        (self.log_fn)(level, msg);
    }
}