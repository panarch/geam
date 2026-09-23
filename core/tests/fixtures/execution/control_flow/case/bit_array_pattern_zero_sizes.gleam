pub fn main() {
  let zero = 0
  #(
    case <<>> {
      <<
        big:unsigned-big-size(zero),
        signed_big:signed-big-size(zero),
        little:unsigned-little-size(zero),
        signed_little:signed-little-size(zero),
        0 as alias:size(zero),
      >> -> #(big, signed_big, little, signed_little, alias)
      _ -> #(-1, -1, -1, -1, -1)
    },
    case <<0, 7, 1:size(1)>> {
      <<
        width,
        empty:bytes-size(width),
        value:size(width),
        byte,
        _:size(1),
        tail:bits-size(zero),
      >> -> #(empty, value, byte, tail)
      _ -> #(<<9>>, -1, -1, <<9>>)
    },
    case <<>> {
      <<1:size(zero)>> -> True
      _ -> False
    },
    case <<7>> {
      <<_:size(zero)>> -> True
      _ -> False
    },
    case <<7>> {
      <<0 as empty:size(zero), value>> if empty == 1 -> value
      <<value>> -> value + 1
      _ -> -1
    },
    case <<>> {
      <<_:size(1 - 1)-unit(8)>> -> True
      _ -> False
    },
  )
}
// @geam:expect Tuple([Tuple([Int(0), Int(0), Int(0), Int(0), Int(0)]), Tuple([BitArray(bytes=[], bit_len=0), Int(0), Int(7), BitArray(bytes=[], bit_len=0)]), Bool(false), Bool(false), Int(8), Bool(true)])
