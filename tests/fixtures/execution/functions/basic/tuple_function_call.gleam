fn first(pair: #(Int, String)) {
  pair.0
}

fn pair() {
  #(1, "one")
}

pub fn main() {
  first(pair())
}

// geam:expect Int(1)
