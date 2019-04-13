use std::collections::HashMap;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

use crate::engine::board::Board;
use crate::engine::mov::Move;
use crate::engine::search::SearchStats;

pub mod mov;
mod bb;
mod board;
mod eval;
mod gen;
mod hash;
mod piece;
mod search;
mod square;

#[allow(dead_code)]
pub enum LogLevel {
    INFO,
    DEBUG,
}

#[derive(Clone)]
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
    num_cpus: usize,
}


pub struct EngineState {
    callbacks: Callbacks,
    position: Board,
    table: Arc<Mutex<HashMap<u64, Transposition>>>,
}


pub struct Callbacks {
    pub log_fn: fn(LogLevel, &str),
    pub best_move_fn: fn(&Move),
}


#[derive(Clone)]
pub struct Transposition {
    eval: Option<i32>,
    eval_depth: Option<i32>,
    q_eval: Option<i32>,
    q_depth: Option<i32>,
    best_move: Option<Move>,
}


impl Engine {
    pub fn new(callbacks: Callbacks) -> Engine {
        let state = EngineState {
            callbacks,
            position: Board::start_pos(),
            table: Arc::new(Mutex::new(HashMap::new())),
        };

        Engine {
            state: Arc::new(Mutex::new(state)),
            num_cpus: num_cpus::get(),
        }
    }

    pub fn reset(&mut self) {
        self.set_start_pos(Vec::new());
        self.state.lock().unwrap().table.lock().unwrap().clear();
    }

    pub fn set_start_pos(&mut self, moves: Vec<Move>) {
        let mut position = Board::start_pos();
        for mov in moves {
            position.push(mov);
        }

        self.state.lock().unwrap().position = position;
    }

    pub fn set_position(&mut self, fen: &str, moves: Vec<Move>) {
        let mut position = Board::new(fen);
        for mov in moves {
            position.push(mov);
        }

        self.state.lock().unwrap().position = position;
    }

    pub fn go(&mut self, p: GoParams) {

        eprintln!();
        eprintln!("---- searching with {} threads", self.num_cpus);

        let mut threads = Vec::new();
        let (tx_stats, rx_stats) = mpsc::channel();

        // start search threads
        for i in 0..self.num_cpus {
            let state = Arc::clone(&self.state);
            let params = p.clone();
            let tx_stats_i = tx_stats.clone();
            let handle = thread::spawn(move || {
                search::search(state, params, i, tx_stats_i.clone());
            });

            threads.push(handle);
        }

        // one more thread to wait for search to end and determine best move
        let state = Arc::clone(&self.state);
        thread::spawn(move || {

            // wait for all searches to end
            while !threads.is_empty() {
                if let Err(_) = threads.pop().unwrap().join() {
                    panic!("failed to join thread")
                }
            }

            // get the PV from the transposition table
            let state = state.lock().unwrap();
            let table = state.table.lock().unwrap();
            let transposition = table.get(&state.position.hash).expect("no root transposition");
            let mov = transposition.best_move.expect("no move");

            // gather search statistics
            let mut stats = SearchStats::new();
            for thread_stats in rx_stats.iter() {
                stats.combine(&thread_stats);
            }

            // report statistics to std error
            eprintln!("---- {}", mov.uci());
            eprintln!("nodes_visited: {}", stats.nodes_visited);
            eprintln!("tt_hits: {}", stats.tt_hits);
            eprintln!("tt_waste: {}", stats.tt_waste);
            eprintln!("time_elapsed (ms): {}", stats.time_elapsed.as_millis());
            eprintln!("max_depth: {}", stats.max_depth);
            eprintln!("nps: {}", stats.nps());

            (state.callbacks.best_move_fn)(&mov);
        });
    }

    pub fn stop(&self) {
        (self.state.lock().unwrap().callbacks.log_fn)(LogLevel::DEBUG, "stopping");
        // TODO stop searching
    }

    pub fn update_log_fn(&mut self, log_fn: fn(LogLevel, &str)) {
        self.state.lock().unwrap().callbacks.log_fn = log_fn;
    }
}
