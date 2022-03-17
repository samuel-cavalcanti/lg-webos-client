use crate::lg_command::{CommandRequest, LGCommandRequest, REQUEST_TYPE};

pub struct FastForward;


impl LGCommandRequest for FastForward {
    fn to_command_request(&self) -> CommandRequest {
        CommandRequest {
  
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://media.controls/fastForward"),
            payload: None,
        }
    }
}