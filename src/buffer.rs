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
    pub fn diff<'b>(&self, other: &'b Buffer) -> Vec<(&'b Cell, Position)> {
        let mut diff = Vec::new();
        for row in 0..=self.size.rows - 1 {
            for column in 0..=self.size.columns - 1 {
                let position = (row, column).into();
                let new_cell = other.cell(position);
                let old_cell = self.cell(position);
                if new_cell != old_cell {
                    diff.push((new_cell, position));
                }
            }
        }
        diff
    }
}
