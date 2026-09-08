import async_provider/native
import geam/future
import gleam/option

pub type Marker { Marker(Int) }
pub type Never

pub fn manual_retained() -> future.Future(Bool) {
  let values = [Marker(20), Marker(22)]
  let captured = fn(value) { #(value + 2, values) }
  let original = native.manual_new(#(values, captured))
  let immediate = native.manual_keep(original)
  let assert True = native.manual_same_allocation(original, immediate)
  let assert True = native.manual_callback(original, fn(passed) {
    native.manual_same_allocation(original, passed)
  })
  use callback_same <- future.then(native.manual_callback_after(original, fn(passed) {
    native.manual_same_allocation(original, passed)
  }))
  let assert True = callback_same
  let #(kept_values, kept_function) = native.manual_read(immediate)
  let assert True = kept_values == values
  let assert True = kept_function(40) == #(42, values)
  use after <- future.then(native.manual_keep_after(original))
  let assert True = after == original
  let assert True = native.manual_same_allocation(original, after)
  use restored <- future.then(native.manual_read_after(after))
  let #(restored_values, restored_function) = restored
  let assert True = restored_values == values && restored_function(40) == #(42, values)
  use same <- future.then(native.manual_same_allocation_after(original, after))
  let assert True = same
  use created <- future.map(native.manual_new_after(restored))
  let assert False = native.manual_same_allocation(original, created)
  native.manual_read(created) == restored
}

pub fn manual_retained_work() -> future.Future(Bool) {
  let original = native.manual_new(future.ready(42))
  use work <- future.then(native.manual_read_after(original))
  use value <- future.map(work)
  value == 42
}

fn increment(value: Int) { value + 1 }
fn keep(value) { value }
fn same(value) { native.identity(value) == value }
fn work_same(value) {
  use returned <- future.map(native.identity_async(value))
  returned == value
}
fn callback_same(value) {
  use returned <- future.map(native.invoke_generic(keep, value))
  returned == value
}
fn all_true(values) {
  case values {
    [] -> True
    [True, ..rest] -> all_true(rest)
    _ -> False
  }
}
fn checks(values) { future.map(future.all(values), all_true) }

pub fn direct(value: Int) { native.double(value) }
pub fn awaited(value: Int) { native.add_one(value) }
pub fn stateful(value: Int) { native.add_to_state(value) }
pub fn stateful_direct(value: Int) { native.add_to_state_direct(value) }
pub fn list_direct(values: List(Int)) { native.list_identity(values) }
pub fn list_awaited(values: List(Int)) { native.list_identity_async(values) }

pub fn arity_families() {
  let assert <<a:utf8_codepoint, b:utf8_codepoint, c:utf8_codepoint, d:utf8_codepoint, e:utf8_codepoint>> = <<"abcde":utf8>>
  checks([
    future.map(native.arity_zero(), fn(value) { value == Nil }),
    future.map(native.arity_one(#(True)), fn(value) { value == #(True) }),
    future.map(native.arity_two(1.5, 2.25), fn(value) { value == 3.75 }),
    future.map(native.arity_three("a", "b", "c"), fn(value) { value == "abc" }),
    future.map(native.arity_four(<<1, 2>>, True, False, Nil), fn(value) { value == #(<<1, 2>>, False, Nil) }),
    future.map(native.arity_five(a, b, c, d, e), fn(value) { value == e }),
    future.map(native.arity_six(1, 2, 3, 4, 5, 6), fn(value) { value == 21 }),
    future.map(native.arity_seven(1, 1.5, "seven", <<3>>, a, True, Nil), fn(value) {
      value == #(1, 1.5, "seven", <<3>>, a, True, Nil)
    }),
  ])
}

pub fn direct_families() {
  let assert <<point:utf8_codepoint>> = <<"A":utf8>>
  let counter = native.new_counter(7)
  all_true([
    native.number_identity(7) == 7,
    native.sum_numbers([2, 3, 4]) == 9,
    native.float_identity(1.5) == 1.5,
    native.string_identity("text") == "text",
    native.bit_array_identity(<<1>>) == <<1>>,
    native.codepoint_identity(point) == point,
    native.bool_identity(True),
    native.nil_identity(Nil) == Nil,
    native.tuple_identity(#("tuple", 7)) == #("tuple", 7),
    native.pass_int_function(increment)(8) == 9,
    native.pass_function(increment)(9) == 10,
    native.invoke_immediate(increment, 40) == 41,
    native.return_nil() == Nil,
    native.scalar_list_matches([#(1.5, <<1>>, point, True, Nil)]),
    native.nested_list_matches([#("nested", #(7, True))]),
    native.first_envelope([native.wrap_counter(12)]) == 12,
    same(1), same(1.5), same("text"), same(<<1>>), same(point), same(True), same(Nil),
    same(#(1, "two")), same(Marker(3)), same(counter),
    same([1]), same(["one"]), same([<<1>>]), same([point]), same([Marker(5)]),
    same([1.5]), same([True]), same([Nil]), same([#(1, True)]), same([[1]]), same([counter]),
    native.identity(increment)(8) == 9,
    case native.identity([increment]) { [f] -> f(9) == 10 _ -> False },
  ])
}

pub fn owned_generic_families() {
  let assert <<point:utf8_codepoint>> = <<"A":utf8>>
  let counter = native.new_counter(7)
  checks([
    work_same(1), work_same(1.5), work_same("text"), work_same(<<1>>),
    work_same(point), work_same(True), work_same(Nil), work_same(#(1, "two")),
    work_same(Marker(3)), work_same(counter), work_same([1]), work_same(["one"]),
    work_same([<<1>>]), work_same([point]), work_same([Marker(5)]), work_same([1.5]),
    work_same([True]), work_same([Nil]), work_same([#(1, True)]), work_same([[1]]),
    work_same([counter]),
    future.map(native.identity_async(increment), fn(f) { f(8) == 9 }),
    future.map(native.identity_async([increment]), fn(fs) {
      case fs { [f] -> f(9) == 10 _ -> False }
    }),
  ])
}

fn return_function(_value) { increment }
fn return_counter_function(_value) { native.new_counter }
fn singleton(value) { [value] }
fn nested_singleton(value) { [[value]] }
fn empty() -> List(value) { [] }
fn nested_empty() -> List(List(value)) { [[]] }

pub fn callback_generic_families() {
  let assert <<point:utf8_codepoint>> = <<"A":utf8>>
  let counter = native.new_counter(7)
  checks([
    callback_same(43), callback_same(1.5), callback_same("text"), callback_same(<<1>>),
    callback_same(point), callback_same(True), callback_same(Nil), callback_same(#(1, "two")),
    callback_same(Marker(3)), callback_same(counter), callback_same([1]), callback_same(["one"]),
    callback_same([<<1>>]), callback_same([point]), callback_same([Marker(5)]),
    callback_same([1.5]), callback_same([True]), callback_same([Nil]),
    callback_same([#(1, True)]), callback_same([[1]]), callback_same([counter]),
    future.map(native.invoke_generic(keep, increment), fn(f) { f(8) == 9 }),
    future.map(native.invoke_generic(keep, [increment]), fn(fs) {
      case fs { [f] -> f(10) == 11 _ -> False }
    }),
    future.map(native.invoke_generic_map(Marker, 4), fn(x) { x == Marker(4) }),
    future.map(native.invoke_generic_map(return_function, 0), fn(f) { f(9) == 10 }),
    future.map(native.invoke_generic_map(return_counter_function, 0), fn(f) {
      native.read_counter(f(17)) == 17
    }),
    future.map(native.invoke_generic_map(singleton, 11), fn(xs) { xs == [11] }),
    future.map(native.invoke_generic_map(nested_singleton, 12), fn(xs) { xs == [[12]] }),
    future.map(native.invoke_generic_produce(empty), fn(xs) { case xs { [] -> True _ -> False } }),
    future.map(native.invoke_generic_produce(nested_empty), fn(xs) { case xs { [[]] -> True _ -> False } }),
  ])
}

fn transform(pair, result, option, values) {
  let #(number, label) = pair
  #(#(number + 10, label <> "!"), result, option, values)
}
fn scalars() {
  let assert <<point:utf8_codepoint>> = <<"A":utf8>>
  #(1.5, <<1>>, point, True, Nil)
}
fn return_increment() { increment }
fn apply_function(function, value) { function(value) }
fn inspect_external(counter, envelope) {
  let native.Wrapped(second) = envelope
  native.read_counter(counter) + native.read_counter(second)
}

pub fn callback_typed_families() {
  let captured = 1
  let unobserved = native.fail_async()
  let assert True = native.retain_future(unobserved)
  checks([
    future.map(native.invoke_twice(increment, 40), fn(x) { x == 42 }),
    future.map(native.invoke_future_twice(native.add_one, 40), fn(x) { x == 42 }),
    future.map(native.invoke_future_twice(fn(x) { native.add_one(x + captured) }, 38), fn(x) { x == 42 }),
    future.map(native.observe_future(native.numbers(42)), fn(x) { x == 42 }),
    future.map(native.observe_nested_future(future.ready(native.add_one(41))), fn(x) { x == 42 }),
    future.map(native.invoke_structured_callback(transform, 10), fn(x) { x == #(20, 11, 12, 13) }),
    future.map(native.invoke_scalar_callback(scalars), fn(x) { x == scalars() }),
    future.map(native.invoke_function_callback(return_increment), fn(f) { f(15) == 16 }),
    future.map(native.invoke_generic_map(native.pass_int_function, increment), fn(f) { f(15) == 16 }),
    future.map(native.invoke_function_argument(apply_function, increment, 15), fn(x) { x == 16 }),
    future.map(native.invoke_external_callback(inspect_external, 20), fn(x) { x == 41 }),
    future.map(native.invoke_box_callback(keep, native.box_value(44)), fn(x) { x == 44 }),
    future.map(native.cancel_queued_callback(fn(_) { panic as "cancelled callback ran" }, 42), fn(x) { x == 42 }),
  ])
}

pub fn owned_collections() {
  let tokens = [native.new_token("shared")]
  checks([
    future.ready(native.is_numbers(native.make_numbers(5))),
    future.ready(!native.is_numbers(native.make_counter_batch(5))),
    native.is_numbers_after(native.make_numbers(5)),
    future.map(native.is_numbers_after(native.make_counter_batch(5)), fn(x) { !x }),
    future.map(native.sum_numbers_async([2, 3, 4]), fn(x) { x == 9 }),
    native.qualified_bool_list([True]),
    future.map(native.first_token_async(tokens), fn(x) { x == native.first_token(tokens) }),
    future.map(native.list_identity_async([1, 2]), fn(xs) { xs == [1, 2] }),
    future.map(native.numbers(20), fn(xs) { xs == [20, 21] }),
    future.map(native.structured(10), fn(x) { x == #(10, Ok(11), option.Some(12)) }),
    future.map(native.summarize_batch_async(native.make_numbers(5)), fn(x) { x == #("numbers", 11) }),
    future.map(native.make_pairs("pair", 13), fn(x) { native.summarize_batch(x) == #("pair", 13) }),
    future.then(native.make_pairs("pair", 13), fn(x) {
      future.map(native.summarize_batch_async(x), fn(x) { x == #("pair", 13) })
    }),
    future.map(native.summarize_batch_async(native.make_counter_batch(45)), fn(x) { x == #("counters", 45) }),
    future.map(native.summarize_first_batch_async([native.make_counter_batch(45)]), fn(x) { x == #("counters", 45) }),
    future.ready(native.summarize_first_batch([native.make_numbers(6)]) == #("numbers", 13)),
    future.ready(native.summarize_first_batch([native.make_counter_batch(45)]) == #("counters", 45)),
  ])
}

pub fn owned_externals() {
  let box = native.box_value(9)
  let direct = native.unbox(box)
  use box <- future.then(native.rebox_async(box))
  use unboxed <- future.then(native.unbox_async(box))
  let counter = native.new_counter(30)
  let before = native.read_counter(counter)
  use incremented <- future.then(native.increment_counter(counter))
  let after = native.read_counter(counter)
  let counters = [native.new_counter(40), native.new_counter(50)]
  use updated <- future.then(native.increment_first_counter(counters))
  let envelope = native.wrap_counter(50)
  use envelope_increment <- future.map(native.increment_counter_envelope(envelope))
  all_true([
    direct == 9, unboxed == 9,
    before == 30, incremented == 31, after == 30,
    native.first_counter(counters) == 40, native.first_counter(updated) == 41,
    envelope_increment == 51, native.read_counter_envelope(envelope) == 50,
  ])
}

fn panic_immediately(_value: Int) -> Int { panic as "immediate callback panic" }
fn panicking_callback(_value: Int) -> Int { panic as "async callback panic" }
fn never_callback(_value: Int) -> Never { panic as "async never callback panic" }
pub fn callback_direct_panic() { native.invoke_immediate(panic_immediately, 1) }
pub fn callback_panic() { native.invoke_twice(panicking_callback, 1) }
pub fn callback_never() {
  use _ <- future.map(future.all([native.invoke_generic_map(never_callback, 1)]))
  0
}
pub fn direct_failure() { native.fail_direct() }
pub fn nil_failure() { native.fail_nil() }
pub fn future_failure() { native.fail_async() }
pub fn composed_failure() {
  use value <- future.then(native.add_one(1))
  use _ <- future.then(native.invoke_twice(increment, value))
  native.fail_async()
}

pub fn callback_future_failure() {
  native.invoke_future_twice(fn(_) { native.fail_async() }, 1)
}
