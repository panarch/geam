fn and_false() {
  True && False
}

fn or_true() {
  False || True
}

pub fn main() {
  and_false() || or_true()
}

// geam:expect Bool(true)
