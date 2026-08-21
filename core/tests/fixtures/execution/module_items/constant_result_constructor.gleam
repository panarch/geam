const result: Result(Int, Nil) = Ok(1)

pub fn main() {
  result
}

// @geam:expect Custom(type=/gleam/Result(Int, Nil), constructor=Ok#0, fields=[Int(1)])
