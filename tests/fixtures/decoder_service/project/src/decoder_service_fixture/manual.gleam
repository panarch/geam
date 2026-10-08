import gleam/dynamic/decode.{type Decoder}

@external(erlang, "fixture", "keep")
pub fn keep(value: Decoder(Int)) -> Decoder(Int)
