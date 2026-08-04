use im::OrdMap;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Catalog {
    entries: OrdMap<String, String>,
}

impl Catalog {
    pub fn insert(&self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let mut entries = self.entries.clone();
        entries.insert(key.into(), value.into());
        Self { entries }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(String::as_str)
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = (&String, &String)> {
        self.entries.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::Catalog;

    #[test]
    fn updates_preserve_prior_catalog_versions() {
        let empty = Catalog::default();
        let first = empty.insert("one", "1");
        let replaced = first.insert("one", "uno");

        assert_eq!(empty.get("one"), None);
        assert_eq!(first.get("one"), Some("1"));
        assert_eq!(replaced.get("one"), Some("uno"));
        assert_eq!(replaced.iter().len(), 1);
        assert_eq!(first.clone(), first);
    }
}
