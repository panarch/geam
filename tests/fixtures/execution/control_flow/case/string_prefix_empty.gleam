pub fn main() {
  case "Geam" {
    "" <> rest -> rest
    _ -> "Unknown"
  }
}

// geam:expect String("Geam")
