use serde_json::json;

use crate::lg_command::{CommandRequest, LGCommandRequest, REQUEST_TYPE};

pub struct TurnOnScreen;

impl LGCommandRequest for TurnOnScreen {
    fn to_command_request(&self) -> CommandRequest {
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://com.webos.service.tv.power/turnOnScreen"),
            payload: Some(json! ({"standbyMode": "active"})),
        }
    }
}
