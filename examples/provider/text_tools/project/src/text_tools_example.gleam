import example_text_tools
import example_text_tools/casing
import example_text_tools/checks

pub fn main() {
  assert example_text_tools.join("geam", "-", "provider") == "geam-provider"
  assert example_text_tools.surround("ready", "[", "]") == "[ready]"

  assert casing.upper("Geam") == "GEAM"
  assert casing.lower("HOST") == "host"

  assert checks.starts_with("geam-provider", "geam-")
  assert checks.ends_with("module.gleam", ".gleam")
  assert !checks.starts_with("provider", "geam-")
  assert !checks.ends_with("module.gleam", ".erl")
}
