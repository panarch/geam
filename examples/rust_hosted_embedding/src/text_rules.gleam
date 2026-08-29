import gleam/string

pub fn normalize(value: String) -> String {
  string.lowercase(value)
}
