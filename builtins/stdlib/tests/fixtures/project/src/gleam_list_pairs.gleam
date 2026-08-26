import gleam/list

pub fn main() {
  assert list.zip([1, 2], [3, 4]) == [#(1, 3), #(2, 4)]
  assert list.strict_zip([1, 2], [3, 4]) == Ok([#(1, 3), #(2, 4)])
  assert list.strict_zip([1, 2], [3]) == Error(Nil)
  assert list.unzip([#(1, "a"), #(2, "b")]) == #([1, 2], ["a", "b"])
  assert list.intersperse([1, 2, 3], 0) == [1, 0, 2, 0, 3]
  assert list.split([1, 2, 3, 4], 2) == #([1, 2], [3, 4])
  assert list.split_while([1, 2, 3, 2], fn(value) { value < 3 })
    == #([1, 2], [3, 2])
  let keywords = [#("a", 0), #("b", 1), #("a", 2)]
  assert list.key_find(keywords, "b") == Ok(1)
  assert list.key_filter(keywords, "a") == [0, 2]
  assert list.key_pop(keywords, "b") == Ok(#(1, [#("a", 0), #("a", 2)]))
  assert list.key_set(keywords, "b", 10)
    == [#("a", 0), #("b", 10), #("a", 2)]
  assert list.key_set(keywords, "c", 3)
    == [#("a", 0), #("b", 1), #("a", 2), #("c", 3)]
  Nil
}
// @geam:expect Nil
