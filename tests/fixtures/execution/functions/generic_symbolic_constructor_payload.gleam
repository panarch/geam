pub type HandoffBox(value) {
  HandoffBox(
    function: fn(value) -> value,
    values: List(value),
    nested: List(List(value)),
  )
}

fn identity(value) {
  value
}

fn box_values(
  function: fn(value) -> value,
  values: List(value),
  nested: List(List(value)),
) {
  let make = HandoffBox
  make(function, values, nested)
}

pub fn main() {
  let box = box_values(identity, [], [[]])
  case box {
    HandoffBox(function:, values:, nested:) ->
      function == identity && values == [] && nested == [[]]
  }
}

// geam:expect Bool(true)
