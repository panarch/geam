@external(erlang, "application_arguments", "strings")
pub fn strings() -> List(String)

@external(erlang, "application_arguments", "native_units")
pub fn native_units() -> List(List(Int))

pub fn snapshot() -> #(List(String), List(List(Int))) {
  #(strings(), native_units())
}
