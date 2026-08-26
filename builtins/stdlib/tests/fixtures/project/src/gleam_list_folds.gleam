import gleam/int
import gleam/list

pub fn main() {
  assert list.fold([1, 2, 3], 0, fn(acc, value) { acc + value }) == 6
  assert list.fold_right([1, 2, 3], [], fn(acc, value) {
    [value, ..acc]
  }) == [1, 2, 3]
  assert list.index_fold([10, 20, 30], 0, fn(acc, value, index) {
    acc + value * index
  }) == 80
  assert list.try_fold([1, 2, 3], 0, fn(acc, value) {
    Ok(acc + value)
  }) == Ok(6)
  assert list.try_fold([1, 2, 3], 0, fn(acc, value) {
    case value < 3 {
      True -> Ok(acc + value)
      False -> Error(value)
    }
  }) == Error(3)
  assert list.fold_until([1, 2, 3, 4], 0, fn(acc, value) {
    case value < 3 {
      True -> list.Continue(acc + value)
      False -> list.Stop(acc)
    }
  }) == 3
  assert list.find([1, 2, 3], fn(value) { value > 2 }) == Ok(3)
  assert list.find_map([[], [2], [3]], list.first) == Ok(2)
  assert list.all([2, 4], int.is_even)
  assert list.any([1, 2, 3], int.is_even)
  assert list.each([1, 2, 3], fn(value) { value + 1 }) == Nil
  assert list.try_each([1, 2, 3], fn(value) { Ok(value + 1) }) == Ok(Nil)
  assert list.partition([1, 2, 3, 4], int.is_odd) == #([1, 3], [2, 4])
  assert list.reduce([1, 2, 3], fn(acc, value) { acc + value }) == Ok(6)
  assert list.scan([1, 2, 3], 100, fn(acc, value) { acc + value }) == [101, 103, 106]
  assert list.max([3, 1, 4, 2], int.compare) == Ok(4)
  Nil
}
// @geam:expect Nil
