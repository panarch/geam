import gleam/http/request

pub fn main() -> Nil {
  let headers =
    request.new()
    |> request.set_header("X-Mode", "UPPERCASE-VALUE")
    |> request.prepend_header("X-Mode", "first")

  assert headers.headers
    == [#("x-mode", "first"), #("x-mode", "UPPERCASE-VALUE")]
  assert request.get_header(headers, "X-MODE") == Ok("first")
  assert request.get_header(headers, "missing") == Error(Nil)

  let cookies =
    headers
    |> request.set_cookie("first", "1")
    |> request.set_cookie("second", "2")
    |> request.set_cookie("first", "updated")

  assert request.get_cookies(cookies)
    == [#("first", "updated"), #("second", "2")]

  let remaining = request.remove_cookie(cookies, "first")
  assert request.get_cookies(remaining) == [#("second", "2")]
  assert request.remove_cookie(remaining, "missing") == remaining

  let malformed =
    request.new()
    |> request.prepend_header("cookie", "invalid; =empty; good=value")
  assert request.get_cookies(malformed) == [#("good", "value")]

  Nil
}

// @geam:expect Nil
