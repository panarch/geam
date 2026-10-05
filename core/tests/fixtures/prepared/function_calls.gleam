fn identity(value: Int) -> Int {
  value
}

fn chain(previous: fn(Int) -> Int, depth: Int) -> fn(Int) -> Int {
  case depth <= 0 {
    True -> previous
    False -> chain(fn(value) { previous(value) + 1 }, depth - 1)
  }
}

fn compose(depth: Int) -> fn(Int) -> Int {
  chain(identity, depth)
}

pub fn capture_chain(depth: Int, value: Int) -> Int {
  let calculate = compose(depth)
  calculate(value)
}

fn offset(value: Int) -> Int {
  value + 7
}

fn selected(flag: Bool) -> fn(Int) -> Int {
  case flag {
    True -> identity
    False -> offset
  }
}

pub fn dynamic_target(flag: Bool, value: Int) -> Int {
  let calculate = selected(flag)
  calculate(value) + 3
}

fn non_tail(depth: Int) -> Int {
  case depth <= 0 {
    True -> 0
    False -> non_tail(depth - 1) + 1
  }
}

pub fn nested(depth: Int) -> Int {
  non_tail(depth) + 2
}

fn even(depth: Int) -> Bool {
  case depth <= 0 {
    True -> True
    False -> odd(depth - 1)
  }
}

fn odd(depth: Int) -> Bool {
  case depth <= 0 {
    True -> False
    False -> even(depth - 1)
  }
}

pub fn mutual(depth: Int) -> Bool {
  !even(depth)
}

fn negate(flag: Bool) -> Bool {
  !flag
}

fn predicate(offset: Int, previous: fn(Bool) -> Bool) -> fn(Int) -> Bool {
  fn(value) { previous(value > offset) }
}

pub fn callable_captures(offset: Int, value: Int) -> Bool {
  let check = predicate(offset, negate)
  check(value)
}

pub fn aliases(offset: Int, value: Int) -> Bool {
  let add = fn(input) { input + offset }
  let alias = add
  let repeated = fn(input) { alias(input) + add(input) }
  repeated(value) == 2 * { value + offset }
}

fn echo_value(value: Int) -> Int {
  echo value
  value + 1
}

pub fn canonical(value: Int) -> Int {
  echo_value(value) + 2
}

fn multiply(value: Int) -> Int {
  value * value
}

pub fn big_return(value: Int) -> Int {
  multiply(value) + 3
}

fn failing(value: Int) -> Int {
  let before = value + 1
  echo before
  panic as "call suffix"
}

pub fn failure(value: Int) -> Int {
  failing(value) + 3
}

fn int_suffix(offset: Int) -> fn(Int) -> Int {
  let next = offset + 1
  echo next
  fn(value) { value + next }
}

pub fn producer_suffix_int(offset: Int, value: Int) -> Int {
  let calculate = int_suffix(offset)
  calculate(value) + 2
}

fn bool_suffix(offset: Int) -> fn(Int) -> Bool {
  let next = offset + 1
  echo next
  fn(value) { value > next }
}

pub fn producer_suffix_bool(offset: Int, value: Int) -> Bool {
  let check = bool_suffix(offset)
  !check(value)
}

// String input keeps the outer loop canonical. Each iteration completes a
// separate generated callback with fresh captures and nested typed returns.
pub fn reuse_callback(value: Int, offset: Int) -> Int {
  let add = fn(input) { input + offset }
  add(value) + 1
}

fn repeat_canonical(
  remaining: Int,
  total: Int,
  offset: Int,
  marker: String,
) -> Int {
  case remaining <= 0 {
    True -> total
    False ->
      repeat_canonical(
        remaining - 1,
        reuse_callback(total, offset),
        offset + 1,
        marker,
      )
  }
}

pub fn repeated_roots(count: Int, initial: Int, offset: Int) -> Int {
  repeat_canonical(count, initial, offset, "canonical caller")
}

fn boolean_suffix(value: Int) -> Bool {
  let next = value + 1
  echo next
  next > 0
}

pub fn canonical_bool(value: Int) -> Bool {
  !boolean_suffix(value)
}

fn flagged_predicate(enabled: Bool, offset: Int) -> fn(Int) -> Bool {
  fn(value) {
    case enabled {
      True -> value > offset
      False -> False
    }
  }
}

fn forward_predicate(enabled: Bool, offset: Int) -> fn(Int) -> Bool {
  flagged_predicate(enabled, offset)
}

pub fn bool_captures(enabled: Bool, offset: Int, value: Int) -> Bool {
  let check = forward_predicate(enabled, offset)
  check(value)
}
