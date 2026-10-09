fn keep(value: String) -> String {
  value
}

fn spin(flag: Bool) -> Bool {
  spin(flag)
}

pub fn choose(flag: Bool) -> String {
  let value = keep("kept")
  case flag {
    True -> {
      let _ = spin(True)
      value
    }
    False -> value
  }
}

fn identity(value: Int) -> Int {
  value
}

pub fn wide() -> Int {
  let calculate = identity
  let value = calculate(1_099_511_627_776)
  value * value * value * value
}
