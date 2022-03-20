use crate::lg_command::{CommandRequest, LGCommandRequest, REQUEST_TYPE};

pub struct SendEnterKey;

impl LGCommandRequest for SendEnterKey {
    fn to_command_request(&self) -> CommandRequest {
        CommandRequest {
            r#type: String::from(REQUEST_TYPE),
            uri: String::from("ssap://com.webos.service.ime/sendEnterKey"),
            payload: None,
        }
    }
}
