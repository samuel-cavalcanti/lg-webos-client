use crate::lg_command::{CommandRequest, LGCommandRequest, REQUEST_TYPE};

pub struct VolumeDown;

impl LGCommandRequest for VolumeDown {
    fn to_command_request(&self) -> CommandRequest {
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://audio/volumeDown"),
            payload: None,
        }
    }
}
