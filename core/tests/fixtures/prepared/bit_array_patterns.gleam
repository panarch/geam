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
