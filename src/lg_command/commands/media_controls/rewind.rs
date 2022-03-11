use crate::lg_command::{CommandRequest, LGCommandRequest, REQUEST_TYPE};

pub struct Rewind;


impl LGCommandRequest for Rewind {
    fn to_command_request(&self, id: u8) -> CommandRequest {
        CommandRequest {
            id,
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://media.controls/rewind"),
            payload: None,
        }
    }
}