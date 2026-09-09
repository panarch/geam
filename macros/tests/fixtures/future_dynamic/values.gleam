import dynamic_provider/declarations

@external(erlang, "dynamic_provider", "Dynamic")
pub type Dynamic

@external(erlang, "dynamic_provider", "Box")
pub type Box(item)

@external(erlang, "dynamic_provider", "Snapshot")
pub type Snapshot

@external(erlang, "dynamic_provider", "box_value")
fn box_value(value: item) -> Box(item)

@external(erlang, "dynamic_provider", "snapshot")
fn snapshot(value: value) -> Snapshot

@external(erlang, "dynamic_provider", "cast")
fn cast(value: value) -> Dynamic

@external(erlang, "dynamic_provider", "cast_int")
fn cast_int(value: Int) -> Dynamic

@external(erlang, "dynamic_provider", "cast_int_list")
fn cast_int_list(values: List(Int)) -> Dynamic

@external(erlang, "dynamic_provider", "kind")
fn kind(value: Dynamic) -> String

@external(erlang, "dynamic_provider", "restore_int")
fn restore_int(value: Dynamic) -> Result(Int, Nil)

@external(erlang, "dynamic_provider", "restore_int_list_length")
fn restore_int_list_length(value: Dynamic) -> Result(Int, Nil)

@external(erlang, "dynamic_provider", "is_token")
fn is_token(value: Dynamic) -> Bool

@external(erlang, "dynamic_provider", "is_box")
fn is_box(value: Dynamic) -> Bool

@external(erlang, "dynamic_provider", "token_text")
fn token_text(value: Dynamic) -> Result(String, Nil)

@external(erlang, "dynamic_provider", "box_contains_nine")
fn box_contains_nine(value: Dynamic) -> Result(Bool, Nil)

@external(erlang, "dynamic_provider", "boxed_token_text")
fn boxed_token_text(value: Dynamic) -> Result(String, Nil)

@external(erlang, "dynamic_provider", "tuple_size")
fn tuple_size(value: value) -> Int

@external(erlang, "dynamic_provider", "nested_tuple_size")
fn nested_tuple_size(value: value) -> Int

@external(erlang, "dynamic_provider", "same_hash")
fn same_hash(first: value, second: value) -> Bool

@external(erlang, "dynamic_provider", "inspect_value")
fn inspect_value(value: value) -> String

@external(erlang, "dynamic_provider", "has_exact_type")
fn has_exact_type(stored: Dynamic, witness: value) -> Bool

@external(erlang, "dynamic_provider", "list_summary")
fn list_summary(values: List(item), expected: item) -> #(Int, Bool)

@external(erlang, "dynamic_provider", "list_length")
fn list_length(values: List(item)) -> Int

fn check_list_values() {
  let assert 0 = list_length([])
  let assert 3 = list_length([1, 2, 3])
  let assert #(0, False) = list_summary([], 1)
  let assert #(1, True) = list_summary([increment], increment)
  let assert #(1, True) = list_summary([#(1, [2, 3])], #(1, [2, 3]))
  let assert #(1, True) = list_summary([Ok([1, 2])], Ok([1, 2]))
  let token = declarations.token("retained List item")
  let assert #(1, True) = list_summary([token], token)
  Nil
}

pub type Marker {
  Marker
}

fn increment(value: Int) -> Int {
  value + 1
}

pub fn transfer_flow() {
  check_list_values()
  let first = cast(7)
  let equal = cast(7)
  let text = cast("seven")
  let token = declarations.token("opaque")
  let identity_token = declarations.identity_token(token)
  let #(paired_token, pair_flag) = declarations.identity_token_pair(token)
  let first_token = declarations.first_token([token])
  let token_value = cast(token)
  let boxed = cast(box_value(9))
  let boxed_equal = cast(box_value(9))
  let direct_box = box_value(9)
  let direct_box_equal = box_value(9)
  let boxed_token = cast(box_value(token))
  let typed_int = cast_int(8)
  let typed_list = cast_int_list([1, 2])
  let #(list_length, list_matches) = list_summary([1, 2, 3], 1)
  echo snapshot(11)
  #(
    kind(first) == "Int"
      && kind(text) == "String"
      && kind(token_value) == "External"
      && identity_token == token
      && pair_flag
      && paired_token == token
      && first_token == token,
    restore_int(typed_int) == Ok(8)
      && restore_int(text) == Error(Nil)
      && has_exact_type(first, 0)
      && !has_exact_type(first, "zero"),
    restore_int_list_length(typed_list) == Ok(2),
    is_token(token_value) && !is_token(first) && is_box(boxed),
    token_text(token_value) == Ok("opaque") && box_contains_nine(boxed) == Ok(True),
    boxed_token_text(boxed_token) == Ok("opaque"),
    #(
      tuple_size(#(1, "two", True)) == 3 && tuple_size(1) == 0,
      nested_tuple_size(#(1, #("two", True))) == 2,
      same_hash(first, equal),
      same_hash(boxed, boxed_equal) && direct_box == direct_box_equal,
      inspect_value(first) == "7",
      inspect_value(boxed) == "Box(<opaque>)",
      list_length == 3 && list_matches && first == equal && boxed == boxed_equal,
    ),
  )
}

pub fn main() {
  check_list_values()
  let assert <<codepoint:utf8_codepoint>> = <<"A":utf8>>
  let first = cast(7)
  let equal = cast(7)
  let text = cast("seven")
  let token = declarations.token("opaque")
  let identity_token = declarations.identity_token(token)
  let #(paired_token, pair_flag) = declarations.identity_token_pair(token)
  let assert True = declarations.first_token([token]) == token
  let token_value = cast(token)
  let boxed = cast(box_value(9))
  let boxed_token = cast(box_value(token))
  let typed_int = cast_int(8)
  let typed_list = cast_int_list([1, 2])
  #(
    kind(first),
    kind(text),
    kind(token_value),
    kind(cast(1.5)),
    kind(cast(<<1>>)),
    kind(cast(codepoint)),
    kind(cast(True)),
    kind(cast(Nil)),
    kind(cast([1])),
    kind(cast(#(1, True))),
    kind(cast(Marker)),
    kind(cast(increment)),
    kind(typed_int),
    kind(typed_list),
    restore_int(first),
    restore_int(text),
    restore_int_list_length(typed_list),
    is_token(token_value),
    !is_token(first),
    is_box(boxed),
    !is_box(token_value),
    token_text(token_value),
    token_text(first),
    box_contains_nine(boxed),
    box_contains_nine(first),
    boxed_token_text(boxed_token),
    pair_flag,
    paired_token == token,
    tuple_size(#(1, "two", True)),
    tuple_size(1),
    nested_tuple_size(#(1, #("two", True))),
    same_hash(first, equal),
    has_exact_type(first, 0),
    !has_exact_type(first, "zero"),
    list_summary([1, 2, 3], 1),
    list_summary([], 1),
    first == equal,
    first,
    token,
    identity_token,
  )
}
