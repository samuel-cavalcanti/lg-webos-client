use lg_webos_client::lg_command::pointer_input_commands::{Pointer, PointerInputCommand};

#[test]
fn test_pointer() {
    let dx = 123.0;
    let dy = 321.1;
    let move_it_drag_false = Pointer::move_it(dx, dy, false);
    let move_it_drag_true = Pointer::move_it(dx, dy, true);
    let scroll_y = Pointer::scroll(dx, dy);
    let click = Pointer::click();

    assert_eq!("type:click\n\n".to_string(), click.to_string());

    assert_eq!(
        scroll_y.to_string(),
        format!("type:scroll\ndx:{dx}\ndy:{dy}\n\n")
    );

    assert_eq!(
        move_it_drag_false.to_string(),
        format!("type:move\ndx:{dx}\ndy:{dy}\ndown:0\n\n")
    );

    assert_eq!(
        move_it_drag_true.to_string(),
        format!("type:move\ndx:{dx}\ndy:{dy}\ndown:1\n\n")
    );
}
