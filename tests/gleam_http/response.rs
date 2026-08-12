use super::{ExpectedSurface, assert_surface, run_fixture};

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "Response",
        "expire_cookie",
        "get_cookies",
        "get_header",
        "map",
        "new",
        "prepend_header",
        "redirect",
        "set_body",
        "set_cookie",
        "set_header",
        "try_map",
    ],
    types: &[("Response", 1)],
    type_aliases: &[],
    constructors: &[("Response", "Response", 3)],
    functions: r#"
expire_cookie: fn(Response(body), String, cookie.Attributes) -> Response(body)
get_cookies: fn(Response(body)) -> List(#(String, String))
get_header: fn(Response(body), String) -> Result(String, Nil)
map: fn(Response(old_body), fn(old_body) -> new_body) -> Response(new_body)
new: fn(Int) -> Response(String)
prepend_header: fn(Response(body), String, String) -> Response(body)
redirect: fn(String) -> Response(String)
set_body: fn(Response(old_body), new_body) -> Response(new_body)
set_cookie: fn(Response(body), String, String, cookie.Attributes) -> Response(body)
set_header: fn(Response(body), String, String) -> Response(body)
try_map: fn(Response(old_body), fn(old_body) -> Result(new_body, error)) -> Result(Response(new_body), error)
"#,
};

#[test]
fn tracks_official_gleam_http_response_public_surface() {
    assert_surface("gleam/http/response", &SURFACE);
}

#[test]
fn runs_official_http_response_construction_and_updates() {
    run_fixture("gleam_http_response_basics");
}

#[test]
fn runs_official_http_response_cookies() {
    run_fixture("gleam_http_response_cookies");
}
