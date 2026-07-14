pub type Inner {
  Inner(Int)
}

pub type Payload {
  Payload(
    List(Inner),
    fn(Int) -> Inner,
    fn() -> Inner,
  )
}

fn make_closure(
  value: Inner,
  values: List(Inner),
  constructor: fn(Int) -> Inner,
) {
  fn() {
    case value, values {
      Inner(one), [Inner(two)] -> constructor(one + two)
      _, _ -> value
    }
  }
}

pub fn main() {
  let value = Inner(1)
  let values = [Inner(2)]
  let constructor = Inner
  Payload(values, constructor, make_closure(value, values, constructor))
}

// geam:expect Custom(type=geam/main/Payload, constructor=Payload#0, fields=[List(geam/main/Inner)([Custom(type=geam/main/Inner, constructor=Inner#0, fields=[Int(2)])]), Function(fn(Int) -> geam/main/Inner), Function(fn() -> geam/main/Inner)])
