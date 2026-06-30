fn apply_int(function: fn(Int) -> Int, value: Int) {
  function(value)
}

fn apply_string(function: fn(String) -> String, value: String) {
  function(value)
}

fn apply_bool(function: fn(Bool) -> Bool, value: Bool) {
  function(value)
}

fn apply_nil(function: fn(Nil) -> Nil, value: Nil) {
  function(value)
}

pub fn main() {
  apply_string(fn(value) { value }, "geam")
  apply_bool(fn(value) { value }, True)
  apply_nil(fn(value) { value }, Nil)

  apply_int(fn(value) { value + 1 }, 41)
}

// geam:expect Int(42)
