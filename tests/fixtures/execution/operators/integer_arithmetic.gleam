fn one() {
  1
}

pub fn main() {
  #(1 + 2 * 3, -one())
}

// geam:expect Tuple([Int(7), Int(-1)])
