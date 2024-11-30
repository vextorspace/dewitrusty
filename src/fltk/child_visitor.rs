use fltk::prelude::WidgetExt;

pub struct ChildVisitor {
    parent: Box<dyn WidgetExt>,
    operation: Box<dyn VisitOperation>,
}

impl ChildVisitor {
    pub fn visit(&self, visitor: &dyn VisitOperation) {
        todo!()
    }
}

impl ChildVisitor {
    pub fn new(parent: &dyn WidgetExt) -> Self {
        todo!()
    }
}

impl ChildVisitor {}

pub trait VisitOperation {
    fn visit(&self, child: Box<dyn WidgetExt>);
}