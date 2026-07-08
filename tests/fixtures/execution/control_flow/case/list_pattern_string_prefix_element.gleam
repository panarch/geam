pub fn main() {
  case ["Hello, Geam"] {
    ["Hello, " as prefix <> name] -> prefix <> name
    _ -> "none"
  }
}

// geam:expect String("Hello, Geam")
