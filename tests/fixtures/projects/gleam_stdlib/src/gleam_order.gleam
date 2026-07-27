import gleam/order.{Eq, Gt, Lt}

fn compare_int(left: Int, right: Int) -> order.Order {
  case left {
    _ if left < right -> Lt
    _ if left == right -> Eq
    _ -> Gt
  }
}

pub fn main() {
  let reversed = order.reverse(compare_int)

  let assert #(Gt, Eq, Lt) = #(
    order.negate(Lt),
    order.negate(Eq),
    order.negate(Gt),
  )

  let assert #(-1, 0, 1) = #(
    order.to_int(Lt),
    order.to_int(Eq),
    order.to_int(Gt),
  )

  let assert #(Eq, Lt, Lt, Gt, Eq, Lt, Gt, Gt, Eq) = #(
    order.compare(Lt, with: Lt),
    order.compare(Lt, with: Eq),
    order.compare(Lt, with: Gt),
    order.compare(Eq, with: Lt),
    order.compare(Eq, with: Eq),
    order.compare(Eq, with: Gt),
    order.compare(Gt, with: Lt),
    order.compare(Gt, with: Eq),
    order.compare(Gt, with: Gt),
  )

  let assert #(Gt, Eq, Lt) = #(reversed(1, 2), reversed(2, 2), reversed(2, 1))

  let assert #(Lt, Gt, Gt) = #(
    order.break_tie(in: Lt, with: Gt),
    order.break_tie(in: Eq, with: Gt),
    order.break_tie(in: Gt, with: Lt),
  )

  let assert #(Lt, Gt, Gt) = #(
    order.lazy_break_tie(in: Lt, with: fn() {
      panic as "unselected Lt comparison"
    }),
    order.lazy_break_tie(in: Eq, with: fn() { Gt }),
    order.lazy_break_tie(in: Gt, with: fn() {
      panic as "unselected Gt comparison"
    }),
  )

  Gt
}
// @geam:expect Gt
