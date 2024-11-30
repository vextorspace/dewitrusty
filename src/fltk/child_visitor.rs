use fltk::prelude::WidgetExt;

pub struct ChildVisitor<'a> {
    parent: Box<&'a dyn WidgetExt>,
}

impl<'a> ChildVisitor<'a> {
    pub fn new(parent: Box<&'a dyn WidgetExt>) -> Self {
        Self {
            parent,
        }
    }

    pub fn visit<'b>(&'b self, visitor: &mut dyn VisitOperation<'b>) {
        visitor.visit(self.parent.clone())
    }
}

pub trait VisitOperation<'a> {
    fn visit(&mut self, child: Box<&'a dyn WidgetExt>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use fltk::prelude::{GroupExt, WidgetBase};

    #[test]
    fn test_visitor_goes_through_item() {
        let hpack = fltk::group::Pack::new(100, 100, 400, 300, "Parent");
        hpack.end();

        let mut finder = ItemFinder::new(Box::new(&hpack));
        let visitor = ChildVisitor::new(Box::new(&hpack));
        visitor.visit(&mut finder);

        assert!(finder.found, "Should have found the parent");
    }

    struct ItemFinder<'a> {
        item: Box<&'a dyn WidgetExt>,
        found: bool,
    }

    impl<'a> ItemFinder<'a> {
        fn new(item: Box<&'a dyn WidgetExt>) -> Self {
            ItemFinder {
                item,
                found: false,
            }
        }
    }

    impl<'a> VisitOperation<'a> for ItemFinder<'a> {
        fn visit(&mut self, child: Box<&'a dyn WidgetExt>) {
            println!("Visiting: {} looking for: {}", child.label(), self.item.label());
            if std::ptr::eq(&**child, &**self.item) {
                self.found = true;
            }
        }
    }
}