use crate::ast::TypedModule;
use crate::test_support::{analyse_result, analyse_with, io_imports};

fn analyse(src: &str) -> Result<TypedModule, crate::analyse::AnalyseError> {
    analyse_result(src)
}

#[test]
fn accept_annotated_function_args_and_inferred_return() {
    insta::assert_debug_snapshot!(
        "accept_annotated_function_args_and_inferred_return",
        analyse(
            r#"
pub fn add(x: Int, y: Int) {
  x + y
}
"#
        )
    );
}

#[test]
fn accept_literals_binops_lists_tuples_and_calls() {
    insta::assert_debug_snapshot!(
        "accept_literals_binops_lists_tuples_and_calls",
        analyse(
            r#"
pub fn identity(x: Int) -> Int {
  x
}

pub fn apply(callback: fn(Int) -> Int, value: Int) {
  [1, 2, 3]
  #(value, "ok")
  callback(value)
}
"#
        )
    );
}

#[test]
fn accept_imported_module_select_call() {
    insta::assert_debug_snapshot!(
        "accept_imported_module_select_call",
        analyse_with(
            r#"
import gleam/io

pub fn main(message: String) {
  io.println(message)
}
"#,
            io_imports(),
        )
    );
}

#[test]
fn accept_import_alias_module_select_call() {
    insta::assert_debug_snapshot!(
        "accept_import_alias_module_select_call",
        analyse_with(
            r#"
import gleam/io as logger

pub fn main(message: String) {
  logger.println(message)
}
"#,
            io_imports(),
        )
    );
}

#[test]
fn accept_custom_type_constructor_and_pattern_case() {
    insta::assert_debug_snapshot!(
        "accept_custom_type_constructor_and_pattern_case",
        analyse(
            r#"
pub type Option(a) {
  Some(a)
  None
}

pub fn make(value: Int) {
  Some(value)
}

pub fn unwrap(option: Option(Int)) {
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
fn accept_type_alias_and_let_pattern_binding() {
    insta::assert_debug_snapshot!(
        "accept_type_alias_and_let_pattern_binding",
        analyse(
            r#"
pub type UserId = Int

pub fn main(pair: #(UserId, String)) {
  let #(id, name) = pair
  id
}
"#
        )
    );
}

#[test]
fn accept_case_guard_tuple_index_and_pipeline() {
    insta::assert_debug_snapshot!(
        "accept_case_guard_tuple_index_and_pipeline",
        analyse(
            r#"
pub fn sanitize(value: String) -> String {
  value
}

pub fn display(value: String) -> Nil {
  Nil
}

pub fn main(pair: #(Int, String)) {
  pair.0
  case pair.0 {
    value if value == 1 -> pair.1 |> sanitize |> display
    _ -> Nil
  }
}
"#
        )
    );
}

#[test]
fn accept_unannotated_function_argument_inference() {
    insta::assert_debug_snapshot!(
        "accept_unannotated_function_argument_inference",
        analyse(
            r#"
pub fn identity(value) {
  value
}

pub fn add_one(value) {
  value + 1
}
"#
        )
    );
}

#[test]
fn reject_unknown_import_module() {
    insta::assert_debug_snapshot!(
        "reject_unknown_import_module",
        analyse(
            r#"
import gleam/io

pub fn main(message: String) {
  message
}
"#
        )
    );
}

#[test]
fn reject_unknown_imported_module_value() {
    insta::assert_debug_snapshot!(
        "reject_unknown_imported_module_value",
        analyse_with(
            r#"
import gleam/io

pub fn main(message: String) {
  io.print(message)
}
"#,
            io_imports(),
        )
    );
}

#[test]
fn reject_unknown_local_variable() {
    insta::assert_debug_snapshot!(
        "reject_unknown_local_variable",
        analyse(
            r#"
pub fn main() {
  value
}
"#
        )
    );
}

#[test]
fn reject_unknown_type_annotation() {
    insta::assert_debug_snapshot!(
        "reject_unknown_type_annotation",
        analyse(
            r#"
pub fn main(value: Missing) {
  value
}
"#
        )
    );
}

#[test]
fn reject_type_hole_annotation() {
    insta::assert_debug_snapshot!(
        "reject_type_hole_annotation",
        analyse(
            r#"
pub fn main(value: _type) {
  value
}
"#
        )
    );
}

#[test]
fn reject_type_mismatch_in_binop_list_and_return() {
    insta::assert_debug_snapshot!(
        "reject_type_mismatch_in_binop",
        analyse(
            r#"
pub fn main() {
  1 + "x"
}
"#
        )
    );
    insta::assert_debug_snapshot!(
        "reject_type_mismatch_in_list",
        analyse(
            r#"
pub fn main() {
  [1, "x"]
}
"#
        )
    );
    insta::assert_debug_snapshot!(
        "reject_type_mismatch_in_return",
        analyse(
            r#"
pub fn main() -> String {
  1
}
"#
        )
    );
}

#[test]
fn reject_wrong_function_arity_and_non_function_call() {
    insta::assert_debug_snapshot!(
        "reject_wrong_function_arity",
        analyse(
            r#"
pub fn add(x: Int, y: Int) -> Int {
  x + y
}

pub fn main() {
  add(1)
}
"#
        )
    );
    insta::assert_debug_snapshot!(
        "reject_calling_non_function",
        analyse(
            r#"
pub fn main() {
  1()
}
"#
        )
    );
}

#[test]
fn reject_unsupported_record_field_access() {
    insta::assert_debug_snapshot!(
        "reject_unsupported_record_field_access",
        analyse(
            r#"
pub type User {
  User
}

pub fn main(user: User) {
  user.name
}
"#
        )
    );
}

#[test]
fn reject_empty_function_body_and_block() {
    insta::assert_debug_snapshot!(
        "reject_empty_function_body",
        analyse(
            r#"
pub fn main() {
}
"#
        )
    );
    insta::assert_debug_snapshot!(
        "reject_empty_block",
        analyse(
            r#"
pub fn main() {
  {}
}
"#
        )
    );
}

#[test]
fn reject_case_pattern_arity_and_type_mismatch() {
    insta::assert_debug_snapshot!(
        "reject_case_pattern_arity_mismatch",
        analyse(
            r#"
pub fn main(a: Int, b: Int) {
  case a, b {
    x -> x
  }
}
"#
        )
    );
    insta::assert_debug_snapshot!(
        "reject_case_pattern_type_mismatch",
        analyse(
            r#"
pub fn main(value: Int) {
  case value {
    "x" -> 1
    _ -> 0
  }
}
"#
        )
    );
}
