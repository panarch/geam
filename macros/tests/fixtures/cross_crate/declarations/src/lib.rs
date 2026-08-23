#[geam_macros::provider(
    package = "macro_declarations",
    modules = [values],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "macro_declarations/values", crate_path = geam_core)]
pub mod values {
    use ecow::EcoString;
    use num_bigint::BigInt;

    #[geam_macros::external(name = "Token")]
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Token(pub EcoString);

    #[geam_macros::custom(input = StatusInput)]
    pub enum Status {
        Ready,
        Count(BigInt),
        Tagged(Token),
    }
}
