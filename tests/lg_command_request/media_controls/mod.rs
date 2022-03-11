
use lg_webos_client::{lg_command::{CommandRequest, LGCommandRequest}};

use lg_webos_client::lg_command::{commands, REQUEST_TYPE};

use crate::lg_command_request::assert_command_request;

#[test]
fn test_media_control_play() {
    let expected = CommandRequest {
        id: 12,
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://media.controls/play"),
        payload: None,
    };

    let result = commands::media_controls::Play.to_command_request(expected.id);

    assert_command_request(result, expected);
}

#[test]
fn test_media_control_stop() {
    let expected = CommandRequest {
        id: 54,
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://media.controls/stop"),
        payload: None,
    };

    let result = commands::media_controls::Stop.to_command_request(expected.id);

    assert_command_request(result, expected);
}

#[test]
fn test_media_control_pause() {
    let expected = CommandRequest {
        id: 23,
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://media.controls/pause"),
        payload: None,
    };

    let result = commands::media_controls::Pause.to_command_request(expected.id);

    assert_command_request(expected, result)
}

#[test]
fn test_media_controls_rewind() {
    let expected = CommandRequest {
        id: 1,
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://media.controls/rewind"),
        payload: None,
    };

    let result = commands::media_controls::Rewind.to_command_request(expected.id);

    assert_command_request(result, expected);
}

#[test]
fn test_media_controls_fast_forward() {
    let expected = CommandRequest {
        id: 1,
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://media.controls/fastForward"),
        payload: None,
    };

    let result = commands::media_controls::FastForward.to_command_request(expected.id);

    assert_command_request(result, expected);
}