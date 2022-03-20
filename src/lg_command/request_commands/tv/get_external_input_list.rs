use crate::lg_command::REQUEST_TYPE;
use crate::lg_command::{CommandRequest, LGCommandRequest};

pub struct GetExternalInputList;

impl LGCommandRequest for GetExternalInputList {
    fn to_command_request(&self) -> CommandRequest {
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://tv/getExternalInputList"),
            payload: None,
        }
    }
}
