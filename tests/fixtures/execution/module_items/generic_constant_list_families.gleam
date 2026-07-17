pub type Token {
  Token(Int)
}

fn add_one(value: Int) {
  value + 1
}

const empty = []
const empty_alias = empty
const nested = [empty_alias]

fn forwarded_empty() -> List(value) {
  empty_alias
}

fn empty_int(values: List(Int)) { values == [] }
fn empty_string(values: List(String)) { values == [] }
fn empty_bit_array(values: List(BitArray)) { values == [] }
fn empty_utf_codepoint(values: List(UtfCodepoint)) { values == [] }
fn empty_custom(values: List(Token)) { values == [] }
fn empty_float(values: List(Float)) { values == [] }
fn empty_bool(values: List(Bool)) { values == [] }
fn empty_nil(values: List(Nil)) { values == [] }
fn empty_tuple(values: List(#(Int, String))) { values == [] }
fn empty_list(values: List(List(Int))) { values == [] }
fn empty_function(values: List(fn(Int) -> Int)) { values == [] }

fn nested_int(values: List(List(Int))) { values == [[]] }
fn nested_string(values: List(List(String))) { values == [[]] }
fn nested_bit_array(values: List(List(BitArray))) { values == [[]] }
fn nested_utf_codepoint(values: List(List(UtfCodepoint))) { values == [[]] }
fn nested_custom(values: List(List(Token))) { values == [[]] }
fn nested_float(values: List(List(Float))) { values == [[]] }
fn nested_bool(values: List(List(Bool))) { values == [[]] }
fn nested_nil(values: List(List(Nil))) { values == [[]] }
fn nested_tuple(values: List(List(#(Int, String)))) { values == [[]] }
fn nested_list(values: List(List(List(Int)))) { values == [[]] }
fn nested_function(values: List(List(fn(Int) -> Int))) { values == [[]] }

pub fn main() {
  let _ = add_one
  #(
    empty_int(forwarded_empty()),
    empty_string(empty_alias),
    empty_bit_array(empty_alias),
    empty_utf_codepoint(empty_alias),
    empty_custom(empty_alias),
    empty_float(empty_alias),
    empty_bool(empty_alias),
    empty_nil(empty_alias),
    empty_tuple(empty_alias),
    empty_list(empty_alias),
    empty_function(empty_alias),
    nested_int(nested),
    nested_string(nested),
    nested_bit_array(nested),
    nested_utf_codepoint(nested),
    nested_custom(nested),
    nested_float(nested),
    nested_bool(nested),
    nested_nil(nested),
    nested_tuple(nested),
    nested_list(nested),
    nested_function(nested),
  )
}

// geam:expect Tuple([Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true)])
