import gleam/dynamic/decode.{type Decoder}
import gleam/erlang/process.{type Subject}
import gleam/option.{type Option}

pub type Message(item) {
  Configure(Decoder(item))
  Dispatch(item)
}

@external(erlang, "fixture", "keep")
pub fn keep(value: Decoder(item)) -> Decoder(item)

@external(erlang, "fixture", "restored")
pub fn restored(value: Decoder(item)) -> Option(Decoder(item))

@external(erlang, "fixture", "last_or")
pub fn last_or(
  values: List(Decoder(item)),
  default: Decoder(item),
) -> Decoder(item)

@external(erlang, "fixture", "pair")
pub fn pair(
  value: #(Decoder(item), Decoder(item)),
) -> #(Decoder(item), Decoder(item))

@external(erlang, "fixture", "message")
pub fn message(value: Message(item)) -> Message(item)

@external(erlang, "fixture", "subject")
pub fn subject(value: Subject(Message(item))) -> Subject(Message(item))
