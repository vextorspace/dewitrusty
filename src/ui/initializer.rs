use ui::slint_exports::AppWindow;
use crate::data::item_loader::ItemLoader;
use crate::ui;

pub struct Initializer {
    title: String,
}

impl Initializer {
    pub fn with_item_loader(&self, item_loader: Box<dyn ItemLoader>) -> Initializer {
        todo!()
    }
}

impl Default for Initializer {
    fn default() -> Self {
        Self {
            title: "Dew-It".into()
        }
    }
}

impl Initializer {
    pub fn run(&self, app: &AppWindow) {
        app.set_window_title(self.title.clone().into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization_sets_title() {
        let app = super::AppWindow::new().unwrap();

        let initializer = Initializer::default();
        initializer.run(&app);

        assert_eq!(app.get_window_title(), "Dew-It");
    }
}