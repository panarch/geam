@external(erlang, "geam_example_run_metrics", "Metrics")
pub type Metrics

@external(erlang, "geam_example_run_metrics", "new")
pub fn new() -> Metrics

@external(erlang, "geam_example_run_metrics", "record")
pub fn record(metrics: Metrics, name: String, value: Float) -> Metrics

@external(erlang, "geam_example_run_metrics", "count")
pub fn count(metrics: Metrics, name: String) -> Int

@external(erlang, "geam_example_run_metrics", "total")
pub fn total(metrics: Metrics, name: String) -> Float
