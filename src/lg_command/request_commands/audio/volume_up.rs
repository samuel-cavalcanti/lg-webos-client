use crate::lg_command::{CommandRequest, LGCommandRequest, REQUEST_TYPE};

/// Increase the Volume, like **volume +=1**
pub struct VolumeUP;

impl LGCommandRequest for VolumeUP {
    fn to_command_request(&self) -> CommandRequest {
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://audio/volumeUp"),
            payload: None,
        }
    }
}
