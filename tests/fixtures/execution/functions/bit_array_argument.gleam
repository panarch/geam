fn identity(values: BitArray) {
  values
}

pub fn main() {
  identity(<<1>>)
}

// @geam:expect BitArray(bytes=[1], bit_len=8)
