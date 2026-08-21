fn identity(value: Int) {
  value
}

pub fn main() {
  let left: Result(fn(Int) -> Int, Nil) = Ok(identity)
  let right: Result(fn(Int) -> Int, Nil) = Ok(identity)
  left == right
}

// @geam:expect Bool(true)
