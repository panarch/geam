fn add_one(value: Int) {
  value + 1
}

fn get() {
  add_one
}

fn get_get() {
  get
}

fn run_get(getter: fn() -> fn() -> fn(Int) -> Int, value: Int) {
  getter()()(value)
}

pub fn main() {
  get_get()()(0)

  let getter = get_get
  let selected = case True {
    True -> getter
    False -> get_get
  }
  let selected_by_false = case False {
    True -> get_get
    False -> selected
  }
  let from_int_case = case 0 {
    0 -> {
      let scoped = selected_by_false
      scoped
    }
    _ -> get_get
  }
  let from_int_fallback = case 1 {
    0 -> get_get
    _ -> from_int_case
  }

  from_int_fallback()()(1) + run_get(get_get, 2)
}

// @geam:expect Int(5)
