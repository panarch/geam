use ecow::EcoString;

#[geam_macros::provider(
    package = "lists",
    modules = [lists],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "lists", crate_path = geam_core)]
mod lists {
    use super::EcoString;

    #[geam_macros::external(name = "Tag")]
    #[derive(PartialEq, Eq, Hash)]
    struct Tag(EcoString);

    fn retain(_: &'static Tag) {}

    #[geam_macros::function]
    fn inspect(values: geam_core::List<Tag>) -> bool {
        let tag = values.get(0).unwrap();
        retain(&tag);
        true
    }
}

fn main() {}
