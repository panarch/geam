pub type Token {
  Token(Int)
}

fn identity(value: value) {
  value
}

fn int_value() {
  1
}

fn codepoint(value: Int) -> UtfCodepoint {
  let assert <<codepoint:utf8_codepoint>> = <<value>>
  codepoint
}

const generic_function = identity

fn specialized_identity(_sample: value) -> fn(value) -> value {
  generic_function
}

pub fn main() {
  #(
    specialized_identity(1)(2),
    specialized_identity(1.5)(2.5),
    specialized_identity("one")("two"),
    specialized_identity(<<1>>)(<<2>>),
    specialized_identity(codepoint(65))(codepoint(66)) == codepoint(66),
    specialized_identity(Token(1))(Token(2)),
    specialized_identity(True)(False),
    specialized_identity(Nil)(Nil),
    specialized_identity(#(1, "one"))(#(2, "two")),
    specialized_identity([1])([2]),
    specialized_identity(int_value)(int_value)(),
  )
}

// geam:expect Tuple([Int(2), Float(2.5), String("two"), BitArray(bytes=[2], bit_len=8), Bool(true), Custom(type=geam/main/Token, constructor=Token#0, fields=[Int(2)]), Bool(false), Nil, Tuple([Int(2), String("two")]), List(Int)([Int(2)]), Int(1)])
