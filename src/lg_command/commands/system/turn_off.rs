use crate::lg_command::REQUEST_TYPE;
use crate::lg_command::commands::{CommandRequest, LGCommandRequest};

pub struct TurnOff;

impl LGCommandRequest for TurnOff {
    fn to_command_request(&self, id: u8) -> CommandRequest {
        CommandRequest {
            id,
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://system/turnOff"),
            payload: None,
        }
    }
}
