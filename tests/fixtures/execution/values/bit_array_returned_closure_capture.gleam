fn identity(value: BitArray) -> BitArray {
  value
}

pub fn main() -> fn(BitArray) -> BitArray {
  let bits = <<1>>
  let values = [<<2>>]
  let function = identity

  fn(value) {
    let assert [first] = values
    function(<<bits:bits, first:bits, value:bits>>)
  }
}

// geam:expect Function(fn(BitArray) -> BitArray)
