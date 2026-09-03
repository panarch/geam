import example_tag_set as tags

pub fn main() {
  let empty = tags.new()
  let rust = tags.insert(empty, "rust")
  let rust_again = tags.insert(rust, "rust")
  let both = tags.insert(rust_again, "gleam")

  let equivalent = tags.insert(tags.new(), "gleam")
  let equivalent = tags.insert(equivalent, "rust")

  assert empty == tags.new()
  assert empty != rust
  assert tags.size(empty) == 0
  assert tags.size(rust) == 1
  assert tags.size(rust_again) == 1
  assert tags.size(both) == 2
  assert tags.contains(both, "rust")
  assert tags.contains(both, "gleam")
  assert !tags.contains(both, "elixir")
  assert both == equivalent
}
