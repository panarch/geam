import gleam/bit_array
import gleam/http
import gleam/list

pub fn main() -> Nil {
  let header_input = <<
    "preamble\r\n--frontier\r\nContent-Type: text/plain\r\n\r\nbody":utf8,
  >>
  use #(before, after) <- list.each(slices(<<>>, header_input, []))

  let assert Ok(parsed) = http.parse_multipart_headers(before, "frontier")
  let assert Ok(http.MultipartHeaders(headers, remaining)) = case parsed {
    http.MoreRequiredForHeaders(continue) -> continue(after)
    http.MultipartHeaders(headers, remaining) ->
      Ok(http.MultipartHeaders(headers, bit_array.append(remaining, after)))
  }
  assert headers == [#("content-type", "text/plain")]
  assert remaining == <<"body":utf8>>

  let body_input = <<"body\r\n--frontier--tail":utf8>>
  use #(before, after) <- list.each(slices(<<>>, body_input, []))

  let assert Ok(parsed) = http.parse_multipart_body(before, "frontier")
  let assert Ok(http.MultipartBody(body, True, remaining)) = case parsed {
    http.MoreRequiredForBody(chunk, continue) -> {
      let assert Ok(http.MultipartBody(body, done, remaining)) = continue(after)
      Ok(http.MultipartBody(bit_array.append(chunk, body), done, remaining))
    }
    http.MultipartBody(body, done, remaining) ->
      Ok(http.MultipartBody(body, done, bit_array.append(remaining, after)))
  }
  assert body == <<"body":utf8>>
  assert remaining == <<"tail":utf8>>

  let assert Ok(http.MoreRequiredForHeaders(continue_headers)) =
    http.parse_multipart_headers(<<"--front":utf8>>, "frontier")
  assert continue_headers(<<>>) == Error(Nil)

  let assert Ok(http.MoreRequiredForBody(_, continue_body)) =
    http.parse_multipart_body(<<"partial":utf8>>, "frontier")
  assert continue_body(<<>>) == Error(Nil)

  Nil
}

fn slices(
  before: BitArray,
  after: BitArray,
  acc: List(#(BitArray, BitArray)),
) -> List(#(BitArray, BitArray)) {
  case after {
    <<first, rest:bytes>> ->
      slices(<<before:bits, first>>, rest, [#(before, after), ..acc])
    _ -> [#(before, after), ..acc]
  }
}

// @geam:expect Nil
