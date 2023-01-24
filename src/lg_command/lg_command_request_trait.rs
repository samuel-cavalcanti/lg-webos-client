use crate::lg_command::CommandRequest;

pub trait LGCommandRequest: Send {
    fn to_command_request(&self) -> CommandRequest;
}

impl<T: LGCommandRequest + ?Sized> LGCommandRequest for Box<T> {
    fn to_command_request(&self) -> CommandRequest {
        (**self).to_command_request()
    }
}
