pub fn main() {
  case "Goodbye, Geam" {
    "Hello, " <> name -> name
    _ -> "Unknown"
  }
}

// @geam:expect String("Unknown")
