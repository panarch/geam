import gleam/string

pub fn prefixes(value: String, count: Int) -> Int {
  case value {
    "x" <> rest -> prefixes(rest, count + 1)
    "" -> count
    _ -> panic as "unexpected prefix workload input"
  }
}

pub fn graphemes(value: String, count: Int) -> Int {
  case string.pop_grapheme(value) {
    Ok(#(_, rest)) -> graphemes(rest, count + 1)
    Error(Nil) -> count
  }
}
