fn with_pair(continue: fn(#(Int, #(Int, Int))) -> Bool) {
  continue(#(1, #(2, 3)))
}

pub fn main() {
  use #(one, #(two, _) as inner) as pair <- with_pair
  one == 1 && two == 2 && inner.0 == 2 && pair.0 == 1 && pair.1.1 == 3
}

// @geam:expect Bool(true)
