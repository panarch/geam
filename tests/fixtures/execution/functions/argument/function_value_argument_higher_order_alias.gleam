fn add_one(value: Int) {
  value + 1
}

fn apply_int(function: fn(Int) -> Int, value: Int) {
  function(value)
}

pub fn main() {
  let apply_alias = apply_int

  apply_alias(add_one, 41)
}

// geam:expect Int(42)
