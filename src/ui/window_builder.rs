use fltk::{
    group::{Pack, PackType},
    prelude::{GroupExt, WidgetExt},
    window::Window,
};

pub struct WindowBuilder {}

impl WindowBuilder {
    pub fn new() -> WindowBuilder {
        WindowBuilder {}
    }

    pub fn build(self) -> Window {
        let mut window = Window::default().with_size(900, 700).with_label("DewIt Rusty");

        let mut vpack = Pack::default().with_size(900, 700).center_of_parent();
        vpack.set_type(PackType::Vertical);

        // Optionally add a toolbar here
        let mut hpack = Pack::default().with_size(900, 100).center_of_parent();
        hpack.set_type(PackType::Horizontal);
        hpack.set_label("DewIt Now");

        // Optionally add widgets to the horizontal pack here

        hpack.end();
        vpack.add(&hpack);

        // Optionally add a status-bar here
        vpack.end();
        window.end();
        window.show();

        window
    }
}

