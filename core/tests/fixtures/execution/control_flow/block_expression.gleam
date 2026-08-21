pub fn main() {
  let base = {
    let x = 1
    {
      let y = 2
      x + y
    }
  }

  let x = 10
  {
    let x = 20
    x + 1
  }

  base + x
}

// @geam:expect Int(13)
