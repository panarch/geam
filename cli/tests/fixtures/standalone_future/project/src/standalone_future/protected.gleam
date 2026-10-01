import standalone_future/native

pub fn protect(body: fn() -> value) -> Result(value, String) {
  case native.rescue(body) {
    Ok(value) -> Ok(value)
    Error(reason) -> {
      let native.Caught(message) = reason
      Error(message)
    }
  }
}
