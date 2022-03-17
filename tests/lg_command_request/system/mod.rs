use crate::lg_command_request::assert_command_request;
use lg_webos_client::lg_command::{CommandRequest, LGCommandRequest};

use lg_webos_client::lg_command::{commands, REQUEST_TYPE};

#[test]
fn test_turn_off() {
    let expected = CommandRequest {
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://system/turnOff"),
        payload: None,
    };

    let result = commands::system::TurnOff.to_command_request();

    assert_command_request(result, expected);
}
