import left
import right

pub fn main() {
  #(
    left.value(),
    right.value(),
    left.value == left.value,
    left.value == right.value,
    left.token(),
    right.token(),
  )
}
// @geam:expect Tuple([Int(1), Int(2), Bool(true), Bool(false), Custom(type=geam/left/Token, constructor=Token#0, fields=[Int(1)]), Custom(type=geam/right/Token, constructor=Token#0, fields=[Int(2)])])
