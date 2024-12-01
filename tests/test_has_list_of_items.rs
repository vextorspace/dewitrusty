use fltk::prelude::WidgetExt;

struct ItemListFinder {
    item: Option<Box<dyn WidgetExt>>,
}

impl ItemListFinder {
    pub fn new() -> Self {
        Self {
            item: None,
        }
    }
}
