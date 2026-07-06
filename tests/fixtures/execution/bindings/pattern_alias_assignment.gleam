pub fn main() {
  let #(one, _) as pair = #(1, 2)
  one == pair.0
}

// geam:expect Bool(true)
