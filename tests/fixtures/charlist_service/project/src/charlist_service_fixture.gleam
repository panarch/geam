import gleam/dict
import gleam/erlang/charlist.{type Charlist}
import native_observer

@external(erlang, "charlist_service_fixture", "from_text")
pub fn from_text(text: String) -> Charlist

@external(erlang, "charlist_service_fixture", "header")
pub fn header() -> #(Charlist, Charlist)

@external(erlang, "charlist_service_fixture", "metadata")
pub fn metadata() -> #(#(Charlist, Int, Charlist), List(#(Charlist, Charlist)))

pub fn main() {
  assert charlist.to_string(from_text("")) == ""
  assert charlist.to_string(from_text("AZ")) == "AZ"
  assert charlist.to_string(from_text("e\u{301}")) == "e\u{301}"
  let unicode = from_text("\u{0}Aé🙂")
  assert charlist.to_string(unicode) == "\u{0}Aé🙂"
  assert unicode == charlist.from_string("\u{0}Aé🙂")
  assert unicode != from_text("different")
  let values = dict.from_list([#(unicode, "original key")])
  assert dict.get(values, charlist.from_string("\u{0}Aé🙂"))
    == Ok("original key")

  let #(name, value) = header()
  assert charlist.to_string(name) == "user-agent"
  assert charlist.to_string(value) == "geam-charlist-fixture/1.0"

  let response = metadata()
  let #(status, headers) = response
  assert charlist.to_string(status.0) == "HTTP/1.1"
  assert status.1 == 200
  assert charlist.to_string(status.2) == "OK"
  let assert [#(empty_name, empty), #(unicode_name, unicode_value)] = headers
  assert charlist.to_string(empty_name) == "x-empty"
  assert charlist.to_string(empty) == ""
  assert charlist.to_string(unicode_name) == "x-unicode"
  assert charlist.to_string(unicode_value) == "\u{0}Aé🙂"

  native_observer.assert_equal(unicode_value, [0, 65, 233, 128_578])
  native_observer.assert_equal(empty, [])
  native_observer.assert_equal(
    response,
    #(#([72, 84, 84, 80, 47, 49, 46, 49], 200, [79, 75]), [
      #([120, 45, 101, 109, 112, 116, 121], []),
      #([120, 45, 117, 110, 105, 99, 111, 100, 101], [0, 65, 233, 128_578]),
    ]),
  )
  #(unicode, response)
}
