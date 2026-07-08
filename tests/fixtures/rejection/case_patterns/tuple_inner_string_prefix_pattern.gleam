pub fn main() {
  let pair = #("Hello, Geam", 1)
  case pair {
    #("Hello, " <> name, value) -> value
    _ -> 0
  }
}
