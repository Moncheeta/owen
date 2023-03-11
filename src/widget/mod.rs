mod text;

pub use text::Text;

use crate::frame::Window;

// A renderable element of the terminal user interface
pub trait Widget {
    fn render(&self, window: &mut Window);
}
