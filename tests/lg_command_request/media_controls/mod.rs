use lg_webos_client::lg_command::{CommandRequest, LGCommandRequest};

use lg_webos_client::lg_command::{request_commands, REQUEST_TYPE};

use crate::lg_command_request::assert_command_request;

#[test]
fn test_all_commands() {
    let expected = vec![
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://media.controls/play"),
            payload: None,
        },
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://media.controls/stop"),
            payload: None,
        },
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://media.controls/pause"),
            payload: None,
        },
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://media.controls/rewind"),
            payload: None,
        },
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://media.controls/fastForward"),
            payload: None,
        },
    ];

    let commands: Vec<Box<dyn LGCommandRequest>> = vec![
        Box::new(request_commands::media_controls::Play),
        Box::new(request_commands::media_controls::Stop),
        Box::new(request_commands::media_controls::Pause),
        Box::new(request_commands::media_controls::Rewind),
        Box::new(request_commands::media_controls::FastForward),
    ];

    for (command, expected_request) in commands.iter().zip(expected) {
        assert_command_request(command.to_command_request(), expected_request);
    }
}
