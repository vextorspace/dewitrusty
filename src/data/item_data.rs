pub struct ItemData {
    pub text: String,
}


impl ItemData {
    pub fn new(text: &str) -> ItemData {
        ItemData {
            text: text.to_string(),
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
}