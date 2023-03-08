use crate::{
    arragement::{Position, Size},
    buffer::Buffer,
    widget::Widget,
};
use textwrap::wrap;

pub struct Text<'s> {
    // The text in the widget
    pub content: &'s str,
    // Whether the text should continue on the next line
    pub wrap: bool,
    // Whether the text should be cut without regards for whitespace
    pub cut: bool,
}

impl<'s> Text<'s> {
    pub fn with(content: &'s str, wrap: bool, cut: bool) -> Text {
        Text { content, wrap, cut }
    }

    pub fn line(content: &'s str) -> Text {
        Text {
            content,
            wrap: false,
            cut: true,
        }
    }

    pub fn paragraph(content: &'s str) -> Text {
        Text {
            content,
            wrap: true,
            cut: false,
        }
    }
}

impl<'s> Widget for Text<'s> {
    fn render(&self, buffer: &mut Buffer, position: Position, size: Size) {
        let mut row = position.row;
        let mut column = position.column;
        if self.cut {
            for symbol in self.content.chars() {
                if column >= position.column + size.columns || symbol == '\n' {
                    row += 1;
                    column = position.column;
                    if row >= position.row + size.rows || !self.wrap {
                        return;
                    } else if symbol == '\n' {
                        continue;
                    }
                }
                buffer.get_mut((row, column).into()).symbol = symbol;
                column += 1;
            }
            return;
        }
        for line in wrap(self.content, size.columns as usize) {
            for symbol in line.chars() {
                buffer.get_mut((row, column).into()).symbol = symbol;
                column += 1;
            }
            row += 1;
            column = position.column;
            if row >= position.row + size.rows || !self.wrap {
                return;
            }
        }
    }
}
