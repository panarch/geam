fn fold(
  values: List(element),
  total: result,
  calculate: fn(result, element) -> result,
) -> result {
  case values {
    [] -> total
    [first, ..rest] -> fold(rest, calculate(total, first), calculate)
  }
}

fn keep_int(value: Int) -> Int {
  value
}

pub fn integers(
  values: List(Int),
  initial: Int,
  bias: Int,
  factor: Int,
) -> Int {
  let transform = fn(value) { value * factor + bias }
  fold(values, initial, fn(total, value) { total + keep_int(transform(value)) })
}

pub fn make_integers(prefix: List(Int), initial: Int) -> fn(List(Int)) -> Int {
  let keep = keep_int
  fn(values) {
    let from_values = fold(values, initial, fn(_, value) { keep(value) })
    fold([initial, ..prefix], from_values, fn(_, value) { keep(value) })
  }
}

fn keep_float(value: Float) -> Float {
  value
}

pub fn floats(
  values: List(Float),
  initial: Float,
  bias: Float,
  factor: Float,
) -> Float {
  let transform = fn(value) { { value +. bias } *. factor /. 2.0 }
  fold(values, initial, fn(total, value) {
    total +. keep_float(transform(value))
  })
}

pub fn make_floats(
  prefix: List(Float),
  initial: Float,
) -> fn(List(Float)) -> Float {
  let keep = keep_float
  fn(values) {
    let from_values = fold(values, initial, fn(_, value) { keep(value) })
    fold([initial, ..prefix], from_values, fn(_, value) { keep(value) })
  }
}

fn keep_bool(value: Bool) -> Bool {
  value
}

pub fn booleans(
  values: List(Bool),
  initial: Bool,
  bias: Bool,
  _factor: Bool,
) -> Bool {
  let transform = fn(value) { value == bias }
  fold(values, initial, fn(total, value) {
    total == keep_bool(transform(value))
  })
}

pub fn make_booleans(
  prefix: List(Bool),
  initial: Bool,
) -> fn(List(Bool)) -> Bool {
  let keep = keep_bool
  fn(values) {
    let from_values = fold(values, initial, fn(_, value) { keep(value) })
    fold([initial, ..prefix], from_values, fn(_, value) { keep(value) })
  }
}

fn keep_string(value: String) -> String {
  value
}

pub fn strings(
  values: List(String),
  initial: String,
  _bias: String,
  _factor: String,
) -> String {
  let transform = fn(value) { value }
  fold(values, initial, fn(_, value) { keep_string(transform(value)) })
}

pub fn make_strings(
  prefix: List(String),
  initial: String,
) -> fn(List(String)) -> String {
  let keep = keep_string
  fn(values) {
    let from_values = fold(values, initial, fn(_, value) { keep(value) })
    fold([initial, ..prefix], from_values, fn(_, value) { keep(value) })
  }
}

fn keep_bit_array(value: BitArray) -> BitArray {
  value
}

pub fn bit_arrays(
  values: List(BitArray),
  initial: BitArray,
  _bias: BitArray,
  _factor: BitArray,
) -> BitArray {
  let transform = fn(value) { value }
  fold(values, initial, fn(_, value) { keep_bit_array(transform(value)) })
}

pub fn make_bit_arrays(
  prefix: List(BitArray),
  initial: BitArray,
) -> fn(List(BitArray)) -> BitArray {
  let keep = keep_bit_array
  fn(values) {
    let from_values = fold(values, initial, fn(_, value) { keep(value) })
    fold([initial, ..prefix], from_values, fn(_, value) { keep(value) })
  }
}

fn keep_utf_codepoint(value: UtfCodepoint) -> UtfCodepoint {
  value
}

pub fn codepoints(
  values: List(UtfCodepoint),
  initial: UtfCodepoint,
  _bias: UtfCodepoint,
  _factor: UtfCodepoint,
) -> UtfCodepoint {
  let transform = fn(value) { value }
  fold(values, initial, fn(_, value) { keep_utf_codepoint(transform(value)) })
}

pub fn make_codepoints(
  prefix: List(UtfCodepoint),
  initial: UtfCodepoint,
) -> fn(List(UtfCodepoint)) -> UtfCodepoint {
  let keep = keep_utf_codepoint
  fn(values) {
    let from_values = fold(values, initial, fn(_, value) { keep(value) })
    fold([initial, ..prefix], from_values, fn(_, value) { keep(value) })
  }
}

