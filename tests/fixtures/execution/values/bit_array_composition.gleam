fn tail(count: Int, value: BitArray) -> BitArray {
  case count {
    0 -> value
    _ -> tail(count - 1, value)
  }
}

fn capture(value: BitArray) -> fn() -> BitArray {
  fn() { value }
}

fn capture_list(values: List(BitArray)) -> fn() -> List(BitArray) {
  fn() { values }
}

pub fn main() {
  let bits = tail(2, <<1>>)
  let #(tuple_bits) = #(bits)
  let values = [tuple_bits, ..[<<2>>]]
  let assert [first, ..rest] = values
  let captured = capture(first)
  let captured_values = capture_list(values)

  case #(captured(), rest), bits {
    #(from_capture, [second]), alias as whole ->
      #(from_capture, second, alias, whole, alias == whole, values, [values], captured_values())
    _, _ -> #(bits, bits, bits, bits, False, values, [values], values)
  }
}

// @geam:expect Tuple([BitArray(bytes=[1], bit_len=8), BitArray(bytes=[2], bit_len=8), BitArray(bytes=[1], bit_len=8), BitArray(bytes=[1], bit_len=8), Bool(true), List(BitArray)([BitArray(bytes=[1], bit_len=8), BitArray(bytes=[2], bit_len=8)]), List(List(BitArray))([List(BitArray)([BitArray(bytes=[1], bit_len=8), BitArray(bytes=[2], bit_len=8)])]), List(BitArray)([BitArray(bytes=[1], bit_len=8), BitArray(bytes=[2], bit_len=8)])])
