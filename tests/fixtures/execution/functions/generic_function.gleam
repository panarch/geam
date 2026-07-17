fn select(first: Int, second: a) -> a {
  let _ = first
  second
}

fn apply(function: fn(Int, a) -> a, value: a) -> a {
  function(0, value)
}

pub fn main() {
  let as_function: fn(Int, Int) -> Int = select
  #(select(0, 41), as_function(0, 42), apply(select, 43))
}

// geam:expect Tuple([Int(41), Int(42), Int(43)])
