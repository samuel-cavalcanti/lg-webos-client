use crate::lg_command::CommandRequest;

pub trait LGCommandRequest: Send {
    fn to_command_request(&self) -> CommandRequest;
}
