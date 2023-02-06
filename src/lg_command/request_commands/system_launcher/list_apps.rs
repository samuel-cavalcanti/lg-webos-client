use crate::lg_command::{CommandRequest, LGCommandRequest, REQUEST_TYPE};

//ssap://com.webos.applicationManager/listApps
pub struct ListApps;

impl LGCommandRequest for ListApps {
    fn to_command_request(&self) -> CommandRequest {
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://com.webos.applicationManager/listApps"),
            payload: None,
        }
    }
}
