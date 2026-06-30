fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let suffix = "am"
  let enabled = True
  let marker = Nil
  let bump = add_one
  let run = fn(value) {
    let name = "ge" <> suffix
    marker

    case name == "geam" && enabled {
      True -> bump(value) + 1
      False -> 0
    }
  }

  run(40)
}

// geam:expect Int(42)
