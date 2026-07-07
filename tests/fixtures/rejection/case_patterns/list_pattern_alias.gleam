pub fn main() {
  let values = [1, 2]
  case values {
    [first, ..] as whole -> first
    _ -> 0
  }
}
