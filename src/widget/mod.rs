use crate::{
    arragement::{Position, Size},
    cell::Cell,
};

// A renderable element of the terminal user interface
pub trait Widget {
    fn render(&self, position: Position, size: Size, buffer: &[Cell]);
}
