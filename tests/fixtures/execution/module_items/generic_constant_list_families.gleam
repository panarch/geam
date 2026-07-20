pub type Token {
  Token(Int)
}

fn add_one(value: Int) {
  value + 1
}

const empty = []
const empty_alias = empty
const nested = [empty_alias]
const nested_tail = [empty_alias]
const nested_spread = [empty_alias, ..nested_tail]
const pair = #(empty_alias, nested)

const int_empty: List(Int) = empty_alias
const string_empty: List(String) = empty_alias
const bit_array_empty: List(BitArray) = empty_alias
const utf_codepoint_empty: List(UtfCodepoint) = empty_alias
const custom_empty: List(Token) = empty_alias
const float_empty: List(Float) = empty_alias
const bool_empty: List(Bool) = empty_alias
const nil_empty: List(Nil) = empty_alias
const tuple_empty: List(#(Int, String)) = empty_alias
const list_empty: List(List(Int)) = empty_alias
const function_empty: List(fn(Int) -> Int) = empty_alias

const int_nested: List(List(Int)) = nested
const string_nested: List(List(String)) = nested
const bit_array_nested: List(List(BitArray)) = nested
const utf_codepoint_nested: List(List(UtfCodepoint)) = nested
const custom_nested: List(List(Token)) = nested
const float_nested: List(List(Float)) = nested
const bool_nested: List(List(Bool)) = nested
const nil_nested: List(List(Nil)) = nested
const tuple_nested: List(List(#(Int, String))) = nested
const list_nested: List(List(List(Int))) = nested
const function_nested: List(List(fn(Int) -> Int)) = nested
const int_nested_spread: List(List(Int)) = nested_spread

const int_pair_value: #(List(Int), List(List(Int))) = pair
const string_pair_value: #(List(String), List(List(String))) = pair
const bit_array_pair_value: #(List(BitArray), List(List(BitArray))) = pair
const utf_codepoint_pair_value: #(List(UtfCodepoint), List(List(UtfCodepoint))) = pair
const custom_pair_value: #(List(Token), List(List(Token))) = pair
const float_pair_value: #(List(Float), List(List(Float))) = pair
const bool_pair_value: #(List(Bool), List(List(Bool))) = pair
const nil_pair_value: #(List(Nil), List(List(Nil))) = pair
const tuple_pair_value: #(List(#(Int, String)), List(List(#(Int, String)))) = pair
const list_pair_value: #(List(List(Int)), List(List(List(Int)))) = pair
const function_pair_value: #(List(fn(Int) -> Int), List(List(fn(Int) -> Int))) = pair

fn forwarded_empty() -> List(value) {
  empty_alias
}

fn forwarded_nested() -> List(List(value)) {
  nested
}

fn forwarded_nested_spread() -> List(List(value)) {
  nested_spread
}

fn forwarded_pair() -> #(List(value), List(List(value))) {
  pair
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

fn int_pair(values: #(List(Int), List(List(Int)))) { values == #([], [[]]) }
fn string_pair(values: #(List(String), List(List(String)))) { values == #([], [[]]) }
fn bit_array_pair(values: #(List(BitArray), List(List(BitArray)))) { values == #([], [[]]) }
fn utf_codepoint_pair(values: #(List(UtfCodepoint), List(List(UtfCodepoint)))) { values == #([], [[]]) }
fn custom_pair(values: #(List(Token), List(List(Token)))) { values == #([], [[]]) }
fn float_pair(values: #(List(Float), List(List(Float)))) { values == #([], [[]]) }
fn bool_pair(values: #(List(Bool), List(List(Bool)))) { values == #([], [[]]) }
fn nil_pair(values: #(List(Nil), List(List(Nil)))) { values == #([], [[]]) }
fn tuple_pair(values: #(List(#(Int, String)), List(List(#(Int, String))))) { values == #([], [[]]) }
fn list_pair(values: #(List(List(Int)), List(List(List(Int))))) { values == #([], [[]]) }
fn function_pair(values: #(List(fn(Int) -> Int), List(List(fn(Int) -> Int)))) { values == #([], [[]]) }

pub fn main() {
  let _ = add_one
  #(
    empty_int(forwarded_empty()),
    empty_string(forwarded_empty()),
    empty_bit_array(forwarded_empty()),
    empty_utf_codepoint(forwarded_empty()),
    empty_custom(forwarded_empty()),
    empty_float(forwarded_empty()),
    empty_bool(forwarded_empty()),
    empty_nil(forwarded_empty()),
    empty_tuple(forwarded_empty()),
    empty_list(forwarded_empty()),
    empty_function(forwarded_empty()),
    nested_int(forwarded_nested()),
    nested_string(forwarded_nested()),
    nested_bit_array(forwarded_nested()),
    nested_utf_codepoint(forwarded_nested()),
    nested_custom(forwarded_nested()),
    nested_float(forwarded_nested()),
    nested_bool(forwarded_nested()),
    nested_nil(forwarded_nested()),
    nested_tuple(forwarded_nested()),
    nested_list(forwarded_nested()),
    nested_function(forwarded_nested()),
    int_pair(forwarded_pair()),
    string_pair(forwarded_pair()),
    bit_array_pair(forwarded_pair()),
    utf_codepoint_pair(forwarded_pair()),
    custom_pair(forwarded_pair()),
    float_pair(forwarded_pair()),
    bool_pair(forwarded_pair()),
    nil_pair(forwarded_pair()),
    tuple_pair(forwarded_pair()),
    list_pair(forwarded_pair()),
    function_pair(forwarded_pair()),
    empty_int(int_empty),
    empty_string(string_empty),
    empty_bit_array(bit_array_empty),
    empty_utf_codepoint(utf_codepoint_empty),
    empty_custom(custom_empty),
    empty_float(float_empty),
    empty_bool(bool_empty),
    empty_nil(nil_empty),
    empty_tuple(tuple_empty),
    empty_list(list_empty),
    empty_function(function_empty),
    nested_int(int_nested),
    nested_string(string_nested),
    nested_bit_array(bit_array_nested),
    nested_utf_codepoint(utf_codepoint_nested),
    nested_custom(custom_nested),
    nested_float(float_nested),
    nested_bool(bool_nested),
    nested_nil(nil_nested),
    nested_tuple(tuple_nested),
    nested_list(list_nested),
    nested_function(function_nested),
    int_pair(int_pair_value),
    string_pair(string_pair_value),
    bit_array_pair(bit_array_pair_value),
    utf_codepoint_pair(utf_codepoint_pair_value),
    custom_pair(custom_pair_value),
    float_pair(float_pair_value),
    bool_pair(bool_pair_value),
    nil_pair(nil_pair_value),
    tuple_pair(tuple_pair_value),
    list_pair(list_pair_value),
    function_pair(function_pair_value),
    forwarded_nested_spread() == [[], []],
    int_nested_spread == [[], []],
  )
}

// geam:expect Tuple([Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true)])
