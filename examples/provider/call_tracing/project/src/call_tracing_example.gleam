import example_call_tracing as tracing

fn work() {
  tracing.record_later("inside")
  42
}

pub fn main() {
  assert tracing.entries() == []
  assert tracing.around(work) == 42
  assert tracing.entries() == ["before", "inside", "after"]
}
