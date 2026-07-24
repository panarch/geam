pub type Recursive {
  End
  Next(Recursive)
  Function(fn() -> Int)
}

fn value() {
  1
}

pub fn main() {
  Next(Function(value)) == Next(Function(value))
}

// @geam:expect Bool(true)
