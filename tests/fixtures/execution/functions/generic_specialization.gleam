fn identity(value: value) -> value {
  value
}

fn recursive(count: Int, value: value) -> value {
  case count {
    0 -> value
    _ -> recursive(count - 1, value)
  }
}

fn mutual_left(count: Int, value: value) -> value {
  case count {
    0 -> value
    _ -> mutual_right(count - 1, value)
  }
}

fn mutual_right(count: Int, value: value) -> value {
  case count {
    0 -> value
    _ -> mutual_left(count - 1, value)
  }
}

pub fn main() {
  #(
    identity(1),
    identity("two"),
    identity(3),
    recursive(2, 4),
    recursive(1, "five"),
    mutual_left(2, 6),
    mutual_right(1, "seven"),
  )
}

// @geam:expect Tuple([Int(1), String("two"), Int(3), Int(4), String("five"), Int(6), String("seven")])
