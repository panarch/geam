import gleam/dict
import gleam/int
import gleam/list

pub fn main() {
  let grouped = list.group([1, 2, 3, 4], fn(value) { value % 2 })
  assert dict.get(grouped, 0) == Ok([4, 2])
  assert dict.get(grouped, 1) == Ok([3, 1])
  assert list.filter([1, 2, 3], fn(value) { value > 1 }) == [2, 3]
  assert list.filter_map([1, 2, 3], fn(value) {
    case value % 2 {
      0 -> Ok(value * 10)
      _ -> Error(Nil)
    }
  }) == [20]
  assert list.map([1, 2, 3], fn(value) { value * 2 }) == [2, 4, 6]
  assert list.map2([1, 2, 3], [10, 20], fn(left, right) {
    left + right
  }) == [11, 22]
  assert list.map_fold([1, 2, 3], 100, fn(acc, value) {
    #(acc + value, value * 2)
  }) == #(106, [2, 4, 6])
  assert list.index_map([10, 20], fn(value, index) {
    value + index
  }) == [10, 21]
  assert list.try_map([1, 2, 3], fn(value) { Ok(value + 2) }) == Ok([3, 4, 5])
  assert list.flat_map([2, 4], fn(value) { [value, value + 1] }) == [2, 3, 4, 5]
  assert list.unique([3, 1, 3, 2, 1]) == [3, 1, 2]
  assert list.sort([3, 1, 2], int.compare) == [1, 2, 3]
  Nil
}
// @geam:expect Nil
