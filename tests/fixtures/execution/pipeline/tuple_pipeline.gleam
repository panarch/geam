fn second(pair: #(Int, String)) {
  pair.1
}

pub fn main() {
  #(1, "one")
  |> second
}

// geam:expect String("one")
