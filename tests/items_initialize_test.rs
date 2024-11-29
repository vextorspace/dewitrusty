mod tests {
    use super::test_utils::TestItemLoader;
    use dewitrusty::data::item_loader::ItemLoader;
    use dewitrusty::ui::initializer::Initializer;
    use dewitrusty::ui::slint_exports::{AppWindow, ItemData, ListItem};

    #[test]
    fn test_initialize_uses_item_loader_for_ui() {
        i_slint_backend_testing::init_no_event_loop();
        let app = AppWindow::new().unwrap();
        let loader: Box<dyn ItemLoader> = Box::new(
            TestItemLoader::new(vec!(
                ListItem { data: ItemData { id: "i1".into(), text: "Item 1".into() } },
                ListItem { data: ItemData { id: "i2".into(), text: "Item 2".into() } },
            ))
        );
        Initializer::default().with_item_loader(loader).run(&app);
        assert_eq!(app.get_window_title(), "Dew-It");
    }
}

pub mod test_utils {
    use dewitrusty::data::item_loader::ItemLoader;
    use dewitrusty::ui::slint_exports::ListItem;

    pub struct TestItemLoader {
        pub items: Vec<ListItem>,
    }

    impl ItemLoader for TestItemLoader {
        fn load(&self) -> Vec<ListItem> {
            self.items.clone()
        }
    }

    impl TestItemLoader {
        pub fn new(items: Vec<ListItem>) -> Self {
            Self { items }
        }
    }
}