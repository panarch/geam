import gleam/order
import gleam/string

pub fn main() {
  assert string.is_empty("")
  assert string.length("A👍🏽é") == 3
  assert string.reverse("A👍🏽é") == "é👍🏽A"
  assert string.replace("a-b-a", each: "a", with: "x") == "x-b-x"
  assert string.lowercase("Gleam İ") == "gleam i̇"
  assert string.uppercase("Gleam ß") == "GLEAM SS"
  assert string.compare("A", "B") == order.Lt
  assert string.compare("A", "A") == order.Eq
  assert string.compare("B", "A") == order.Gt
  assert string.crop(from: "The Lone Gunmen", before: "Lone") == "Lone Gunmen"
  assert string.crop(from: "The Lone Gunmen", before: "Fox") == "The Lone Gunmen"
  assert string.contains(does: "theory", contain: "ory")
  assert string.starts_with("theory", "the")
  assert string.ends_with("theory", "ory")
  assert string.append(to: "butter", suffix: "fly") == "butterfly"
  assert string.concat(["never", "the", "less"]) == "nevertheless"
  assert string.repeat("ha", times: 3) == "hahaha"
  assert string.join(["home", "gleam"], with: "/") == "home/gleam"
  assert string.pad_start("121", to: 5, with: ".") == "..121"
  assert string.pad_end("123", to: 5, with: ".") == "123.."
  assert string.byte_size("👍") == 4
  assert string.remove_prefix(from: "@lpil", matching: "@") == "lpil"
  assert string.remove_prefix(from: "hello!", matching: "@") == "hello!"
  assert string.remove_suffix(from: "Hello!", matching: "!") == "Hello"
  assert string.remove_suffix(from: "Hello!?", matching: "!") == "Hello!?"

  "strings"
}

// @geam:expect "strings"
