import token

pub fn main() {
  let value = token.new(41)
  token.increment(value)
  |> token.to_int
}
// geam:expect Int(42)
