type Boxed {
  Boxed(Int)
}

fn calculate(value: Int, pair: #(Int, Bool), boxed: Boxed, items: List(Int)) {
  let scaled = value * 2 + 1
  let first = pair.0
  let negative = -first
  let Boxed(field) = boxed
  let assert [head, ..] = items
  scaled + field + head - negative
}

pub fn main() {
  calculate(7, #(2, True), Boxed(5), [20])
}
