pub type Boxed {
  Boxed(Int)
}

fn stop() -> value { stop() }
fn int_value() -> Int { int_value() }
fn float_value() -> Float { float_value() }
fn string_value() -> String { string_value() }
fn bit_array_value() -> BitArray { bit_array_value() }
fn utf_codepoint_value() -> UtfCodepoint { utf_codepoint_value() }
fn custom_value() -> Boxed { custom_value() }
fn bool_value() -> Bool { bool_value() }
fn nil_value() -> Nil { nil_value() }
fn tuple_value() -> #(Int) { tuple_value() }

pub fn main() {
  let _ = #(
    stop,
    float_value,
    string_value,
    bit_array_value,
    utf_codepoint_value,
    custom_value,
    bool_value,
    nil_value,
    tuple_value,
  )
  int_value()
}

// geam:explain
// module main
// main int#0
//
// function never#0
//   graph entry=b0
//   b0 instructions=0 tail never#0 args=0
//
// function int#0
//   graph entry=b0
//   b0 instructions=10 tail int#1 args=0
//
// function int#1
//   graph entry=b0
//   b0 instructions=0 tail int#1 args=0
//
// function float#0
//   graph entry=b0
//   b0 instructions=0 tail float#0 args=0
//
// function string#0
//   graph entry=b0
//   b0 instructions=0 tail string#0 args=0
//
// function bit_array#0
//   graph entry=b0
//   b0 instructions=0 tail bit_array#0 args=0
//
// function utf_codepoint#0
//   graph entry=b0
//   b0 instructions=0 tail utf_codepoint#0 args=0
//
// function custom#0
//   graph entry=b0
//   b0 instructions=0 tail custom#0 args=0
//
// function bool#0
//   graph entry=b0
//   b0 instructions=0 tail bool#0 args=0
//
// function nil#0
//   graph entry=b0
//   b0 instructions=0 tail nil#0 args=0
//
// function tuple#0
//   graph entry=b0
//   b0 instructions=0 tail tuple#0 args=0
