import gleam/uri

pub fn main() {
  assert uri.parse_query("weebl+bob=1&city=%C3%B6rebro")
    == Ok([#("weebl bob", "1"), #("city", "örebro")])
  assert uri.parse_query("") == Ok([])
  assert uri.parse_query("a") == Ok([#("a", "")])
  assert uri.parse_query("=x") == Ok([#("", "x")])
  assert uri.parse_query("a=") == Ok([#("a", "")])
  assert uri.parse_query("a=b=c") == Ok([#("a", "b=c")])
  assert uri.parse_query("&&") == Ok([#("", ""), #("", ""), #("", "")])
  assert uri.parse_query("a[]=1&a[]=2")
    == Ok([#("a[]", "1"), #("a[]", "2")])
  assert uri.parse_query("%C2") == Error(Nil)

  let query = [
    #("weebl bob", "1+1-1*1.1~1!1'1(1);%"),
    #("city", "örebro"),
  ]
  let encoded_query = uri.query_to_string(query)
  assert encoded_query
    == "weebl%20bob=1%2B1-1*1.1~1!1'1(1)%3B%25&city=%C3%B6rebro"
  assert uri.parse_query(encoded_query) == Ok(query)

  assert uri.percent_encode("!$'()*+-._~") == "!$'()*+-._~"
  assert uri.percent_encode(" ,;:?[]@/\\&#=")
    == "%20%2C%3B%3A%3F%5B%5D%40%2F%5C%26%23%3D"
  assert uri.percent_encode("ñ") == "%C3%B1"
  assert uri.percent_decode("%c3%b1") == Ok("ñ")
  assert uri.percent_decode("+") == Ok("+")
  assert uri.percent_decode("%") == Error(Nil)
  assert uri.percent_decode("%FF") == Error(Nil)

  Nil
}
// @geam:expect Nil
