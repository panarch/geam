pub fn arithmetic(remaining: Int, total: Int) -> Int {
  case remaining {
    0 -> total
    n ->
      case n % 2 == 0 {
        True -> arithmetic(n - 1, total + n * 3 + 1)
        False -> arithmetic(n - 1, total + n * 2 - 1)
      }
  }
}

pub fn shuffle(remaining: Int, left: Int, flag: Bool, right: Int) -> Int {
  case remaining <= 0 {
    True ->
      case flag {
        True -> left - right
        False -> right - left
      }
    False -> shuffle(remaining - 1, right, !flag, left)
  }
}

pub fn choice(value: Int, flag: Bool) -> Bool {
  case value < 0 {
    True -> !flag
    False ->
      case value >= 10 {
        True -> flag
        False -> False
      }
  }
}

pub fn switch(value: Int) -> Int {
  let selected = case value {
    0 -> -3
    1 -> 2
    _ -> value
  }
  case selected > 0 {
    True -> selected + 42
    False -> -selected
  }
}

pub fn quotient(left: Int, right: Int, negate: Bool) -> Int {
  let divided = left / right
  case negate {
    True -> -divided
    False -> divided
  }
}

pub fn operators(left: Int, right: Int, flag: Bool) -> Int {
  let product = left * right + left - right
  let value = -product
  case flag {
    True -> value / right
    False -> value % right
  }
}

pub fn divmod(left: Int, right: Int, flag: Bool) -> Int {
  let value = left / right + left % right
  case flag {
    True -> value
    False -> -value
  }
}

pub fn product(left: Int, right: Int) -> Int {
  let multiply = case left == right {
    True -> False
    False -> True
  }
  case multiply {
    True -> left * right
    False -> left
  }
}

pub fn discarded(value: Int, flag: Bool) -> Int {
  let _discarded = value + 1 + 2
  case flag {
    True -> value
    False -> -value
  }
}

pub fn captured(value: Int, offset: Int) -> Int {
  let calculate = fn(input) {
    case input >= 0 {
      True -> input + offset
      False -> offset - input
    }
  }
  calculate(value)
}

pub fn caller(
  value: Int,
  remaining: Int,
  text: String,
) -> #(String, Int, List(Int), Int) {
  let values = [value, remaining]
  let first = arithmetic(remaining, value)
  #(text, first, values, captured(value, remaining))
}

pub fn main() -> Int {
  arithmetic(12, 0) + shuffle(3, 7, True, 11) + switch(1)
}

pub fn running() -> Int {
  echo "entered"
  arithmetic(-1, 0)
}
