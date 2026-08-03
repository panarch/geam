import gleam/set

pub fn main() {
  let base = set.from_list([1, 2, 3, 4])
  let other = set.from_list([3, 4, 5])

  assert set.filter(in: base, keeping: fn(value) { value % 2 == 0 })
    == set.from_list([2, 4])
  let mapped = set.map(base, with: fn(value) { value % 2 })
  assert mapped == set.from_list([0, 1])
  assert set.size(mapped) == 2
  assert set.drop(from: base, drop: [1, 3, 8]) == set.from_list([2, 4])
  assert set.take(from: base, keeping: [1, 3, 8]) == set.from_list([1, 3])

  assert set.union(of: base, and: other) == set.from_list([1, 2, 3, 4, 5])
  assert set.intersection(of: base, and: other) == set.from_list([3, 4])
  assert set.difference(from: base, minus: other) == set.from_list([1, 2])
  assert set.is_subset(set.from_list([1, 2]), of: base)
  assert !set.is_subset(set.from_list([1, 5]), of: base)
  assert set.is_disjoint(set.from_list([1, 2]), from: set.from_list([3, 4]))
  assert !set.is_disjoint(base, from: other)
  assert set.symmetric_difference(of: base, and: other)
    == set.from_list([1, 2, 5])

  assert base == set.from_list([4, 3, 2, 1])
  assert other == set.from_list([5, 4, 3])

  Nil
}
// @geam:expect Nil
