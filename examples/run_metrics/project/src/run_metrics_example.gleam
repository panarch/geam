import example_run_metrics as metrics

pub fn main() {
  let empty = metrics.new()
  let latency_one = metrics.record(empty, "latency_ms", 12.5)
  let latency_two = metrics.record(latency_one, "latency_ms", 7.5)
  let measured = metrics.record(latency_two, "payload_kb", 4.0)

  let equivalent = metrics.record(metrics.new(), "latency_ms", 12.5)
  let equivalent = metrics.record(equivalent, "latency_ms", 7.5)
  let equivalent = metrics.record(equivalent, "payload_kb", 4.0)

  assert empty == metrics.new()
  assert empty != latency_one
  assert measured == equivalent

  assert metrics.count(empty, "latency_ms") == 0
  assert metrics.total(empty, "latency_ms") == 0.0
  assert metrics.count(latency_one, "latency_ms") == 1
  assert metrics.total(latency_one, "latency_ms") == 12.5
  assert metrics.count(latency_two, "latency_ms") == 2
  assert metrics.total(latency_two, "latency_ms") == 20.0

  assert metrics.count(measured, "payload_kb") == 1
  assert metrics.total(measured, "payload_kb") == 4.0
  assert metrics.count(measured, "missing") == 0
  assert metrics.total(measured, "missing") == 0.0
}
