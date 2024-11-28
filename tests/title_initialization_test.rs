use dewitrusty::ui::initializer::Initializer;

use dewitrusty::data::item_loader::ItemLoader;
use dewitrusty::ui::slint_exports::AppWindow;

mod common;

#[test]
fn test_title_correct() {
    let app = AppWindow::new().unwrap();
    Initializer::default().run(&app);
    assert_eq!(app.get_window_title(), "Dew-It");
}