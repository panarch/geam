import gleam/http
import gleam/http/request.{type Request}
import gleam/http/response
import gleam/http/service
import gleam/string

pub fn main() -> Nil {
  let base: service.Service(String, String) = fn(request: Request(String)) {
    response.new(200)
    |> response.set_body(request.body)
  }

  let mapped = service.map_response_body(base, with: string.length)
  assert mapped(request.set_body(request.new(), "body")).body == 4

  let middleware: service.Middleware(String, String, String, String) = fn(next) {
    service.prepend_response_header(next, "X-Service", "active")
  }
  let wrapped = middleware(base)
  assert wrapped(request.new()).headers == [#("x-service", "active")]

  let report_method = fn(request: Request(String)) { request.method }
  let override = service.method_override(report_method)

  let delete =
    request.new()
    |> request.set_method(http.Post)
    |> request.set_query([#("_method", "DELETE")])
  assert override(delete) == http.Delete

  let disallowed =
    request.new()
    |> request.set_method(http.Post)
    |> request.set_query([#("_method", "GET")])
  assert override(disallowed) == http.Post

  let invalid =
    request.new()
    |> request.set_method(http.Post)
    |> request.set_query([#("_method", "not valid")])
  assert override(invalid) == http.Post

  let not_post =
    request.new()
    |> request.set_method(http.Put)
    |> request.set_query([#("_method", "DELETE")])
  assert override(not_post) == http.Put

  Nil
}

// @geam:expect Nil
