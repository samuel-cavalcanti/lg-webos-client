use crate::lg_command::{CommandRequest, LGCommandRequest, REQUEST_TYPE};

pub struct Stop;

impl LGCommandRequest for Stop {
    fn to_command_request(&self) -> CommandRequest {
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://media.controls/stop"),
            payload: None,
        }
    }
}
