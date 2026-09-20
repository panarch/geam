pub fn compose(depth: Int) -> Int {
  let callback = chain(depth, fn(value: Int) { value })
  callback(1)
}

fn chain(depth: Int, previous: fn(Int) -> Int) -> fn(Int) -> Int {
  case depth {
    0 -> previous
    _ -> chain(depth - 1, fn(value) { previous(value) + 1 })
  }
}
