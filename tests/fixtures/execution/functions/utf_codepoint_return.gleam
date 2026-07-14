fn codepoint() -> UtfCodepoint {
  case <<"A":utf8>> {
    <<value:utf8_codepoint>> -> value
    _ -> panic
  }
}

pub fn main() {
  case <<codepoint():utf8_codepoint>> {
    <<"A":utf8>> -> 1
    _ -> 0
  }
}

// geam:expect Int(1)
