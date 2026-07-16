pub type Boxed(value) {
  Boxed(value)
}

pub type Holder {
  Holder(fn(Int) -> Int)
}

fn add_one(value: Int) {
  value + 1
}

fn subtract_one(value: Int) {
  value - 1
}

fn identity_function(function: fn(Int) -> Int) {
  function
}

fn make_identity() -> fn(Int) -> Int {
  fn(value) { value }
}

fn make_adder(amount: Int) {
  fn(value) { value + amount }
}

fn direct_guard(left: fn(Int) -> Int, right: fn(Int) -> Int) {
  case Nil {
    Nil if left == right -> True
    _ -> False
  }
}

fn nested_guard(left: Holder, right: Holder) {
  case Nil {
    Nil if left == right -> True
    _ -> False
  }
}

pub fn main() {
  let closure = make_adder(1)
  let returned = make_identity()
  let captured = fn() { closure }
  let through_case = case True {
    True -> closure
    False -> add_one
  }
  let through_block = {
    let value = closure
    value
  }
  let constructor: fn(Int) -> Boxed(Int) = Boxed
  let other_constructor: fn(Int) -> Boxed(Int) = Boxed

  #(
    add_one == add_one,
    add_one == subtract_one,
    closure == closure,
    make_identity() == make_identity(),
    make_adder(1) == make_adder(1),
    constructor == constructor,
    constructor == other_constructor,
    identity_function(closure) == closure,
    captured() == closure,
    returned == identity_function(returned),
    through_case == closure,
    through_block == closure,
    [Holder(closure)] == [Holder(closure)],
    [Holder(make_identity())] == [Holder(make_identity())],
    direct_guard(closure, closure),
    direct_guard(make_identity(), make_identity()),
    nested_guard(Holder(closure), Holder(closure)),
    nested_guard(Holder(make_identity()), Holder(make_identity())),
    add_one != add_one,
    make_identity() != make_identity(),
  )
}

// geam:expect Tuple([Bool(true), Bool(false), Bool(true), Bool(false), Bool(false), Bool(true), Bool(false), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(false), Bool(true), Bool(false), Bool(true), Bool(false), Bool(false), Bool(true)])
