fn choose_first(value: Float, flag: Bool) -> Float {
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  case flag {
    True -> value
    False -> 0.0
  }
}

fn choose_second(value: Float, flag: Bool) -> Float {
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  let value = value +. 1.0
  case flag {
    True -> value
    False -> 0.0
  }
}

pub fn main() -> Float {
  let first_success = choose_first(2.0, True)
  let first_failure = choose_first(5.0, False)
  let second_success = choose_second(2.0, True)
  let second_failure = choose_second(5.0, False)
  first_success +. first_failure +. second_success +. second_failure
}
