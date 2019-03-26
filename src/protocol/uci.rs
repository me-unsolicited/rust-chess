use std::process;
use crate::protocol::Protocol;

pub struct Uci {
    // TODO uci state
}


impl Uci {
    pub fn new() -> Uci {
        Uci {}
    }

    fn uci(&self) {
        println!("id name {}", env!("CARGO_PKG_NAME"));
        println!("id author {}", env!("CARGO_PKG_AUTHORS"));
        println!("option");
        println!("uciok");
    }

    fn debug(&self, _args: Vec<&str>) {
        unimplemented!();
    }

    fn isready(&self) {
        unimplemented!();
    }

    fn setoption(&self, _args: Vec<&str>) {
        unimplemented!();
    }

    fn register(&self, _args: Vec<&str>) {
        unimplemented!();
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