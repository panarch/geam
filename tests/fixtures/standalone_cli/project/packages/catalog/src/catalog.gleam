@external(erlang, "geam_catalog", "catalog")
pub type Catalog

pub type Summary {
  Summary(count: Int, items: List(String))
}

@external(erlang, "geam_catalog", "new")
pub fn new() -> Catalog

@external(erlang, "geam_catalog", "insert")
pub fn insert(catalog: Catalog, key: String, value: String) -> Catalog

@external(erlang, "geam_catalog", "summarize")
pub fn summarize(
  catalog: Catalog,
  transform: fn(String) -> String,
) -> Summary
