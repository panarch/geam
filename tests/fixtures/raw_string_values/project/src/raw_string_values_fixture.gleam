import gleam/bytes_tree
import gleam/io
import gleam/string_tree

pub type ShaHash {
  Sha1
}

@external(erlang, "crypto", "hash")
fn crypto_hash(hash: ShaHash, data: String) -> String

@external(erlang, "base64", "encode")
fn base64_encode(data: String) -> String

@external(erlang, "fixture", "sample")
pub fn sample(index: Int) -> Result(String, Nil)

@external(erlang, "fixture", "byte_slice")
pub fn byte_slice(value: String, start: Int, length: Int) -> Result(String, Nil)

@external(erlang, "fixture", "bytes")
pub fn bytes(value: String) -> List(Int)

@external(erlang, "fixture", "identity")
fn identity(value: item) -> item

type Held {
  Held(String)
}

pub fn captured_round_trip(value: String) -> String {
  let held = identity(Held(value))
  let callback =
    identity(fn() {
      let Held(restored) = held
      identity(restored)
    })
  callback()
}

pub fn handshake(key: String) -> #(String, String) {
  let digest = crypto_hash(Sha1, key <> "258EAFA5-E914-47DA-95CA-C5AB0DC85B11")
  #(digest, base64_encode(digest))
}

pub fn strip_prefix(value: String) -> String {
  case value {
    "λ" <> rest -> rest
    _ -> value
  }
}

pub fn count_prefix(value: String, count: Int) -> Int {
  case value {
    "λ" <> rest -> count_prefix(rest, count + 1)
    _ -> count
  }
}

pub fn nested(value: String) -> #(List(String), Result(String, Nil)) {
  #([value, value <> value], Ok(value))
}

pub fn main() {
  let #(digest, accept) = handshake("dGhlIHNhbXBsZSBub25jZQ==")
  assert bytes(digest)
    == [
      179, 122, 79, 44, 192, 98, 79, 22, 144, 246, 70, 6, 207, 56, 89, 69, 178,
      190, 196, 234,
    ]
  assert accept == "s3pPLMBiTxaQ9kYGzzhZRbK+xOo="
  let assert Ok(raw) = sample(3)
  let assert Ok(codepoint) = sample(5)
  assert bytes(strip_prefix("λ" <> raw)) == [255, 254]
  assert bytes(strip_prefix(raw)) == [255, 254]
  assert count_prefix("λλλ" <> raw, 7) == 10
  assert count_prefix(raw, 7) == 7
  assert bytes(codepoint) == [195, 169, 240, 159, 153, 130]
  let assert Ok(first) = byte_slice(codepoint, 0, 1)
  let assert Ok(second) = byte_slice(codepoint, 1, 1)
  assert bytes(first) == [195]
  assert first <> second == "é"
  assert captured_round_trip(raw) == raw
  assert nested(raw) == #([raw, raw <> raw], Ok(raw))
  let tree =
    string_tree.concat([
      string_tree.from_string("λ"),
      string_tree.from_string(raw),
    ])
  assert bytes_tree.to_bit_array(bytes_tree.from_string_tree(tree))
    == <<206, 187, 255, 254>>
  io.println(accept)
  io.println(raw)
}
