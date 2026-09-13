use crate::plan::execution::prepared::rust::{Emit, Rust};
use ecow::EcoString;

#[derive(Clone)]
pub enum Text {
    Owned(EcoString),
    Static(&'static str),
}

impl Text {
    pub(crate) fn as_str(&self) -> &str {
        self.as_ref()
    }

    pub(crate) fn materialize(&self) -> EcoString {
        match self {
            Self::Owned(text) => text.clone(),
            Self::Static(text) => (*text).into(),
        }
    }
}

impl Emit for Text {
    fn emit(&self, output: &mut Rust) {
        output.call("Text::Static", &[&self.as_str()]);
    }
}

impl From<EcoString> for Text {
    fn from(text: EcoString) -> Self {
        Self::Owned(text)
    }
}

impl From<&'static str> for Text {
    fn from(text: &'static str) -> Self {
        Self::Static(text)
    }
}

impl AsRef<str> for Text {
    fn as_ref(&self) -> &str {
        match self {
            Self::Owned(text) => text,
            Self::Static(text) => text,
        }
    }
}

impl std::ops::Deref for Text {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_ref()
    }
}

impl std::fmt::Debug for Text {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self.as_ref(), formatter)
    }
}

impl std::fmt::Display for Text {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_ref())
    }
}

impl PartialEq for Text {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl Eq for Text {}

impl PartialOrd for Text {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Text {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl std::hash::Hash for Text {
    fn hash<Hasher: std::hash::Hasher>(&self, state: &mut Hasher) {
        self.as_ref().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::{EcoString, Text};
    use std::hash::{Hash, Hasher};

    #[test]
    fn reads_owned_or_static_utf8_and_materializes_only_on_request() {
        static SOURCE: &str = "a source name longer than the inline text capacity \u{ac00}";
        let text = EcoString::from(SOURCE);
        let owned = Text::from(text.clone());
        let borrowed = Text::from(SOURCE);
        assert!(std::ptr::eq(owned.as_ptr(), text.as_ptr()));
        assert!(std::ptr::eq(borrowed.as_ptr(), SOURCE.as_ptr()));
        assert_eq!(borrowed.as_str(), SOURCE);
        assert!(std::ptr::eq(owned.clone().as_ptr(), owned.as_ptr()));
        assert!(std::ptr::eq(borrowed.clone().as_ptr(), SOURCE.as_ptr()));
        assert_eq!(owned.materialize(), text);
        assert_eq!(borrowed.materialize(), text);
        assert!(std::ptr::eq(owned.materialize().as_ptr(), text.as_ptr()));
    }

    #[test]
    fn equality_hash_and_formatting_preserve_text_not_storage_identity() {
        let owned = Text::from(EcoString::from("line\n\"quoted\""));
        let borrowed = Text::from("line\n\"quoted\"");
        assert_eq!(owned, borrowed);
        assert_ne!(owned, Text::from("different"));
        assert_eq!(owned.cmp(&borrowed), std::cmp::Ordering::Equal);
        assert_eq!(
            owned.partial_cmp(&Text::from("z")),
            Some(std::cmp::Ordering::Less)
        );
        assert_eq!(Text::from("z").cmp(&borrowed), std::cmp::Ordering::Greater);
        let mut expected = std::collections::hash_map::DefaultHasher::new();
        "line\n\"quoted\"".hash(&mut expected);
        for text in [&owned, &borrowed] {
            assert_eq!(text.to_string(), "line\n\"quoted\"");
            assert_eq!(format!("{text:?}"), "\"line\\n\\\"quoted\\\"\"");
            let mut actual = std::collections::hash_map::DefaultHasher::new();
            text.hash(&mut actual);
            assert_eq!(actual.finish(), expected.finish());
        }
    }
}
