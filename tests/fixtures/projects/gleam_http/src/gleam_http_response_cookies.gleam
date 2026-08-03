import gleam/http
import gleam/http/cookie
import gleam/http/response
import gleam/option.{None, Some}

pub fn main() -> Nil {
  let empty =
    cookie.Attributes(
      max_age: None,
      domain: None,
      path: None,
      secure: False,
      http_only: False,
      same_site: None,
    )
  let with_cookie = response.set_cookie(response.new(200), "first", "1", empty)
  assert response.get_header(with_cookie, "set-cookie") == Ok("first=1")
  assert response.get_cookies(with_cookie) == [#("first", "1")]

  let secure =
    cookie.Attributes(
      max_age: Some(60),
      domain: Some("example.com"),
      path: Some("/"),
      secure: True,
      http_only: True,
      same_site: Some(cookie.Strict),
    )
  let secure_response =
    response.set_cookie(response.new(200), "session", "abc", secure)
  assert response.get_header(secure_response, "set-cookie")
    == Ok(
      "session=abc; Max-Age=60; Domain=example.com; Path=/; Secure; HttpOnly; SameSite=Strict",
    )

  let expired =
    response.expire_cookie(response.new(200), "session", cookie.defaults(http.Http))
  assert response.get_header(expired, "set-cookie")
    == Ok(
      "session=; Expires=Thu, 01 Jan 1970 00:00:00 GMT; Max-Age=0; Path=/; HttpOnly; SameSite=Lax",
    )

  let malformed =
    response.new(200)
    |> response.prepend_header("set-cookie", "invalid")
  assert response.get_cookies(malformed) == []

  Nil
}

// @geam:expect Nil
