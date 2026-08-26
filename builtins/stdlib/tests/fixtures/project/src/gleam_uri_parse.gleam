import gleam/option.{None, Some}
import gleam/uri

pub fn main() {
  assert uri.empty
    == uri.Uri(
      scheme: None,
      userinfo: None,
      host: None,
      port: None,
      path: "",
      query: None,
      fragment: None,
    )

  let assert Ok(full) =
    uri.parse("https://weebl:bob@example.com:1234/path?query=true#fragment")
  assert full
    == uri.Uri(
      scheme: Some("https"),
      userinfo: Some("weebl:bob"),
      host: Some("example.com"),
      port: Some(1234),
      path: "/path",
      query: Some("query=true"),
      fragment: Some("fragment"),
    )

  let assert Ok(empty) = uri.parse("")
  assert empty == uri.empty

  let assert Ok(empty_host) = uri.parse("//:")
  assert empty_host.host == Some("")
  assert empty_host.port == None

  let unicode_source =
    "HTTPS://EXAMPLE.COM/경로/가나다라마바사아자차카타파하/🙂🙂🙂?도시=서울#조각"
  let assert Ok(unicode) = uri.parse(unicode_source)
  assert unicode.scheme == Some("https")
  assert unicode.host == Some("EXAMPLE.COM")
  assert unicode.path == "/경로/가나다라마바사아자차카타파하/🙂🙂🙂"
  assert unicode.query == Some("도시=서울")
  assert unicode.fragment == Some("조각")
  assert uri.to_string(unicode)
    == "https://EXAMPLE.COM/경로/가나다라마바사아자차카타파하/🙂🙂🙂?도시=서울#조각"

  assert uri.parse(":https") == Error(Nil)

  Nil
}
// @geam:expect Nil
