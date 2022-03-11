use crate::lg_command_request::assert_command_request;
use lg_webos_client::{lg_command::{CommandRequest, LGCommandRequest}};
use serde_json::json;
use lg_webos_client::lg_command::{commands, REQUEST_TYPE};


#[test]
fn test_set_mute() {
    let mute = true;
    let expected = CommandRequest {
        id: 11,
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://audio/setMute"),
        payload: Some(json!({ "mute": mute })),
    };

    let set_mute = commands::audio::SetMute { mute };

    let result = set_mute.to_command_request(expected.id);
    assert_command_request(result, expected);
}


#[test]
fn test_set_volume() {
    let volume = 23;
    let expected = CommandRequest {
        id: 123,
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://audio/setVolume"),
        payload: Some(json!({ "volume": volume })),
    };

    let result = commands::audio::SetVolume { volume }.to_command_request(expected.id);

    assert_command_request(result, expected);
}

#[test]
fn test_get_audio_status() {
    let expected = CommandRequest {
        id: 222,
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://audio/getStatus"),
        payload: None,
    };

    let result = commands::audio::AudioStatus.to_command_request(expected.id);

    assert_command_request(result, expected);
}

#[test]
fn test_get_volume() {
    let expected = CommandRequest {
        id: 1,
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://audio/getVolume"),
        payload: None,
    };

    let result = commands::audio::GetVolume.to_command_request(expected.id);

    assert_command_request(result, expected);
}
