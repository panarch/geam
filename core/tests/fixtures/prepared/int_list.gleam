pub fn count(values: List(Int), total: Int) -> Int {
  case values {
    [] -> total
    [1, ..tail] -> count(tail, total + 1)
    [_, ..tail] -> count(tail, total)
  }
}

pub fn asserted(values: List(Int), total: Int) -> Int {
  case values {
    [] -> total
    _ -> {
      let assert [head, ..tail] = values
      let total = case head {
        1 -> total + 1
        _ -> total
      }
      asserted(tail, total)
    }
  }
}

pub fn equal_walk(values: List(Int), stop: List(Int), total: Int) -> Int {
  case values == stop {
    True -> total
    False ->
      case values {
        [] -> total
        [1, ..tail] -> equal_walk(tail, stop, total + 1)
        [_, ..tail] -> equal_walk(tail, stop, total)
      }
  }
}

pub fn prefix(values: List(Int)) -> Int {
  let assert [0, first as alias, _, second, ..tail] as original = values
  case tail == original {
    True -> alias
    False -> first + second
  }
}

pub fn same(left: List(Int), right: List(Int), negate: Bool) -> Bool {
  case negate {
    True -> left != right
    False -> left == right
  }
}

pub fn shuffle(left: List(Int), right: List(Int), steps: Int) -> Int {
  case steps <= 0 {
    True -> {
      let assert [first, ..] = left
      let assert [second, ..] = right
      first - second
    }
    False -> shuffle(right, left, steps - 1)
  }
}

pub fn duplicate(values: List(Int), other: List(Int), steps: Int) -> Int {
  case steps {
    0 -> {
      let assert [head, ..] = values
      case values == other {
        True -> head
        False -> -head
      }
    }
    _ -> duplicate(values, values, steps - 1)
  }
}

pub fn captured(values: List(Int), offset: Int) -> Int {
  let calculate = fn(input: Int) {
    let assert [head, ..] = values
    case input < 0 {
      True -> head - offset
      False -> head + offset + input
    }
  }
  calculate(3)
}

pub fn caller(
  values: List(Int),
  offset: Int,
  text: String,
) -> #(String, Int, List(Int), Int) {
  let first = count(values, offset)
  #(text, first, values, captured(values, offset))
}

pub fn stop(values: List(Int)) -> Int {
  case values {
    [] -> panic
    [head, ..] -> head
  }
}

pub fn late(values: List(Int)) -> Int {
  let assert [head, 2, ..tail] as original = values
  case tail == original {
    True -> head
    False -> head + 3
  }
}

fn forever(values: List(Int), total: Int) -> Int {
  case values {
    [] -> total
    _ -> forever(values, total + 1)
  }
}

pub fn running() -> Int {
  echo "entered-list"
  forever([1], 0)
}

pub fn main() -> Int {
  count([1, 0, 1], 5) + asserted([0, 1], 2) + equal_walk([1, 1], [], 0)
}
