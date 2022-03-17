use serde_json::json;

use crate::{lg_command::{CommandRequest, LGCommandRequest}};
use crate::lg_command::REQUEST_TYPE;

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
