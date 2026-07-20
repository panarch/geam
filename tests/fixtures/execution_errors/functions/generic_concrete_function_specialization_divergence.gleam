pub type Token {
  Token
}

fn fail() -> value {
  panic as "concrete function specialization failed"
}

fn provide(_trigger) -> fn(Int) -> value {
  fn(_value) { panic as "provided function must not run" }
}

fn function_call() -> fn(Int) -> value {
  let provider = provide
  provider(fail())
}

fn function_panic() -> fn(Int) -> value {
  panic as "specialized function panic"
}

fn call_int() -> fn(Int) -> Int { function_call() }
fn call_float() -> fn(Int) -> Float { function_call() }
fn call_string() -> fn(Int) -> String { function_call() }
fn call_bit_array() -> fn(Int) -> BitArray { function_call() }
fn call_utf_codepoint() -> fn(Int) -> UtfCodepoint { function_call() }
fn call_custom() -> fn(Int) -> Token { function_call() }
fn call_bool() -> fn(Int) -> Bool { function_call() }
fn call_nil() -> fn(Int) -> Nil { function_call() }
fn call_tuple() -> fn(Int) -> #(Int) { function_call() }
fn call_list() -> fn(Int) -> List(Int) { function_call() }
fn call_function() -> fn(Int) -> fn(Int) -> Int { function_call() }

fn panic_int() -> fn(Int) -> Int { function_panic() }
fn panic_float() -> fn(Int) -> Float { function_panic() }
fn panic_string() -> fn(Int) -> String { function_panic() }
fn panic_bit_array() -> fn(Int) -> BitArray { function_panic() }
fn panic_utf_codepoint() -> fn(Int) -> UtfCodepoint { function_panic() }
fn panic_custom() -> fn(Int) -> Token { function_panic() }
fn panic_bool() -> fn(Int) -> Bool { function_panic() }
fn panic_nil() -> fn(Int) -> Nil { function_panic() }
fn panic_tuple() -> fn(Int) -> #(Int) { function_panic() }
fn panic_list() -> fn(Int) -> List(Int) { function_panic() }
fn panic_function() -> fn(Int) -> fn(Int) -> Int { function_panic() }

pub fn main() {
  let _ = #(
    call_int == call_int,
    call_float == call_float,
    call_string == call_string,
    call_bit_array == call_bit_array,
    call_utf_codepoint == call_utf_codepoint,
    call_custom == call_custom,
    call_bool == call_bool,
    call_nil == call_nil,
    call_tuple == call_tuple,
    call_list == call_list,
    call_function == call_function,
    panic_int == panic_int,
    panic_float == panic_float,
    panic_string == panic_string,
    panic_bit_array == panic_bit_array,
    panic_utf_codepoint == panic_utf_codepoint,
    panic_custom == panic_custom,
    panic_bool == panic_bool,
    panic_nil == panic_nil,
    panic_tuple == panic_tuple,
    panic_list == panic_list,
    panic_function == panic_function,
  )
  call_int()
}

// geam:expect-error
// geam::panic
//
//   x panic: concrete function specialization failed
//    ,-[tests/fixtures/execution_errors/functions/generic_concrete_function_specialization_divergence.gleam:6:3]
//  5 | fn fail() -> value {
//  6 |   panic as "concrete function specialization failed"
//    :   ^^^^^^^^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^^^^^^^
//    :                            `-- panic in main.fail
//  7 | }
//    `----
