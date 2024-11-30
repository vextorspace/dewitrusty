use dewitrusty::fltk::child_visitor::{ChildVisitor, VisitOperation};
use dewitrusty::ui::window_builder::WindowBuilder;
use fltk::app::App;
use fltk::prelude::WidgetExt;
use fltk::window::Window;

#[test]
fn there_exists_hpack_child() {
    let app = App::default();
    let wind: Window = WindowBuilder::new().build();

    let finder = ItemListFinder::new();
    ChildVisitor::new(&wind).visit(&finder);

    assert_eq!(finder.item.unwrap().label(), "DewIt Now");
}

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

impl VisitOperation for ItemListFinder {
    fn visit(&self, child: Box<dyn WidgetExt>) {
        todo!()
    }
}