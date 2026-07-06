pub fn main() {
  let #(one, #(two, _) as inner) as pair = #(1, #(2, 3))
  one == 1 && two == 2 && inner.1 == 3 && pair.0 == 1 && pair.1.1 == 3
}

// geam:expect Bool(true)
