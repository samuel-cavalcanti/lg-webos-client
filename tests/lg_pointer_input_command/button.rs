use lg_webos_client::lg_command::pointer_input_commands::{self, PointerInputCommand};

#[test]
fn test_buttons() {
    /*
        After testing in TV the correct payload what the client must send. This test is was created
    */
    let input_commands: Vec<Box<dyn PointerInputCommand + Send>> = vec![
        Box::new(pointer_input_commands::ButtonKey::HOME),
        Box::new(pointer_input_commands::ButtonKey::LEFT),
        Box::new(pointer_input_commands::ButtonKey::RIGHT),
        Box::new(pointer_input_commands::ButtonKey::UP),
        Box::new(pointer_input_commands::ButtonKey::DOWN),
        Box::new(pointer_input_commands::ButtonKey::BACK),
        Box::new(pointer_input_commands::ButtonKey::ENTER),
    ];

    let expected_strings = ["HOME", "LEFT", "RIGHT", "UP", "DOWN", "BACK", "ENTER"]
        .map(|name| format!("type:button\nname:{name}\n\n"))
        .to_vec();

    for (command, expected) in input_commands.iter().zip(expected_strings) {
        assert_eq!(command.to_request_string(), expected);
    }
}
