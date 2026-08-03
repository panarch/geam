import gleam/bit_array

pub fn main() {
  let hello = bit_array.from_string("hello")
  assert bit_array.base64_encode(hello, True) == "aGVsbG8="
  assert bit_array.base64_encode(hello, False) == "aGVsbG8"
  assert bit_array.base64_decode("aGVsbG8=") == Ok(hello)
  assert bit_array.base64_decode("aGVsbG8") == Ok(hello)
  assert bit_array.base64_decode("AB==") == Ok(<<0>>)
  assert bit_array.byte_size(bit_array.from_string("aG  \t\nVsbG8=")) == 12
  assert bit_array.base64_decode("aG  \t\nVsbG8=") == Ok(hello)
  assert bit_array.base64_decode("***") == Error(Nil)
  assert bit_array.base64_url_encode(<<251, 255>>, True) == "-_8="
  assert bit_array.base64_url_decode("-_8=") == Ok(<<251, 255>>)
  assert bit_array.base16_encode(hello) == "68656C6C6F"
  assert bit_array.base16_decode("68656c6c6f") == Ok(hello)
  assert bit_array.base16_decode("ABC") == Error(Nil)
  assert bit_array.base16_decode("GG") == Error(Nil)
  assert bit_array.base64_encode(<<5:size(3)>>, True) == "oA=="
  assert bit_array.base16_encode(<<5:size(3)>>) == "A0"

  "codecs"
}

// @geam:expect "codecs"
