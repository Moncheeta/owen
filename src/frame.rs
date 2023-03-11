use crate::{
    arragement::{Position, Size},
    buffer::Buffer,
    cell::Cell,
    widget::Widget,
};

pub struct Frame {
    pub buffer: Buffer,
}

impl Frame {
    pub fn new(size: Size) -> Frame {
        Frame {
            buffer: Buffer::new(size),
        }
    }

    // Draws a widget onto the frame
    pub fn render_widget(&mut self, widget: impl Widget, position: Position, size: Size) {
        // Returns early if there is nothing to render to
        if size.area() == 0 {
            return;
        }
        let mut window = Window::new(&mut self.buffer, position, size);
        widget.render(&mut window);
    }
}

// A part of a buffer
pub struct Window<'b> {
    buffer: &'b mut Buffer,
    position: Position,
    size: Size,
}

impl<'b> Window<'b> {
    // Creates a new window
    pub fn new(buffer: &'b mut Buffer, position: Position, size: Size) -> Window {
        Window {
            buffer,
            position,
            size,
        }
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
    pub fn cell(&self, position: Position) -> &Cell {
        self.buffer.cell(self.offset(position))
    }

    // Gets a mutable reference to a cell at a position
    pub fn cell_mut(&mut self, position: Position) -> &mut Cell {
        self.buffer.cell_mut(self.offset(position))
    }

    // Gets the size of the window
    pub fn size(&self) -> Size {
        self.size
    }
}
