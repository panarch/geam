pub fn count_ones_assert(values: List(Int), count: Int) -> Int {
  case values {
    [] -> count
    _ -> {
      let assert [head, ..tail] = values
      let count = case head {
        1 -> count + 1
        _ -> count
      }
      count_ones_assert(tail, count)
    }
  }
}

pub fn count_ones_equal(values: List(Int), stop: List(Int), count: Int) -> Int {
  case values == stop {
    True -> count
    False ->
      case values {
        [1, ..tail] -> count_ones_equal(tail, stop, count + 1)
        [_, ..tail] -> count_ones_equal(tail, stop, count)
        [] -> count
      }
  }
}
