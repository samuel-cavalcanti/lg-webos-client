pub trait PointerInputCommand {
    fn to_string(&self) -> String;
}

mod button;

pub use button::Button;
