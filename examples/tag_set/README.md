# Tag Set Provider Example

This is the smallest external-value provider. It has no process-local state and
accepts no configuration. `#[geam::external]` generates ordinary source
equality, hashing, and opaque `TagSet(<opaque>)` inspection from the Rust value.

The complete Gleam API is:

```gleam
pub type TagSet

pub fn new() -> TagSet
pub fn insert(tags: TagSet, tag: String) -> TagSet
pub fn contains(tags: TagSet, tag: String) -> Bool
pub fn size(tags: TagSet) -> Int
```

The matching Rust declarations use only the default provider and external
semantics:

```rust
#[geam::provider(package = "example_tag_set", modules = [tag_set])]
pub struct Component;

#[geam::external(name = "TagSet")]
#[derive(Clone, Default, PartialEq, Eq, Hash)]
struct TagSet {
    tags: BTreeSet<EcoString>,
}
```

Run it through the generated standalone runner:

```sh
cd examples/tag_set/project
geam provider add --path ../provider
geam prepare
geam run
```

A successful run produces no application output. The entrypoint executes every public
function and checks persistent old values, duplicate insertion, membership,
size, and equality between independently constructed values.

Read [`project/packages/example_tag_set/src/example_tag_set.gleam`](project/packages/example_tag_set/src/example_tag_set.gleam),
[`provider/src/lib.rs`](provider/src/lib.rs), and
[`project/src/tag_set_example.gleam`](project/src/tag_set_example.gleam) together.
