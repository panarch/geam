pub type Maybe(value) {
  None
  Some(value)
}

pub type Phantom(value) {
  Phantom
}

pub type Boxed(value) {
  Boxed(value)
}

const empty = []
const nested = [[]]
const none = None
const phantom = Phantom
const make = Boxed
const explicit_int = <<1:int>>
const default_float = <<1.5:float>>
const sized_bits = <<<<1, 2>>:bits-size(8)>>
const utf8 = <<"A":utf8>>
const utf16_little = <<"A":utf16-little>>
const utf32_little = <<"A":utf32-little>>
const zero_size = <<1:size(-1)>>

fn empty_int(values: List(Int)) {
  case values {
    [] -> True
    _ -> False
  }
}

fn empty_string(values: List(String)) {
  case values {
    [] -> True
    _ -> False
  }
}

fn nested_int(values: List(List(Int))) {
  case values {
    [[]] -> True
    _ -> False
  }
}

fn is_none(value: Maybe(Int)) {
  case value {
    None -> True
    Some(_) -> False
  }
}

fn is_phantom(value: Phantom(String)) {
  case value {
    Phantom -> True
  }
}

pub fn main() {
  #(
    empty_int(empty),
    empty_string(empty),
    nested_int(nested),
    is_none(none),
    is_phantom(phantom),
    make(42),
    explicit_int,
    default_float,
    sized_bits,
    utf8,
    utf16_little,
    utf32_little,
    zero_size,
  )
}

// @geam:expect Tuple([Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Custom(type=geam/main/Boxed(Int), constructor=Boxed#0, fields=[Int(42)]), BitArray(bytes=[1], bit_len=8), BitArray(bytes=[63, 248, 0, 0, 0, 0, 0, 0], bit_len=64), BitArray(bytes=[1], bit_len=8), BitArray(bytes=[65], bit_len=8), BitArray(bytes=[65, 0], bit_len=16), BitArray(bytes=[65, 0, 0, 0], bit_len=32), BitArray(bytes=[], bit_len=0)])
