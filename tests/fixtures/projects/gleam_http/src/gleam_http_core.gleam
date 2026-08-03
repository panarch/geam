import gleam/http

pub fn main() -> Nil {
  assert http.parse_method("CONNECT") == Ok(http.Connect)
  assert http.parse_method("DELETE") == Ok(http.Delete)
  assert http.parse_method("GET") == Ok(http.Get)
  assert http.parse_method("HEAD") == Ok(http.Head)
  assert http.parse_method("OPTIONS") == Ok(http.Options)
  assert http.parse_method("PATCH") == Ok(http.Patch)
  assert http.parse_method("POST") == Ok(http.Post)
  assert http.parse_method("PUT") == Ok(http.Put)
  assert http.parse_method("TRACE") == Ok(http.Trace)
  assert http.parse_method("custom") == Ok(http.Other("custom"))
  assert http.parse_method("!#$%&'*+-.^_`|~abcABC123")
    == Ok(http.Other("!#$%&'*+-.^_`|~abcABC123"))
  assert http.parse_method("") == Error(Nil)
  assert http.parse_method("not valid") == Error(Nil)

  assert http.method_to_string(http.Connect) == "CONNECT"
  assert http.method_to_string(http.Delete) == "DELETE"
  assert http.method_to_string(http.Get) == "GET"
  assert http.method_to_string(http.Head) == "HEAD"
  assert http.method_to_string(http.Options) == "OPTIONS"
  assert http.method_to_string(http.Patch) == "PATCH"
  assert http.method_to_string(http.Post) == "POST"
  assert http.method_to_string(http.Put) == "PUT"
  assert http.method_to_string(http.Trace) == "TRACE"
  assert http.method_to_string(http.Other("CUSTOM")) == "CUSTOM"

  assert http.scheme_to_string(http.Http) == "http"
  assert http.scheme_to_string(http.Https) == "https"
  assert http.scheme_from_string("HTTP") == Ok(http.Http)
  assert http.scheme_from_string("Https") == Ok(http.Https)
  assert http.scheme_from_string("ftp") == Error(Nil)

  assert http.parse_content_disposition("inline")
    == Ok(http.ContentDisposition("inline", []))
  assert http.parse_content_disposition(
      "form-data; NAME=upload; filename=\"file\\\".txt\"",
    )
    == Ok(http.ContentDisposition("form-data", [
      #("name", "upload"),
      #("filename", "file\".txt"),
    ]))
  assert http.parse_content_disposition("file; filename=\"unfinished")
    == Error(Nil)

  Nil
}

// @geam:expect Nil
