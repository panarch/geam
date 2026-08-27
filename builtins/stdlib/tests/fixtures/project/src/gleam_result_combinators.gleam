import gleam/result

pub fn main() {
  let ok: Result(Int, String) = Ok(1)
  let error: Result(Int, String) = Error("first")

  assert result.try(ok, fn(value) { Ok(value + 1) }) == Ok(2)
  assert result.try(ok, fn(_) { Error("next") }) == Error("next")
  assert result.try(error, fn(_) { panic as "unselected try callback" }) == error

  assert result.all([Ok(1), Ok(2), Ok(3)]) == Ok([1, 2, 3])
  assert result.all([Ok(1), Error("first"), Error("second")])
    == Error("first")
  assert result.partition([Ok(1), Error("a"), Error("b"), Ok(2)])
    == #([2, 1], ["b", "a"])
  assert result.values([Ok(1), Error("ignored"), Ok(3)]) == [1, 3]

  assert result.try_recover(ok, with: fn(_) {
    panic as "unselected recovery callback"
  }) == ok
  assert result.try_recover(error, with: fn(value) {
    assert value == "first"
    Ok(2)
  }) == Ok(2)
  assert result.try_recover(error, with: fn(_) { Error("failed") })
    == Error("failed")

  Nil
}
// @geam:expect Nil
