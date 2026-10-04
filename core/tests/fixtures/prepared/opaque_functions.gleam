pub type Holder(item) {
  Held(fn(item) -> String)
}

@external(erlang, "native", "keep")
fn keep(value: Holder(item)) -> Holder(item)

pub type CompoundHolder(item) {
  CompoundHeld(fn(List(item)) -> List(item))
}

@external(erlang, "native", "keep_compound")
fn keep_compound(value: CompoundHolder(item)) -> CompoundHolder(item)

pub fn main() {
  let captured = "retained"
  let callback = fn(_) { captured }
  let assert Held(alias) = keep(Held(callback))
  alias == callback
}

pub fn concrete() {
  let captured = "retained"
  let callback = fn(_: Nil) { captured }
  let assert Held(alias) = keep(Held(callback))
  #(alias == callback, alias(Nil))
}

pub fn compound() {
  let callback = fn(items) { items }
  let assert CompoundHeld(alias) = keep_compound(CompoundHeld(callback))
  #(alias == callback, alias([]) == [])
}
