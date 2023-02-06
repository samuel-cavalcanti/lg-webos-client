use crate::lg_command::request_commands::{CommandRequest, LGCommandRequest};
use crate::lg_command::REQUEST_TYPE;

pub struct TurnOffTV;

impl LGCommandRequest for TurnOffTV {
    fn to_command_request(&self) -> CommandRequest {
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://system/turnOff"),
            payload: None,
        }
    }
}
