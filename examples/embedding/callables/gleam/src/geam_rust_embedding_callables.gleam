pub fn calculate(value: Int, adjust: fn(Int) -> Int) -> Int {
  adjust(value)
}

pub fn keep(adjust: fn(Int) -> Int) -> fn(Int) -> Int {
  adjust
}

pub opaque type Saved {
  Saved(fn(Int) -> Int)
}

pub fn save(adjust: fn(Int) -> Int) -> Saved {
  Saved(adjust)
}

pub fn restore(saved: Saved) -> fn(Int) -> Int {
  let Saved(adjust) = saved
  adjust
}

fn wrap(callback: fn(argument) -> output) -> fn(argument) -> output {
  fn(value) { callback(value) }
}

pub fn wrapped(adjust: fn(Int) -> Int) -> fn(Int) -> Int {
  wrap(adjust)
}