fn keep_nil(value: Nil) -> Nil {
  value
}

pub fn nils(values: List(Nil), initial: Nil, _bias: Nil, _factor: Nil) -> Nil {
  let transform = fn(value) { value }
  fold(values, initial, fn(_, value) { keep_nil(transform(value)) })
}

pub fn make_nils(prefix: List(Nil), initial: Nil) -> fn(List(Nil)) -> Nil {
  let keep = keep_nil
  fn(values) {
    let from_values = fold(values, initial, fn(_, value) { keep(value) })
    fold([initial, ..prefix], from_values, fn(_, value) { keep(value) })
  }
}

pub fn count_floats(values: List(Float)) -> Int {
  fold(values, 0, fn(total, _) { total + 1 })
}

pub fn count_nils(values: List(Nil)) -> Int {
  fold(values, 0, fn(total, _) { total + 1 })
}

pub fn divide(value: Float, divisor: Float) -> Float {
  let operation = fn(input) { input /. divisor }
  operation(value)
}

fn observe_float(value: Float) -> Float {
  echo value
  value -. 0.25
}

pub fn canonical_floats(values: List(Float), initial: Float) -> Float {
  fold(values, initial, fn(total, value) { total +. observe_float(value) })
}

pub fn stopped_float(values: List(Float), initial: Float) -> Float {
  fold(values, initial, fn(total, value) {
    let current = total +. value
    echo current
    panic as "float callback stopped"
  })
}

