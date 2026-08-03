import gleam/option.{None, Some}
import gleam/uri

fn parsed(source: String) -> uri.Uri {
  let assert Ok(value) = uri.parse(source)
  value
}

pub fn main() {
  assert uri.path_segments("/") == []
  assert uri.path_segments("/weebl//bob") == ["weebl", "bob"]
  assert uri.path_segments("/weebl/../bob") == ["bob"]
  assert uri.path_segments("../bob") == ["bob"]

  assert uri.to_string(uri.Uri(None, None, None, None, "/teapot", None, None))
    == "/teapot"
  assert uri.to_string(uri.Uri(Some("ftp"), None, None, None, "thing.txt", None, None))
    == "ftp:thing.txt"
  assert uri.to_string(uri.Uri(None, None, Some("example.com"), None, "", None, None))
    == "//example.com/"
  assert uri.to_string(uri.Uri(None, None, Some("example.com"), Some(81), "noslash", None, None))
    == "//example.com:81/noslash"
  assert uri.to_string(uri.Uri(None, Some("ignored"), None, Some(81), "noslash", None, Some("frag")))
    == "noslash#frag"

  assert uri.origin(parsed("http://example.test/path?weebl#bob"))
    == Ok("http://example.test")
  assert uri.origin(parsed("http://example.test:8080"))
    == Ok("http://example.test:8080")
  assert uri.origin(parsed("https://mozilla.org:443/"))
    == Ok("https://mozilla.org")
  assert uri.origin(parsed("http://localhost:80/")) == Ok("http://localhost")
  assert uri.origin(parsed("/path")) == Error(Nil)
  assert uri.origin(parsed("file:///dev/null")) == Error(Nil)

  let relative_base = parsed("/relative")
  assert relative_base.scheme == None
  assert relative_base.host == None
  assert relative_base.path == "/relative"
  assert uri.merge(relative_base, parsed("")) == Error(Nil)
  assert uri.merge(
      parsed("http://google.com/weebl"),
      parsed("http://example.com/baz"),
    )
    == uri.parse("http://example.com/baz")
  assert uri.merge(
      parsed("http://google.com/weebl"),
      parsed("http://example.com/.././bob/../../baz"),
    )
    == uri.parse("http://example.com/baz")
  assert uri.merge(
      parsed("http://google.com/weebl"),
      parsed("//example.com/.././bob/../../../baz"),
    )
    == uri.parse("http://example.com/baz")
  assert uri.merge(parsed("http://example.com/weebl/bob"), parsed("/baz"))
    == uri.parse("http://example.com/baz")
  assert uri.merge(parsed("http://example.com/weebl/bob"), parsed("baz"))
    == uri.parse("http://example.com/weebl/baz")
  assert uri.merge(parsed("http://example.com/weebl/"), parsed("baz"))
    == uri.parse("http://example.com/weebl/baz")
  assert uri.merge(parsed("http://example.com"), parsed("baz"))
    == uri.parse("http://example.com/baz")
  assert uri.merge(
      parsed("http://example.com"),
      parsed("/.././bob/../../../baz"),
    )
    == uri.parse("http://example.com/baz")
  assert uri.merge(parsed("http://example.com/weebl/bob"), parsed(""))
    == uri.parse("http://example.com/weebl/bob")
  assert uri.merge(
      parsed("http://example.com/weebl/bob"),
      parsed("#fragment"),
    )
    == uri.parse("http://example.com/weebl/bob#fragment")
  assert uri.merge(
      parsed("http://example.com/weebl/bob"),
      parsed("?query"),
    )
    == uri.parse("http://example.com/weebl/bob?query")
  assert uri.merge(
      parsed("http://example.com/weebl/bob?query1"),
      parsed("?query2"),
    )
    == uri.parse("http://example.com/weebl/bob?query2")
  assert uri.merge(
      parsed("http://example.com/weebl/bob?query"),
      parsed(""),
    )
    == uri.parse("http://example.com/weebl/bob?query")

  Nil
}
// @geam:expect Nil
