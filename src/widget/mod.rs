mod text;

pub use text::Text;

use crate::{
    arragement::{Position, Size},
    buffer::Buffer,
};

// A renderable element of the terminal user interface
pub trait Widget {
    fn render(&self, buffer: &mut Buffer, position: Position, size: Size);
}
