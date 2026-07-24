fn add_one(value: Int) {
  value + 1
}

fn get1() {
  add_one
}

fn get2() {
  get1
}

fn get3() {
  get2
}

fn get4() {
  get3
}

fn get5() {
  get4
}

pub fn main() {
  get5()()()()()(41)
}

// @geam:expect Int(42)
