pub type Boxed(value) {
  Boxed(value)
}

fn codepoint(value: Int) -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<value>>
  value
}

fn wrapped(selector: Bool, first: value, second: value) -> #(value) {
  #(
    case selector {
      True -> first
      False -> second
    },
  )
}

pub fn main() {
  #(
    wrapped(False, <<1>>, <<2>>),
    wrapped(False, "first", "second"),
    wrapped(False, Boxed(1), Boxed(2)),
    wrapped(False, Nil, Nil),
    wrapped(False, #(1), #(2)),
    wrapped(False, [1], [2]),
    wrapped(False, codepoint(65), codepoint(66)),
  )
}

// @geam:expect Tuple([Tuple([BitArray(bytes=[2], bit_len=8)]), Tuple([String("second")]), Tuple([Custom(type=geam/main/Boxed(Int), constructor=Boxed#0, fields=[Int(2)])]), Tuple([Nil]), Tuple([Tuple([Int(2)])]), Tuple([List(Int)([Int(2)])]), Tuple([UtfCodepoint('B')])])
