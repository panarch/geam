import gleam/bit_array
import gleam/dict
import gleam/order

pub fn main() {
  let assert <<_:8, selected:bytes-size(2), _:bytes>> = <<0, "AB":utf8, 255>>
  assert bit_array.bit_size(selected) == 16
  assert bit_array.byte_size(selected) == 2
  assert bit_array.pad_to_bytes(selected) == <<65, 66>>
  assert bit_array.to_string(selected) == Ok("AB")
  assert bit_array.inspect(selected) == "<<65, 66>>"
  assert dict.get(dict.from_list([#(selected, "selected")]), <<65, 66>>)
    == Ok("selected")
  assert bit_array.from_string("AB") == <<"AB":utf8>>
  assert bit_array.bit_size(<<1, 2:size(2)>>) == 10
  assert bit_array.byte_size(<<1, 2:size(2)>>) == 2
  assert bit_array.pad_to_bytes(<<5:size(3)>>) == <<5:size(3), 0:size(5)>>
  assert bit_array.append(to: <<1, 2:size(2)>>, suffix: <<3:size(2)>>)
    == <<1, 2:size(2), 3:size(2)>>
  assert bit_array.concat([<<1>>, <<2:size(2)>>, <<3:size(2)>>])
    == <<1, 2:size(2), 3:size(2)>>
  let assert Ok(middle) =
    bit_array.slice(from: <<1, 2, 3, 4>>, at: 1, take: 2)
  assert middle == <<2, 3>>
  assert bit_array.inspect(middle) == "<<2, 3>>"
  assert dict.get(dict.from_list([#(middle, "middle")]), <<2, 3>>)
    == Ok("middle")
  assert bit_array.slice(from: <<1, 2, 3, 4>>, at: 3, take: -2)
    == Ok(<<2, 3>>)
  assert bit_array.slice(from: <<1, 2, 3, 4>>, at: 4, take: 0) == Ok(<<>>)
  assert bit_array.slice(from: <<1, 2>>, at: -1, take: 1) == Error(Nil)
  assert bit_array.slice(from: <<1, 2>>, at: 3, take: 1) == Error(Nil)
  assert bit_array.slice(from: <<1:size(2)>>, at: 0, take: 0) == Error(Nil)
  assert bit_array.is_utf8(bit_array.from_string("💜"))
  assert !bit_array.is_utf8(<<255>>)
  assert bit_array.to_string(bit_array.from_string("gleam")) == Ok("gleam")
  assert bit_array.to_string(<<255>>) == Error(Nil)
  assert bit_array.inspect(<<100, 5:size(3)>>) == "<<100, 5:size(3)>>"
  assert bit_array.compare(<<1>>, with: <<2>>) == order.Lt
  assert bit_array.compare(<<1, 2:size(2)>>, with: <<1, 2:size(2)>>)
    == order.Eq
  assert bit_array.starts_with(<<1, 2, 3>>, <<1, 2>>)
  assert bit_array.starts_with(<<1, 2:size(2)>>, <<1, 2:size(2)>>)

  "bit arrays"
}

// @geam:expect "bit arrays"
