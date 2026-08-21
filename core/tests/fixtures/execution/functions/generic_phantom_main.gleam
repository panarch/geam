pub type Phantom(value) {
  Phantom
}

pub type Pair(value) {
  Pair(Phantom(value), Phantom(value))
}

pub type Loop(value) {
  Loop(Loop(value))
}

fn recur() -> Loop(value) {
  panic
}

pub fn main() {
  #(Phantom, Pair(Phantom, Phantom), recur == recur)
}

// @geam:expect Tuple([Custom(type=geam/main/Phantom(Parameter(0)), constructor=Phantom#0, fields=[]), Custom(type=geam/main/Pair(Parameter(1)), constructor=Pair#0, fields=[Custom(type=geam/main/Phantom(Parameter(1)), constructor=Phantom#0, fields=[]), Custom(type=geam/main/Phantom(Parameter(1)), constructor=Phantom#0, fields=[])]), Bool(true)])
