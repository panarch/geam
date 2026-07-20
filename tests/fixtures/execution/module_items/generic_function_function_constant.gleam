fn identity(value: value) {
  value
}

fn provide(_nil: Nil) -> fn(value) -> value {
  identity
}

const provider_constant = provide

pub fn main() {
  #(provider_constant, provider_constant(Nil)(1))
}

// geam:expect Tuple([Function(fn(Nil) -> fn(Parameter(0)) -> Parameter(0)), Int(1)])
