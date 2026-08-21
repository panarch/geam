fn unresolved() -> Result(value, error) {
  panic
}

fn exact_ok() {
  Ok(panic)
}

fn identity(result: Result(value, error)) -> Result(value, error) {
  case result {
    Ok(value) -> Ok(value)
    Error(error) -> Error(error)
  }
}

fn identity_alias(result: Result(value, error)) -> Result(value, error) {
  case result {
    Ok(value) as whole -> whole
    Error(error) -> Error(error)
  }
}

pub fn main() {
  #(
    unresolved == unresolved,
    exact_ok == exact_ok,
    identity(Ok(1)) == Ok(1),
    identity_alias(Ok(2)) == Ok(2),
  )
}

// @geam:expect Tuple([Bool(true), Bool(true), Bool(true), Bool(true)])
