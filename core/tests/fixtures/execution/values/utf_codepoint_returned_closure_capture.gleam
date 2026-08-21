fn identity(value: UtfCodepoint) -> UtfCodepoint {
  value
}

pub fn main() -> fn(UtfCodepoint) -> UtfCodepoint {
  case <<"A":utf8>>, <<"B":utf8>> {
    <<first:utf8_codepoint>>, <<second:utf8_codepoint>> -> {
      let values = [second]
      let function = identity

      fn(value) {
        let assert [captured] = values
        case first == value {
          True -> function(captured)
          False -> function(value)
        }
      }
    }
    _, _ -> identity
  }
}

// @geam:expect Function(fn(UtfCodepoint) -> UtfCodepoint)
