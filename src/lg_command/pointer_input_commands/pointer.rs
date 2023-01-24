use super::PointerInputCommand;

pub struct Pointer {
    command: String,
}

impl Pointer {
    pub fn move_it(dx: f32, dy: f32, drag: bool) -> Pointer {
        let drag = match drag {
            true => "1",
            false => "0",
        };

        Pointer {
            command: format!("type:move\ndx:{dx}\ndy:{dy}\ndown:{drag}\n\n"),
        }
    }

    pub fn scroll(dx: f32, dy: f32) -> Pointer {
        Pointer {
            command: format!("type:scroll\ndx:{dx}\ndy:{dy}\n\n"),
        }
    }

    pub fn click() -> Pointer {
        Pointer {
            command: "type:click\n\n".to_string(),
        }
    }
}

impl PointerInputCommand for Pointer {
    fn to_request_string(&self) -> String {
        self.command.clone()
    }
}
