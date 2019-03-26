use std::cell::Cell;
use std::process;

use crate::protocol::Protocol;

pub struct Uci {
    debug: Cell<bool>,
}


impl Uci {
    pub fn new() -> Uci {
        Uci {
            debug: Cell::new(false),
        }
    }

    fn uci(&self) {
        println!("id name {}", env!("CARGO_PKG_NAME"));
        println!("id author {}", env!("CARGO_PKG_AUTHORS"));
        println!("option");
        println!("uciok");
    }

    fn debug(&self, args: Vec<&str>) {
        let arg: &str = *args.first().unwrap_or(&"off");
        self.debug.set("on" == arg);
        print_debug(format!("debug is {}", self.debug.get()))
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

    fn ucinewgame(&self) {
        unimplemented!();
    }

    fn position(&self, _args: Vec<&str>) {
        unimplemented!();
    }

    fn go(&self, _args: Vec<&str>) {
        unimplemented!();
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
    fn send_command(&self, command_args: String) {
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
