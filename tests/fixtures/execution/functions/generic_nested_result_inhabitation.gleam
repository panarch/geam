pub type Wrapped(value) {
  Wrapped(value)
}

fn values() -> #(
  Wrapped(Result(Int, first_error)),
  Wrapped(Result(second_ok, Nil)),
) {
  #(Wrapped(Ok(1)), Wrapped(Error(Nil)))
}

pub fn main() {
  values()
}

// geam:expect Tuple([Custom(type=geam/main/Wrapped(/gleam/Result(Int, Parameter(0))), constructor=Wrapped#0, fields=[Custom(type=/gleam/Result(Int, Parameter(0)), constructor=Ok#0, fields=[Int(1)])]), Custom(type=geam/main/Wrapped(/gleam/Result(Parameter(1), Nil)), constructor=Wrapped#0, fields=[Custom(type=/gleam/Result(Parameter(1), Nil), constructor=Error#1, fields=[Nil])])])
