pub type Option(a) {
  Some(a)
  None
}

fn matches(expected: Option(Int)) -> Bool {
  case expected {
    candidate if Some(7) == candidate -> True
    _ -> False
  }
}

pub fn main() {
  #(matches(Some(7)), matches(Some(8)), matches(None))
}

// @geam:expect Tuple([Bool(true), Bool(false), Bool(false)])
