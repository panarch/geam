use super::{ExpectedSurface, assert_surface, run_fixture};

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "Request",
        "from_uri",
        "get_cookies",
        "get_header",
        "get_query",
        "map",
        "new",
        "path_segments",
        "prepend_header",
        "remove_cookie",
        "set_body",
        "set_cookie",
        "set_header",
        "set_host",
        "set_method",
        "set_path",
        "set_port",
        "set_query",
        "set_scheme",
        "to",
        "to_uri",
    ],
    types: &[("Request", 1)],
    type_aliases: &[],
    constructors: &[("Request", "Request", 8)],
    functions: r#"
from_uri: fn(Uri) -> Result(Request(String), Nil)
get_cookies: fn(Request(body)) -> List(#(String, String))
get_header: fn(Request(body), String) -> Result(String, Nil)
get_query: fn(Request(body)) -> Result(List(#(String, String)), Nil)
map: fn(Request(old_body), fn(old_body) -> new_body) -> Request(new_body)
new: fn() -> Request(String)
path_segments: fn(Request(body)) -> List(String)
prepend_header: fn(Request(body), String, String) -> Request(body)
remove_cookie: fn(Request(body), String) -> Request(body)
set_body: fn(Request(old_body), new_body) -> Request(new_body)
set_cookie: fn(Request(body), String, String) -> Request(body)
set_header: fn(Request(body), String, String) -> Request(body)
set_host: fn(Request(body), String) -> Request(body)
set_method: fn(Request(body), Method) -> Request(body)
set_path: fn(Request(body), String) -> Request(body)
set_port: fn(Request(body), Int) -> Request(body)
set_query: fn(Request(body), List(#(String, String))) -> Request(body)
set_scheme: fn(Request(body), Scheme) -> Request(body)
to: fn(String) -> Result(Request(String), Nil)
to_uri: fn(Request(body)) -> Uri
"#,
};

#[test]
fn tracks_official_gleam_http_request_public_surface() {
    assert_surface("gleam/http/request", &SURFACE);
}

#[test]
fn runs_official_http_request_construction_and_updates() {
    run_fixture("gleam_http_request_basics");
}

#[test]
fn runs_official_http_request_headers_and_cookies() {
    run_fixture("gleam_http_request_headers_and_cookies");
}
