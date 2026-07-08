pub fn main() {
  case "Hello, Geam" {
    "Hello, " <> name if name == "Nobody" -> "wrong"
    "Hello, " <> name if name == "Geam" -> name
    _ -> "Unknown"
  }
}

// geam:expect String("Geam")
