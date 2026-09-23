use crate::execution::{Destination, ReferenceId};
use crate::schema::{AtomSchema, MonitorSchema, ReferenceSchema};
use crate::{Component, GleamErlangHostProfile, Pid, PidSchema, Reference, reference};
use geam_core::execution::ExecutionUnit;
use geam_core::host::native::NativeRules;
use geam_core::host::{
    HostCall, HostConstruction, HostCustom, HostExternal, HostProvider, HostType, HostTypeList,
    HostTypeListEnd,
};
use geam_core::provider::advanced::NativeValue;
use geam_stdlib::provider_support::DynamicSchema;

/// Decodes a source Charlist through its original retained list representation.
pub fn charlist_string<'call, Profile, Provider, Return>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    value: HostExternal<'call, crate::Charlist>,
) -> geam_core::StringValue
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    let characters = call
        .external_payload_with::<Component<Profile>, crate::CharlistSchema, HostTypeListEnd>(value)
        .restore(call, |characters| characters);
    let mut output = ecow::EcoString::new();
    let mut index = 0;
    while let Some(character) = call.list_item::<char>(characters, index) {
        output.push(character);
        index += 1;
    }
    output.into()
}

/// Adds the process producer's native conversions to a consumer registration.
/// Exact typed values keep their original source representation and binding.
pub fn native_rules<Profile, Provider, Return>(
    rules: NativeRules<Profile, Provider, Return>,
) -> NativeRules<Profile, Provider, Return>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    rules
        .external::<AtomSchema, HostTypeListEnd>(|call, construction, value| {
            Some(call.construct_external_with_binding::<Component<Profile>, AtomSchema, HostTypeListEnd>(construction, value))
        })
        .external::<ReferenceSchema, HostTypeListEnd>(|call, construction, value| {
            Some(call.construct_external_with_binding::<Component<Profile>, ReferenceSchema, HostTypeListEnd>(construction, reference::Payload::View(value)))
        })
        .external::<DynamicSchema, HostTypeListEnd>(|call, construction, value| {
            Some(call.construct_external_with_binding::<Component<Profile>, DynamicSchema, HostTypeListEnd>(construction, geam_stdlib::Dynamic::from_native(value)))
        })
        .external::<MonitorSchema, HostTypeListEnd>(|call, construction, value| {
            Some(call.construct_external_with_binding::<Component<Profile>, MonitorSchema, HostTypeListEnd>(construction, value))
        })
}

/// Constructs a fresh source Reference using the producer's original identity store.
pub fn new_reference<'call, Profile, Provider, Return>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    construction: HostConstruction<'call, Reference>,
) -> HostExternal<'call, Reference>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    call.construct_external_with_binding::<Component<Profile>, ReferenceSchema, HostTypeListEnd>(
        construction,
        reference::Payload::Identity(ReferenceId::new()),
    )
}

/// Constructs a source Pid for an existing logical unit without extending its lifetime.
pub fn pid_value<'call, Profile, Provider, Return>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    construction: HostConstruction<'call, Pid>,
    unit: ExecutionUnit,
) -> HostExternal<'call, Pid>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    call.construct_external_with_binding::<Component<Profile>, PidSchema, HostTypeListEnd>(
        construction,
        unit,
    )
}

pub(crate) fn fresh_name<Profile, Provider, Return>(
    call: &mut HostCall<'_, Profile, Provider, Return>,
    prefix: &str,
) -> Result<ecow::EcoString, geam_core::HostCallError>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    let name = format!("{prefix}${}", ReferenceId::new().number()).into();
    Ok(Profile::erlang_execution(call.execution_state()).intern(name)?)
}

pub(crate) fn subject_parts<'call, Profile, Provider, Return, Message>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    subject: HostCustom<'call, super::types::Subject<Message>>,
) -> (Destination, NativeValue)
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Message: HostType,
{
    use super::types::{NamedSubject, OrdinarySubject};
    if let Some((owner, (tag, ()))) = call.custom_fields::<OrdinarySubject<Message>>(subject) {
        let owner = call
            .external_payload_with::<Component<Profile>, PidSchema, HostTypeListEnd>(owner)
            .id();
        let tag = call
            .external_payload_with::<Component<Profile>, DynamicSchema, HostTypeListEnd>(tag)
            .native_value()
            .clone();
        (Destination::Pid(owner), tag)
    } else {
        let (name, ()) =
            call.provider_borrow_remaining_custom_fields::<NamedSubject<Message>>(subject);
        let destination = call.external_payload_with::<Component<Profile>, crate::NameSchema, HostTypeList<Message, HostTypeListEnd>>(name).clone();
        (
            Destination::Name(destination),
            call.native_value::<crate::Name<Message>>(name),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::new_reference;
    use crate::{Component, GleamErlangProfile, Reference, ReferenceSchema};
    use geam_core::host::{
        HostCall, HostCallCompletion, HostCallError, HostConstructions, HostProviderModule,
        HostTypeIndex0, HostTypeList, HostTypeListEnd,
    };
    use geam_core::{ModuleSource, PackageSource};

    #[test]
    fn fresh_references_are_distinct_producer_values_with_stable_identity() {
        let producer = HostProviderModule::new("gleam_erlang", "gleam/erlang/reference")
            .unwrap()
            .with_external_type::<Component<GleamErlangProfile>, ReferenceSchema>()
            .unwrap();
        let consumer = HostProviderModule::new("application", "main")
            .unwrap()
            .with_scoped_function_and_constructions::<Component<GleamErlangProfile>, (), bool, HostTypeList<Reference, HostTypeListEnd>, _>("check", check)
            .unwrap();
        let result = crate::test_support::run_main(
            [
                PackageSource::new(
                    "gleam_erlang",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "gleam/erlang/reference",
                        "reference.gleam",
                        "pub type Reference",
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["gleam_erlang"],
                    [ModuleSource::new(
                        "main",
                        "main.gleam",
                        "@external(erlang, \"host\", \"check\") fn check() -> Bool\npub fn main() { check() }",
                    )],
                ),
            ],
            [producer, consumer],
        );
        assert_eq!(result, geam_core::Value::Bool(true));
    }

    fn check<'call>(
        mut call: HostCall<'call, GleamErlangProfile, Component<GleamErlangProfile>, bool>,
        constructions: HostConstructions<'call, HostTypeList<Reference, HostTypeListEnd>>,
    ) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
        let first = new_reference(&mut call, constructions.at::<HostTypeIndex0>());
        let second = new_reference(&mut call, constructions.at::<HostTypeIndex0>());
        let alias = first;
        assert!(call.equal::<Reference>(first, alias));
        assert_eq!(
            call.source_hash::<Reference>(first),
            call.source_hash::<Reference>(alias)
        );
        let result = !call.equal::<Reference>(first, second);
        Ok(call.return_value(result))
    }
}
