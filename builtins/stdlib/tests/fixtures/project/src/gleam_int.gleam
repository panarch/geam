import gleam/int
import gleam/order

pub fn main() {
  let very_large = 1234567890123456789012345678901234567890
  assert int.absolute_value(-12) == 12
  assert int.power(2, of: -1.0) == Ok(0.5)
  assert int.power(-1, of: 0.5) == Error(Nil)
  assert int.square_root(4) == Ok(2.0)
  assert int.square_root(-16) == Error(Nil)
  assert int.parse("+12") == Ok(12)
  assert int.parse("ABC") == Error(Nil)
  assert int.parse("1234567890123456789012345678901234567890") == Ok(very_large)
  assert int.base_parse("-FF", 16) == Ok(-255)
  assert int.base_parse("2", 2) == Error(Nil)
  assert int.base_parse("10", 1) == Error(Nil)
  assert int.to_string(very_large) == "1234567890123456789012345678901234567890"
  assert int.to_base_string(255, 16) == Ok("FF")
  assert int.to_base_string(1, 37) == Error(Nil)
  assert int.to_base2(10) == "1010"
  assert int.to_base8(15) == "17"
  assert int.to_base16(255) == "FF"
  assert int.to_base36(48) == "1C"
  assert int.to_float(-3) == -3.0
  assert int.clamp(40, min: 50, max: 60) == 50
  assert int.clamp(40, min: 50, max: 30) == 40
  assert int.compare(2, with: 3) == order.Lt
  assert int.min(2, 3) == 2
  assert int.max(2, 3) == 3
  assert int.is_even(2)
  assert int.is_odd(3)
  assert int.negate(1) == -1
  assert int.sum([1, 2, 3]) == 6
  assert int.product([2, 3, 4]) == 24
  assert int.divide(-99, by: 2) == Ok(-49)
  assert int.divide(1, by: 0) == Error(Nil)
  assert int.remainder(-13, by: 3) == Ok(-1)
  assert int.modulo(-13, by: 3) == Ok(2)
  assert int.floor_divide(-99, by: 2) == Ok(-50)
  assert int.add(1, 2) == 3
  assert int.multiply(2, 4) == 8
  assert int.subtract(3, 1) == 2
  assert int.bitwise_and(5, 3) == 1
  assert int.bitwise_not(5) == -6
  assert int.bitwise_or(5, 2) == 7
  assert int.bitwise_exclusive_or(5, 3) == 6
  assert int.bitwise_shift_left(1, 5) == 32
  assert int.bitwise_shift_left(8, -1) == 4
  assert int.bitwise_shift_right(32, 2) == 8
  assert int.bitwise_shift_right(8, -1) == 16
  assert int.range(from: 0, to: 4, with: 0, run: fn(acc, value) {
    acc + value
  }) == 6
  assert int.range(from: 3, to: 0, with: 0, run: fn(acc, value) {
    acc + value
  }) == 6
  let random = int.random(1000)
  assert random >= 0 && random < 1000
  random
}
// @geam:expect 976
