use crate::{
    arragement::{Position, Size},
    buffer::{Buffer, Cell},
    widget::Widget,
};
use std::mem::replace;

// A part of a buffer
pub struct Window {
    buffer: Buffer,
    position: Position,
    size: Size,
}

impl Window {
    pub fn new(size: Size) -> Window {
        Window {
            buffer: Buffer::new(size),
            position: (0, 0).into(),
            size,
        }
    }

    // Gets the size of the window
    pub fn size(&self) -> Size {
        self.size
    }

    // Offsets a position on the window to the position on the buffer
    fn offset(&self, position: Position) -> Position {
        (
            self.position.row + position.row,
            self.position.column + position.column,
        )
            .into()
    }

    // Gets a cell at a position
    pub fn cell(&self, position: Position) -> Option<&Cell> {
        self.buffer.cell(self.offset(position))
    }

    // Gets a mutable reference to a cell at a position
    pub fn cell_mut(&mut self, position: Position) -> Option<&mut Cell> {
        self.buffer.cell_mut(self.offset(position))
    }

    // Gets the diff between two windows
    pub fn diff<'d>(&self, other: &'d Window) -> Vec<(&'d Cell, Position)> {
        self.buffer
            .diff(&other.buffer)
            .into_iter()
            .filter(|(_, position)| {
                if position.row > self.position.row + self.size.rows
                    || position.column > self.position.column + self.size.columns
                {
                    return false;
                }
                true
            })
            .collect()
    }

    // Draws a widget onto the frame
    pub fn render_widget(&mut self, widget: impl Widget, position: Position, size: Size) {
        // Doesn't render the widget if it's unnecessary
        if self.size.area() == 0
            || size.area() == 0
            || position.row > self.position.row + self.size.rows
            || position.column > self.position.column + self.size.columns
        {
            return;
        }
        let previous_position = replace(&mut self.position, position);
        let previous_size = replace(&mut self.size, size);
        widget.render(self);
        self.position = previous_position;
        self.size = previous_size;
    }
}
