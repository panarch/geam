fn is_empty(values: List(value)) {
  case values {
    [] -> True
    _ -> False
  }
}

fn preserve_empty(values: List(value)) {
  case values {
    [first, ..tail] -> [first, ..tail]
    _ -> []
  }
}

fn preserve_empty_local(values: List(value)) {
  let result = case values {
    [first, ..tail] -> [first, ..tail]
    _ -> []
  }
  result
}

fn tail_or_empty(values: List(value)) {
  case values {
    [_, ..tail] -> tail
    _ -> []
  }
}

fn total_tail(values: List(value)) {
  case values {
    [..tail] -> tail
  }
}

pub fn main() {
  #(
    is_empty([]),
    preserve_empty([]),
    preserve_empty_local([]),
    tail_or_empty([]),
    total_tail([]),
  )
}

// @geam:expect Tuple([Bool(true), List(Parameter(0))([]), List(Parameter(1))([]), List(Parameter(2))([]), List(Parameter(3))([])])
