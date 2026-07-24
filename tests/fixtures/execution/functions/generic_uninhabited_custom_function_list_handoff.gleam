pub type Boxed(value) {
  Boxed(value)
}

fn diverge() -> Boxed(value) {
  panic as "diverging function must not run"
}

fn choose(flag: Bool) -> fn() -> Boxed(value) {
  case flag {
    True -> case [diverge] {
      [function] -> function
      _ -> diverge
    }
    False -> diverge
  }
}

pub fn main() {
  let function = choose(True)
  function == function
}

// @geam:expect Bool(true)
