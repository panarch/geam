fn ok() -> Result(Int, String) {
  Ok(1)
}

fn error() -> Result(Int, String) {
  Error("failed")
}

pub fn main() {
  let make: fn(Int) -> Result(Int, String) = Ok
  #(ok(), error(), make(2))
}

// @geam:expect Tuple([Custom(type=/gleam/Result(Int, String), constructor=Ok#0, fields=[Int(1)]), Custom(type=/gleam/Result(Int, String), constructor=Error#1, fields=[String("failed")]), Custom(type=/gleam/Result(Int, String), constructor=Ok#0, fields=[Int(2)])])
