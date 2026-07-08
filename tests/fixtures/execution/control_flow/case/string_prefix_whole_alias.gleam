pub fn main() {
  let value = "Hello, Geam"
  case value {
    "Hello, " <> name as greeting -> greeting <> name
    _ -> "Unknown"
  }
}

// geam:expect String("Hello, GeamGeam")
