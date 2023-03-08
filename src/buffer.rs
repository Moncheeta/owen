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
    fn index_of(&self, position: Position) -> usize {
        ((position.row * self.size.columns) + position.column).into()
    }

    // Resizes the frame to a new size
    pub fn resize(&mut self, size: Size) {
        self.content
            .resize_with(size.area().into(), Default::default);
    }

    // Finds the difference between itself and another frame
    pub fn diff(&self, other: &Buffer) -> Vec<(Cell, Position)> {
        let mut diff = Vec::new();
        for row in 0..=self.size.rows - 1 {
            for column in 0..=self.size.columns - 1 {
                let new_cell = self.content[self.index_of((row, column).into())];
                if new_cell != other.content[other.index_of((row, column).into())] {
                    diff.push((new_cell, Position { row, column }));
                }
            }
        }
        diff
    }
}
