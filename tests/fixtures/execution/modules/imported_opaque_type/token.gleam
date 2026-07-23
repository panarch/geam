pub opaque type Token {
  Token(Int)
}

pub fn new(value: Int) {
  Token(value)
}

pub fn increment(token: Token) {
  case token {
    Token(value) -> Token(value + 1)
  }
}

pub fn to_int(token: Token) {
  case token {
    Token(value) -> value
  }
}
