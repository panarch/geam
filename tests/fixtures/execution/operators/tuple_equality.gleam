pub fn main() {
  #(#(1, "one"), True) == #(#(1, "one"), True)
}

// @geam:expect Bool(true)
