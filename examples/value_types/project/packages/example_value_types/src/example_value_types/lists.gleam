@external(erlang, "geam_example_value_types_lists", "length")
pub fn length(values: List(Int)) -> Int

@external(erlang, "geam_example_value_types_lists", "first_or")
pub fn first_or(values: List(String), fallback: String) -> String

@external(erlang, "geam_example_value_types_lists", "identity")
pub fn identity(values: List(Int)) -> List(Int)

@external(erlang, "geam_example_value_types_lists", "reverse")
pub fn reverse(values: List(String)) -> List(String)

@external(erlang, "geam_example_value_types_lists", "labels")
pub fn labels(values: List(#(String, Int))) -> List(String)
