import example_feature_flags as flags

pub fn main() {
  assert flags.environment() == "staging"
  assert flags.enabled("new_checkout")
  assert flags.enabled("audit_log")
  assert !flags.enabled("production_only")
}
