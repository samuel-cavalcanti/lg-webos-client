pub trait PointerInputCommand {
    fn to_string(&self) -> String;
}

mod button;
mod pointer;
pub use button::Button;
pub use pointer::Pointer;
