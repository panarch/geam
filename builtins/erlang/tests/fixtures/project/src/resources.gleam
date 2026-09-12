import gleam/erlang/application
import gleam/string

pub fn main() {
  let assert Ok(root) = application.priv_directory("geam_erlang_test")
  let assert Ok(erlang) = application.priv_directory("gleam_erlang")
  let assert Ok(stdlib) = application.priv_directory("gleam_stdlib")
  #(
    root |> string.replace("\\", "/") |> string.ends_with("/project/priv"),
    erlang
      |> string.replace("\\", "/")
      |> string.ends_with("/build/packages/gleam_erlang/priv"),
    stdlib
      |> string.replace("\\", "/")
      |> string.ends_with("/build/packages/gleam_stdlib/priv"),
    application.priv_directory("../gleam_stdlib"),
    application.priv_directory("unknown_package"),
  )
}
// @geam:expect #(True, True, True, Error(Nil), Error(Nil))
