use crate::{frame::Window, widget::Widget};
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
    fn render(&self, window: &mut Window) {
        let mut row = 0;
        let mut column = 0;
        let size = window.size();
        if self.cut {
            for symbol in self.content.chars() {
                if column >= size.columns || symbol == '\n' {
                    row += 1;
                    column = 0;
                    if row >= size.rows || !self.wrap {
                        return;
                    } else if symbol == '\n' {
                        continue;
                    }
                }
                window.cell_mut((row, column).into()).symbol = symbol;
                column += 1;
            }
            return;
        }
        for line in wrap(self.content, size.columns as usize) {
            for symbol in line.chars() {
                window.cell_mut((row, column).into()).symbol = symbol;
                column += 1;
            }
            row += 1;
            column = 0;
            if row >= size.rows || !self.wrap {
                return;
            }
        }
    }
}
