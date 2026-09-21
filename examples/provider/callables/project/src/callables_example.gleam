import example_callables as native

pub fn main() {
  let add = native.make_adder(10)
  let alias = add
  assert alias == add
  assert add != native.make_adder(10)
  assert alias(5) == 15

  let values = [add, native.wrap(add)]
  let assert [first, second] = values
  assert first(6) == 16
  assert second(7) == 17
  assert native.calls() == 3

  let constant = native.make_constant(#("retained", [1, 2]))
  assert constant() == #("retained", [1, 2])
  let stringify = native.wrap(fn(value) { #(value + 1, "wrapped") })
  assert stringify(4) == #(5, "wrapped")

  let reply = native.reply(native.wrap(fn(value) { value + 2 }))
  assert native.deliver(reply, 40) == 42
  assert native.calls() == 3
}
