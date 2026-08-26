import gleam/int
import gleam/list

pub fn main() {
  let source = [1, 2, 3, 4, 5, 6]
  let shuffled = list.shuffle(source)
  assert list.sort(shuffled, int.compare) == source

  let sample = list.sample(source, 3)
  assert list.length(sample) == 3
  assert list.length(list.unique(sample)) == 3
  assert list.all(sample, fn(value) { list.contains(source, value) })
  Nil
}
// @geam:expect Nil
