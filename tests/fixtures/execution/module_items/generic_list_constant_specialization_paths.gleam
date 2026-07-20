pub type Token {
  Token
}

fn int_value() {
  1
}

fn codepoint(value: Int) -> UtfCodepoint {
  let assert <<codepoint:utf8_codepoint>> = <<value>>
  codepoint
}

const empty = []

fn specialized_empty(_sample: value) -> List(value) {
  empty
}

fn unresolved_empty() -> List(value) {
  empty
}

pub fn main() {
  #(
    unresolved_empty(),
    specialized_empty([]),
    specialized_empty(1) == [],
    specialized_empty("one") == [],
    specialized_empty(<<1>>) == [],
    specialized_empty(codepoint(65)) == [],
    specialized_empty(Token) == [],
    specialized_empty(1.5) == [],
    specialized_empty(True) == [],
    specialized_empty(Nil) == [],
    specialized_empty(#(1, "one")) == [],
    specialized_empty([1]) == [],
    specialized_empty(int_value) == [],
  )
}

// geam:expect Tuple([List(Parameter(0))([]), List(List(Parameter(1)))([]), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true)])
