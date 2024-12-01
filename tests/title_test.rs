use dewitrusty::ui::window_builder::WindowBuilder;
use fltk::prelude::WidgetExt;
use fltk::window::Window;

#[test]
fn title_is_good() {
    let wind: Window = WindowBuilder::new().build();

    let title = wind.label();
    assert_eq!(title, "DewIt Rusty");
}