fn string_loop(n: Int, value: String) {
  case n {
    0 -> value
    _ -> string_loop(n - 1, value)
  }
}

fn nil_loop(n: Int) {
  case n {
    0 -> Nil
    _ -> nil_loop(n - 1)
  }
}

pub fn main() {
  nil_loop(10000)
  string_loop(10000, "done")
}

// geam:expect String("done")
