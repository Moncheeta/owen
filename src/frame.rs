use crate::{
    arragement::{Position, Size},
    cell::Cell,
    widget::Widget,
};
use std::io::Error;

pub struct Frame {
    content: Vec<Cell>,
    size: Size,
}

impl Frame {
    pub fn new(size: Size) -> Frame {
        Frame {
            content: vec![Cell::default(); size.area().into()],
            size,
        }
    }

    // Gets the Cell at a position
    pub fn get(&self, position: Position) -> &Cell {
        let index = self.index_of(position);
        &self.content[index]
    }

    // Gets a mutable reference to a Cell at a position
    pub fn get_mut(&mut self, position: Position) -> &mut Cell {
        let index = self.index_of(position);
        &mut self.content[index]
    }

    // Gets the index of the position in self.content
    pub fn index_of(&self, position: Position) -> usize {
        ((position.row * self.size.columns) + position.column).into()
    }

    // Resizes the frame to a new size
    pub fn resize(&mut self, size: Size) {
        self.content
            .resize_with(size.area().into(), Default::default);
    }

    // Finds the difference between itself and another frame
    pub fn diff(&self, other: &Frame) -> Vec<(Cell, Position)> {
        let mut diff = Vec::new();
        for row in 0..=self.size.rows {
            for column in 0..=self.size.columns {
                let new_cell = self.content[self.index_of(Position { row, column })];
                if new_cell != other.content[other.index_of(Position { row, column })] {
                    diff.push((new_cell, Position { row, column }))
                }
            }
        }
        diff
    }

    // Draws a widget onto the frame
    // TODO: Write widget routine
    pub fn render_widget(widget: impl Widget, position: Position, size: Size) -> Result<(), Error> {
        todo!()
    }
}
