type Never

fn observe(value: Never) {
  let same = value
  let _ = same
  1
}

pub fn main() {
  1
}

// geam:expect Int(1)
