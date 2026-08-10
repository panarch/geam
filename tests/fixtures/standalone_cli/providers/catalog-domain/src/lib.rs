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

    pub fn entries(&self) -> impl ExactSizeIterator<Item = (&String, &String)> {
        self.entries.iter()
    }
}
