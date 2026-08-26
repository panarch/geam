use super::{ExpectedSurface, assert_surface, run_fixture};

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "Connect",
        "ContentDisposition",
        "Delete",
        "Get",
        "Head",
        "Http",
        "Https",
        "MoreRequiredForBody",
        "MoreRequiredForHeaders",
        "MultipartBody",
        "MultipartHeaders",
        "Options",
        "Other",
        "Patch",
        "Post",
        "Put",
        "Trace",
        "method_to_string",
        "parse_content_disposition",
        "parse_method",
        "parse_multipart_body",
        "parse_multipart_headers",
        "scheme_from_string",
        "scheme_to_string",
    ],
    types: &[
        ("ContentDisposition", 0),
        ("Method", 0),
        ("MultipartBody", 0),
        ("MultipartHeaders", 0),
        ("Scheme", 0),
    ],
    type_aliases: &["Header"],
    constructors: &[
        ("ContentDisposition", "ContentDisposition", 2),
        ("Method", "Connect", 0),
        ("Method", "Delete", 0),
        ("Method", "Get", 0),
        ("Method", "Head", 0),
        ("Method", "Options", 0),
        ("Method", "Other", 1),
        ("Method", "Patch", 0),
        ("Method", "Post", 0),
        ("Method", "Put", 0),
        ("Method", "Trace", 0),
        ("MultipartBody", "MoreRequiredForBody", 2),
        ("MultipartBody", "MultipartBody", 3),
        ("MultipartHeaders", "MoreRequiredForHeaders", 1),
        ("MultipartHeaders", "MultipartHeaders", 2),
        ("Scheme", "Http", 0),
        ("Scheme", "Https", 0),
    ],
    functions: r#"
method_to_string: fn(Method) -> String
parse_content_disposition: fn(String) -> Result(ContentDisposition, Nil)
parse_method: fn(String) -> Result(Method, Nil)
parse_multipart_body: fn(BitArray, String) -> Result(MultipartBody, Nil)
parse_multipart_headers: fn(BitArray, String) -> Result(MultipartHeaders, Nil)
scheme_from_string: fn(String) -> Result(Scheme, Nil)
scheme_to_string: fn(Scheme) -> String
"#,
};

#[test]
fn tracks_official_gleam_http_public_surface() {
    assert_surface("gleam/http", &SURFACE);
}

#[test]
fn runs_official_http_methods_schemes_and_content_disposition() {
    run_fixture("gleam_http_core");
}

#[test]
fn runs_official_http_multipart_parsing() {
    run_fixture("gleam_http_multipart");
}

#[test]
fn runs_official_http_streaming_multipart_continuations() {
    run_fixture("gleam_http_multipart_streaming");
}
