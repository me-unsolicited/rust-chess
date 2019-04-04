use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use rand::prelude::*;

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
        self.set_start_pos();
    }

    pub fn set_start_pos(&mut self) {
        self.state.lock().unwrap().position = Board::start_pos();
    }

    pub fn set_position(&mut self, fen: &str) {
        self.state.lock().unwrap().position = Board::new(fen);
    }

    pub fn go(&mut self, p: GoParams) {
        let state = Arc::clone(&self.state);
        let handle = thread::spawn(move || {
            Self::search(state, p);
        });

        self.threads.push(handle);
    }

    fn search(state: Arc<Mutex<EngineState>>, _p: GoParams) {
        let position = &state.lock().unwrap().position;
        let moves = position.gen_moves();

        // galaxy brain search algorithm: pick a random move
        let index = rand::thread_rng().gen_range(0, moves.len());
        let mov = moves.get(index);

        (state.lock().unwrap().callbacks.best_move_fn)(mov.unwrap());
    }

    pub fn stop(&self) {
        (self.state.lock().unwrap().callbacks.log_fn)(LogLevel::DEBUG, "stopping");
        // TODO stop searching
    }

    pub fn update_log_fn(&mut self, log_fn: fn(LogLevel, &str)) {
        self.state.lock().unwrap().callbacks.log_fn = log_fn;
    }
}