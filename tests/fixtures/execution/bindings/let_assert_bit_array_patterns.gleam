pub fn main() {
  let assert <<size, payload:bits-size(size), _ as alias:bits-size(8), rest:bits>> as whole =
    <<16, 1, 2, 3, 4>>
  let assert [<<1 as first, nested_rest:bits>>] = [<<1, 2>>]
  #(size, payload, alias, rest, whole, first, nested_rest)
}

// @geam:expect Tuple([Int(16), BitArray(bytes=[1, 2], bit_len=16), BitArray(bytes=[3], bit_len=8), BitArray(bytes=[4], bit_len=8), BitArray(bytes=[16, 1, 2, 3, 4], bit_len=40), Int(1), BitArray(bytes=[2], bit_len=8)])
