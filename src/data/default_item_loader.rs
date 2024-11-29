use super::item_loader::ItemLoader;
use crate::ui::slint_exports::{ItemData, ListItem};

pub struct DefaultItemLoader {}

impl ItemLoader for DefaultItemLoader {
    fn load(&self) -> Vec<ListItem> {
        vec!(
            ListItem { data: ItemData { id: "inbox".into(), text: "Inbox".into() } },
            ListItem { data: ItemData { id: "todo".into(), text: "Todo".into() } },
            ListItem { data: ItemData { id: "projects".into(), text: "Projects".into() } },
            ListItem { data: ItemData { id: "waiting".into(), text: "Waiting".into() } },
            ListItem { data: ItemData { id: "shelved".into(), text: "Shelved".into() } },
            ListItem { data: ItemData { id: "reference".into(), text: "Reference".into() } },
        )
    }
}

impl DefaultItemLoader {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::super::item_loader::ItemLoader;
    use super::*;

    #[test]
    fn default_items_test() {
        let loader = DefaultItemLoader::new();

        let items = loader.load();

        assert_eq!(items[0].data.text, "Inbox");
        assert_eq!(items[1].data.text, "Todo");
        assert_eq!(items[2].data.text, "Projects");
        assert_eq!(items[3].data.text, "Waiting");
        assert_eq!(items[4].data.text, "Shelved");
        assert_eq!(items[5].data.text, "Reference");
    }
}