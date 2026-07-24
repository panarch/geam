pub type Boxed(value) {
  Boxed(value)
}

pub type Chain(value) {
  End
  Next(value, Chain(value))
}

pub type Left {
  Left(Right)
}

pub type Right {
  Stop
  Continue(Left)
}

pub opaque type Secret {
  Secret(Int)
}

fn reveal(secret: Secret) {
  case secret {
    Secret(value) -> value
  }
}

pub fn main() {
  let chain = Next(1, Next(2, End))
  let values = [Boxed(1), ..[Boxed(2)]]
  let nested = [[Boxed(3)]]
  let mutual = Left(Continue(Left(Stop)))

  case
    values,
    nested,
    chain,
    mutual,
    chain == Next(1, Next(2, End)),
    values == [Boxed(1), Boxed(2)]
  {
    [Boxed(one), Boxed(two)],
    [[Boxed(three)]],
    Next(1, Next(2, End)),
    Left(Continue(Left(Stop))),
    True,
    True -> one + two + three + reveal(Secret(4))
    _, _, _, _, _, _ -> 0
  }
}

// @geam:expect Int(10)
