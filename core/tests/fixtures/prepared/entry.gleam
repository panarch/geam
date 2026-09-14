pub fn main() {
  echo second([0, 42])
  fn(value) { value }
}

fn second(items) {
  case items {
    [] -> 0
    [item] -> item
    [_, item, ..] -> item
  }
}
