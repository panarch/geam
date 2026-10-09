@external(erlang, "native_loop", "observe")
fn observe(value: Int) -> Int

@external(erlang, "native_loop", "keep")
fn keep(value: a) -> a

@external(erlang, "native_loop", "begin")
fn begin() -> Nil

pub fn captured(count: Int, value: Int) -> Int {
  repeat(count, fn() { value })
}

pub fn computed(count: Int, value: Int) -> Int {
  repeat(count, fn() { value + 1 })
}

pub fn computed_cancellable(count: Int, value: Int) -> Int {
  begin()
  repeat(count, fn() { value + 1 })
}

pub fn ordinary_computed(count: Int, value: Int) -> Int {
  repeat(count, fn() { value + 1 }) + 1
}

pub fn dynamic_computed(count: Int, value: Int) -> Int {
  dispatch_repeat(repeat, count, fn() { value + 1 })
}

pub fn dynamic_computed_cancellable(count: Int, value: Int) -> Int {
  begin()
  dispatch_repeat(repeat, count, fn() { value + 1 })
}

fn dispatch_repeat(
  loop: fn(Int, fn() -> Int) -> Int,
  count: Int,
  producer: fn() -> Int,
) -> Int {
  loop(count, producer) + 1
}

pub fn cancellable(count: Int, value: Int) -> Int {
  begin()
  repeat(count, fn() { value })
}

fn repeat(count: Int, producer: fn() -> Int) -> Int {
  let result = observe(producer())
  case count {
    1 -> result
    _ -> repeat(count - 1, producer)
  }
}

pub fn retained_value(count: Int, value: Int) -> Int {
  repeat_opaque(count, fn() { value })
}

fn repeat_opaque(count: Int, producer: fn() -> Int) -> Int {
  let result = keep(producer())
  case count {
    1 -> result
    _ -> repeat_opaque(count - 1, producer)
  }
}

pub fn compound(value: List(Int)) -> List(Int) {
  repeat_compound(3, fn() { value })
}

fn repeat_compound(count: Int, producer: fn() -> List(Int)) -> List(Int) {
  let result = keep(producer())
  case count {
    1 -> result
    _ -> repeat_compound(count - 1, producer)
  }
}

@external(erlang, "native_loop", "observe_float")
fn observe_float(value: Float) -> Float

pub fn captured_float(count: Int, value: Float) -> Float {
  repeat_float(count, fn() { value })
}

fn repeat_float(count: Int, producer: fn() -> Float) -> Float {
  let result = observe_float(producer())
  case count {
    1 -> result
    _ -> repeat_float(count - 1, producer)
  }
}

@external(erlang, "native_loop", "observe_string")
fn observe_string(value: String) -> String

pub fn captured_string(count: Int, value: String) -> String {
  repeat_string(count, fn() { value })
}

fn repeat_string(count: Int, producer: fn() -> String) -> String {
  let result = observe_string(producer())
  case count {
    1 -> result
    _ -> repeat_string(count - 1, producer)
  }
}

@external(erlang, "native_loop", "observe_bit_array")
fn observe_bit_array(value: BitArray) -> BitArray

pub fn captured_bit_array(count: Int, value: BitArray) -> BitArray {
  repeat_bit_array(count, fn() { value })
}

fn repeat_bit_array(count: Int, producer: fn() -> BitArray) -> BitArray {
  let result = observe_bit_array(producer())
  case count {
    1 -> result
    _ -> repeat_bit_array(count - 1, producer)
  }
}

@external(erlang, "native_loop", "observe_utf_codepoint")
fn observe_utf_codepoint(value: UtfCodepoint) -> UtfCodepoint

pub fn captured_utf_codepoint(count: Int, value: UtfCodepoint) -> UtfCodepoint {
  repeat_utf_codepoint(count, fn() { value })
}

fn repeat_utf_codepoint(
  count: Int,
  producer: fn() -> UtfCodepoint,
) -> UtfCodepoint {
  let result = observe_utf_codepoint(producer())
  case count {
    1 -> result
    _ -> repeat_utf_codepoint(count - 1, producer)
  }
}

@external(erlang, "native_loop", "observe_bool")
fn observe_bool(value: Bool) -> Bool

pub fn captured_bool(count: Int, value: Bool) -> Bool {
  repeat_bool(count, fn() { value })
}

pub fn computed_bool(count: Int, value: Bool) -> Bool {
  repeat_bool(count, fn() { !value })
}

fn repeat_bool(count: Int, producer: fn() -> Bool) -> Bool {
  let result = observe_bool(producer())
  case count {
    1 -> result
    _ -> repeat_bool(count - 1, producer)
  }
}

@external(erlang, "native_loop", "observe_nil")
fn observe_nil(value: Nil) -> Nil

pub fn captured_nil(count: Int, value: Nil) -> Nil {
  repeat_nil(count, fn() { value })
}

fn repeat_nil(count: Int, producer: fn() -> Nil) -> Nil {
  let result = observe_nil(producer())
  case count {
    1 -> result
    _ -> repeat_nil(count - 1, producer)
  }
}

@external(erlang, "native_loop", "float_to_bool")
fn float_to_bool(value: Float) -> Bool

pub fn mixed(count: Int, value: Float) -> Bool {
  repeat_mixed(count, fn() { value })
}

fn repeat_mixed(count: Int, producer: fn() -> Float) -> Bool {
  let result = float_to_bool(producer())
  case count {
    1 -> result
    _ -> repeat_mixed(count - 1, producer)
  }
}

pub fn literal_nil(count: Int) -> Nil {
  repeat_nil(count, fn() { Nil })
}

pub fn computed_float(count: Int, value: Float) -> Float {
  repeat_float(count, fn() { value +. 1.0 })
}

pub fn retained_float(count: Int, value: Float) -> Float {
  repeat_generic(count, fn() { value })
}

pub fn retained_string(count: Int, value: String) -> String {
  repeat_generic(count, fn() { value })
}

pub fn retained_bit_array(count: Int, value: BitArray) -> BitArray {
  repeat_generic(count, fn() { value })
}

fn repeat_generic(count: Int, producer: fn() -> a) -> a {
  let result = keep(producer())
  case count {
    1 -> result
    _ -> repeat_generic(count - 1, producer)
  }
}

pub fn graph_captured(count: Int, value: Int) -> Int {
  repeat_graph(count, fn() { value })
}

fn graph_keep(value: Int) -> Int {
  value
}

fn repeat_graph(count: Int, producer: fn() -> Int) -> Int {
  let result = graph_keep(producer())
  case count {
    1 -> result
    _ -> repeat_graph(count - 1, producer)
  }
}

fn failing_producer() -> Int {
  panic as "producer failed"
}

pub fn producer_failure(count: Int) -> Int {
  repeat(count, failing_producer)
}
