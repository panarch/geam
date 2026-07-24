import generic

const copied_empty = generic.empty

fn empty_int(values: List(Int)) {
  values == []
}

fn empty_string(values: List(String)) {
  values == []
}

fn generic_empty() -> List(value) {
  copied_empty
}

fn generic_nested_empty() -> List(List(value)) {
  copied_empty
}

pub fn main() {
  #(
    generic.identity(1),
    generic.identity("two"),
    generic.identity([3]),
    generic.identity(4),
    generic.capture(5)("six"),
    empty_int(copied_empty),
    empty_string(copied_empty),
    generic_empty() == [],
    generic_nested_empty() == [],
  )
}
// @geam:expect Tuple([Int(1), String("two"), List(Int)([Int(3)]), Int(4), Tuple([Int(5), String("six")]), Bool(true), Bool(true), Bool(true), Bool(true)])
