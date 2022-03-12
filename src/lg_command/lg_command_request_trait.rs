use crate::lg_command::CommandRequest;

pub trait LGCommandRequest {
    fn to_command_request(&self,id: u8) -> CommandRequest;
}