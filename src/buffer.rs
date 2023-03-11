use crate::{
    arragement::{Position, Size},
    cell::Cell,
};

pub struct Buffer {
    content: Vec<Cell>,
    size: Size,
}

impl Buffer {
    // Creates a new buffer with a certain size
    pub fn new(size: Size) -> Buffer {
        Buffer {
            content: vec![Cell::default(); size.area() as usize],
            size,
        }
    }

    // Gets the index of the position in self.content
    fn index_of(&self, position: Position) -> usize {
        ((position.row * self.size.columns) + position.column).into()
    }

    // Gets the cell at a position
    pub fn cell(&self, position: Position) -> &Cell {
        let index = self.index_of(position);
        &self.content[index]
    }

    // Gets a mutable reference to a cell at a position
    pub fn cell_mut(&mut self, position: Position) -> &mut Cell {
        let index = self.index_of(position);
        &mut self.content[index]
    }

    // Resizes the frame to a new size
    pub fn resize(&mut self, size: Size) {
        self.content
            .resize_with(size.area().into(), Default::default);
        self.size = size;
    }

    // Finds the difference between itself and another frame
    pub fn diff(&self, other: &Buffer) -> Vec<(Cell, Position)> {
        let mut diff = Vec::new();
        for row in 0..=self.size.rows - 1 {
            for column in 0..=self.size.columns - 1 {
                let new_cell = self.content[self.index_of((row, column).into())];
                if new_cell != other.content[other.index_of((row, column).into())] {
                    diff.push((new_cell, (row, column).into()));
                }
            }
        }
        diff
    }
}
