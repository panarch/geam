pub fn main() {
  let pair = #(11, 37)
  let text = "Hello, Geam"
  case pair, text {
    #(left, right), "Hello, " <> name if left < right -> name
    _, _ -> "fallback"
  }
}

// geam:expect String("Geam")
