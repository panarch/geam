use crate::{HostExternalSchema, HostExternalType, HostListType, HostTypeParameter};

pub(in crate::gleam_stdlib) struct DynamicSchema;

pub(in crate::gleam_stdlib) type Dynamic = HostExternalType<DynamicSchema>;
pub(in crate::gleam_stdlib) type DynamicList = HostListType<Dynamic>;
pub(super) type Parameter = HostTypeParameter<0>;

impl HostExternalSchema for DynamicSchema {
    const PACKAGE: &'static str = "gleam_stdlib";
    const MODULE: &'static str = "gleam/dynamic";
    const NAME: &'static str = "Dynamic";
    const PARAMETER_COUNT: usize = 0;
}

#[cfg(test)]
mod tests {
    use super::DynamicSchema;
    use crate::HostExternalTypeSchema;

    #[test]
    fn describes_the_exact_official_dynamic_schema() {
        assert_eq!(
            HostExternalTypeSchema::of::<DynamicSchema>(),
            HostExternalTypeSchema::new("gleam_stdlib", "gleam/dynamic", "Dynamic", 0),
        );
    }
}
