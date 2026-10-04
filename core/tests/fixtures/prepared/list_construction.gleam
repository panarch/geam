pub fn empty() -> List(Int) {
  []
}

pub fn numeric_tail(flag: Bool) -> List(Int) {
  case flag {
    True -> empty()
    False -> fixed()
  }
}

fn fixed() -> List(Int) {
  [7, -9]
}

pub fn prefix(first: Int, second: Int, tail: List(Int)) -> List(Int) {
  [first, second, ..tail]
}

pub fn choose(
  flag: Bool,
  first: Int,
  second: Int,
  left: List(Int),
  right: List(Int),
) -> List(Int) {
  let selected = case flag {
    True -> [first, second, ..left]
    False -> [second, first, ..right]
  }
  [first, ..selected]
}

pub fn reverse(values: List(Int)) -> List(Int) {
  reverse_loop(values, [])
}

fn reverse_loop(values: List(Int), result: List(Int)) -> List(Int) {
  case values {
    [] -> result
    [head, ..tail] -> reverse_loop(tail, [head, ..result])
  }
}

pub fn selected_reverse(values: List(Int), result: List(Int)) -> List(Int) {
  case values {
    [] -> reverse(result)
    [head, ..tail] ->
      case head % 2 {
        0 -> selected_reverse(tail, [head, ..result])
        _ -> selected_reverse(tail, result)
      }
  }
}

pub fn promoted(value: Int, tail: List(Int)) -> List(Int) {
  let next = value + 1
  [next, ..tail]
}

fn uncompiled(value: Int, tail: List(Int)) -> List(Int) {
  let text = "uncompiled"
  case text {
    "uncompiled" -> [value, ..tail]
    _ -> tail
  }
}

pub fn interpreted_tail(value: Int, tail: List(Int), flag: Bool) -> List(Int) {
  case flag {
    True -> uncompiled(value, tail)
    False -> uncompiled(0 - value, tail)
  }
}

pub fn main() -> List(Int) {
  selected_reverse([3, 4, 5, 6], [])
}

pub fn caller(
  values: List(Int),
  offset: Int,
  text: String,
) -> #(String, Int, List(Int), List(Int)) {
  let result = reverse([offset, ..values])
  #(text, offset, values, result)
}

fn forever(value: Int, values: List(Int)) -> List(Int) {
  case value >= 0 {
    True -> forever(value + 1, [value, ..values])
    False -> values
  }
}

pub fn running() -> List(Int) {
  echo "entered-construction"
  forever(0, [])
}
