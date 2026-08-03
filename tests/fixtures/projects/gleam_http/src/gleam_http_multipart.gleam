import gleam/http

pub fn main() -> Nil {
  let input = <<
    "preamble\r\n--boundary\r\nX-Name: first\r\n folded\r\nContent-Type: text/plain\r\n\r\nbody\r\n--boundary--epilogue":utf8,
  >>

  let assert Ok(http.MultipartBody(preamble, False, input)) =
    http.parse_multipart_body(input, "boundary")
  assert preamble == <<"preamble":utf8>>

  let assert Ok(http.MultipartHeaders(headers, input)) =
    http.parse_multipart_headers(input, "boundary")
  assert headers
    == [#("x-name", "firstfolded"), #("content-type", "text/plain")]

  let assert Ok(http.MultipartBody(body, True, remaining)) =
    http.parse_multipart_body(input, "boundary")
  assert body == <<"body":utf8>>
  assert remaining == <<"epilogue":utf8>>

  let assert Ok(http.MultipartHeaders([], final_remaining)) =
    http.parse_multipart_headers(<<"--boundary--tail":utf8>>, "boundary")
  assert final_remaining == <<"tail":utf8>>

  let assert Ok(http.MultipartHeaders([], empty_header_remaining)) =
    http.parse_multipart_headers(
      <<"--boundary\r\n\r\nbody":utf8>>,
      "boundary",
    )
  assert empty_header_remaining == <<"body":utf8>>

  assert http.parse_multipart_headers(<<"--boundary invalid":utf8>>, "boundary")
    == Error(Nil)
  assert http.parse_multipart_headers(
      <<"--boundary\r\n":utf8, 255, ": value\r\n\r\nbody":utf8>>,
      "boundary",
    )
    == Error(Nil)

  Nil
}

// @geam:expect Nil
