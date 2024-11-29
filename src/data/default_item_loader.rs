use super::item_loader::ItemLoader;
use crate::ui::slint_exports::ListItem;

pub struct DefaultItemLoader {}

impl ItemLoader for DefaultItemLoader {
    fn load(&self) -> Vec<ListItem> {
        vec!(
            ListItem { text: "Inbox".into() },
            ListItem { text: "Todo".into() },
            ListItem { text: "Projects".into() },
            ListItem { text: "Waiting".into() },
            ListItem { text: "Shelved".into() },
            ListItem { text: "Reference".into() },
        )
    }
}

impl DefaultItemLoader {
    fn new() -> Self {
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

        assert_eq!(items[0].text, "Inbox");
        assert_eq!(items[1].text, "Todo");
        assert_eq!(items[2].text, "Projects");
        assert_eq!(items[3].text, "Waiting");
        assert_eq!(items[4].text, "Shelved");
        assert_eq!(items[5].text, "Reference");
    }
}