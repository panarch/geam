fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let pair = #([1, 2], add_one)
  case pair {
    #(values as same_values, f as same_f) ->
      values == [1, 2] && same_values == values && f(41) == 42 && same_f(40) == 41
  }
}

// geam:expect Bool(true)
