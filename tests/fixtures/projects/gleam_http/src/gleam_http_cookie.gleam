import gleam/http
import gleam/http/cookie
import gleam/option.{None, Some}

pub fn main() -> Nil {
  let http_defaults = cookie.defaults(http.Http)
  assert http_defaults.secure == False
  assert http_defaults.http_only == True
  assert http_defaults.path == Some("/")
  assert http_defaults.same_site == Some(cookie.Lax)

  let https_defaults = cookie.defaults(http.Https)
  assert https_defaults.secure == True

  let strict =
    cookie.Attributes(
      max_age: Some(0),
      domain: Some("example.com"),
      path: Some("/account"),
      secure: True,
      http_only: True,
      same_site: Some(cookie.Strict),
    )
  assert cookie.set_header("session", "abc", strict)
    == "session=abc; Expires=Thu, 01 Jan 1970 00:00:00 GMT; Max-Age=0; Domain=example.com; Path=/account; Secure; HttpOnly; SameSite=Strict"

  let none =
    cookie.Attributes(
      max_age: None,
      domain: None,
      path: None,
      secure: False,
      http_only: False,
      same_site: Some(cookie.None),
    )
  assert cookie.set_header("cross-site", "yes", none)
    == "cross-site=yes; SameSite=None"

  assert cookie.parse("first=1; second=2, third=3")
    == [#("first", "1"), #("second", "2"), #("third", "3")]
  assert cookie.parse(" duplicate=first; duplicate=second ")
    == [#("duplicate", "first"), #("duplicate", "second")]
  assert cookie.parse("missing; =empty; line\rbreak=value; valid=ok")
    == [#("valid", "ok")]

  Nil
}

// @geam:expect Nil
