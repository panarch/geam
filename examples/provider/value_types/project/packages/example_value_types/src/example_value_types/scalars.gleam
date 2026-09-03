@external(erlang, "geam_example_value_types_scalars", "join")
pub fn join(left: String, right: String) -> String

@external(erlang, "geam_example_value_types_scalars", "add")
pub fn add(left: Int, right: Int) -> Int

@external(erlang, "geam_example_value_types_scalars", "multiply")
pub fn multiply(left: Float, right: Float) -> Float

@external(erlang, "geam_example_value_types_scalars", "keep_bits")
pub fn keep_bits(value: BitArray) -> BitArray

@external(erlang, "geam_example_value_types_scalars", "keep_codepoint")
pub fn keep_codepoint(value: UtfCodepoint) -> UtfCodepoint

@external(erlang, "geam_example_value_types_scalars", "invert")
pub fn invert(value: Bool) -> Bool

@external(erlang, "geam_example_value_types_scalars", "keep_nil")
pub fn keep_nil(value: Nil) -> Nil
