pub fn checksum(input: BitArray, total: Int) -> Int {
  case input {
    <<a:8, b:8, c:8, d:8, rest:bits>> ->
      checksum(rest, total + a + b * 2 + c * 3 + d * 4)
    <<>> -> total
    _ -> panic as "incomplete record"
  }
}

pub fn parse(input: BitArray, current: Int, total: Int) -> Result(Int, Nil) {
  case input {
    <<digit:8, rest:bits>> if digit >= 48 && digit <= 57 ->
      parse(rest, current * 10 + digit - 48, total)
    <<separator:8, rest:bits>> if separator == 44 || separator == 10 ->
      parse(rest, 0, total + current)
    <<>> -> Ok(total + current)
    _ -> Error(Nil)
  }
}

pub fn wide(input: BitArray, total: Int) -> Int {
  case input {
    <<value:64, rest:bits>> -> wide(rest, total + value)
    <<>> -> total
    _ -> -1
  }
}

pub fn late_failure(input: BitArray) -> Int {
  case input {
    <<value:64, 255>> -> value
    _ -> -1
  }
}

pub fn paired(left: BitArray, right: BitArray, total: Int) -> Int {
  case left {
    <<a:7, left_rest:bits>> -> case right {
      <<b:7, right_rest:bits>> -> paired(left_rest, right_rest, total + a + b)
      _ -> -1
    }
    <<>> -> case right { <<>> -> total _ -> -1 }
    _ -> -1
  }
}

pub fn aliases(input: BitArray, total: Int) -> Int {
  case input {
    <<1 as first, 2:8, middle:bits-size(8), rest:bytes>> as whole -> {
      case middle {
        <<value:8>> -> case whole {
          <<_:24, _:bytes>> -> aliases(rest, total + first + value)
          _ -> -2
        }
        _ -> -2
      }
    }
    <<>> -> total
    _ -> -1
  }
}

pub fn little(input: BitArray, total: Int) -> Int {
  case input {
    <<value:9-signed-little, rest:bits>> -> little(rest, total + value)
    <<>> -> total
    _ -> -1
  }
}

pub fn toggle(input: BitArray, flag: Bool) -> Bool {
  case input {
    <<1:8, rest:bits>> -> toggle(rest, !flag)
    <<>> -> flag
    _ -> False
  }
}
