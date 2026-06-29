fn add_one(value: Int) {
  value + 1
}

fn apply_int(function: fn(Int) -> Int, value: Int) {
  function(value)
}

pub fn main() {
  let add = add_one

  apply_int(add, 41)
}

// geam:expect Int(42)
