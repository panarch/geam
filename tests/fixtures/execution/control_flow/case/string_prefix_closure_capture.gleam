pub fn main() {
  case "Hello, Geam" {
    "Hello, " <> name -> fn() { name }()
    _ -> "Unknown"
  }
}

// geam:expect String("Geam")
