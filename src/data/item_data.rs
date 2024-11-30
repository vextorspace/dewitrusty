pub struct ItemData {
    pub text: String,
    pub id: String,
}


impl ItemData {
    pub fn new(text: &str) -> ItemData {
        ItemData {
            text: text.to_string(),
            id: uuid::Uuid::new_v4().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_has_text() {
        let item = ItemData::new("foo");
        assert_eq!(item.text, "foo");
    }

    #[test]
    fn item_has_id() {
        let item = ItemData::new("foo");
        assert!(item.id.len() > 0);
    }
}