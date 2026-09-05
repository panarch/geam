@external(erlang, "geam_rust_embedding_async_host", "pause")
fn pause(value: Int) -> Int {
  value
}

@external(erlang, "geam_rust_embedding_async_host", "around")
fn around(callback: fn(Int) -> Int, value: Int) -> Int {
  callback(value)
}

fn double_after_pause(value: Int) -> Int {
  pause(value * 2)
}

pub fn calculate(value: Int) -> Int {
  echo value as "input"
  let value = around(double_after_pause, value)
  echo value as "after host"
  value + 1
}
