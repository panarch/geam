import gleam/list
import gleam/set

pub fn main() {
  let empty = set.new()
  let one = set.insert(empty, 1)
  let two = set.insert(one, 2)
  let duplicate = set.insert(two, 2)

  assert set.is_empty(empty)
  assert set.size(empty) == 0
  assert set.size(one) == 1
  assert set.size(two) == 2
  assert set.size(duplicate) == 2
  assert !set.contains(empty, 1)
  assert set.contains(one, 1)
  assert !set.contains(one, 2)
  assert set.contains(two, 2)

  assert set.delete(two, 2) == one
  assert set.delete(two, 3) == two

  let values = set.from_list([3, 1, 3, 2, 1])
  let members = set.to_list(values)
  assert set.size(values) == 3
  assert list.length(members) == 3
  assert list.contains(members, 1)
  assert list.contains(members, 2)
  assert list.contains(members, 3)
  assert set.fold(over: values, from: 0, with: fn(total, value) {
    total + value
  }) == 6

  assert set.from_list([1, 2, 3]) == set.from_list([3, 2, 1, 2])
  assert set.each(set.from_list([7]), fn(value) {
    assert value == 7
    "ignored"
  }) == Nil

  Nil
}
// @geam:expect Nil
