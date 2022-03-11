use serde_json::json;

use crate::{lg_command::{CommandRequest, LGCommandRequest}};
use crate::lg_command::REQUEST_TYPE;

pub struct SetVolume {
   pub volume: i8,
}

impl LGCommandRequest for SetVolume {
    fn to_command_request(&self, id: u8) -> CommandRequest {
        CommandRequest {
            id,
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://audio/setVolume"),
            payload: Some(json!({ "volume": self.volume })),
        }
    }
}
