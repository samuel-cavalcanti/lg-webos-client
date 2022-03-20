use crate::lg_command_request::assert_command_request;
use lg_webos_client::lg_command::{request_commands, REQUEST_TYPE};
use lg_webos_client::lg_command::{CommandRequest, LGCommandRequest};
use serde_json::json;

#[test]
fn test_set_mute() {
    for mute in [true, false] {
        let expected = CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://audio/setMute"),
            payload: Some(json!({ "mute": mute })),
        };

        let set_mute = request_commands::audio::SetMute { mute };

        let result = set_mute.to_command_request();
        assert_command_request(result, expected);
    }
}

#[test]
fn test_set_volume() {
    for volume in [10, 23, 12] {
        let expected = CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://audio/setVolume"),
            payload: Some(json!({ "volume": volume })),
        };

        let result = request_commands::audio::SetVolume { volume }.to_command_request();

        assert_command_request(result, expected);
    }
}

#[test]
fn test_get_audio_status() {
    let expected = CommandRequest {
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://audio/getStatus"),
        payload: None,
    };

    let result = request_commands::audio::AudioStatus.to_command_request();

    assert_command_request(result, expected);
}

#[test]
fn test_get_volume() {
    let expected = CommandRequest {
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://audio/getVolume"),
        payload: None,
    };

    let result = request_commands::audio::GetVolume.to_command_request();

    assert_command_request(result, expected);
}
