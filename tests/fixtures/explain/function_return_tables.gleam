pub type Boxed {
  Boxed(Int)
}

fn int_function() -> fn() -> Int { int_function() }
fn float_function() -> fn() -> Float { float_function() }
fn string_function() -> fn() -> String { string_function() }
fn bit_array_function() -> fn() -> BitArray { bit_array_function() }
fn utf_codepoint_function() -> fn() -> UtfCodepoint { utf_codepoint_function() }
fn custom_function() -> fn() -> Boxed { custom_function() }
fn bool_function() -> fn() -> Bool { bool_function() }
fn nil_function() -> fn() -> Nil { nil_function() }
fn tuple_function() -> fn() -> #(Int) { tuple_function() }
fn generic_function() -> fn(value) -> value { generic_function() }
fn never_function() -> fn() -> value { never_function() }
fn function_function() -> fn() -> fn() -> Int { function_function() }

pub fn main() -> fn() -> Int {
  let _ = #(
    float_function,
    string_function,
    bit_array_function,
    utf_codepoint_function,
    custom_function,
    bool_function,
    nil_function,
    tuple_function,
    generic_function,
    never_function,
    function_function,
  )
  int_function()
}

// geam:explain
// module main
// main function.int#0
//
// function function.int#0
//   entry steps=1
//   graph entry=b0
//   b0 tail function.int#1 args=0
//
// function function.int#1
//   entry steps=0
//   graph entry=b0
//   b0 tail function.int#1 args=0
//
// function function.float#0
//   entry steps=0
//   graph entry=b0
//   b0 tail function.float#0 args=0
//
// function function.string#0
//   entry steps=0
//   graph entry=b0
//   b0 tail function.string#0 args=0
//
// function function.bit_array#0
//   entry steps=0
//   graph entry=b0
//   b0 tail function.bit_array#0 args=0
//
// function function.utf_codepoint#0
//   entry steps=0
//   graph entry=b0
//   b0 tail function.utf_codepoint#0 args=0
//
// function function.custom#0
//   entry steps=0
//   graph entry=b0
//   b0 tail function.custom#0 args=0
//
// function function.bool#0
//   entry steps=0
//   graph entry=b0
//   b0 tail function.bool#0 args=0
//
// function function.nil#0
//   entry steps=0
//   graph entry=b0
//   b0 tail function.nil#0 args=0
//
// function function.tuple#0
//   entry steps=0
//   graph entry=b0
//   b0 tail function.tuple#0 args=0
//
// function function.generic#0
//   entry steps=0
//   graph entry=b0
//   b0 tail function.generic#0 args=0
//
// function function.never#0
//   entry steps=0
//   graph entry=b0
//   b0 tail function.never#0 args=0
//
// function function.function#0
//   entry steps=0
//   graph entry=b0
//   b0 tail function.function#0 args=0
