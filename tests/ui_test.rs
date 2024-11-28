
use dewitrusty::ui::initializer::Initializer;

use dewitrusty::ui::slint_exports::AppWindow;

#[test]
fn test_title_correct() {
    let app = AppWindow::new().unwrap();
    Initializer::default().run(&app);
    assert_eq!(app.get_window_title(), "Dew-It");
}