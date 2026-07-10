fn to_string(value: Int) {
  "ok"
}

fn to_bool(value: Int) {
  value == 0
}

fn to_nil(value: Int) {
  Nil
}

fn to_tuple(value: Int) {
  #(value, "ok")
}

fn to_float(value: Int) {
  1.0
}

fn to_list(value: Int) {
  [value]
}

fn get_string(n: Int) {
  case n {
    0 -> to_string
    _ -> get_string(n - 1)
  }
}

fn get_bool(n: Int) {
  case n {
    0 -> to_bool
    _ -> get_bool(n - 1)
  }
}

fn get_nil(n: Int) {
  case n {
    0 -> to_nil
    _ -> get_nil(n - 1)
  }
}

fn get_tuple(n: Int) {
  case n {
    0 -> to_tuple
    _ -> get_tuple(n - 1)
  }
}

fn get_float(n: Int) {
  case n {
    0 -> to_float
    _ -> get_float(n - 1)
  }
}

fn get_list(n: Int) {
  case n {
    0 -> to_list
    _ -> get_list(n - 1)
  }
}

fn get_getter(n: Int) {
  case n {
    0 -> get_string
    _ -> get_getter(n - 1)
  }
}

pub fn main() {
  let string_fn = get_string(10000)
  let bool_fn = get_bool(10000)
  let nil_fn = get_nil(10000)
  let tuple_fn = get_tuple(10000)
  let float_fn = get_float(10000)
  let list_fn = get_list(10000)
  let getter = get_getter(10000)

  nil_fn(0)
  assert float_fn(0) == 1.0
  assert list_fn(1) == [1]

  case bool_fn(0) {
    True -> string_fn(0) <> getter(0)(0) <> tuple_fn(0).1
    False -> "bad"
  }
}

// geam:expect String("okokok")