pub fn unsupported(values: List(#(Int, Float))) -> Int {
  fold(values, 0, fn(total, value) { total + value.0 })
}

fn score(value: Bool) -> Int {
  case value {
    True -> 1
    False -> 0
  }
}

pub fn float_comparisons(left: Float, right: Float) -> Int {
  score(left <. right)
  + 2
  * score(left <=. right)
  + 4
  * score(left >. right)
  + 8
  * score(left >=. right)
  + 16
  * score(left == right)
  + 32
  * score(left != right)
  + 64
  * score(left == left)
  + 128
  * score(left != left)
  + 256
  * score(left <=. left)
  + 512
  * score(left >=. left)
}

fn list_checks(
  values: List(element),
  value: element,
  expected: List(element),
) -> Int {
  let single = [value]
  let extended = [value, ..values]
  let same = score(extended == expected)
  let different = 2 * score(single != expected)
  let suffix = case values {
    [] -> 0
    [_, ..rest] ->
      case rest {
        [] -> 4
        _ -> 8
      }
  }
  same + different + suffix
}

pub fn check_integers(
  values: List(Int),
  value: Int,
  expected: List(Int),
) -> Int {
  list_checks(values, value, expected)
}

pub fn check_floats(
  values: List(Float),
  value: Float,
  expected: List(Float),
) -> Int {
  list_checks(values, value, expected)
}

pub fn check_booleans(
  values: List(Bool),
  value: Bool,
  expected: List(Bool),
) -> Int {
  list_checks(values, value, expected)
}

pub fn check_strings(
  values: List(String),
  value: String,
  expected: List(String),
) -> Int {
  list_checks(values, value, expected)
}

pub fn check_bit_arrays(
  values: List(BitArray),
  value: BitArray,
  expected: List(BitArray),
) -> Int {
  list_checks(values, value, expected)
}

pub fn check_codepoints(
  values: List(UtfCodepoint),
  value: UtfCodepoint,
  expected: List(UtfCodepoint),
) -> Int {
  list_checks(values, value, expected)
}

pub fn check_nils(values: List(Nil), value: Nil, expected: List(Nil)) -> Int {
  list_checks(values, value, expected)
}

fn forward_integers(prefix: List(Int), initial: Int) -> fn(List(Int)) -> Int {
  make_integers(prefix, initial)
}

pub fn through_integers(
  prefix: List(Int),
  initial: Int,
  values: List(Int),
) -> Int {
  let calculate = forward_integers(prefix, initial)
  let result = calculate(values)
  keep_int(result)
}

fn forward_floats(
  prefix: List(Float),
  initial: Float,
) -> fn(List(Float)) -> Float {
  make_floats(prefix, initial)
}

pub fn through_floats(
  prefix: List(Float),
  initial: Float,
  values: List(Float),
) -> Float {
  let calculate = forward_floats(prefix, initial)
  let result = calculate(values)
  keep_float(result) -. 1.0
}

fn forward_booleans(
  prefix: List(Bool),
  initial: Bool,
) -> fn(List(Bool)) -> Bool {
  make_booleans(prefix, initial)
}

pub fn through_booleans(
  prefix: List(Bool),
  initial: Bool,
  values: List(Bool),
) -> Bool {
  let calculate = forward_booleans(prefix, initial)
  let result = calculate(values)
  keep_bool(result)
}

fn forward_strings(
  prefix: List(String),
  initial: String,
) -> fn(List(String)) -> String {
  make_strings(prefix, initial)
}

pub fn through_strings(
  prefix: List(String),
  initial: String,
  values: List(String),
) -> String {
  let calculate = forward_strings(prefix, initial)
  let result = calculate(values)
  let result = keep_string(result)
  case result {
    "마" <> rest -> rest
    _ -> result
  }
}

fn forward_bit_arrays(
  prefix: List(BitArray),
  initial: BitArray,
) -> fn(List(BitArray)) -> BitArray {
  make_bit_arrays(prefix, initial)
}

pub fn through_bit_arrays(
  prefix: List(BitArray),
  initial: BitArray,
  values: List(BitArray),
) -> BitArray {
  let calculate = forward_bit_arrays(prefix, initial)
  let result = calculate(values)
  keep_bit_array(result)
}

fn forward_codepoints(
  prefix: List(UtfCodepoint),
  initial: UtfCodepoint,
) -> fn(List(UtfCodepoint)) -> UtfCodepoint {
  make_codepoints(prefix, initial)
}

pub fn through_codepoints(
  prefix: List(UtfCodepoint),
  initial: UtfCodepoint,
  values: List(UtfCodepoint),
) -> UtfCodepoint {
  let calculate = forward_codepoints(prefix, initial)
  let result = calculate(values)
  keep_utf_codepoint(result)
}

fn forward_nils(prefix: List(Nil), initial: Nil) -> fn(List(Nil)) -> Nil {
  make_nils(prefix, initial)
}

pub fn through_nils(prefix: List(Nil), initial: Nil, values: List(Nil)) -> Nil {
  let calculate = forward_nils(prefix, initial)
  let result = calculate(values)
  keep_nil(result)
}

fn prefix_length(value: String, count: Int) -> Int {
  case value {
    "x" <> rest -> prefix_length(rest, count + 1)
    "" -> count
    _ -> panic as "unexpected prefix"
  }
}

pub fn call_prefix_length(value: String) -> Int {
  let count = prefix_length(value, 0)
  keep_int(count)
}

fn strip_tag(value: String) -> String {
  case value {
    "tag:" <> rest -> rest
    _ -> value
  }
}

pub fn call_string_slice(value: String, tail: Bool) -> String {
  let sliced = strip_tag(value)
  case tail {
    True -> strip_tag(sliced)
    False -> {
      let result = strip_tag(sliced)
      keep_string(result)
    }
  }
}

pub fn call_string_slice_echo(value: String) -> String {
  let sliced = strip_tag(value)
  echo sliced
  strip_tag(sliced)
}

fn strip_bit_prefix(value: BitArray) -> BitArray {
  case value {
    <<1:size(1), rest:bits>> -> rest
    _ -> value
  }
}

pub fn call_bit_slice(value: BitArray, tail: Bool) -> BitArray {
  let sliced = strip_bit_prefix(value)
  case tail {
    True -> strip_bit_prefix(sliced)
    False -> {
      let result = strip_bit_prefix(sliced)
      keep_bit_array(result)
    }
  }
}

fn checksum_bytes(input: BitArray, total: Int) -> Int {
  case input {
    <<value:8, rest:bits>> -> checksum_bytes(rest, total + value)
    <<>> -> total
    _ -> panic as "incomplete byte"
  }
}

fn apply_integer(calculate: fn() -> Int) -> Int {
  calculate()
}

pub fn call_unconnected_bit_checksum(input: BitArray) -> Int {
  let checksum = fn() { checksum_bytes(input, 0) }
  let constant = fn() { 7 }
  apply_integer(checksum) + apply_integer(constant)
}
