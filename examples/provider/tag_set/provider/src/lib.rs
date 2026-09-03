use geam::provider::{BigInt, EcoString};
use std::collections::BTreeSet;

#[geam::provider(
    package = "example_tag_set",
    modules = [tag_set],
)]
pub struct Component;

#[geam::module(path = "example_tag_set")]
mod tag_set {
    use super::{BTreeSet, BigInt, EcoString};

    #[geam::external(name = "TagSet")]
    #[derive(Clone, Default, PartialEq, Eq, Hash)]
    struct TagSet {
        tags: BTreeSet<EcoString>,
    }

    #[geam::function]
    fn new() -> TagSet {
        TagSet::default()
    }

    #[geam::function]
    fn insert(tags: &TagSet, tag: EcoString) -> TagSet {
        let mut updated = tags.clone();
        updated.tags.insert(tag);
        updated
    }

    #[geam::function]
    fn contains(tags: &TagSet, tag: EcoString) -> bool {
        tags.tags.contains(&tag)
    }

    #[geam::function]
    fn size(tags: &TagSet) -> BigInt {
        BigInt::from(tags.tags.len())
    }
}
