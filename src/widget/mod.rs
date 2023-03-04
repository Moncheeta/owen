use crate::{
    arragement::{Position, Size},
    cell::Cell,
};

// A renderable element of the terminal user interface
pub trait Widget {
    fn render(&self, buffer: &mut [Cell], position: Position, size: Size);
}
