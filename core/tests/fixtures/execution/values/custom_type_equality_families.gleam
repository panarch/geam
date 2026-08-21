pub type Boxed {
  Boxed(Int)
}

pub type Families(value) {
  Families(
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    #(Int, String),
    List(Int),
    Result(Int, String),
    value,
  )
}

pub type Grow(value) {
  Stop
  Grow(Grow(List(value)))
}

fn nested() -> Grow(Int) {
  Grow(Stop)
}

pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  Families(1, 1.5, "one", <<1>>, codepoint, True, Nil, #(2, "two"), [3], Ok(4), Boxed(5))
    == Families(1, 1.5, "one", <<1>>, codepoint, True, Nil, #(2, "two"), [3], Ok(4), Boxed(5))
    && nested() == nested()
}

// @geam:expect Bool(true)
