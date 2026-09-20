import gleam/list

pub fn arithmetic(remaining: Int, total: Int) -> Int {
  case remaining {
    0 -> total
    n if n % 2 == 0 -> arithmetic(n - 1, total + n * 3 + 1)
    n -> arithmetic(n - 1, total + n * 2 - 1)
  }
}

pub fn capturing_fold(values: List(Int), factor: Int, bias: Int) -> Int {
  let transform = fn(value) { value * factor + bias }
  list.fold(values, 0, fn(total, value) { total + transform(value) })
}

pub type Entry {
  Credit(Int)
  Debit(Int)
  Ignored
}

pub fn custom_match(values: List(Int)) -> Int {
  values
  |> list.map(fn(value) {
    case value % 3 {
      0 -> Credit(value)
      1 -> Debit(value)
      _ -> Ignored
    }
  })
  |> list.fold(0, fn(total, entry) {
    case entry {
      Credit(amount) -> total + amount
      Debit(amount) -> total - amount
      Ignored -> total
    }
  })
}
