const empty_width = 0

pub fn zero_fields(input: BitArray, width: Int) {
  case input {
    <<
      int:signed-little-size(width),
      float:float-little-size(width),
      empty:bytes-size(width),
      value,
    >> -> #(int, float, empty, value)
    _ -> #(-1, -1.0, <<255>>, -1)
  }
}

pub fn signed_little(input: BitArray, width: Int, offset: Int) {
  case input {
    <<_:size(offset), value:signed-little-size(width), _:bits>> -> value
    _ -> -1
  }
}

pub fn dependent_fields(input: BitArray) {
  case input {
    <<unused, _ as length, first:size(length), second:size(length)>>
      if first >= second -> first + second
    _ -> -1
  }
}

pub fn fixed_fields(input: BitArray) {
  case input {
    <<first:size(4)-unit(2), zero:size(empty_width), second:signed-little-size(2)-unit(8), _:bits>>
      if first > 10 -> first + zero + second
    _ -> -1
  }
}

pub fn fixed_failure(input: BitArray) {
  case input {
    <<head, _:size(18446744073709551615)>> -> head + 100
    <<head, _:size(9223372036854775808)-unit(2)>> -> head + 200
    <<head, tail>> -> head + tail
    _ -> -1
  }
}
