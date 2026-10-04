pub type Outcome {
  Done(Int)
  Bad
}

fn scan(input: BitArray, total: Int) -> Outcome {
  case input {
    <<value:8, rest:bits>> -> scan(rest, total + value)
    <<>> -> Done(total)
    _ -> Bad
  }
}

pub fn main() {
  echo scan(<<1, 2, 3>>, 0)
  echo scan(<<1:7>>, 0)
  Nil
}
