@external(erlang, "geam_example_generic_box", "Box")
pub type Box(item)

@external(erlang, "geam_example_generic_box", "new")
pub fn new(value: item) -> Box(item)

@external(erlang, "geam_example_generic_box", "get")
pub fn get(boxed: Box(item)) -> item

@external(erlang, "geam_example_generic_box", "replace")
pub fn replace(boxed: Box(old), value: new) -> Box(new)

@external(erlang, "geam_example_generic_box", "contains")
pub fn contains(boxed: Box(item), expected: item) -> Bool

@external(erlang, "geam_example_generic_box", "map")
pub fn map(boxed: Box(input), mapper: fn(input) -> output) -> Box(output)
