use crate::ui::slint_exports::ListItem;

pub trait ItemLoader {
    fn load(&self) -> Vec<ListItem>;
}