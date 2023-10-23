use serde_json::json;

use crate::lg_command::{CommandRequest, LGCommandRequest, REQUEST_TYPE};

/// Insert Text in virtual keyboard
pub struct InsertText {
    /// the Text in virutal keyboard
    pub text: String,
    /// if you want to replace the the current text set replace to true
    pub replace: bool,
}

impl LGCommandRequest for InsertText {
    fn to_command_request(&self) -> CommandRequest {
        let replace = match self.replace {
            true => 1,
            false => 0,
        };
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: "ssap://com.webos.service.ime/insertText".to_string(),
            payload: Some(json!({"text":self.text,"replace":replace})),
        }
    }
}
