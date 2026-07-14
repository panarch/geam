pub type Token {
  Empty
  Full(Int)
  Labelled(value: Int, name: String)
}

pub fn main() {
  #(Empty, Full(1), Labelled(value: 2, name: "two"))
}

// geam:expect Tuple([Custom(type=geam/main/Token, constructor=Empty#0, fields=[]), Custom(type=geam/main/Token, constructor=Full#1, fields=[Int(1)]), Custom(type=geam/main/Token, constructor=Labelled#2, fields=[value: Int(2), name: String("two")])])
