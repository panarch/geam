@external(erlang, "geam_example_tag_set", "TagSet")
pub type TagSet

@external(erlang, "geam_example_tag_set", "new")
pub fn new() -> TagSet

@external(erlang, "geam_example_tag_set", "insert")
pub fn insert(tags: TagSet, tag: String) -> TagSet

@external(erlang, "geam_example_tag_set", "contains")
pub fn contains(tags: TagSet, tag: String) -> Bool

@external(erlang, "geam_example_tag_set", "size")
pub fn size(tags: TagSet) -> Int
