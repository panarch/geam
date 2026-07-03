pub fn main() {
  let values = [1, 2]
  case values {
    [first, ..] -> first
    [] -> 0
  }
}
