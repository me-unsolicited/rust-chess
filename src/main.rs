use std::{io, process};

use colored::*;

mod uci;


fn main() {
    println!("{} {}", env!("CARGO_PKG_NAME").red().bold(), env!("CARGO_PKG_VERSION"));

    let handle: fn(String);
    let protocol: String = read_line();
    match protocol.as_ref() {
        "uci" => handle = uci::send_command,
        _ => handle = handle_unknown,
    }

    handle(protocol);
    loop {
        handle(read_line());
    }
}

fn handle_unknown(command: String) {
    println!("unknown protocol: {}", command);
    process::exit(1);
}

fn read_line() -> String {
    let mut input: String = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => return input.trim().to_string(),
        Err(error) => panic!("input error: {}", error),
    }
}
