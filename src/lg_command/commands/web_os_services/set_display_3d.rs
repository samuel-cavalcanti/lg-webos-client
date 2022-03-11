use crate::lg_command::{CommandRequest, LGCommandRequest, REQUEST_TYPE};

pub struct SetDisplay3D {
    pub turn_3d: bool,
}


impl LGCommandRequest for SetDisplay3D {
    fn to_command_request(&self, id: u8) -> CommandRequest {
        let uri = match self.turn_3d {
            true => { String::from("ssap://com.webos.service.tv.display/set3DOn") }
            false => { String::from("ssap://com.webos.service.tv.display/set3DOff") }
        };

        CommandRequest {
            id,
            r#type: REQUEST_TYPE.to_string(),
            uri,
            payload: None,
        }
    }
}