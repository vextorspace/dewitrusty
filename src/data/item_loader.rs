use crate::ui::slint_exports::ListItem;

pub trait ItemLoader {
    fn load(&self) -> Vec<ListItem>;
}

#[cfg(test)]
pub mod test_item_loader {
    use super::ItemLoader;
    use crate::ui::slint_exports::ListItem;

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