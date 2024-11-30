use dewitrusty::ui::window_builder::WindowBuilder;
use fltk::app::App;
use fltk::prelude::WidgetExt;
use fltk::window::Window;

#[test]
fn title_is_good() {
    let app = App::default();
    let wind: Window = WindowBuilder::new().build();


    let title = wind.label();
    assert_eq!(title, "DewIt Rusty");
}