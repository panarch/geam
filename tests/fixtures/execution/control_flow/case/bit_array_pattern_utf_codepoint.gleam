pub fn main() {
  case <<"A":utf8>> {
    <<_:utf8_codepoint>> -> 1
    _ -> 0
  }
}

// geam:expect Int(1)
