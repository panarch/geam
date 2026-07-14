fn codepoint() -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}

fn with_value(
  value: UtfCodepoint,
  callback: fn(UtfCodepoint) -> #(UtfCodepoint, UtfCodepoint),
) -> #(UtfCodepoint, UtfCodepoint) {
  callback(value)
}

fn bind(value: UtfCodepoint) {
  let direct as alias = value
  let #(from_tuple, _) = #(alias, Nil)
  let _ = direct
  use from_use <- with_value(from_tuple)
  #(alias, from_use)
}

pub fn main() {
  bind(codepoint())
}

// geam:expect Tuple([UtfCodepoint('A'), UtfCodepoint('A')])
