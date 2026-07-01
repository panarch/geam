fn add_one(value: Int) {
  value + 1
}

fn get(n: Int) {
  case n {
    0 -> add_one
    _ -> get(n - 1)
  }
}

pub fn main() {
  let f = get(10000)
  f(41)
}

// geam:expect Int(42)
