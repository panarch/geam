pub fn main() {
  case "Hello, Geam" {
    "Hello, " <> _rest -> "matched"
    _ -> "Unknown"
  }
}

// geam:expect String("matched")
