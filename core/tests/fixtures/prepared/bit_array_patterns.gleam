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
