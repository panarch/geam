fn identity(value: UtfCodepoint) -> UtfCodepoint {
  value
}

fn list_tail(count: Int, value: UtfCodepoint) -> List(UtfCodepoint) {
  case count {
    0 -> [value]
    _ -> list_tail(count - 1, value)
  }
}

fn function_tail(count: Int) -> fn(UtfCodepoint) -> UtfCodepoint {
  case count {
    0 -> identity
    _ -> function_tail(count - 1)
  }
}

fn capture(value: UtfCodepoint) {
  fn() { value }
}

pub fn main() {
  case <<"AB":utf8>> {
    <<first:utf8_codepoint, second:utf8_codepoint>> -> {
      let values = [first, ..[second]]
      let assert [head, ..tail] = values
      let assert [last] = tail
      let selected = case values {
        [value, ..] -> value
        _ -> second
      }
      let returned = capture(last)

      #(
        identity(head),
        function_tail(2)(last),
        list_tail(2, selected),
        returned(),
        head == selected,
        head != last,
        values == [first, second],
        #(head, [last]) == #(first, [second]),
        #(head, [last]),
        case <<"A":utf8>>, <<"B":utf8>> {
          <<_ as alias:utf8_codepoint>>, <<right:utf8_codepoint>>
            if alias != right -> #(alias, right)
          _, _ -> #(second, first)
        },
        case <<"B":utf8>> {
          <<value:utf8_codepoint>> | <<value:utf16_codepoint-big>> -> value
          _ -> first
        },
      )
    }
    _ -> panic
  }
}

// @geam:expect Tuple([UtfCodepoint('A'), UtfCodepoint('B'), List(UtfCodepoint)([UtfCodepoint('A')]), UtfCodepoint('B'), Bool(true), Bool(true), Bool(true), Bool(true), Tuple([UtfCodepoint('A'), List(UtfCodepoint)([UtfCodepoint('B')])]), Tuple([UtfCodepoint('A'), UtfCodepoint('B')]), UtfCodepoint('B')])
