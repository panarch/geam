const int = 1
const float = 1.5
const string = "ge" <> "am"
const bool = True
const nil = Nil
const rest = [2, 3]
const list = [1, ..rest]
const alias = list
const tuple = #(int, float, string, bool, nil, alias)

pub fn main() {
  tuple
}

// geam:expect Tuple([Int(1), Float(1.5), String("geam"), Bool(true), Nil, List(Int)([Int(1), Int(2), Int(3)])])
