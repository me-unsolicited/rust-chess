use std::process;

pub fn send_command(command_args: String) {

    let mut tokens = command_args.split_whitespace();
    let command = tokens.next().unwrap_or("");
    let args = tokens.collect::<Vec<&str>>();

    match command {
        "uci" => uci(),
        "debug" => debug(),
        "isready" => isready(),
        "setoption" => setoption(args),
        "register" => register(args),
        "ucinewgame" => ucinewgame(),
        "position" => position(args),
        "go" => go(args),
        "stop" => stop(),
        "ponderhit" => ponderhit(),
        "quit" => quit(),
        _ => (),
    }
}

fn uci() {
    println!("id name {}", env!("CARGO_PKG_NAME"));
    println!("id author {}", env!("CARGO_PKG_AUTHORS"));
    println!("option");
    println!("uciok");
}

fn debug() {
    unimplemented!();
}

fn isready() {
    unimplemented!();
}

fn setoption(_args: Vec<&str>) {
    unimplemented!();
}

fn register(_args: Vec<&str>) {
    unimplemented!();
}

fn ucinewgame() {
    unimplemented!();
}

fn position(_args: Vec<&str>) {
    unimplemented!();
}

fn go(_args: Vec<&str>) {
    unimplemented!();
}

fn stop() {
    unimplemented!();
}

fn ponderhit() {
    unimplemented!();
}

fn quit() {
    process::exit(1);
}
