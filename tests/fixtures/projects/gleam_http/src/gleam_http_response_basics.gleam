import gleam/http/response
import gleam/string

pub fn main() -> Nil {
  let original = response.new(201)
  assert original.status == 201
  assert original.body == ""

  let headers =
    original
    |> response.set_header("X-Mode", "UPPERCASE-VALUE")
    |> response.prepend_header("X-Mode", "first")
  assert headers.headers
    == [#("x-mode", "first"), #("x-mode", "UPPERCASE-VALUE")]
  assert response.get_header(headers, "X-MODE") == Ok("first")
  assert response.get_header(headers, "missing") == Error(Nil)

  let with_body = response.set_body(headers, "abcd")
  assert response.map(with_body, string.reverse).body == "dcba"
  assert response.try_map(with_body, fn(body) { Ok(string.length(body)) })
    == Ok(response.Response(201, headers.headers, 4))
  assert response.try_map(with_body, fn(_) { Error("failed") })
    == Error("failed")

  let redirect = response.redirect("/next")
  assert redirect.status == 303
  assert response.get_header(redirect, "location") == Ok("/next")
  assert redirect.body == "You are being redirected to /next"

  Nil
}

// @geam:expect Nil
