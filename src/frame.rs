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
