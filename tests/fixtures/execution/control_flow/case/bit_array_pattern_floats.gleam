pub fn main() {
  let invalid_float_size = 24
  let zero_size = 0
  #(
    case <<1.5:float-size(16)-big>> {
      <<1.5 as alias:float-size(16)-big>> -> alias +. alias
      _ -> 0.0
    },
    case <<1.5:float-size(32)-little>> {
      <<value:float-size(32)-little>> -> value
      _ -> 0.0
    },
    case <<1.5:float-size(16)-little>> {
      <<value:float-size(16)-little>> -> value
      _ -> 0.0
    },
    case <<1.5:float-size(32)-big>> {
      <<value:float-size(32)-big>> -> value
      _ -> 0.0
    },
    case <<1.5:float-size(64)-big>> {
      <<value:float-size(64)-big>> -> value
      _ -> 0.0
    },
    case <<1.5:float-size(64)-little>> {
      <<value:float-size(64)-little>> -> value
      _ -> 0.0
    },
    case <<1.5:float-size(16)>> {
      <<1.5:float-size(16)>> -> True
      _ -> False
    },
    case <<1.5:float-size(16)>> {
      <<_:float-size(16)>> -> True
      _ -> False
    },
    case <<1.5:float-size(16)>> {
      <<2.0 as alias:float-size(16)>> -> alias
      _ -> 0.0
    },
    case <<1.5:float-size(32)>> {
      <<_:float-size(invalid_float_size)>> -> True
      _ -> False
    },
    case <<1.5:float-size(16)>> {
      <<_:float-size(zero_size)>> -> True
      _ -> False
    },
    case <<1.5:float-size(16)>> {
      <<_:float-size(64)>> -> True
      _ -> False
    },
  )
}

// geam:expect Tuple([Float(3.0), Float(1.5), Float(1.5), Float(1.5), Float(1.5), Float(1.5), Bool(true), Bool(true), Float(0.0), Bool(false), Bool(false), Bool(false)])
