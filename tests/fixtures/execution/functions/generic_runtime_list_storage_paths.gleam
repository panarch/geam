fn codepoint() -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}

fn prepend(value, values: List(value)) -> List(value) {
  [value, ..values]
}

fn tail(values: List(value)) -> List(value) {
  let assert [_, ..rest] = values
  rest
}

pub fn main() {
  let bit_array = <<1>>
  let point = codepoint()
  #(
    [[bit_array]] == [[<<1>>]],
    [[point]] == [[codepoint()]],
    prepend(bit_array, [<<2>>]) == [<<1>>, <<2>>],
    prepend(point, [codepoint()]) == [codepoint(), codepoint()],
    tail([bit_array, <<2>>]) == [<<2>>],
    tail([point, codepoint()]) == [codepoint()],
  )
}

// geam:expect Tuple([Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true)])
