#[cfg(test)]

use dewitrusty::data::item_loader;
use dewitrusty::ui::slint_exports::ListItem;
use item_loader::ItemLoader;

pub struct TestItemLoader {
     pub items: Vec<ListItem>,
}

impl ItemLoader for TestItemLoader {
    fn load(&self) -> Vec<ListItem> {
        todo!()
    }
}

impl TestItemLoader {
    pub fn new(items: Vec<ListItem>) -> Self {
        Self { items }
    }
}