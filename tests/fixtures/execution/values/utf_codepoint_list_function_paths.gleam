fn codepoint() -> UtfCodepoint {
  case <<"A":utf8>> {
    <<value:utf8_codepoint>> -> value
    _ -> panic
  }
}

fn values() {
  [codepoint()]
}

fn getter() {
  values
}

fn selector() {
  getter
}

fn first(function: fn() -> List(UtfCodepoint)) {
  let assert [value] = function()
  value
}

pub fn main() {
  let functions = [values]
  let assert [from_assert] = functions
  let from_case = case functions {
    [function] -> function
    _ -> values
  }
  let assert [from_direct] = values()

  #(
    from_direct,
    first(values),
    first(getter()),
    first(selector()()),
    first(from_assert),
    first(from_case),
  )
}

// @geam:expect Tuple([UtfCodepoint('A'), UtfCodepoint('A'), UtfCodepoint('A'), UtfCodepoint('A'), UtfCodepoint('A'), UtfCodepoint('A')])
