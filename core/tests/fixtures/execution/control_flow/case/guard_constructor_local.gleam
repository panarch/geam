pub type Option(a) {
  Some(a)
  None
}

fn matches(value: Int, expected: Option(Int)) -> Bool {
  case expected {
    candidate if Some(value) == candidate -> True
    _ -> False
  }
}

pub fn main() {
  #(matches(7, Some(7)), matches(7, Some(8)), matches(7, None))
}

// @geam:expect Tuple([Bool(true), Bool(false), Bool(false)])
