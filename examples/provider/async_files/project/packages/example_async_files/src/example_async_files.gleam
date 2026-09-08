import geam/future.{type Future}

/// Reads a UTF-8 file when the Rust host drives the returned Future.
@external(erlang, "example_async_files", "read")
pub fn read(path: String) -> Future(Result(String, String))
