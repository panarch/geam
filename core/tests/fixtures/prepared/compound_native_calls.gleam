@external(erlang, "compound_native", "mirror")
fn mirror(value: String) -> String

pub fn string_native_root(value: String) -> String {
  mirror(value)
}

pub fn string_native_after(value: String) -> String {
  let returned = mirror(value)
  keep_string(returned)
}

fn keep_string(value: String) -> String {
  value
}

@external(erlang, "compound_native", "next")
fn next(value: String) -> Result(#(String, String), Nil)

@external(erlang, "compound_native", "pair")
fn pair(value: String) -> #(Bool, #(String, String))

@external(erlang, "compound_native", "envelope")
fn envelope(value: String) -> Envelope

@external(erlang, "compound_native", "integer")
fn integer(value: Int) -> Result(Int, Nil)

@external(erlang, "compound_native", "marker")
fn marker() -> Marker

pub fn custom_native_root() -> Marker {
  marker()
}

pub fn marker_matches(value: Marker) -> Bool {
  case value {
    Found -> True
  }
}

fn identity_marker(value: Marker) -> Marker {
  value
}

pub fn fieldless_control() -> Bool {
  case identity_marker(marker()) {
    Found -> True
  }
}

pub fn empty_alias(value: String) -> Bool {
  case envelope(value) {
    Empty as found -> is_empty(found)
    _ -> False
  }
}

fn is_empty(value: Envelope) -> Bool {
  case value {
    Empty -> True
    _ -> False
  }
}

pub fn nil_index(value: #(Int, Nil)) -> Nil {
  value.1
}

fn nil_field(value: NilBox) -> Nil {
  value.field
}

pub fn nil_field_control() -> Nil {
  nil_field(NilBox(Nil))
}

@external(erlang, "compound_native", "primitives")
fn primitives(
  i: Int,
  f: Float,
  b: Bool,
  n: Nil,
  u: UtfCodepoint,
  s: String,
  a: BitArray,
) -> #(Int, Float, Bool, Nil, UtfCodepoint, String, BitArray)

pub fn walk(value: String, count: Int) -> Int {
  case next(value) {
    Ok(#(_, rest)) -> walk(rest, count + 1)
    Error(Nil) -> count
  }
}

pub fn walk_pair(value: String, count: Int) -> Int {
  case pair(value) {
    #(True, #(_, rest)) -> walk_pair(rest, count + 1)
    #(False, #(_, _)) -> count
  }
}

pub fn walk_envelope(value: String, count: Int) -> Int {
  case envelope(value) {
    Wrapped(#(True, Ok(#(_, rest)))) -> walk_envelope(rest, count + 1)
    Wrapped(#(False, Error(Nil))) -> count
    Empty -> -1
    _ -> -2
  }
}

pub fn after_native_panic(value: String) -> Int {
  let _ = next(value)
  panic as "after compound native"
}

pub fn guarded(value: Int) -> Int {
  case integer(value) {
    Ok(42) -> 7
    Ok(number as alias) if alias > 0 -> number + alias
    Ok(number) -> number
    Error(Nil) -> -1
  }
}

pub fn identity_tuple(
  i: Int,
  f: Float,
  b: Bool,
  n: Nil,
  u: UtfCodepoint,
  s: String,
  a: BitArray,
) -> #(Int, Float, Bool, Nil, UtfCodepoint, String, BitArray) {
  primitives(i, f, b, n, u, s, a)
}

pub fn all_fields(
  i: Int,
  f: Float,
  b: Bool,
  n: Nil,
  u: UtfCodepoint,
  s: String,
  a: BitArray,
) -> #(Int, Float, Bool, Nil, UtfCodepoint, String, BitArray) {
  case primitives(i, f, b, n, u, s, a) {
    #(number, fraction, flag, Nil, scalar, text, bytes) ->
      identity_tuple(number, fraction, flag, n, scalar, text, bytes)
  }
}

pub fn assert_zero_float(
  i: Int,
  f: Float,
  b: Bool,
  n: Nil,
  u: UtfCodepoint,
  s: String,
  a: BitArray,
) -> Bool {
  let assert #(_, 0.0, _, Nil, _, _, _) = primitives(i, f, b, n, u, s, a)
  True
}

type ProjectionFields {
  ProjectionFields(
    number: Int,
    fraction: Float,
    flag: Bool,
    nothing: Nil,
    point: UtfCodepoint,
    text: String,
    bytes: BitArray,
    marker: Marker,
    pair: #(Int, Int),
  )
}

fn keep_number_pair(value: #(Int, Int)) {
  value
}

fn project_custom_fields(record: ProjectionFields) {
  let marker = identity_marker(record.marker)
  let pair = keep_number_pair(record.pair)
  let number = record.number + pair.0
  let flag = marker_matches(marker) && record.flag
  primitives(
    number,
    record.fraction,
    flag,
    record.nothing,
    record.point,
    record.text,
    record.bytes,
  )
}

pub fn custom_field_values(
  i: Int,
  f: Float,
  b: Bool,
  n: Nil,
  u: UtfCodepoint,
  s: String,
  a: BitArray,
) -> #(Int, Float, Bool, Nil, UtfCodepoint, String, BitArray) {
  project_custom_fields(ProjectionFields(i, f, b, n, u, s, a, Found, #(2, 3)))
}

pub fn aliased(value: String) -> #(Bool, #(String, String)) {
  case pair(value) {
    #(_, #(_, _) as inner) as whole -> keep_pair(whole, inner)
  }
}

fn keep_pair(
  whole: #(Bool, #(String, String)),
  _inner: #(String, String),
) -> #(Bool, #(String, String)) {
  whole
}

pub fn after_native(value: String, count: Int) -> Int {
  let returned = next(value)
  echo returned
  case returned {
    Ok(#(_, _)) -> count + 1
    Error(Nil) -> count
  }
}

import compound_native_types.{
  type Envelope, type Marker, type NilBox, Empty, Found, NilBox, Wrapped,
}
