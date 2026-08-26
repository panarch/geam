pub type Priority {
  Low
  Normal
  High
}

pub type Job {
  Pending
  Named(String)
  Scheduled(label: String, attempt: Int)
  Prioritized(Priority)
  Tags(List(String))
}

@external(erlang, "geam_example_value_types", "low")
pub fn low() -> Priority

@external(erlang, "geam_example_value_types", "normal")
pub fn normal() -> Priority

@external(erlang, "geam_example_value_types", "high")
pub fn high() -> Priority

@external(erlang, "geam_example_value_types", "pending")
pub fn pending() -> Job

@external(erlang, "geam_example_value_types", "named")
pub fn named(label: String) -> Job

@external(erlang, "geam_example_value_types", "scheduled")
pub fn scheduled(label: String, attempt: Int) -> Job

@external(erlang, "geam_example_value_types", "prioritized")
pub fn prioritized() -> Job

@external(erlang, "geam_example_value_types", "tagged")
pub fn tagged(first: String, second: String) -> Job

@external(erlang, "geam_example_value_types", "describe")
pub fn describe(job: Job) -> String

@external(erlang, "geam_example_value_types", "first_priority")
pub fn first_priority(values: List(Priority)) -> String
