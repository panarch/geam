import gleam/http
import gleam/http/request
import gleam/option.{None, Some}
import gleam/string
import gleam/uri.{Uri}

pub fn main() -> Nil {
  let original = request.new()
  assert original.method == http.Get
  assert original.scheme == http.Https
  assert original.host == "localhost"
  assert original.port == None
  assert original.body == ""

  let updated =
    original
    |> request.set_method(http.Post)
    |> request.set_scheme(http.Http)
    |> request.set_host("example.com")
    |> request.set_port(8080)
    |> request.set_path("/one/two")
    |> request.set_query([#("q", "hello world"), #("symbol", "/")])

  assert request.path_segments(updated) == ["one", "two"]
  assert request.get_query(updated)
    == Ok([#("q", "hello world"), #("symbol", "/")])
  assert updated.query == Some("q=hello%20world&symbol=%2F")

  let uri = request.to_uri(updated)
  assert uri
    == Uri(
      Some("http"),
      None,
      Some("example.com"),
      Some(8080),
      "/one/two",
      Some("q=hello%20world&symbol=%2F"),
      None,
    )

  let assert Ok(from_uri) = request.from_uri(uri)
  assert from_uri.host == "example.com"
  assert from_uri.port == Some(8080)
  assert request.from_uri(Uri(None, None, None, None, "", None, None))
    == Error(Nil)

  let assert Ok(parsed) = request.to("https://gleam.run/packages?sort=new")
  assert parsed.host == "gleam.run"
  assert parsed.path == "/packages"
  assert parsed.query == Some("sort=new")
  assert request.to("not a uri") == Error(Nil)

  let with_body = request.set_body(updated, "four")
  let mapped = request.map(with_body, string.length)
  assert mapped.body == 4

  let empty_query = request.set_query(original, [])
  assert empty_query.query == Some("")
  assert request.get_query(original) == Ok([])

  let malformed_query =
    request.Request(
      ..original,
      query: Some("%"),
    )
  assert request.get_query(malformed_query) == Error(Nil)

  Nil
}

// @geam:expect Nil
