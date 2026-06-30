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

fn get_int_identity() {
  int_identity
}

pub fn main() {
  let suffix = "am"
  let enabled = True
  let marker = Nil
  let int_function = int_identity
  let string_function = string_identity
  let bool_function = bool_identity
  let nil_function = nil_identity
  let function_function = get_int_identity

  let stringer = fn(prefix) { { prefix <> suffix } }
  let booler = fn(value) { !False && value && enabled }
  let niler = fn() { marker }
  let getter = fn() { int_function }

  let run = fn(value) {
    let name = { string_function(stringer("ge")) }
    let ok = bool_function(booler(True))
    nil_function(niler())

    let piped = value |> int_identity |> int_identity
    let direct = getter()
    let indirect = function_function()

    case name == "geam" && ok {
      True -> direct(piped)
      False -> indirect(0)
    }
  }

  run(42)
}

// geam:expect Int(42)
