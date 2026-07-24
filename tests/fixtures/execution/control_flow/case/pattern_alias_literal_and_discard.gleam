pub fn main() {
  let literal = case 1 {
    1 as alias -> alias == 1
    _ -> False
  }

  let discard = case 2 {
    1 -> False
    _ as alias -> alias == 2
  }

  literal && discard
}

// @geam:expect Bool(true)
