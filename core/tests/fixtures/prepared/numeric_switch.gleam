pub fn choose(input: Int) -> Int {
  let value = input + 1
  case value {
    0 -> value + 1
    1 -> value + 2
    _ -> value + 3
  }
}

pub fn main() -> Int {
  choose(0)
}
