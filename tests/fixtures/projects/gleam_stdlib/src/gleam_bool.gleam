import gleam/bool

pub fn main() {
  let assert #(False, False, False, True) = #(
    bool.and(False, False),
    bool.and(False, True),
    bool.and(True, False),
    bool.and(True, True),
  )

  let assert #(False, True, True, True) = #(
    bool.or(False, False),
    bool.or(False, True),
    bool.or(True, False),
    bool.or(True, True),
  )

  let assert #(True, False, False, False) = #(
    bool.nor(False, False),
    bool.nor(False, True),
    bool.nor(True, False),
    bool.nor(True, True),
  )

  let assert #(True, True, True, False) = #(
    bool.nand(False, False),
    bool.nand(False, True),
    bool.nand(True, False),
    bool.nand(True, True),
  )

  let assert #(False, True, True, False) = #(
    bool.exclusive_or(False, False),
    bool.exclusive_or(False, True),
    bool.exclusive_or(True, False),
    bool.exclusive_or(True, True),
  )

  let assert #(True, False, False, True) = #(
    bool.exclusive_nor(False, False),
    bool.exclusive_nor(False, True),
    bool.exclusive_nor(True, False),
    bool.exclusive_nor(True, True),
  )

  let assert #(False, True) = #(bool.negate(True), bool.negate(False))
  let assert #("True", "False") = #(bool.to_string(True), bool.to_string(False))
  let assert #(
    "guard consequence",
    "guard alternative",
    "lazy consequence",
    "lazy alternative",
  ) = #(
    bool.guard(True, "guard consequence", fn() {
      panic as "unselected guard alternative"
    }),
    bool.guard(False, "unused guard consequence", fn() { "guard alternative" }),
    bool.lazy_guard(True, fn() { "lazy consequence" }, fn() {
      panic as "unselected lazy alternative"
    }),
    bool.lazy_guard(
      False,
      fn() { panic as "unselected lazy consequence" },
      fn() { "lazy alternative" },
    ),
  )

  Nil
}
// @geam:expect Nil
