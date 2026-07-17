pub type Boxed(value) {
  Boxed(value: value, label: String)
}

fn first_or(values: List(value), fallback: value) -> value {
  case values {
    [first, ..] -> first
    _ -> fallback
  }
}

fn apply(function: fn(value) -> result, value: value) -> result {
  function(value)
}

fn identity(value: value) -> value {
  value
}

fn capture(value: value) -> fn() -> value {
  fn() { value }
}

fn relabel(box: Boxed(value), label: String) -> Boxed(value) {
  Boxed(..box, label: label)
}

fn choose_list(condition: Bool, first: List(value), second: List(value)) -> List(value) {
  case condition {
    True -> first
    False -> second
  }
}

pub fn main() {
  let captured = capture("captured")
  #(
    first_or([1], 0),
    first_or([], "fallback"),
    apply(identity, 2.5),
    captured(),
    relabel(Boxed(value: 3, label: "old"), "new"),
    choose_list(True, [4], [5]),
    choose_list(False, ["left"], ["right"]),
  )
}

// geam:expect Tuple([Int(1), String("fallback"), Float(2.5), String("captured"), Custom(type=geam/main/Boxed(Int), constructor=Boxed#0, fields=[value: Int(3), label: String("new")]), List(Int)([Int(4)]), List(String)([String("right")])])
