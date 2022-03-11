use crate::{lg_command::{CommandRequest, LGCommandRequest}};
use crate::lg_command::REQUEST_TYPE;

pub struct AudioStatus;

impl LGCommandRequest for AudioStatus {
    fn to_command_request(&self, id: u8) -> CommandRequest {
        CommandRequest {
            id,
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://audio/getStatus"),
            payload: None,
        }
    }
}
