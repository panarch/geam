pub type Boxed(value) {
  Boxed(value)
}

pub type Nested {
  Nested(Boxed(Int))
}

fn apply(make: fn(Int) -> Boxed(Int), value: Int) {
  make(value)
}

fn apply_custom(read: fn(Boxed(Int)) -> Int, value: Boxed(Int)) {
  read(value)
}

fn make_adder(offset: Int) {
  fn(value: Int) { Boxed(value + offset) }
}

fn count_down(remaining: Int, value: Int) -> Boxed(Int) {
  case remaining {
    0 -> Boxed(value)
    _ -> count_down(remaining - 1, value + 1)
  }
}

fn bool_value(flag: Bool) -> Boxed(Int) {
  case flag {
    True -> Boxed(1)
    False -> Boxed(2)
  }
}

fn float_value(value: Float) -> Boxed(Int) {
  case value {
    1.0 -> Boxed(3)
    _ -> Boxed(4)
  }
}

fn string_value(value: String) -> Boxed(Int) {
  case value {
    "match" -> Boxed(5)
    _ -> Boxed(6)
  }
}

fn block_value(value: Int) -> Boxed(Int) {
  let value = value + 1
  Boxed(value)
}

fn bool_maker(flag: Bool) -> fn(Int) -> Boxed(Int) {
  case flag {
    True -> make_adder(1)
    False -> make_adder(2)
  }
}

fn float_maker(value: Float) -> fn(Int) -> Boxed(Int) {
  case value {
    1.0 -> make_adder(3)
    _ -> make_adder(4)
  }
}

fn string_maker(value: String) -> fn(Int) -> Boxed(Int) {
  case value {
    "match" -> make_adder(5)
    _ -> make_adder(6)
  }
}

fn block_maker(offset: Int) -> fn(Int) -> Boxed(Int) {
  let offset = offset + 1
  make_adder(offset)
}

fn boxed_list(value: Int) -> List(Boxed(Int)) {
  [Boxed(value)]
}

fn custom_value_case(value: Boxed(Int)) {
  case value {
    bound -> bound
  }
}

fn custom_function_case(function: fn(Int) -> Boxed(Int)) {
  case function {
    bound -> bound(9)
  }
}

fn custom_field_binding(value: Nested) {
  case value {
    Nested(inner) -> inner
  }
}

pub fn main() {
  let constructor: fn(Int) -> Boxed(Int) = Boxed
  let captured = make_adder(2)

  case #(
    apply(constructor, 1),
    captured(2),
    count_down(2, 3),
    bool_value(True),
    float_value(1.0),
    string_value("match"),
    block_value(6),
    bool_maker(True)(1),
    float_maker(1.0)(1),
    string_maker("match")(1),
    block_maker(6)(1),
    custom_value_case(Boxed(8)),
    custom_function_case(Boxed),
    boxed_list(14),
    Boxed(apply_custom(
      fn(value: Boxed(Int)) {
        case value { Boxed(inner) -> inner }
      },
      Boxed(15),
    )),
    custom_field_binding(Nested(Boxed(16))),
  ) {
    #(
      Boxed(one),
      Boxed(two),
      Boxed(three),
      Boxed(four),
      Boxed(five),
      Boxed(six),
      Boxed(seven),
      Boxed(eight),
      Boxed(nine),
      Boxed(ten),
      Boxed(eleven),
      Boxed(twelve),
      Boxed(thirteen),
      [Boxed(fourteen)],
      Boxed(fifteen),
      Boxed(sixteen),
    ) ->
      one + two + three + four + five + six + seven + eight + nine + ten + eleven + twelve + thirteen + fourteen + fifteen + sixteen
    _ -> 0
  }
}

// @geam:expect Int(108)
