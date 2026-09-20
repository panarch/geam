@external(erlang, "geam_benchmarks_ffi", "environment")
@external(javascript, "../geam_benchmarks_ffi.mjs", "environment")
pub fn environment(name: String) -> Result(String, String)

@external(erlang, "geam_benchmarks_ffi", "monotonic_ns")
@external(javascript, "../geam_benchmarks_ffi.mjs", "monotonic_ns")
pub fn monotonic_ns() -> Int

@external(erlang, "geam_benchmarks_ffi", "consume")
@external(javascript, "../geam_benchmarks_ffi.mjs", "consume")
pub fn consume(value: a) -> a
