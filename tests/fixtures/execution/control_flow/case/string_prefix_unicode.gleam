pub fn main() {
  case "안녕, 글림" {
    "안녕, " <> name -> name
    _ -> "Unknown"
  }
}

// geam:expect String("글림")
