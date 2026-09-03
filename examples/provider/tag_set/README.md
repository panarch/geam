# Host Provider: Tag Set

This is the first example where a Gleam value carries data owned by Rust. Gleam
sees a constructorless `TagSet`; the provider stores its `BTreeSet` payload and
returns a new persistent value from each update. No process state or runtime
configuration is involved.

## Read The Example

1. [`project/packages/example_tag_set/src/example_tag_set.gleam`](project/packages/example_tag_set/src/example_tag_set.gleam)
   declares the constructorless type and its public operations.
2. [`provider/src/lib.rs`](provider/src/lib.rs) defines the Rust payload and
   matching functions.
3. [`project/src/tag_set_example.gleam`](project/src/tag_set_example.gleam)
   checks persistence, membership, size, and source equality.

The complete Gleam API is:

```gleam
pub type TagSet

pub fn new() -> TagSet
pub fn insert(tags: TagSet, tag: String) -> TagSet
pub fn contains(tags: TagSet, tag: String) -> Bool
pub fn size(tags: TagSet) -> Int
```

`#[geam::external]` generates source equality, hashing, and opaque
`TagSet(<opaque>)` inspection from the Rust value:

```rust
#[geam::provider(package = "example_tag_set", modules = [tag_set])]
pub struct Component;

#[geam::external(name = "TagSet")]
#[derive(Clone, Default, PartialEq, Eq, Hash)]
struct TagSet {
    tags: BTreeSet<EcoString>,
}
```

## Run

Run it through the generated standalone runner:

```sh
cd examples/provider/tag_set/project
geam provider add --path ../provider
geam prepare
geam run
```

The entrypoint checks that inserting `"rust"` leaves the empty value unchanged,
duplicate insertion keeps the size stable, membership works, and independently
constructed sets compare equal. A successful run is silent because all
assertions pass.

Continue with [request IDs](../request_ids/README.md) to move mutable data out
of source values and into state owned by one execution.
