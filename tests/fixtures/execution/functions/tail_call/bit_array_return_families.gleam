fn list_tail(count: Int) -> List(BitArray) {
  case count {
    0 -> [<<1>>]
    _ -> list_tail(count - 1)
  }
}

fn identity(value: BitArray) -> BitArray {
  value
}

fn function_tail(count: Int) -> fn(BitArray) -> BitArray {
  case count {
    0 -> identity
    _ -> function_tail(count - 1)
  }
}

pub fn main() {
  let assert [value] = list_tail(2)
  #(value, function_tail(2)(<<2>>))
}

// @geam:expect Tuple([BitArray(bytes=[1], bit_len=8), BitArray(bytes=[2], bit_len=8)])
