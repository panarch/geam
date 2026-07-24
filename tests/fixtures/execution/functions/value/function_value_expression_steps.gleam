fn int_value() {
  1
}

fn string_value() {
  "value"
}

fn float_value() {
  1.0
}

fn bool_value() {
  True
}

fn nil_value() {
  Nil
}

fn tuple_value() {
  #(1, "value")
}

fn get_int_value() {
  int_value
}

fn get_tuple_value() {
  tuple_value
}

pub fn main() {
  int_value
  string_value
  float_value
  bool_value
  nil_value
  tuple_value
  get_int_value
  get_tuple_value

  1
}

// @geam:expect Int(1)
