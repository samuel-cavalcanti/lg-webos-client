use serde_json::json;

use crate::{lg_command::{CommandRequest, LGCommandRequest}};
use crate::lg_command::REQUEST_TYPE;

pub struct SetMute {
    pub mute: bool,
}

impl LGCommandRequest for SetMute {
    fn to_command_request(&self, id: u8) -> CommandRequest {
        CommandRequest {
            id,
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://audio/setMute"),
            payload: Some(json!({ "mute": self.mute })),
        }
    }
}
