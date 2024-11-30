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

    pub fn default() -> Self {
        ItemData {
            text: "".to_string(),
            id: uuid::Uuid::new_v4().to_string(),
        }
    }

    pub fn with_text(&self, text: &str) -> Self {
        ItemData {
            text: text.to_string(),
            id: self.id.clone(),
        }
    }

    pub fn with_id(&self, id: &str) -> Self {
        ItemData {
            text: self.text.clone(),
            id: id.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_item_has_text() {
        let text = "::THE TEXT::";
        let item = ItemData::new(text);
        assert_eq!(item.text, text);
    }

    #[test]
    fn new_item_has_id() {
        let item = ItemData::new("foo");
        assert!(item.id.len() > 0);
    }

    #[test]
    fn item_can_be_built_with_text_and_id() {
        let text = "::THE TEXT::";
        let id = "::THE ID::";
        let item = ItemData::default().with_text(text).with_id(id);

        assert_eq!(item.text, text);
        assert_eq!(item.id, id);
    }
}