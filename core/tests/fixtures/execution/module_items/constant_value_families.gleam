const int = 1
const float = 1.5
const string = "ge" <> "am"
const bool = True
const falsehood = False
const nil = Nil
const rest = [2, 3]
const list = [1, ..rest]
const alias = list
const tuple = #(int, float, string, bool, falsehood, nil, alias)

pub fn main() {
  #(int, float, string, bool, falsehood, nil, tuple)
}

// @geam:expect Tuple([Int(1), Float(1.5), String("geam"), Bool(true), Bool(false), Nil, Tuple([Int(1), Float(1.5), String("geam"), Bool(true), Bool(false), Nil, List(Int)([Int(1), Int(2), Int(3)])])])
