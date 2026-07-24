fn inspect(value: UtfCodepoint) {
  case <<value:utf8_codepoint>> {
    <<"A":utf8>> -> 1
    _ -> 0
  }
}

pub fn main() {
  case <<"A":utf8>> {
    <<value:utf8_codepoint>> -> inspect(value)
    _ -> 0
  }
}

// @geam:expect Int(1)
