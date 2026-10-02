fn walk(remaining: Int, total: Int) -> Int {
  case remaining {
    0 -> total
    _ -> walk(remaining - 1, total + 3)
  }
}

pub fn main() {
  echo walk(14, 0)
  Nil
}
