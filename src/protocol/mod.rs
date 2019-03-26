pub mod uci;

pub trait Protocol {
    fn send_command(&self, command_args: String);
}
