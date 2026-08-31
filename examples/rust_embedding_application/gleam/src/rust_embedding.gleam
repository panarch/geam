import gleam/io
import gleam/list
import gleam/option.{type Option, None, Some}
import inventory_rules

pub fn validate_batch(
  rows: List(#(String, Int)),
) -> List(Result(#(String, Int), String)) {
  io.println("validating inventory")
  list.map(rows, fn(row) { validate(row.0, row.1) })
}

pub fn total_quantity(rows: List(Result(#(String, Int), String))) -> Int {
  list.fold(rows, 0, fn(total, row) {
    case row {
      Ok(#(_, quantity)) -> total + quantity
      Error(_) -> total
    }
  })
}

pub fn first_valid(
  rows: List(Result(#(String, Int), String)),
) -> Option(#(String, Int)) {
  case rows {
    [] -> None
    [Ok(row), ..] -> Some(row)
    [Error(_), ..rest] -> first_valid(rest)
  }
}

fn validate(code: String, quantity: Int) -> Result(#(String, Int), String) {
  case inventory_rules.validate(code, quantity) {
    Ok(stock) -> Ok(inventory_rules.to_row(stock))
    Error(message) -> Error(message)
  }
}
