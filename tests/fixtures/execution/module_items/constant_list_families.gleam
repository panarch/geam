pub type Token {
  Token(Int)
}

fn add_one(value: Int) {
  value + 1
}

const ints_tail = [1]
const ints = [0, ..ints_tail]
const ints_alias = ints
const strings_tail = ["two"]
const strings = ["one", ..strings_tail]
const strings_alias = strings
const bit_arrays_tail = [<<3>>]
const bit_arrays = [<<2>>, ..bit_arrays_tail]
const bit_arrays_alias = bit_arrays
const utf_codepoints: List(UtfCodepoint) = []
const utf_codepoints_alias = utf_codepoints
const customs_tail = [Token(4)]
const customs = [Token(3), ..customs_tail]
const customs_alias = customs
const floats_tail = [5.5]
const floats = [4.5, ..floats_tail]
const floats_alias = floats
const bools_tail = [True]
const bools = [False, ..bools_tail]
const bools_alias = bools
const nils_tail = [Nil]
const nils = [Nil, ..nils_tail]
const nils_alias = nils
const tuples_tail = [#(6, "six")]
const tuples = [#(5, "five"), ..tuples_tail]
const tuples_alias = tuples
const lists_tail = [[7]]
const lists = [[6], ..lists_tail]
const lists_alias = lists
const functions_tail = [add_one]
const functions = [add_one, ..functions_tail]
const functions_alias = functions
const bits = <<8>>
const bits_alias = bits
const token = Token(9)
const token_alias = token
const pair = #(10, "ten")
const pair_alias = pair
const function = add_one
const function_alias = function
const aggregate = #(bits_alias, token_alias, pair_alias, function_alias)
const all_lists = #(
  ints_alias,
  strings_alias,
  bit_arrays_alias,
  utf_codepoints_alias,
  customs_alias,
  floats_alias,
  bools_alias,
  nils_alias,
  tuples_alias,
  lists_alias,
  functions_alias,
)

pub fn main() {
  #(all_lists, aggregate)
}

// geam:expect Tuple([Tuple([List(Int)([Int(0), Int(1)]), List(String)([String("one"), String("two")]), List(BitArray)([BitArray(bytes=[2], bit_len=8), BitArray(bytes=[3], bit_len=8)]), List(UtfCodepoint)([]), List(geam/main/Token)([Custom(type=geam/main/Token, constructor=Token#0, fields=[Int(3)]), Custom(type=geam/main/Token, constructor=Token#0, fields=[Int(4)])]), List(Float)([Float(4.5), Float(5.5)]), List(Bool)([Bool(false), Bool(true)]), List(Nil)([Nil, Nil]), List(#(Int, String))([Tuple([Int(5), String("five")]), Tuple([Int(6), String("six")])]), List(List(Int))([List(Int)([Int(6)]), List(Int)([Int(7)])]), List(fn(Int) -> Int)([Function(fn(Int) -> Int), Function(fn(Int) -> Int)])]), Tuple([BitArray(bytes=[8], bit_len=8), Custom(type=geam/main/Token, constructor=Token#0, fields=[Int(9)]), Tuple([Int(10), String("ten")]), Function(fn(Int) -> Int)])])
