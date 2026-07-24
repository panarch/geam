pub type Boxed {
  Boxed(Int)
}

pub type Combined {
  Combined(Boxed, #(Int, String))
}

pub type Optional {
  Present(Int)
  Absent
}

fn with_boxed(continue: fn(Boxed) -> Int) {
  continue(Boxed(4))
}

fn final_custom_binding() {
  let Boxed(value) = Boxed(6)
}

pub fn main() {
  let Boxed(one) = Boxed(1)
  let Combined(Boxed(two), #(three, _)) = Combined(Boxed(2), #(3, "three"))
  let inferred = Present(5)
  let Present(five) = inferred
  let Boxed(six) = final_custom_binding()
  let Boxed(seven) as boxed = Boxed(7)
  let [..results]: List(Result(Int, Nil)) = []
  use Boxed(four) <- with_boxed
  case results {
    [] -> {
      let Boxed(alias_value) = boxed
      one + two + three + four + five + six + seven + alias_value - 7
    }
    _ -> 0
  }
}

// @geam:expect Int(28)
