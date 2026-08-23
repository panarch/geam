import gleam/option.{type Option}

pub type ParseError {
  Empty
  Invalid(String)
}

@external(erlang, "geam_example_value_types", "parse")
pub fn parse(value: String) -> Result(Int, ParseError)

@external(erlang, "geam_example_value_types", "describe")
pub fn describe(value: Result(Int, ParseError)) -> String

@external(erlang, "geam_example_value_types", "optional")
pub fn optional(value: Int, keep: Bool) -> Option(#(String, Int))

@external(erlang, "geam_example_value_types", "describe_option")
pub fn describe_option(value: Option(#(String, Int))) -> String

@external(erlang, "geam_example_value_types", "first")
pub fn first(values: List(Result(Int, ParseError))) -> String

@external(erlang, "geam_example_value_types", "samples")
pub fn samples() -> List(Result(Int, ParseError))
