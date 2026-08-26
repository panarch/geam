import gleam/list

pub fn main() {
  assert list.permutations([1, 2]) == [[1, 2], [2, 1]]
  assert list.window([1, 2, 3, 4, 5], 3)
    == [[1, 2, 3], [2, 3, 4], [3, 4, 5]]
  assert list.window_by_2([1, 2, 3, 4]) == [#(1, 2), #(2, 3), #(3, 4)]
  assert list.drop_while([1, 2, 3, 2], fn(value) { value < 3 }) == [3, 2]
  assert list.take_while([1, 2, 3, 2], fn(value) { value < 3 }) == [1, 2]
  assert list.chunk([1, 2, 2, 3, 4, 4, 6, 7, 7], fn(value) {
    value % 2
  }) == [[1], [2, 2], [3], [4, 4, 6], [7, 7]]
  assert list.sized_chunk([1, 2, 3, 4, 5, 6, 7, 8], 3)
    == [[1, 2, 3], [4, 5, 6], [7, 8]]
  assert list.combinations([1, 2, 3], 2) == [[1, 2], [1, 3], [2, 3]]
  assert list.combination_pairs([1, 2, 3]) == [#(1, 2), #(1, 3), #(2, 3)]
  assert list.interleave([[1, 2], [101, 102], [201, 202]])
    == [1, 101, 201, 2, 102, 202]
  assert list.transpose([[1, 2, 3], [101, 102, 103]])
    == [[1, 101], [2, 102], [3, 103]]
  Nil
}
// @geam:expect Nil
