use serde_json::json;

use crate::lg_command::REQUEST_TYPE;
use crate::lg_command::{CommandRequest, LGCommandRequest};

/// set's the Volume, The max Value is **100**.
pub struct SetVolume {
    pub volume: i8,
}

impl LGCommandRequest for SetVolume {
    fn to_command_request(&self) -> CommandRequest {
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://audio/setVolume"),
            payload: Some(json!({ "volume": self.volume })),
        }
    }
}
