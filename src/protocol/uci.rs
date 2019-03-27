use std::process;
use std::slice::Iter;

use crate::engine::*;
use crate::engine::mov::Move;
use crate::protocol::Protocol;
use std::iter::Peekable;

pub struct Uci {
    engine: Engine,
    debug: bool,
}


impl Uci {
    pub fn new() -> Uci {
        Uci {
            engine: Engine::new(),
            debug: false,
        }
    }

    fn uci(&mut self) {
        println!("id name {}", env!("CARGO_PKG_NAME"));
        println!("id author {}", env!("CARGO_PKG_AUTHORS"));
        println!("option");
        println!("uciok");
    }

    fn debug(&mut self, args: Vec<&str>) {
        let arg: &str = *args.first().unwrap_or(&"off");
        self.debug = "on" == arg;
        print_debug(format!("debug is {}", self.debug))
    }

    fn isready(&self) {
        println!("readyok");
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
        let arg: &str = *args.first().expect("expected fen");
        if arg == "startpos" {
            self.engine.set_start_pos();
        } else {
            self.engine.set_position(arg);
        }
    }

    fn go(&self, args: Vec<&str>) {
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
                _ => {},
            }
            arg = iter.next();
        }

        self.engine.go(search_moves, ponder, wtime, btime, winc, binc, movestogo, depth, nodes,
                       mate, movetime, infinite)
    }

    fn stop(&self) {
        unimplemented!();
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


fn print_debug(msg: String) {
    println!("info string {}", msg)
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
