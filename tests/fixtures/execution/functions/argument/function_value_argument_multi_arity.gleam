fn add_pair(left: Int, right: Int) {
  left + right
}

fn apply_int_pair(function: fn(Int, Int) -> Int, left: Int, right: Int) {
  function(left, right)
}

pub fn main() {
  apply_int_pair(add_pair, 20, 22)
}

// geam:expect Int(42)
