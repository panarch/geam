import config.{answer, operation, size}
import limits

const copied_answer = answer

const copied_float = config.floating

const copied_string = config.text

const copied_bool = config.truth

const copied_nil = config.unit

const copied_tuple = config.pair

const copied_bit_array = config.bits

const copied_custom = config.token

const copied_operation = operation

const copied_ints = config.ints

const copied_strings = config.strings

const copied_bit_arrays = config.bit_arrays

const copied_utf_codepoints = config.utf_codepoints

const copied_customs = config.tokens

const copied_floats = config.floats

const copied_bools = config.bools

const copied_nils = config.nils

const copied_tuples = config.tuples

const copied_lists = config.lists

const copied_functions = config.functions

pub fn main() {
  let matched = case <<42>> {
    <<value:size(size)>> if value == limits.answer -> value
    _ -> 0
  }

  #(
    copied_answer == 42,
    copied_float == 1.5,
    copied_string == "geam",
    copied_bool == True,
    copied_nil == Nil,
    copied_tuple == #(42, "geam"),
    copied_bit_array == <<42>>,
    copied_custom == config.Token(42),
    copied_operation == operation,
    copied_operation(41) == 42,
    config.operation(41) == 42,
    copied_ints == [42],
    copied_strings == ["geam"],
    copied_bit_arrays == [<<42>>],
    copied_utf_codepoints == [],
    copied_customs == [config.Token(42)],
    copied_floats == [1.5],
    copied_bools == [True],
    copied_nils == [Nil],
    copied_tuples == [#(42, "geam")],
    copied_lists == [[42]],
    copied_functions == [operation],
    <<1:size(size)>>,
    matched,
  )
}
// @geam:expect Tuple([Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), BitArray(bytes=[1], bit_len=8), Int(42)])
