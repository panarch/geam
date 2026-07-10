fn one(value: Int) { value + 1 }

fn int_values(count: Int) {
  case count {
    0 -> [1]
    _ -> int_values(count - 1)
  }
}

fn string_values(count: Int) {
  case count {
    0 -> ["one"]
    _ -> string_values(count - 1)
  }
}

fn float_values(count: Int) {
  case count {
    0 -> [1.0]
    _ -> float_values(count - 1)
  }
}

fn bool_values(count: Int) {
  case count {
    0 -> [True]
    _ -> bool_values(count - 1)
  }
}

fn nil_values(count: Int) {
  case count {
    0 -> [Nil]
    _ -> nil_values(count - 1)
  }
}

fn tuple_values(count: Int) {
  case count {
    0 -> [#(1)]
    _ -> tuple_values(count - 1)
  }
}

fn list_values(count: Int) {
  case count {
    0 -> [[1]]
    _ -> list_values(count - 1)
  }
}

fn function_values(count: Int) {
  case count {
    0 -> [one]
    _ -> function_values(count - 1)
  }
}

pub fn main() {
  assert int_values(2) == [1]
  assert string_values(2) == ["one"]
  assert float_values(2) == [1.0]
  assert bool_values(2) == [True]
  assert nil_values(2) == [Nil]
  assert tuple_values(2) == [#(1)]
  assert list_values(2) == [[1]]
  let assert [function] = function_values(2)
  function(41)
}

// geam:expect Int(42)
