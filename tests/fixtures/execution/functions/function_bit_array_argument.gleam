fn apply(callback: fn(BitArray) -> BitArray, value: BitArray) {
  callback(value)
}

pub fn main() {
  apply(fn(value) { value }, <<1>>)
}

// @geam:expect BitArray(bytes=[1], bit_len=8)
