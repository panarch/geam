fn add_one(value: Int) {
  value + 1
}

fn add_ten(value: Int) {
  value + 10
}

pub fn main() {
  let add = 1
  let add = add_one
  let outer = add(1)

  let inner = {
    let add = add_ten
    add(10)
  }

  let primitive_shadow = {
    let add = add_one
    let add = 5
    add + 2
  }

  outer + inner + primitive_shadow + add(1)
}

// @geam:expect Int(31)
