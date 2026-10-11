import gleam/erlang/charlist
import gleam/io
import gleam/result

@external(erlang, "fixture", "produce")
fn produce() -> a

@external(erlang, "fixture", "pair")
fn pair() -> #(a, b)

@external(erlang, "fixture", "result")
fn native_result() -> Result(String, #(a, b))

@external(erlang, "fixture", "delayed")
fn delayed() -> a

@external(erlang, "fixture", "list")
fn native_list(input: List(Int)) -> List(a)

@external(erlang, "fixture", "identity")
fn identity(value: a) -> a

@external(erlang, "fixture", "inspect")
fn inspect(value: a) -> String

// Declaration and call from Mist 6.0.3, without specializing its error payload.
@external(erlang, "mist_ffi", "get_path_and_query")
fn get_path_and_query(
  str: String,
) -> Result(#(String, Result(String, Nil)), #(value, term))

pub type InvalidPath {
  InvalidPath
}

fn parse(path: String) {
  get_path_and_query(path) |> result.replace_error(InvalidPath)
}

fn tail(value: a, times: Int) -> a {
  case times {
    0 -> value
    _ -> tail(value, times - 1)
  }
}

fn captured(value: a) {
  fn() { value }
}

pub fn known_failure() -> Bool {
  produce()
}

pub fn known_pair_failure() -> Bool {
  let value: #(Bool, Int) = pair()
  value.0
}

pub fn known_result_failure() -> Bool {
  let value: Result(String, #(Bool, String)) = native_result()
  case value {
    Ok(_) -> True
    Error(#(flag, _)) -> flag
  }
}

pub fn known_list_failure() -> Bool {
  let values: List(String) = native_list([42])
  values == []
}

pub fn check() -> Bool {
  assert charlist.to_string(charlist.from_string("[")) == "["
  let value = identity(produce())
  assert inspect(value) == "Payload"
  assert inspect(tail(captured(value)(), 3)) == "Payload"
  let #(first, second) = pair()
  assert inspect(first) == "Kind"
  assert inspect(second) == "42"
  let assert Error(#(reason, detail)) = identity(native_result())
  assert inspect(reason) == "\"invalid_uri\""
  assert inspect(detail) == "\"input\""
  assert inspect(delayed()) == "Delayed"
  let values = identity(native_list([41, 42]))
  let assert [head, ..rest] = values
  assert inspect(head) == "41"
  assert inspect(rest) == "[42]"
  assert inspect([head, ..values]) == "[41, 41, 42]"
  assert inspect([values, rest]) == "[[41, 42], [42]]"
  assert inspect(native_list([])) == "[]"
  assert parse("/[") == Error(InvalidPath)
  let assert Error(#(tag, chars)) = get_path_and_query("/[")
  assert inspect(tag) == "InvalidUri"
  assert inspect(chars) == "charlist.from_string(\"[\")"
  assert parse("/ok") == Ok(#("/ok", Error(Nil)))
  True
}

pub fn main() {
  assert check()
  io.println("native generic results: payload, 42, InvalidPath")
}
