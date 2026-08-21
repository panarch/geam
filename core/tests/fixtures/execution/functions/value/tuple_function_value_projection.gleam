fn add_one(value: Int) {
  value + 1
}

fn stringify(value: Int) {
  case value {
    42 -> "ok"
    _ -> "bad"
  }
}

fn to_float(value: Int) {
  value == 42
  42.5
}

fn is_answer(value: Int) {
  value == 42
}

fn ignore(_: Int) {
  Nil
}

fn to_tuple(value: Int) {
  #(value, "tuple")
}

fn tuple_score(pair: #(Int, String)) {
  pair.0
}

fn apply(callback: fn(Int) -> Int, value: Int) {
  callback(value)
}

fn get_add_one() {
  add_one
}

fn pack() {
  #(
    add_one,
    stringify,
    to_float,
    is_answer,
    ignore,
    to_tuple,
    get_add_one,
    tuple_score,
    apply,
    41,
  )
}

pub fn main() {
  let pair = pack()
  let value = pair.0(pair.9)
  let label = pair.1(value)
  let decimal = pair.2(value)
  let ok = pair.3(value)
  pair.4(value)
  let tuple = pair.5(value)
  let getter = pair.6()
  let tuple_argument = pair.7(tuple)
  let function_argument = pair.8(add_one, value - 1)

  case label == "ok" && decimal == 42.5 && ok && tuple.0 == 42 && tuple_argument == 42 && function_argument == 42 {
    True -> getter(value - 1)
    False -> 0
  }
}

// @geam:expect Int(42)
