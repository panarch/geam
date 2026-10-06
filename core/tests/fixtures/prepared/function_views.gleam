pub type Erased

pub type Handler(a, b) {
  Handler(fn(a) -> b)
}

pub type Tree(a) {
  Leaf(a)
  Branch(List(Tree(a)))
}

pub type Empty {
  Again(Empty)
}

@external(erlang, "gleam@function", "identity")
fn coerce(value: a) -> b

@external(erlang, "fixture", "tick")
fn tick() -> Int

fn increment(a: Int) -> Int {
  a + 1
}

fn input(value: Int) -> Erased {
  coerce(value)
}

fn output(value: Erased) -> Int {
  coerce(value)
}

pub fn run() -> Bool {
  let bias = 2
  let original = fn(a: Int) { a + bias }
  let view: fn(Erased) -> Erased = coerce(original)
  let restored: fn(Int) -> Int = coerce(view)
  let handler: Handler(Erased, Erased) = coerce(Handler(increment))
  let Handler(callback) = handler
  let tree: Tree(fn(Erased) -> Erased) = coerce(Branch([Leaf(increment)]))
  let assert Branch([Leaf(leaf)]) = tree
  let nested: fn(Erased) -> fn(Erased) -> Erased =
    coerce(fn(a: Int) { fn(b: Int) { a + b } })
  output(view(input(40))) == 42
  && restored == original
  && restored(40) == 42
  && output(callback(input(41))) == 42
  && output(leaf(input(41))) == 42
  && output(nested(input(20))(input(22))) == 42
}

pub fn invalid_input() -> Bool {
  let view: fn(Erased) -> Erased = coerce(fn(a: Int) { tick() + a })
  let wrong: Erased = coerce("wrong")
  output(view(wrong)) == 42
}

pub fn stopped() -> Bool {
  let view: fn(Erased) -> Empty =
    coerce(fn(a: Int) {
      let _ = tick()
      panic as "view source stopped"
    })
  let _ = view(input(42))
  True
}
