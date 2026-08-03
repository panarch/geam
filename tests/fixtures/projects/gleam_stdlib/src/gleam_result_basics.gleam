import gleam/result

pub fn main() {
  let ok: Result(Int, String) = Ok(1)
  let error: Result(Int, String) = Error("error")

  assert result.is_ok(ok)
  assert !result.is_ok(error)
  assert result.is_error(error)
  assert !result.is_error(ok)

  assert result.map(over: ok, with: fn(value) { value + 1 }) == Ok(2)
  assert result.map(over: error, with: fn(value) { value + 1 }) == error
  assert result.map_error(over: ok, with: fn(value) { value <> "!" }) == ok
  assert result.map_error(over: error, with: fn(value) { value <> "!" })
    == Error("error!")

  assert result.flatten(Ok(Ok(1))) == Ok(1)
  assert result.flatten(Ok(Error("inner"))) == Error("inner")
  assert result.flatten(Error("outer")) == Error("outer")

  assert result.unwrap(ok, or: 0) == 1
  assert result.unwrap(error, or: 0) == 0
  assert result.lazy_unwrap(ok, or: fn() { panic as "unselected lazy default" })
    == 1
  assert result.lazy_unwrap(error, or: fn() { 0 }) == 0
  assert result.unwrap_error(error, or: "default") == "error"
  assert result.unwrap_error(ok, or: "default") == "default"

  assert result.or(ok, Ok(2)) == ok
  assert result.or(error, Ok(2)) == Ok(2)
  assert result.lazy_or(ok, fn() { panic as "unselected lazy result" }) == ok
  assert result.lazy_or(error, fn() { Ok(2) }) == Ok(2)

  assert result.replace(ok, "one") == Ok("one")
  assert result.replace(error, "one") == Error("error")
  assert result.replace_error(ok, Nil) == Ok(1)
  assert result.replace_error(error, Nil) == Error(Nil)

  Nil
}
// @geam:expect Nil
