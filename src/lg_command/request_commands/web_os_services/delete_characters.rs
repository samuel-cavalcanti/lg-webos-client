use serde_json::json;

use crate::lg_command::{CommandRequest, LGCommandRequest, REQUEST_TYPE};

pub struct DeleteCharacters {
    pub number_of_chars: usize,
}

impl LGCommandRequest for DeleteCharacters {
    fn to_command_request(&self) -> CommandRequest {
        CommandRequest {
            r#type: String::from(REQUEST_TYPE),
            uri: String::from("ssap://com.webos.service.ime/deleteCharacters"),
            payload: Some(json!({"count":self.number_of_chars})),
        }
    }
}
