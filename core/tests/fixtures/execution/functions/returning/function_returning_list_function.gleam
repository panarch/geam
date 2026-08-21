fn values() {
  [1, 2, 3]
}

fn get() {
  values
}

pub fn main() {
  get()()
}

// @geam:expect List(Int)([Int(1), Int(2), Int(3)])
