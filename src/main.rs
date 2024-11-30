use dewitrusty::ui::window_builder::WindowBuilder;
use fltk::app;

fn main() {
    let app = app::App::default();
    WindowBuilder::new().build();

    app.run().unwrap();
}

