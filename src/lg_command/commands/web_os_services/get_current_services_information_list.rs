use crate::lg_command::{CommandRequest, LGCommandRequest, REQUEST_TYPE};


pub struct  GetCurrentServicesInformationList;
impl LGCommandRequest for GetCurrentServicesInformationList {
    fn to_command_request(&self, id: u8) -> CommandRequest {
        CommandRequest {
            id,
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://com.webos.service.update/getCurrentSWInformation"),
            payload: None,
        }
    }
}
