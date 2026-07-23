pub const answer = 42

pub const floating = 1.5

pub const text = "geam"

pub const truth = True

pub const unit = Nil

pub const pair = #(42, "geam")

pub const bits = <<42>>

pub const size = 8

pub type Token {
  Token(Int)
}

pub const token = Token(42)

fn add_one(value: Int) {
  value + 1
}

pub const operation = add_one

pub const ints = [42]

pub const strings = ["geam"]

pub const bit_arrays = [<<42>>]

pub const utf_codepoints: List(UtfCodepoint) = []

pub const tokens = [Token(42)]

pub const floats = [1.5]

pub const bools = [True]

pub const nils = [Nil]

pub const tuples = [#(42, "geam")]

pub const lists = [[42]]

pub const functions = [add_one]
