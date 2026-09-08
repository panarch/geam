import geam/future.{type Future}

@external(erlang, "native", "is_numbers")
pub fn is_numbers(value: Batch) -> Bool
@external(erlang, "native", "is_numbers_after")
pub fn is_numbers_after(value: Batch) -> Future(Bool)

pub type Manual(item)

@external(erlang, "native", "manual_new")
pub fn manual_new(value: item) -> Manual(item)
@external(erlang, "native", "manual_keep")
pub fn manual_keep(value: Manual(item)) -> Manual(item)
@external(erlang, "native", "manual_callback")
pub fn manual_callback(value: Manual(item), callback: fn(Manual(item)) -> Bool) -> Bool
@external(erlang, "native", "manual_callback_after")
pub fn manual_callback_after(value: Manual(item), callback: fn(Manual(item)) -> Bool) -> Future(Bool)
@external(erlang, "native", "manual_read")
pub fn manual_read(value: Manual(item)) -> item
@external(erlang, "native", "manual_keep_after")
pub fn manual_keep_after(value: Manual(item)) -> Future(Manual(item))
@external(erlang, "native", "manual_read_after")
pub fn manual_read_after(value: Manual(item)) -> Future(item)
@external(erlang, "native", "manual_same_allocation")
pub fn manual_same_allocation(left: Manual(item), right: Manual(item)) -> Bool
@external(erlang, "native", "manual_same_allocation_after")
pub fn manual_same_allocation_after(left: Manual(item), right: Manual(item)) -> Future(Bool)
@external(erlang, "native", "manual_new_after")
pub fn manual_new_after(value: item) -> Future(Manual(item))
import gleam/option.{type Option}
import async_provider/declarations

@external(erlang, "native", "double")
pub fn double(value: Int) -> Int

@external(erlang, "native", "arity_zero")
pub fn arity_zero() -> Future(Nil)

