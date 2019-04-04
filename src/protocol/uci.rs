use std::collections::HashMap;
use std::iter::Peekable;
use std::process;
use std::slice::Iter;

use crate::engine::*;
use crate::engine::mov::Move;
use crate::protocol::Protocol;

pub struct Uci {
    engine: Engine,
}


impl Uci {
    pub fn new() -> Uci {
        Uci {
            engine: Engine::new(Callbacks {
                log_fn: log::info,
                best_move_fn: uci_out::bestmove,
            }),
        }
    }

    fn uci(&mut self) {
        uci_out::id_name(env!("CARGO_PKG_NAME"));
        uci_out::id_author(env!("CARGO_PKG_AUTHORS"));
        uci_out::option(HashMap::new());
        uci_out::uciok();
    }

    fn debug(&mut self, args: Vec<&str>) {
        let arg: &str = *args.first().unwrap_or(&"off");
        let debug = "on" == arg;
        log::log(&format!("debug is {}", debug));
        let log_fn = if debug { log::debug } else { log::info };
        self.engine.update_log_fn(log_fn);
    }

    fn isready(&self) {
        uci_out::readyok();
    }

    fn setoption(&self, _args: Vec<&str>) {

        // no options are supported; just return
    }

    fn register(&self, _args: Vec<&str>) {

        // this engine does not need to be registered. this is a weird thing to put in a standard
        // protocol, but okay we're not using it so nbd
    }

    fn ucinewgame(&mut self) {
        self.engine.reset();
    }

    fn position(&mut self, args: Vec<&str>) {

        // join "args" back into a FEN string
        let fen = args.join(" ");

        if fen == "startpos" {
            self.engine.set_start_pos();
        } else {
            self.engine.set_position(&fen);
        }
    }

    fn go(&mut self, args: Vec<&str>) {
        let mut search_moves: Vec<Move> = Vec::new();
        let mut ponder = false;
        let mut wtime = 0;
        let mut btime = 0;
        let mut winc = 0;
        let mut binc = 0;
        let mut movestogo = 0;
        let mut depth = 0;
        let mut nodes = 0;
        let mut mate = 0;
        let mut movetime = 0;
        let mut infinite = false;

        let mut iter = args.iter().peekable();
        let mut arg = iter.next();
        while arg.is_some() {
            match *arg.unwrap() {
                "searchmoves" => search_moves = parse_moves(&mut iter),
                "ponder" => ponder = true,
                "wtime" => wtime = iter.next().unwrap_or(&"0").parse().unwrap(),
                "btime" => btime = iter.next().unwrap_or(&"0").parse().unwrap(),
                "winc" => winc = iter.next().unwrap_or(&"0").parse().unwrap(),
                "binc" => binc = iter.next().unwrap_or(&"0").parse().unwrap(),
                "movestogo" => movestogo = iter.next().unwrap_or(&"0").parse().unwrap(),
                "depth" => depth = iter.next().unwrap_or(&"0").parse().unwrap(),
                "nodes" => nodes = iter.next().unwrap_or(&"0").parse().unwrap(),
                "mate" => mate = iter.next().unwrap_or(&"0").parse().unwrap(),
                "movetime" => movetime = iter.next().unwrap_or(&"0").parse().unwrap(),
                "infinite" => infinite = true,
                _ => {}
            }
            arg = iter.next();
        }

        self.engine.go(GoParams {
            search_moves,
            ponder,
            wtime,
            btime,
            winc,
            binc,
            movestogo,
            depth,
            nodes,
            mate,
            movetime,
            infinite,
        })
    }

    fn stop(&self) {
        self.engine.stop();
    }

    fn ponderhit(&self) {
        unimplemented!();
    }

    fn quit(&self) {
        process::exit(1);
    }
}


impl Protocol for Uci {
    fn send_command(&mut self, command_args: String) {
        let mut tokens = command_args.split_whitespace();
        let command = tokens.next().unwrap_or("");
        let args = tokens.collect::<Vec<&str>>();

        match command {
            "uci" => self.uci(),
            "debug" => self.debug(args),
            "isready" => self.isready(),
            "setoption" => self.setoption(args),
            "register" => self.register(args),
            "ucinewgame" => self.ucinewgame(),
            "position" => self.position(args),
            "go" => self.go(args),
            "stop" => self.stop(),
            "ponderhit" => self.ponderhit(),
            "quit" => self.quit(),
            _ => (),
        }
    }
}


fn parse_moves(args: &mut Peekable<Iter<&str>>) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    let mut mov: Option<Move> = Move::parse(args.peek().unwrap_or(&&""));
    while mov.is_some() {
        moves.push(mov.unwrap());
        args.next();
        mov = Move::parse(args.peek().unwrap_or(&&""))
    }

    return moves;
}


mod log {
    use crate::engine::LogLevel;
    use crate::protocol::uci::uci_out;

    pub fn info(level: LogLevel, msg: &str) {
        let enabled = match level {
            LogLevel::INFO => true,
            LogLevel::DEBUG => false,
        };

        if enabled {
            log(msg);
        }
    }

    pub fn debug(level: LogLevel, msg: &str) {
        let enabled = match level {
            LogLevel::INFO => true,
            LogLevel::DEBUG => true,
        };

        if enabled {
            log(msg);
        }
    }

    pub fn log(msg: &str) {
        uci_out::info_string(msg);
    }
}


mod uci_out {
    use std::collections::HashMap;
    use crate::engine::mov::Move;

    pub fn id_name(name: &str) {
        println!("id name {}", name);
    }

    pub fn id_author(author: &str) {
        println!("id author {}", author)
    }

    pub fn uciok() {
        println!("uciok");
    }

    pub fn readyok() {
        println!("readyok");
    }

    pub fn bestmove(mov: &Move) {

        let mut repr = String::new();
        repr.push_str(mov.from.symbol);
        repr.push_str(mov.to.symbol);

        if mov.promotion.is_some() {
            repr.push_str(mov.promotion.unwrap().symbol);
        }

        println!("bestmove {}", repr);
    }

    pub fn info_string(msg: &str) {
        println!("info string {}", msg)
    }

    pub fn option(_options: HashMap<&str, &str>) {
        // TODO implement option
        println!("option");
    }
}
