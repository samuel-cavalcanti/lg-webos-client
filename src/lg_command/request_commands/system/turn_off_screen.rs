use serde_json::json;

use crate::lg_command::{CommandRequest, LGCommandRequest, REQUEST_TYPE};

pub struct TurnOffScreen;

impl LGCommandRequest for TurnOffScreen {
    fn to_command_request(&self) -> CommandRequest {
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://com.webos.service.tv.power/turnOffScreen"),
            payload: Some(json! ({"standbyMode": "active"})),
        }
    }
}
