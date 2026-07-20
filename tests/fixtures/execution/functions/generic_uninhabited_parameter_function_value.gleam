pub type Never

fn impossible(_value: Never) -> Int {
  panic as "uninhabited argument function must not run"
}

pub fn main() {
  impossible
}

// geam:expect Function(fn(geam/main/Never) -> Int)
