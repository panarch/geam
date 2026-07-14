pub fn main() {
  case <<"A":utf8>> {
    <<value:utf8_codepoint>> -> [value]
    _ -> []
  }
}

// geam:expect List(UtfCodepoint)([UtfCodepoint('A')])
