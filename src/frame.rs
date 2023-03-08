use crate::{
    arragement::{Position, Size},
    buffer::Buffer,
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
        // TODO: only let the widget have access to a part of the frame
        // and therefore remove position argument

        // Returns early if there is nothing to render to
        if size.area() == 0 {
            return;
        }
        widget.render(&mut self.buffer, position, size);
    }
}
