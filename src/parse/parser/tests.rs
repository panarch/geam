use crate::parse::test_helpers::{parse, reject};

#[test]
fn accept_import_function_call() {
    insta::assert_snapshot!(
        "accept_import_function_call",
        parse(
            r#"
import gleam/io

pub fn main() {
  io.println("hello")
}
"#
        )
    );
}

#[test]
fn accept_typed_function_args_and_return() {
    insta::assert_snapshot!(
        "accept_typed_function_args_and_return",
        parse(
            r#"
pub fn add(left: Int, right: Int) -> Int {
  left + right
}
"#
        )
    );
}

#[test]
fn accept_custom_type_and_constructor_pattern_case() {
    insta::assert_snapshot!(
        "accept_custom_type_and_constructor_pattern_case",
        parse(
            r#"
pub type Option(a) {
  Some(a)
  None
}

pub fn unwrap(option: Option(Int)) -> Int {
  case option {
    Some(value) -> value
    None -> 0
  }
}
"#
        )
    );
}

#[test]
fn accept_type_alias() {
    insta::assert_snapshot!(
        "accept_type_alias",
        parse(
            r#"
type UserId = Int
type Age = Int
"#
        )
    );
}

#[test]
fn accept_import_alias_and_type_shapes() {
    insta::assert_snapshot!(
        "accept_import_alias_and_type_shapes",
        parse(
            r#"
import gleam/io as logger

pub type Handler {
  Handler(
    callback: fn(Int, #(String, Int)) -> result.Result(Int, Error),
  )
}
"#
        )
    );
}

#[test]
fn accept_let_assignments() {
    insta::assert_snapshot!(
        "accept_let_assignments",
        parse(
            r#"
pub fn main(user) {
  let count = 1
  let typed: Int = count
  let #(name, age) = #(user.name, 42)
  typed
}
"#
        )
    );
}

#[test]
fn accept_literals_blocks_and_unary() {
    insta::assert_snapshot!(
        "accept_literals_blocks_and_unary",
        parse(
            r#"
pub fn main(flag) {
  {
    -1
    !flag
    1.5
  }
}
"#
        )
    );
}

#[test]
fn accept_pipeline_binop_list_tuple_field_tuple_index() {
    insta::assert_snapshot!(
        "accept_pipeline_binop_list_tuple_field_tuple_index",
        parse(
            r#"
pub fn transform(user) {
  user.profile.name
  user.0
  #(user, user.profile)
  [1, 2, 3]
  1 + 2 * 3
  "a" <> "b"
  user |> sanitize |> display
}
"#
        )
    );
}

#[test]
fn accept_labeled_call_arguments() {
    insta::assert_snapshot!(
        "accept_labeled_call_arguments",
        parse(
            r#"
pub fn main(user) {
  display(message: user.name, suffix: "ok")
}
"#
        )
    );
}

#[test]
fn accept_string_prefix_pattern() {
    insta::assert_snapshot!(
        "accept_string_prefix_pattern",
        parse(
            r#"
pub fn greeting(text) {
  case text {
    "Hello, " <> name -> name
    "Hi, " as prefix <> rest -> rest
    _ -> text
  }
}
"#
        )
    );
}

#[test]
fn accept_pattern_matrix() {
    insta::assert_snapshot!(
        "accept_pattern_matrix",
        parse(
            r#"
pub fn describe(value) {
  case value {
    1.5 -> "float"
    #(first, second) as pair -> "tuple"
    [head, tail] -> "list"
    Ok(value: inner) -> "constructor"
    _ -> "other"
  }
}
"#
        )
    );
}

#[test]
fn accept_case_guard_syntax() {
    insta::assert_snapshot!(
        "accept_case_guard_syntax",
        parse(
            r#"
pub fn compare(a, b) {
  case a, b {
    x, y if x == y -> True
    _, _ -> False
  }
}
"#
        )
    );
}

#[test]
fn accept_case_alternative_patterns() {
    insta::assert_snapshot!(
        "accept_case_alternative_patterns",
        parse(
            r#"
pub fn either_zero(a, b) {
  case a, b {
    0, _ | _, 0 -> True
    _, _ -> False
  }
}
"#
        )
    );
}

#[test]
fn accept_multiline_call_arguments_and_parentheses() {
    insta::assert_snapshot!(
        "accept_multiline_call_arguments_and_parentheses",
        parse(
            r#"
pub fn main(user) {
  display(
    (
      user
    ),
    "ok"
  )
}
"#
        )
    );
}

#[test]
fn accept_multiline_case_subjects() {
    insta::assert_snapshot!(
        "accept_multiline_case_subjects",
        parse(
            r#"
pub fn compare(a, b) {
  case a,
    b {
    x, y -> True
  }
}
"#
        )
    );
}

#[test]
fn reject_type_alias_keyword() {
    insta::assert_snapshot!(
        "reject_type_alias_keyword",
        reject("type alias UserId = Int")
    );
}

#[test]
fn reject_adjacent_expression_statements_without_newline() {
    insta::assert_snapshot!(
        "reject_adjacent_expression_statements_without_newline",
        reject(
            r#"
pub fn main() {
  1 2
}
"#
        )
    );
}

#[test]
fn reject_const() {
    insta::assert_snapshot!("reject_const", reject("const answer = 42"));
}

#[test]
fn reject_use() {
    insta::assert_snapshot!(
        "reject_use",
        reject(
            r#"
pub fn main(result) {
  use value <- result.try(result)
  value
}
"#
        )
    );
}

#[test]
fn reject_assert() {
    insta::assert_snapshot!(
        "reject_assert",
        reject(
            r#"
pub fn main() {
  assert True
}
"#
        )
    );
}

#[test]
fn reject_let_assert() {
    insta::assert_snapshot!(
        "reject_let_assert",
        reject(
            r#"
pub fn main(value) {
  let assert Ok(inner) = value
  inner
}
"#
        )
    );
}

#[test]
fn reject_todo() {
    insta::assert_snapshot!(
        "reject_todo",
        reject(
            r#"
pub fn main() {
  todo
}
"#
        )
    );
}

#[test]
fn reject_panic() {
    insta::assert_snapshot!(
        "reject_panic",
        reject(
            r#"
pub fn main() {
  panic
}
"#
        )
    );
}

#[test]
fn reject_echo() {
    insta::assert_snapshot!(
        "reject_echo",
        reject(
            r#"
pub fn main() {
  echo "debug"
}
"#
        )
    );
}

#[test]
fn reject_bit_array() {
    insta::assert_snapshot!(
        "reject_bit_array",
        reject(
            r#"
pub fn main() {
  <<1, 2>>
}
"#
        )
    );
}

#[test]
fn reject_bit_array_pattern() {
    insta::assert_snapshot!(
        "reject_bit_array_pattern",
        reject(
            r#"
pub fn main(value) {
  case value {
    <<1>> -> True
  }
}
"#
        )
    );
}

#[test]
fn reject_record_update() {
    insta::assert_snapshot!(
        "reject_record_update",
        reject(
            r#"
pub fn main(user) {
  User(..user, name: "x")
}
"#
        )
    );
}

#[test]
fn reject_anonymous_fn() {
    insta::assert_snapshot!(
        "reject_anonymous_fn",
        reject(
            r#"
pub fn main() {
  fn(x) { x }
}
"#
        )
    );
}

#[test]
fn reject_attribute() {
    insta::assert_snapshot!(
        "reject_attribute",
        reject(
            r#"
@external(erlang, "module", "function")
pub fn main() {
  1
}
"#
        )
    );
}

#[test]
fn reject_target_attribute() {
    insta::assert_snapshot!(
        "reject_target_attribute",
        reject(
            r#"
@target(erlang)
pub fn main() {
  1
}
"#
        )
    );
}

#[test]
fn reject_opaque() {
    insta::assert_snapshot!(
        "reject_opaque",
        reject(
            r#"
pub opaque type Secret {
  Secret
}
"#
        )
    );
}

#[test]
fn reject_list_spread() {
    insta::assert_snapshot!(
        "reject_list_spread",
        reject(
            r#"
pub fn main(xs) {
  [1, ..xs]
}
"#
        )
    );
}

#[test]
fn reject_list_pattern_spread() {
    insta::assert_snapshot!(
        "reject_list_pattern_spread",
        reject(
            r#"
pub fn main(value) {
  case value {
    [head, ..tail] -> head
  }
}
"#
        )
    );
}
