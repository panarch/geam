fn nested() -> List(List(value)) {
  [[]]
}

fn provide() -> fn() -> List(List(value)) {
  nested
}

pub fn main() {
  let function = provide()
  let result = function()
  let _ = Nil
  result
}

// geam:expect List(List(Parameter(0)))([List(Parameter(0))([])])
