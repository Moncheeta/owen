use crate::{
    cell::Cell,
    arragement::{
        Position,
        Size
    }
};

// A renderable element of the terminal user interface
pub trait Widget {
    fn render(&self, position: Position, size: Size, buffer: &[Cell]);
}
