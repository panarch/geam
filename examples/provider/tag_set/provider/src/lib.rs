use geam::provider::{BigInt, StringValue};
use std::collections::BTreeSet;

#[geam::provider(
    package = "example_tag_set",
    modules = [tag_set],
)]
pub struct Component;

#[geam::module(path = "example_tag_set")]
mod tag_set {
    use super::{BTreeSet, BigInt, StringValue};

    #[geam::external(name = "TagSet")]
    #[derive(Clone, Default, PartialEq, Eq, Hash)]
    struct TagSet {
        tags: BTreeSet<StringValue>,
    }

    #[geam::function]
    fn new() -> TagSet {
        TagSet::default()
    }

    #[geam::function]
    fn insert(tags: &TagSet, tag: StringValue) -> TagSet {
        let mut updated = tags.clone();
        updated.tags.insert(tag);
        updated
    }

    #[geam::function]
    fn contains(tags: &TagSet, tag: StringValue) -> bool {
        tags.tags.contains(&tag)
    }

    #[geam::function]
    fn size(tags: &TagSet) -> BigInt {
        BigInt::from(tags.tags.len())
    }
}
