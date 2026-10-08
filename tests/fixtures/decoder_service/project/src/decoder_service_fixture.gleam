import decoder_service_fixture/manual
import decoder_service_fixture/native
import gleam/dynamic
import gleam/dynamic/decode
import gleam/erlang/process
import gleam/io
import gleam/option

pub fn check() -> Bool {
  let integer = decode.map(decode.int, fn(value) { value + 2 })
  let text = decode.map(decode.string, fn(value) { value <> "!" })
  let assert option.Some(restored) = native.restored(integer)
  let assert Ok(42) = decode.run(dynamic.int(40), restored)
  let assert option.Some(restored_text) = native.restored(text)
  let assert Ok("text!") = decode.run(dynamic.string("text"), restored_text)
  let assert Ok(42) =
    decode.run(dynamic.int(40), manual.keep(native.keep(integer)))
  let assert Ok("text!") = decode.run(dynamic.string("text"), native.keep(text))
  let assert Ok(42) =
    decode.run(
      dynamic.int(40),
      native.last_or([decode.int, integer], decode.int),
    )
  let assert Ok(42) = decode.run(dynamic.int(40), native.last_or([], integer))
  let #(first, second) = native.pair(#(integer, integer))
  let assert Ok(42) = decode.run(dynamic.int(40), first)
  let assert Ok(42) = decode.run(dynamic.int(40), second)
  let assert Error(errors) =
    decode.run(dynamic.string("wrong"), native.keep(integer))
  let assert [decode.DecodeError(expected: "Int", found: "String", path: [])] =
    errors
  let assert native.Configure(configured) =
    native.message(native.Configure(integer))
  let assert Ok(42) = decode.run(dynamic.int(40), configured)
  let assert native.Dispatch(42) = native.message(native.Dispatch(42))
  let assert native.Dispatch(generic) = native.message(native.Dispatch(integer))
  let assert Ok(42) = decode.run(dynamic.int(40), generic)
  let subject = native.subject(process.new_subject())
  process.send(subject, native.Configure(integer))
  let assert Ok(native.Configure(received)) = process.receive(subject, 0)
  let assert Ok(42) = decode.run(dynamic.int(40), received)
  process.send(subject, native.Dispatch(42))
  let assert Ok(native.Dispatch(42)) = process.receive(subject, 0)
  True
}

pub fn main() {
  let assert True = check()
  io.println("decoder SDK: 42")
}
