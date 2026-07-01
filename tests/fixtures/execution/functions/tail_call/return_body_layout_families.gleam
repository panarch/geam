fn int_identity(value: Int) {
  value
}

fn int_increment(value: Int) {
  value + 1
}

fn string_identity(value: String) {
  value
}

fn string_suffix(value: String) {
  value <> "!"
}

fn bool_true(value: Bool) {
  True
}

fn bool_false(value: Bool) {
  False
}

fn nil_identity(value: Nil) {
  value
}

fn nil_other(value: Nil) {
  value
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

fn choose_int_function(flag: Bool) {
  {
    let marker = 1
    marker

    case flag {
      True -> int_identity
      False -> int_increment
    }
  }
}

fn choose_string_function(flag: Bool) {
  {
    let marker = 1
    marker

    case flag {
      True -> string_identity
      False -> string_suffix
    }
  }
}

fn choose_string_function_by_int(value: Int) {
  case value {
    0 -> string_identity
    _ -> string_suffix
  }
}

fn choose_string_function_tail(flag: Bool) {
  choose_string_function(flag)
}

fn choose_bool_function(flag: Bool) {
  {
    let marker = 1
    marker

    case flag {
      True -> bool_true
      False -> bool_false
    }
  }
}

fn choose_bool_function_by_int(value: Int) {
  case value {
    0 -> bool_true
    _ -> bool_false
  }
}

fn choose_bool_function_tail(flag: Bool) {
  choose_bool_function(flag)
}

fn choose_nil_function(flag: Bool) {
  {
    let marker = 1
    marker

    case flag {
      True -> nil_identity
      False -> nil_other
    }
  }
}

fn choose_nil_function_by_int(value: Int) {
  case value {
    0 -> nil_identity
    _ -> nil_other
  }
}

fn choose_nil_function_tail(flag: Bool) {
  choose_nil_function(flag)
}

fn int_case_expression(value: Int) {
  case value {
    0 -> 1
    _ -> 2
  }
}

fn int_bool_case_expression(flag: Bool) {
  case flag {
    True -> 1
    False -> 2
  }
}

fn choose_getter(flag: Bool) {
  {
    let marker = 1
    marker

    case flag {
      True -> choose_int_function
      False -> choose_int_function
    }
  }
}

pub fn main() {
  choose_nil(False)
  string_identity
  bool_true
  nil_identity

  let int_function = choose_int_function(False)
  let string_function = choose_string_function(False)
  let bool_function = choose_bool_function(True)
  let nil_function = choose_nil_function(False)
  let nested_int_function = choose_getter(True)(False)
  let string_tail_function = choose_string_function_tail(False)
  let bool_tail_function = choose_bool_function_tail(True)
  let nil_tail_function = choose_nil_function_tail(False)
  let string_int_case_function = choose_string_function_by_int(1)
  let bool_int_case_function = choose_bool_function_by_int(0)
  let nil_int_case_function = choose_nil_function_by_int(1)
  let int_case_value = int_case_expression(0)
  let int_bool_case_value = int_bool_case_expression(False)

  nil_function(Nil)
  nil_tail_function(Nil)
  nil_int_case_function(Nil)
  nil_identity({
    let marker = 1
    marker

    Nil
  })

  case choose_bool(True)
    && bool_function(False)
    && bool_tail_function(False)
    && bool_int_case_function(False)
    && string_function("geam") == "geam!"
    && string_tail_function("tail") == "tail!"
    && string_int_case_function("case") == "case!" {
    True -> int_function(1) + nested_int_function(1) + int_case_value + int_bool_case_value
    False -> 0
  }
}

// geam:expect Int(7)
