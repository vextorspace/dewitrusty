use fltk::group::Pack;
use std::any::Any;

fn to_pack(value: &dyn Any) -> Option<Pack> {
    match value.downcast_ref::<Pack>() {
        Some(pack) => Some(pack.clone()),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fltk::prelude::WidgetExt;
    use fltk::text::TextDisplay;

    #[test]
    fn test_match_widget() {
        let unique_label = "::MY_UNIOUE_LABEL::";
        let widget = Pack::default().with_label(unique_label);
        let pack = to_pack(&widget);

        assert_eq!(pack.unwrap().label(), unique_label);


        let other_widget = TextDisplay::default();
        let other_pack = to_pack(&other_widget);
        assert_eq!(other_pack, None);
    }
}