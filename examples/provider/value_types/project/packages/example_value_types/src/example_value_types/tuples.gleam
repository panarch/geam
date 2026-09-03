@external(erlang, "geam_example_value_types_tuples", "wrap")
pub fn wrap(value: String) -> #(String)

@external(erlang, "geam_example_value_types_tuples", "unwrap")
pub fn unwrap(value: #(String)) -> String

@external(erlang, "geam_example_value_types_tuples", "swap")
pub fn swap(value: #(String, Int)) -> #(Int, String)

@external(erlang, "geam_example_value_types_tuples", "rotate")
pub fn rotate(value: #(String, Float, Bool)) -> #(Bool, String, Float)

@external(erlang, "geam_example_value_types_tuples", "reassociate")
pub fn reassociate(value: #(String, #(Int, Bool))) -> #(#(String, Int), Bool)
