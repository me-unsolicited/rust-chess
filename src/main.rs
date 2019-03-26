use std::{io, process};

use colored::*;
use crate::protocol::*;


mod protocol;


fn main() {
    println!("{} {}", env!("CARGO_PKG_NAME").red().bold(), env!("CARGO_PKG_VERSION"));

    // first command gives the protocol name
    let protocol_command: String = read_line();
    let protocol: Box<Protocol> = match protocol_command.as_ref() {
        "uci" => Box::from(uci::Uci::new()),
        _ => Box::from(unknown_protocol()),
    };

    // follow the protocol from here on
    protocol.send_command(protocol_command);
    loop {
        protocol.send_command(read_line());
    }
}

fn unknown_protocol() -> impl Protocol {
    struct Unknown {}

    impl Protocol for Unknown {
        fn send_command(&self, command_args: String) {
            println!("unknown protocol: {}", command_args);
            process::exit(1);
        }
    }

    return Unknown {};
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("no input");

    return line.trim().to_string();
}
