use ui::slint_exports::AppWindow;

use crate::ui;

pub struct Initializer {
    title: String,
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