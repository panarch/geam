fn add_one(value: Int) {
  value + 1
}

fn add(left: Int, right: Int) {
  left + right
}

fn apply(function: fn(Int) -> Int, value: Int) {
  function(value)
}

fn get_add_one() {
  add_one
}

fn string_id(value: String) {
  value
}

fn bool_id(value: Bool) {
  value
}

fn nil_id(value: Nil) {
  value
}

pub fn main() {
  let top_level = add_one
  let anonymous = fn(value) { value + 1 }
  let returned = get_add_one()
  let string_function = string_id
  let bool_function = bool_id
  let nil_function = nil_id
  let piped = 1 |> top_level
  let piped_call = 1 |> add(2)
  let piped_returned = 1 |> get_add_one()

  string_function("geam")
  bool_function(True)
  nil_function(Nil)

  top_level(1)
  + anonymous(1)
  + apply(add_one, 1)
  + returned(1)
  + piped
  + piped_call
  + piped_returned
}

// geam:expect Int(15)
