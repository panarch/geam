import gleam/option.{None, Some}

pub fn main() {
  let assert Some([1, 2]) = option.all([Some(1), Some(2)])
  let assert None = option.all([Some(1), None])
  let assert #(True, False, True, False) = #(
    option.is_some(Some(1)),
    option.is_some(None),
    option.is_none(None),
    option.is_none(Some(1)),
  )
  let assert #(Ok(1), Error("missing")) = #(
    option.to_result(Some(1), "missing"),
    option.to_result(None, "missing"),
  )
  let assert #(Some(1), None) = #(
    option.from_result(Ok(1)),
    option.from_result(Error("missing")),
  )
  let assert #(1, 0) = #(
    option.unwrap(Some(1), or: 0),
    option.unwrap(None, or: 0),
  )
  let assert #(1, 0) = #(
    option.lazy_unwrap(Some(1), or: fn() { panic as "unselected lazy default" }),
    option.lazy_unwrap(None, or: fn() { 0 }),
  )
  let assert #(Some(2), None) = #(
    option.map(over: Some(1), with: fn(value) { value + 1 }),
    option.map(over: None, with: fn(value) { value + 1 }),
  )
  let assert #(Some(1), None, None) = #(
    option.flatten(Some(Some(1))),
    option.flatten(Some(None)),
    option.flatten(None),
  )
  let assert #(Some(2), None) = #(
    option.then(Some(1), apply: fn(value) { Some(value + 1) }),
    option.then(None, apply: fn(value) { Some(value + 1) }),
  )
  let assert #(Some(1), Some(2)) = #(
    option.or(Some(1), Some(2)),
    option.or(None, Some(2)),
  )
  let assert #(Some(1), Some(2)) = #(
    option.lazy_or(Some(1), fn() { panic as "unselected lazy option" }),
    option.lazy_or(None, fn() { Some(2) }),
  )
  let assert [1, 3] = option.values([Some(1), None, Some(3)])

  Nil
}
// @geam:expect Nil
