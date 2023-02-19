use crate::lg_command_request::assert_command_request;
use lg_webos_client::lg_command::{CommandRequest, LGCommandRequest};

use lg_webos_client::lg_command::{request_commands, REQUEST_TYPE};
use serde_json::json;

#[test]
fn test_turn_off_tv() {
    let expected = CommandRequest {
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://system/turnOff"),
        payload: None,
    };

    let result = request_commands::system::TurnOffTV.to_command_request();

    assert_command_request(result, expected);
}

#[test]
fn test_power_screen() {
    let expected_turn_off = CommandRequest {
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://com.webos.service.tv.power/turnOffScreen"),
        payload: Some(json! ({"standbyMode": "active"})),
    };

    let expected_turn_on = CommandRequest {
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://com.webos.service.tv.power/turnOnScreen"),
        payload: Some(json! ({"standbyMode": "active"})),
    };

    let turn_on_screen = request_commands::system::TurnOnScreen;
    let turn_off_screen = request_commands::system::TurnOffScreen;

    assert_command_request(expected_turn_off, turn_off_screen.to_command_request());

    assert_command_request(expected_turn_on, turn_on_screen.to_command_request());
}
