use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use crate::engine::board::Board;
use crate::engine::mov::Move;

pub mod mov;
mod bb;
mod board;
mod eval;
mod gen;
mod piece;
mod search;
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
    state: Arc<Mutex<EngineState>>,
    threads: Vec<JoinHandle<()>>,
}


pub struct EngineState {
    callbacks: Callbacks,
    position: Board,
}


pub struct Callbacks {
    pub log_fn: fn(LogLevel, &str),
    pub best_move_fn: fn(&Move),
}


impl Engine {
    pub fn new(callbacks: Callbacks) -> Engine {
        let state = EngineState {
            callbacks,
            position: Board::start_pos(),
        };

        Engine {
            state: Arc::new(Mutex::new(state)),
            threads: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.set_start_pos(Vec::new());
    }

    pub fn set_start_pos(&mut self, moves: Vec<Move>) {
        let mut position = Board::start_pos();
        for mov in moves {
            position = position.update(mov);
        }

        self.state.lock().unwrap().position = position;
    }

    pub fn set_position(&mut self, fen: &str, moves: Vec<Move>) {
        let mut position = Board::new(fen);
        for mov in moves {
            position = position.update(mov);
        }

        self.state.lock().unwrap().position = position;
    }

    pub fn go(&mut self, p: GoParams) {
        let state = Arc::clone(&self.state);
        let handle = thread::spawn(move || {
            search::search(state, p);
        });

        self.threads.push(handle);
    }

    pub fn stop(&self) {
        (self.state.lock().unwrap().callbacks.log_fn)(LogLevel::DEBUG, "stopping");
        // TODO stop searching
    }

    pub fn update_log_fn(&mut self, log_fn: fn(LogLevel, &str)) {
        self.state.lock().unwrap().callbacks.log_fn = log_fn;
    }
}