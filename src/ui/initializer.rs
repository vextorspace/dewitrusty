use crate::data::item_loader::ItemLoader;
use crate::ui;
use ui::slint_exports::AppWindow;

pub struct Initializer {
    title: String,
    item_loader: Option<Box<dyn ItemLoader>>,
}

impl Initializer {
    pub fn with_item_loader(&self, item_loader: Box<dyn ItemLoader>) -> Initializer {
        Initializer::new(self.title.clone(), Some(item_loader))
    }

    pub fn new(title: String, item_loader: Option<Box<dyn ItemLoader>>) -> Initializer {
        Initializer {
            title,
            item_loader,
        }
    }
}

impl Default for Initializer {
    fn default() -> Self {
        Self {
            title: "Dew-It".into(),
            item_loader: None,
        }
    }
}

impl Initializer {
    pub fn run(&self, app: &AppWindow) {
        app.set_window_title(self.title.clone().into());

        self.initialize_items(app);
    }

    fn initialize_items(&self, app: &AppWindow) {
        let items = if let Some(loader) = &self.item_loader {
            loader.load()
        } else {
            Vec::new()
        };

        let items_model = std::rc::Rc::new(slint::VecModel::from(items));
        app.set_items(items_model.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::item_loader::test_item_loader::TestItemLoader;
    use crate::ui::slint_exports::{ItemData, ListItem};
    use slint::Model;

    #[test]
    fn test_initialization_sets_title() {
        i_slint_backend_testing::init_no_event_loop();
        let app = super::AppWindow::new().unwrap();

        let initializer = Initializer::default();
        initializer.run(&app);

        assert_eq!(app.get_window_title(), "Dew-It");
    }

    #[test]
    fn test_initialization_uses_item_loader() {
        i_slint_backend_testing::init_no_event_loop();
        let app = super::AppWindow::new().unwrap();

        let loader: Box<dyn ItemLoader> = Box::new(
            TestItemLoader::new(vec!(
                ListItem { data: ItemData { text: "Item 1".into(), id: "i1".into() } },
                ListItem { data: ItemData { text: "Item 2".into(), id: "i2".into() } },
            ))
        );

        let initializer = Initializer::default().with_item_loader(loader);
        initializer.run(&app);

        let items: Vec<ListItem> = app.get_items().iter().collect();
        assert_eq!(items.len(), 2);
        assert_eq!(items.get(0).unwrap().data.text, "Item 1");
        assert_eq!(items.get(1).unwrap().data.text, "Item 2");
    }
}