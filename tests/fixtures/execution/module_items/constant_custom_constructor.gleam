pub type Token {
  Empty
  Full(Int)
}

const empty = Empty
const full = Full(1)
const make = Full

pub fn main() {
  #(empty, full, make(2))
}

// @geam:expect Tuple([Custom(type=geam/main/Token, constructor=Empty#0, fields=[]), Custom(type=geam/main/Token, constructor=Full#1, fields=[Int(1)]), Custom(type=geam/main/Token, constructor=Full#1, fields=[Int(2)])])
