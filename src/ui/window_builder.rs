use fltk::prelude::{GroupExt, WidgetExt};
use fltk::window::Window;

pub struct WindowBuilder {}

impl WindowBuilder {
    pub fn new() -> WindowBuilder {
        WindowBuilder {}
    }

    pub fn build(self) -> Window {
        let mut window = Window::default().with_size(900, 700).with_label("DewIt Rusty");
        window.end();
        window.show();

        window
    }
}

