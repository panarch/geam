import gleam/float
import gleam/order

pub fn main() {
  assert float.parse("2.5") == Ok(2.5)
  assert float.parse("2") == Error(Nil)
  assert float.to_string(2.0) == "2.0"
  assert float.clamp(1.2, min: 1.4, max: 1.6) == 1.4
  assert float.clamp(1.2, min: 1.4, max: 0.6) == 1.2
  assert float.compare(1.0, 2.0) == order.Lt
  assert float.loosely_compare(5.0, with: 5.3, tolerating: 0.5) == order.Eq
  assert float.loosely_equals(5.0, with: 5.3, tolerating: 0.5)
  assert float.min(2.0, 3.0) == 2.0
  assert float.max(2.0, 3.0) == 3.0
  assert float.ceiling(2.3) == 3.0
  assert float.floor(2.7) == 2.0
  assert float.round(2.5) == 3
  assert float.round(-2.5) == -3
  assert float.truncate(-2.9) == -2
  assert float.to_precision(2.434, 2) == 2.43
  assert float.to_precision(547_890.4, -3) == 548_000.0
  assert float.absolute_value(-12.5) == 12.5
  assert float.power(2.0, of: 3.0) == Ok(8.0)
  assert float.power(-1.0, of: 0.5) == Error(Nil)
  assert float.square_root(4.0) == Ok(2.0)
  assert float.negate(1.5) == -1.5
  assert float.sum([1.0, 2.0, 3.0]) == 6.0
  assert float.product([2.0, 3.0, 4.0]) == 24.0
  let random = float.random()
  assert random >=. 0.0 && random <. 1.0
  assert float.modulo(13.0, by: 3.0) == Ok(1.0)
  assert float.modulo(1.0, by: 0.0) == Error(Nil)
  assert float.divide(6.0, by: 2.0) == Ok(3.0)
  assert float.divide(1.0, by: 0.0) == Error(Nil)
  assert float.add(1.0, 2.0) == 3.0
  assert float.multiply(2.0, 4.0) == 8.0
  assert float.subtract(3.0, 1.0) == 2.0
  assert float.logarithm(1.0) == Ok(0.0)
  assert float.logarithm(0.0) == Error(Nil)
  assert float.loosely_equals(
    float.exponential(1.0),
    with: 2.718281828459045,
    tolerating: 0.000000000000001,
  )
}
// @geam:expect Nil
