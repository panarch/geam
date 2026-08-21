pub fn main() {
  let value = "Hello, Geam"
  case value {
    "Hello, " <> name -> name
    _ -> "Unknown"
  }
}

// @geam:expect String("Geam")
