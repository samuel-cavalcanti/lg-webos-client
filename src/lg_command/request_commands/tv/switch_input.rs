use serde_json::json;

use crate::lg_command::REQUEST_TYPE;
use crate::lg_command::{CommandRequest, LGCommandRequest};

/// You Can get the Input Id using `GetExternalInputList`, than set the Input
/// using `SwitchInput`
pub struct SwitchInput {
    pub input_id: String,
}

impl LGCommandRequest for SwitchInput {
    fn to_command_request(&self) -> CommandRequest {
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://tv/switchInput"),
            payload: Some(json!({ "inputId":self.input_id })),
        }
    }
}
