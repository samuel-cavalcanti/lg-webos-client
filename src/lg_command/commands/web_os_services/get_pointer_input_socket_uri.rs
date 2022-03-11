
use crate::lg_command::{CommandRequest, LGCommandRequest, REQUEST_TYPE};
pub struct GetPointerInputSocketUri;

impl LGCommandRequest for GetPointerInputSocketUri {
    fn to_command_request(&self, id: u8) -> CommandRequest {
        CommandRequest {
            id,
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://com.webos.service.networkinput/getPointerInputSocket"),
            payload: None,
        }
    }
}