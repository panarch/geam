use super::{ExpectedSurface, assert_surface, run_fixture};

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "Attributes",
        "Lax",
        "None",
        "Strict",
        "defaults",
        "parse",
        "set_header",
    ],
    types: &[("Attributes", 0), ("SameSitePolicy", 0)],
    type_aliases: &[],
    constructors: &[
        ("Attributes", "Attributes", 6),
        ("SameSitePolicy", "Lax", 0),
        ("SameSitePolicy", "None", 0),
        ("SameSitePolicy", "Strict", 0),
    ],
    functions: r#"
defaults: fn(Scheme) -> Attributes
parse: fn(String) -> List(#(String, String))
set_header: fn(String, String, Attributes) -> String
"#,
};

#[test]
fn tracks_official_gleam_http_cookie_public_surface() {
    assert_surface("gleam/http/cookie", &SURFACE);
}

#[test]
fn runs_every_official_http_cookie_function() {
    run_fixture("gleam_http_cookie");
}
