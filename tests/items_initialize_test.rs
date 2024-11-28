use dewitrusty::ui::initializer::Initializer;

use dewitrusty::ui::slint_exports::AppWindow;
use dewitrusty::ui::slint_exports::ListItem;
use dewitrusty::data::item_loader::ItemLoader;

mod common;
use common::test_item_loader::TestItemLoader;

#[test]
fn test_initialize_uses_item_loader_for_ui() {
    let app = AppWindow::new().unwrap();
    let loader: Box<dyn ItemLoader> = Box::new(
        TestItemLoader::new(vec!(
            ListItem{text: "Item 1".into()},
            ListItem{text: "Item 2".into()},
        ))
    );
    Initializer::default().with_item_loader(loader).run(&app);
    assert_eq!(app.get_window_title(), "Dew-It");
}