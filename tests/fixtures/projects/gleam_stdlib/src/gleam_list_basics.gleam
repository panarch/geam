import gleam/list

pub fn main() {
  assert list.length([1, 2, 3]) == 3
  assert list.count([1, 2, 3, 4], fn(value) { value % 2 == 0 }) == 2
  assert list.reverse([1, 2, 3]) == [3, 2, 1]
  assert list.is_empty([])
  assert list.contains([1, 2, 3], 2)
  assert list.first([1, 2, 3]) == Ok(1)
  assert list.rest([1, 2, 3]) == Ok([2, 3])
  assert list.drop([1, 2, 3], 2) == [3]
  assert list.take([1, 2, 3], 2) == [1, 2]
  let empty: List(Int) = list.new()
  assert empty == []
  assert list.wrap(1) == [1]
  assert list.append([1, 2], [3, 4]) == [1, 2, 3, 4]
  assert list.prepend([2, 3], 1) == [1, 2, 3]
  assert list.flatten([[1, 2], [], [3]]) == [1, 2, 3]
  assert list.repeat("a", times: 3) == ["a", "a", "a"]
  assert list.last([1, 2, 3]) == Ok(3)
  Nil
}
// @geam:expect Nil
