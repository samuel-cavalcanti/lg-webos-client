use crate::lg_command::{CommandRequest, LGCommandRequest, REQUEST_TYPE};

pub struct Pause;


impl LGCommandRequest for Pause {
    fn to_command_request(&self, id: u8) -> CommandRequest {
        CommandRequest {
            id,
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://media.controls/pause"),
            payload: None,
        }
    }
}