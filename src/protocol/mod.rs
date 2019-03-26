pub mod uci;

pub trait Protocol {
    fn send_command(&mut self, command_args: String);
}