@external(erlang, "native", "arity_one")
pub fn arity_one(value: #(Bool)) -> Future(#(Bool))

@external(erlang, "native", "arity_two")
pub fn arity_two(a: Float, b: Float) -> Future(Float)

@external(erlang, "native", "arity_three")
pub fn arity_three(a: String, b: String, c: String) -> Future(String)

@external(erlang, "native", "arity_four")
pub fn arity_four(a: BitArray, b: Bool, c: Bool, d: Nil) -> Future(#(BitArray, Bool, Nil))

@external(erlang, "native", "arity_five")
pub fn arity_five(a: UtfCodepoint, b: UtfCodepoint, c: UtfCodepoint, d: UtfCodepoint, e: UtfCodepoint) -> Future(UtfCodepoint)

@external(erlang, "native", "arity_six")
pub fn arity_six(a: Int, b: Int, c: Int, d: Int, e: Int, f: Int) -> Future(Int)

@external(erlang, "native", "arity_seven")
pub fn arity_seven(a: Int, b: Float, c: String, d: BitArray, e: UtfCodepoint, f: Bool, g: Nil) -> Future(#(Int, Float, String, BitArray, UtfCodepoint, Bool, Nil))

@external(erlang, "native", "number_identity")
pub fn number_identity(value: Int) -> Int

@external(erlang, "native", "sum_numbers")
pub fn sum_numbers(values: List(Int)) -> Int

@external(erlang, "native", "sum_numbers_async")
pub fn sum_numbers_async(values: List(Int)) -> Future(Int)

@external(erlang, "native", "qualified_bool_list")
pub fn qualified_bool_list(values: List(Bool)) -> Future(Bool)

@external(erlang, "native", "float_identity")
pub fn float_identity(value: Float) -> Float

@external(erlang, "native", "string_identity")
pub fn string_identity(value: String) -> String

@external(erlang, "native", "bit_array_identity")
pub fn bit_array_identity(value: BitArray) -> BitArray

@external(erlang, "native", "codepoint_identity")
pub fn codepoint_identity(value: UtfCodepoint) -> UtfCodepoint

@external(erlang, "native", "bool_identity")
pub fn bool_identity(value: Bool) -> Bool

@external(erlang, "native", "nil_identity")
pub fn nil_identity(value: Nil) -> Nil

@external(erlang, "native", "tuple_identity")
pub fn tuple_identity(value: #(String, Int)) -> #(String, Int)

@external(erlang, "native", "pass_function")
pub fn pass_function(callback: fn(value) -> value) -> fn(value) -> value

@external(erlang, "native", "pass_int_function")
pub fn pass_int_function(callback: fn(Int) -> Int) -> fn(Int) -> Int

@external(erlang, "native", "fail_direct")
pub fn fail_direct() -> Int

@external(erlang, "native", "fail_nil")
pub fn fail_nil() -> Nil

@external(erlang, "native", "return_nil")
pub fn return_nil() -> Nil

@external(erlang, "native", "new_token")
pub fn new_token(value: String) -> declarations.Token

@external(erlang, "native", "first_token")
pub fn first_token(values: List(declarations.Token)) -> String

@external(erlang, "native", "first_token_async")
pub fn first_token_async(values: List(declarations.Token)) -> Future(String)

@external(erlang, "native", "add_one")
pub fn add_one(value: Int) -> Future(Int)

@external(erlang, "native", "add_to_state")
pub fn add_to_state(value: Int) -> Future(Int)

@external(erlang, "native", "add_to_state_direct")
pub fn add_to_state_direct(value: Int) -> #(Int, Int)

@external(erlang, "native", "invoke_immediate")
pub fn invoke_immediate(callback: fn(Int) -> Int, value: Int) -> Int

@external(erlang, "native", "invoke_twice")
pub fn invoke_twice(callback: fn(Int) -> Int, value: Int) -> Future(Int)

@external(erlang, "native", "invoke_future_twice")
pub fn invoke_future_twice(callback: fn(Int) -> Future(Int), value: Int) -> Future(Int)

@external(erlang, "native", "observe_future")
pub fn observe_future(value: Future(List(Int))) -> Future(Int)

@external(erlang, "native", "retain_future")
pub fn retain_future(value: Future(Int)) -> Bool

@external(erlang, "native", "observe_nested_future")
pub fn observe_nested_future(value: Future(Future(Int))) -> Future(Int)

@external(erlang, "native", "cancel_queued_callback")
pub fn cancel_queued_callback(callback: fn(Int) -> Int, value: Int) -> Future(Int)

	@external(erlang, "native", "invoke_generic")
	pub fn invoke_generic(callback: fn(value) -> value, value: value) -> Future(value)

	@external(erlang, "native", "invoke_generic_map")
	pub fn invoke_generic_map(callback: fn(input) -> output, value: input) -> Future(output)

	@external(erlang, "native", "invoke_generic_produce")
	pub fn invoke_generic_produce(callback: fn() -> output) -> Future(output)

@external(erlang, "native", "invoke_structured_callback")
pub fn invoke_structured_callback(
  callback: fn(
    #(Int, String),
    Result(Int, String),
    Option(Int),
    List(Int),
  ) -> #(#(Int, String), Result(Int, String), Option(Int), List(Int)),
  value: Int,
) -> Future(#(Int, Int, Int, Int))

@external(erlang, "native", "invoke_scalar_callback")
pub fn invoke_scalar_callback(
  callback: fn() -> #(Float, BitArray, UtfCodepoint, Bool, Nil),
) -> Future(#(Float, BitArray, UtfCodepoint, Bool, Nil))

@external(erlang, "native", "invoke_function_callback")
pub fn invoke_function_callback(
  callback: fn() -> fn(Int) -> Int,
) -> Future(fn(Int) -> Int)

@external(erlang, "native", "invoke_function_argument")
pub fn invoke_function_argument(
  callback: fn(fn(Int) -> Int, Int) -> Int,
  function: fn(Int) -> Int,
  value: Int,
) -> Future(Int)

@external(erlang, "native", "invoke_external_callback")
pub fn invoke_external_callback(
  callback: fn(Counter, CounterEnvelope) -> Int,
  value: Int,
) -> Future(Int)

@external(erlang, "native", "invoke_box_callback")
pub fn invoke_box_callback(
  callback: fn(Box(value)) -> Box(value),
  boxed: Box(value),
) -> Future(value)

@external(erlang, "native", "fail_async")
pub fn fail_async() -> Future(Int)

@external(erlang, "native", "list_identity")
pub fn list_identity(values: List(Int)) -> List(Int)

@external(erlang, "native", "list_identity_async")
pub fn list_identity_async(values: List(Int)) -> Future(List(Int))

@external(erlang, "native", "scalar_list_matches")
pub fn scalar_list_matches(
  values: List(#(Float, BitArray, UtfCodepoint, Bool, Nil)),
) -> Bool

@external(erlang, "native", "nested_list_matches")
pub fn nested_list_matches(values: List(#(String, #(Int, Bool)))) -> Bool

@external(erlang, "native", "first_envelope")
pub fn first_envelope(values: List(CounterEnvelope)) -> Int

@external(erlang, "native", "identity")
pub fn identity(value: value) -> value

@external(erlang, "native", "identity_async")
pub fn identity_async(value: value) -> Future(value)

@external(erlang, "native", "Box")
pub type Box(value)

@external(erlang, "native", "box_value")
pub fn box_value(value: value) -> Box(value)

@external(erlang, "native", "unbox")
pub fn unbox(boxed: Box(value)) -> value

@external(erlang, "native", "unbox_async")
pub fn unbox_async(boxed: Box(value)) -> Future(value)

@external(erlang, "native", "rebox_async")
pub fn rebox_async(boxed: Box(value)) -> Future(Box(value))

@external(erlang, "native", "structured")
pub fn structured(value: Int) -> Future(#(Int, Result(Int, Int), Option(Int)))

@external(erlang, "native", "numbers")
pub fn numbers(value: Int) -> Future(List(Int))

@external(erlang, "native", "Counter")
pub type Counter

@external(erlang, "native", "new_counter")
pub fn new_counter(value: Int) -> Counter

@external(erlang, "native", "read_counter")
pub fn read_counter(counter: Counter) -> Int

@external(erlang, "native", "increment_counter")
pub fn increment_counter(counter: Counter) -> Future(Int)

@external(erlang, "native", "first_counter")
pub fn first_counter(counters: List(Counter)) -> Int

@external(erlang, "native", "increment_first_counter")
pub fn increment_first_counter(counters: List(Counter)) -> Future(List(Counter))

pub type CounterEnvelope {
  Wrapped(Counter)
}

@external(erlang, "native", "wrap_counter")
pub fn wrap_counter(value: Int) -> CounterEnvelope

@external(erlang, "native", "read_counter_envelope")
pub fn read_counter_envelope(value: CounterEnvelope) -> Int

@external(erlang, "native", "increment_counter_envelope")
pub fn increment_counter_envelope(value: CounterEnvelope) -> Future(Int)

pub type Batch {
  Numbers(List(Int))
  Pairs(List(#(String, Int)))
  Counters(List(Counter))
}

@external(erlang, "native", "make_numbers")
pub fn make_numbers(value: Int) -> Batch

@external(erlang, "native", "make_counter_batch")
pub fn make_counter_batch(value: Int) -> Batch

@external(erlang, "native", "make_pairs")
pub fn make_pairs(label: String, value: Int) -> Future(Batch)

@external(erlang, "native", "summarize_batch")
pub fn summarize_batch(value: Batch) -> #(String, Int)

@external(erlang, "native", "summarize_first_batch")
pub fn summarize_first_batch(values: List(Batch)) -> #(String, Int)

@external(erlang, "native", "summarize_first_batch_async")
pub fn summarize_first_batch_async(values: List(Batch)) -> Future(#(String, Int))

@external(erlang, "native", "summarize_batch_async")
pub fn summarize_batch_async(value: Batch) -> Future(#(String, Int))
