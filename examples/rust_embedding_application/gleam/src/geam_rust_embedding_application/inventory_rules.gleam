import example_text_pattern as pattern
import gleam/string

pub opaque type Stock {
  Stock(code: String, quantity: Int)
}

pub fn normalize(code: String) -> String {
  code |> string.trim |> string.uppercase
}

pub fn validate(code: String, quantity: Int) -> Result(Stock, String) {
  let code = normalize(code)
  let assert Ok(code_pattern) = pattern.compile("^[A-Z][A-Z0-9]*-[0-9]+$")
  case pattern.is_match(code_pattern, code) {
    False -> Error("invalid code")
    True ->
      case quantity < 0 {
        True -> Error("quantity must not be negative")
        False -> Ok(Stock(code, quantity))
      }
  }
}

pub fn to_row(stock: Stock) -> #(String, Int) {
  #(stock.code, stock.quantity)
}
