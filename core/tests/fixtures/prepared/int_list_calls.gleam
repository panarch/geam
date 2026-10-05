fn fold(values: List(Int), total: Int, calculate: fn(Int, Int) -> Int) -> Int {
  case values {
    [] -> total
    [first, ..rest] -> fold(rest, calculate(total, first), calculate)
  }
}

pub fn capturing_fold(values: List(Int), factor: Int, bias: Int) -> Int {
  let transform = fn(value) { value * factor + bias }
  fold(values, 0, fn(total, value) { total + transform(value) })
}

fn sum(values: List(Int), initial: Int) -> Int {
  fold(values, initial, fn(total, value) { total + value })
}

pub fn verify(values: List(Int), expected: Int) -> Bool {
  sum(values, 0) == expected
}

pub fn make_sum(prefix: List(Int), offset: Int) -> fn(List(Int)) -> Int {
  fn(values) { sum([offset, ..prefix], sum(values, 0)) }
}

pub fn make_check(prefix: List(Int)) -> fn(List(Int)) -> Bool {
  fn(values) {
    let same = prefix == values
    let different = prefix != values
    let total = sum(values, 0)
    case same {
      True -> total > 0
      False -> !different
    }
  }
}

pub fn selected(values: List(Int), other: List(Int), choose: Bool) -> Int {
  let calculate = make_sum(values, 7)
  let selected = case choose {
    True -> [1, 2, ..other]
    False -> []
  }
  calculate(selected)
}

fn echo_value(value: Int) -> Int {
  echo value
  value + 1
}

pub fn canonical(values: List(Int)) -> Int {
  fold(values, 0, fn(total, value) { total + echo_value(value) })
}

fn stopped(value: Int) -> Int {
  echo value
  panic as "list callback stopped"
}

pub fn failure(values: List(Int)) -> Int {
  fold(values, 0, fn(total, value) { total + stopped(value) })
}

fn identity_list(values: List(Int)) -> List(Int) {
  values
}

pub fn list_return(values: List(Int)) -> Int {
  sum(identity_list(values), 5)
}

pub fn non_tail(values: List(Int)) -> Int {
  case values {
    [] -> 0
    [first, ..rest] -> non_tail(rest) + first
  }
}

pub fn main() -> Int {
  capturing_fold([1, 2, 3], 2, 1)
}
