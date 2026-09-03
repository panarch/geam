import gleam/list
import gleam/option.{type Option}

pub fn first(values: List(String)) -> Option(String) {
  values
  |> list.first
  |> option.from_result
}
