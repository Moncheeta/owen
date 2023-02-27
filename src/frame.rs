use std::io::Error;
use crate::{
    cell::Cell,
    arragement::{
        Size,
        Position
    },
    widget::Widget
};

pub struct Frame {
    content: Vec<Cell>,
    size: Size
}

impl Frame {
    pub fn new(size: Size) -> Frame {
        Frame {
            content: vec![Cell::default(); size.area().into()],
            size
        }
    }

    // Gets the Cell at a position
    pub fn get(&self, position: Position) -> &Cell {
        let index = self.index_of(position.row, position.column);
        &self.content[index]
    }

    // Gets a mutable reference to a Cell at a position
    pub fn get_mut(&mut self, position: Position) -> &mut Cell {
        let index = self.index_of(position.row, position.column);
        &mut self.content[index]
    }

    // Returns the index of a Cell at a position
    pub fn index_of(&self, row: u16, column: u16) -> usize {
        ((row * self.size.columns) + column).into()
    }

    // Resizes the frame to a new size
    pub fn resize(&mut self, size: Size) {
        self.content.resize_with(size.area().into(), Default::default);
    }

    // Draws a widget onto the frame
    // TODO: Write widget routine
    pub fn render_widget(widget: impl Widget, position: Position, size: Size) -> Result<(), Error> {
        todo!()
    }
}
