pub fn main() {
  #(
    case <<1, 2, 3>> {
      <<_, rest:bytes>> -> rest
      _ -> <<>>
    },
    case <<0x77:size(7)>> {
      <<_:size(4), middle:bits-size(2), _:size(1)>> -> middle
      _ -> <<>>
    },
    case <<1, 2, 3>> {
      <<head:bytes-size(2), rest:bits>> -> #(head, rest)
      _ -> #(<<>>, <<>>)
    },
    case <<0x7:size(4)>> {
      <<_:bytes>> -> 1
      _ -> 0
    },
    case <<1>> {
      <<all:bits>> -> all
    },
    case <<1>> {
      <<_ as inner:bits>> as whole -> #(inner, whole)
    },
  )
}

// geam:expect Tuple([BitArray(bytes=[2, 3], bit_len=16), BitArray(bytes=[192], bit_len=2), Tuple([BitArray(bytes=[1, 2], bit_len=16), BitArray(bytes=[3], bit_len=8)]), Int(0), BitArray(bytes=[1], bit_len=8), Tuple([BitArray(bytes=[1], bit_len=8), BitArray(bytes=[1], bit_len=8)])])
