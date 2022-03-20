use crate::lg_command::REQUEST_TYPE;
use crate::lg_command::{CommandRequest, LGCommandRequest};

pub struct GetVolume;

impl LGCommandRequest for GetVolume {
    fn to_command_request(&self) -> CommandRequest {
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://audio/getVolume"),
            payload: None,
        }
    }
}
