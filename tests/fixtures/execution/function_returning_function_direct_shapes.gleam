fn int_identity(value: Int) {
  value
}

fn string_identity(value: String) {
  value
}

fn bool_identity(value: Bool) {
  value
}

fn nil_identity(value: Nil) {
  value
}

fn get_int() {
  int_identity
}

fn get_string() {
  string_identity
}

fn get_bool() {
  bool_identity
}

fn get_nil() {
  nil_identity
}

fn run_int(getter: fn() -> fn(Int) -> Int, value: Int) {
  getter()(value)
}

fn run_string(getter: fn() -> fn(String) -> String, value: String) {
  getter()(value)
}

fn run_bool(getter: fn() -> fn(Bool) -> Bool, value: Bool) {
  getter()(value)
}

fn run_nil(getter: fn() -> fn(Nil) -> Nil) {
  getter()(Nil)
}

pub fn main() {
  get_int()(1)
  run_int(get_int, 2)
  get_bool()(True)
  run_bool(get_bool, False)
  get_nil()(Nil)
  run_nil(get_nil)
  get_string()("ge") <> run_string(get_string, "am")
}

// geam:expect String("geam")
