pub type Phantom(value) {
  Phantom
}

fn make(_value: value) -> Phantom(value) {
  Phantom
}

const make_constant = make

pub fn main() {
  #(make_constant, make_constant(1))
}

// geam:expect Tuple([Function(fn(Parameter(0)) -> geam/main/Phantom(Parameter(0))), Custom(type=geam/main/Phantom(Int), constructor=Phantom#0, fields=[])])
