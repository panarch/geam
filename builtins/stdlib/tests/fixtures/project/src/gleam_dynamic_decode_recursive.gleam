import gleam/dynamic
import gleam/dynamic/decode

type Nested {
  Branch(List(Nested))
  Leaf(String)
}

fn nested_decoder() -> decode.Decoder(Nested) {
  use <- decode.recursive
  decode.one_of(decode.string |> decode.map(Leaf), or: [
    decode.list(nested_decoder()) |> decode.map(Branch),
  ])
}

pub fn main() {
  let data = dynamic.list([
    dynamic.string("one"),
    dynamic.list([
      dynamic.string("two"),
      dynamic.list([dynamic.string("three")]),
    ]),
  ])
  let expected =
    Branch([
      Leaf("one"),
      Branch([Leaf("two"), Branch([Leaf("three")])]),
    ])

  assert decode.run(data, nested_decoder()) == Ok(expected)

  Nil
}

// @geam:expect Nil
