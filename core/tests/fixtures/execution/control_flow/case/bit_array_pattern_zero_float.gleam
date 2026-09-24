pub fn main() {
  let zero = 0
  #(
    case <<>> {
      <<value:float-size(zero)>> -> value
      _ -> -9.0
    },
    case <<>> {
      <<value:float-little-size(zero)>> -> value
      _ -> -9.0
    },
    case <<>> {
      <<0.0 as alias:float-size(zero)>> -> alias
      _ -> -9.0
    },
    case <<>> {
      <<1.0:float-size(zero)>> -> True
      _ -> False
    },
    case <<7>> {
      <<_:float-size(zero), value>> -> value
      _ -> -9
    },
    case <<>> {
      <<_:float-size(zero)>> -> True
      _ -> False
    },
  )
}
// @geam:expect Tuple([Float(0.0), Float(0.0), Float(0.0), Bool(false), Int(7), Bool(true)])
