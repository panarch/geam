fn choose_string(flag: Bool) {
  {
    let marker = 1
    marker

    case flag {
      True -> "yes"
      False -> "no"
    }
  }
}

fn choose_bool(flag: Bool) {
  {
    let marker = 1
    marker

    case flag {
      True -> True
      False -> False
    }
  }
}

fn choose_nil(flag: Bool) {
  {
    let marker = 1
    marker

    case flag {
      True -> Nil
      False -> Nil
    }
  }
}

pub fn main() {
  choose_nil(False)

  case choose_bool(True) && choose_string(False) == "no" {
    True -> 1
    False -> 0
  }
}

// @geam:expect Int(1)
