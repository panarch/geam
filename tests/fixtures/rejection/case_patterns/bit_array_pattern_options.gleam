pub fn main() {
  case <<1>> {
    <<value:bytes>> -> value
    _ -> <<>>
  }
}
