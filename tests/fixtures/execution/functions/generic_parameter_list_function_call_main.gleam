fn empty() -> List(value) {
  []
}

pub fn main() {
  let function = empty
  let result = function()
  let _ = Nil
  result
}

// geam:expect List(Parameter(0))([])
