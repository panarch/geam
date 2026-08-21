fn add_one(value: Int) {
  value + 1
}

fn add_half(value: Float) {
  value +. 0.5
}

pub fn main() {
  let suffix = "am"
  let base = 1.5
  let enabled = True
  let marker = Nil
  let bump = add_one
  let float_bump = add_half
  let run = fn(value) {
    let name = "ge" <> suffix
    let float_ok = float_bump(base) == 2.0
    marker

    case name == "geam" && enabled && float_ok {
      True -> bump(value) + 1
      False -> 0
    }
  }

  run(40)
}

// @geam:expect Int(42)
