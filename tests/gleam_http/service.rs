use super::{ExpectedSurface, assert_full_project_graph, assert_surface, run_fixture};

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "map_response_body",
        "method_override",
        "prepend_response_header",
    ],
    types: &[],
    type_aliases: &["Middleware", "Service"],
    constructors: &[],
    functions: r#"
map_response_body: fn(fn(c) -> Response(a), with: fn(a) -> b) -> fn(c) -> Response(b)
method_override: fn(fn(Request(a)) -> b) -> fn(Request(a)) -> b
prepend_response_header: fn(fn(a) -> Response(b), String, String) -> fn(a) -> Response(b)
"#,
};

#[test]
#[ignore = "requires `gleam deps download` in the gleam_http fixture"]
fn tracks_official_gleam_http_service_public_surface_and_project_graph() {
    assert_surface("gleam/http/service", &SURFACE);
    assert_full_project_graph();
}

#[test]
#[ignore = "requires `gleam deps download` in the gleam_http fixture"]
fn runs_every_official_deprecated_http_service_function() {
    run_fixture("gleam_http_service");
}
