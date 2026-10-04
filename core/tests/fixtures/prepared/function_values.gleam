pub type Holder(item) {
  Held(fn(item) -> String)
}

@external(erlang, "native", "keep")
fn keep(function: fn(item) -> String) -> fn(item) -> String

@external(erlang, "native", "has_callback")
fn has_callback(function: fn(item) -> String) -> Bool

@external(erlang, "native", "keep_holder")
fn keep_holder(value: Holder(item)) -> Holder(item)

@external(erlang, "native", "keep_compound")
fn keep_compound(
  function: fn(List(item)) -> List(item),
) -> fn(List(item)) -> List(item)

pub fn main() {
  let label = "retained"
  let function = fn(_) { label }
  let alias = keep(function)
  let assert False = has_callback(alias)
  let assert Held(field) = keep_holder(Held(function))
  alias == function && field == function
}

pub fn concrete() {
  let label = "retained"
  let function = fn(_: Int) { label }
  let alias = keep(function)
  #(alias == function, has_callback(alias), alias(7))
}

pub fn compound() {
  let function = fn(items) { items }
  let alias = keep_compound(function)
  #(alias == function, alias([]) == [])
}
