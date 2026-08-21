type Maybe(value) {
  Some(value)
  None
}

type Choice(value) {
  First(value)
  Second(value)
  Third(value)
}

fn list_case(values: List(Int)) {
  case values {
    [] -> 0
    [first, ..] -> first
  }
}

fn guarded_list_case(values: List(Int)) {
  case values {
    [first, ..] if first > 10 -> 0
    [] -> 0
    [first, ..] -> first
  }
}

fn alternative_list_case(values: List(Int)) {
  case values {
    [] | [_, _, ..] -> 0
    [only] -> only
  }
}

fn alternative_custom_case(choice: Choice(Int)) {
  case choice {
    First(value) | Second(value) -> value
    Third(value) -> value
  }
}

fn guarded_custom_case(choice: Choice(Int)) {
  case choice {
    First(value) if value > 10 -> 0
    First(value) -> value
    Second(value) | Third(value) -> value
  }
}

fn multiple_subject_case(left: Bool, right: Bool) {
  case left, right {
    True, True -> 0
    True, False -> 1
    False, _ -> 2
  }
}

fn nested_case(values: List(Int), maybe: Maybe(Int)) {
  case values, maybe {
    [], _ -> 0
    [_, ..], None -> 0
    [first, ..], Some(second) -> first + second
  }
}

fn nested_custom_case(maybe: Maybe(Maybe(Int))) {
  case maybe {
    None -> 0
    Some(None) -> 0
    Some(Some(value)) -> value
  }
}

pub fn main() {
  #(
    list_case([1]),
    guarded_list_case([2]),
    alternative_list_case([3]),
    alternative_custom_case(Second(4)),
    multiple_subject_case(True, False),
    nested_case([4], Some(5)),
    nested_custom_case(Some(Some(6))),
    guarded_custom_case(First(7)),
  )
}
// @geam:expect Tuple([Int(1), Int(2), Int(3), Int(4), Int(1), Int(9), Int(6), Int(7)])
