const escaped = "line\n\tend"

pub fn main() {
  assert <<escaped:utf8>> == <<108, 105, 110, 101, 10, 9, 101, 110, 100>>

  let assert "line\n\tend" = escaped

  assert case escaped {
    "line\n\tend" -> True
    _ -> False
  }

  assert case escaped {
    "line\n\t" <> suffix -> suffix == "end"
    _ -> False
  }

  let bits = <<"A\n":utf8>>
  assert bits == <<65, 10>>
  let assert <<"A\n":utf8>> = bits

  "escaped"
}

// @geam:expect String("escaped")
