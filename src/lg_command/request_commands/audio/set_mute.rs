use serde_json::json;

use crate::lg_command::REQUEST_TYPE;
use crate::lg_command::{CommandRequest, LGCommandRequest};

/// Mute the TV fi the mute is true, and enable the audio if mute is false
pub struct SetMute {
    pub mute: bool,
}

impl LGCommandRequest for SetMute {
    fn to_command_request(&self) -> CommandRequest {
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://audio/setMute"),
            payload: Some(json!({ "mute": self.mute })),
        }
    }
}
