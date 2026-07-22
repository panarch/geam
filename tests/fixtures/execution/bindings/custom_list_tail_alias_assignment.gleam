pub type Wrapper {
  Wrapper(List(Int))
}

pub fn main() {
  let Wrapper([..] as whole) = Wrapper([1, 2])
  whole
}

// geam:expect List(Int)([Int(1), Int(2)])
