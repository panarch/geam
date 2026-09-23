use crate::{Component, GleamErlangHostProfile, Pid as HostPid, PidSchema};
use geam_core::execution::ExecutionUnit;
use geam_core::host::{
    HostCall, HostCallCompletion, HostCallError, HostExternal, HostProvider, HostType,
    HostTypeListEnd,
};
use geam_core::provider::{
    ProviderConstruction, ProviderConstructions, ProviderExternalPayloadAccess, ProviderInputValue,
    ProviderListInputCodec, ProviderListInputValue, ProviderListItemDecoder, ProviderListItemValue,
    ProviderNoConstructions, ProviderOutputValue, ProviderOwnedExternal, ProviderRootOutputValue,
    ProviderStaticValueForms, ProviderTypedListItemDecoder, ProviderValue, ProviderValueForms,
};

/// A process identity for provider authoring, with the original source Pid binding.
///
/// Incoming values retain their source payload without copying it. A new Pid
/// wrapper is constructed only when a service operation first returns an identity.
#[derive(Clone)]
pub struct Pid {
    value: Representation,
}

#[derive(Clone)]
enum Representation {
    Source(ProviderOwnedExternal<ExecutionUnit>),
    Unit(ExecutionUnit),
}

impl Pid {
    pub(super) fn new(unit: ExecutionUnit) -> Self {
        Self {
            value: Representation::Unit(unit),
        }
    }

    /// Returns the non-owning logical identity used by domain service operations.
    pub fn execution_unit(&self) -> ExecutionUnit {
        match &self.value {
            Representation::Source(source) => source.with(ExecutionUnit::clone),
            Representation::Unit(unit) => unit.clone(),
        }
    }
}

impl ProviderValue for Pid {
    type Host = HostPid;
    type OutputRequirements = ProviderConstruction<HostPid>;
    type RootRequirements = Self::OutputRequirements;
}

impl ProviderValueForms for Pid {
    type InvocationRequirements = ();
    type Runtime<Profile: geam_core::HostProfile> = ProviderStaticValueForms<Self>;
    type Output = Self;
    type ImmediateInput = Self;
    type OwnedInput = Self;
    type ImmediateListInput = Self;
    type OwnedListInput = Self;
    type ImmediateListDecoder = PidListDecoder;
    type OwnedListDecoder = PidListDecoder;
}

#[doc(hidden)]
#[derive(Clone)]
pub struct PidListDecoder(ProviderExternalPayloadAccess<ExecutionUnit>);

impl ProviderListItemDecoder<Pid> for PidListDecoder {
    type View = Pid;
    fn decode(&self, value: ProviderListItemValue<'_>) -> Pid {
        Pid {
            value: Representation::Source(value.into_external(&self.0)),
        }
    }
}

impl ProviderTypedListItemDecoder<Pid> for PidListDecoder {
    type Host = HostPid;
}
impl ProviderListInputValue for Pid {
    type Host = HostPid;
    type View = Self;
    type Decoder = PidListDecoder;
}
impl<Profile: GleamErlangHostProfile, Provider: HostProvider<Profile>>
    ProviderListInputCodec<Profile, Provider> for Pid
{
    type Requirements = ProviderNoConstructions;
    fn decoder_with<'call, Return: HostType>(
        call: &HostCall<'call, Profile, Provider, Return>,
        _: &ProviderConstructions<'call, Self::Requirements>,
    ) -> PidListDecoder {
        PidListDecoder(
            call.provider_external_payload_access_with::<Component<Profile>, PidSchema>(),
        )
    }
}

impl<Profile, Provider, Return> ProviderInputValue<Profile, Provider, Return> for Pid
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    type Host = HostPid;
    type Requirements = ProviderNoConstructions;

    fn from_host_with<'call>(
        call: &mut HostCall<'call, Profile, Provider, Return>,
        value: HostExternal<'call, HostPid>,
        _: &ProviderConstructions<'call, Self::Requirements>,
    ) -> Self {
        Self {
            value: Representation::Source(
                call.provider_external_item_with::<Component<Profile>, PidSchema, HostTypeListEnd>(
                    value,
                ),
            ),
        }
    }
}

impl<Profile, Provider, Return> ProviderOutputValue<Profile, Provider, Return> for Pid
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    type Error = std::convert::Infallible;

    fn into_host<'call>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        constructions: &ProviderConstructions<'call, Self::OutputRequirements>,
    ) -> Result<HostExternal<'call, HostPid>, Self::Error> {
        Ok(match self.value {
            Representation::Source(value) => {
                call.provider_external_from_item::<PidSchema, HostTypeListEnd, ExecutionUnit>(value)
            }
            Representation::Unit(unit) => call
                .construct_external_with_binding::<Component<Profile>, PidSchema, HostTypeListEnd>(
                    constructions.token(),
                    unit,
                ),
        })
    }
}

impl<Profile, Provider> ProviderRootOutputValue<Profile, Provider> for Pid
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
{
    fn complete<'call>(
        self,
        mut call: HostCall<'call, Profile, Provider, HostPid>,
        constructions: &ProviderConstructions<'call, Self::RootRequirements>,
    ) -> Result<HostCallCompletion<'call, HostPid>, HostCallError> {
        let value = self.into_host_infallible(&mut call, constructions);
        Ok(call.return_value(value))
    }
}

#[cfg(test)]
mod tests {
    use super::Pid;
    use crate::{Component, GleamErlangProfile, Pid as HostPid, PidSchema};
    use geam_core::host::{
        HostCall, HostCallCompletion, HostCallError, HostConstructions, HostExternal, HostList,
        HostListType, HostProviderModule, HostTypeList, HostTypeListEnd,
    };
    use geam_core::provider::{
        ProviderConstructions, ProviderInputValue, ProviderListInputCodec, ProviderOutputValue,
        ProviderRootOutputValue,
    };
    use geam_core::{ModuleSource, PackageSource};

    type Call<'call> = HostCall<'call, GleamErlangProfile, Component<GleamErlangProfile>, HostPid>;
    type Constructions<'call> = HostConstructions<'call, HostTypeList<HostPid, HostTypeListEnd>>;

    #[test]
    fn retained_values_and_list_decoders_preserve_the_producer_identity() {
        let producer = HostProviderModule::new("gleam_erlang", "gleam/erlang/process")
            .unwrap()
            .with_external_type::<Component<GleamErlangProfile>, PidSchema>()
            .unwrap();
        let consumer = HostProviderModule::new("application", "main")
            .unwrap()
            .with_scoped_function_and_constructions::<Component<GleamErlangProfile>, (), HostPid, HostTypeList<HostPid, HostTypeListEnd>, _>("make", make)
            .unwrap()
            .with_scoped_function_and_constructions::<Component<GleamErlangProfile>, (HostPid, HostListType<HostPid>), HostPid, HostTypeList<HostPid, HostTypeListEnd>, _>("round_trip", round_trip)
            .unwrap();
        let result = crate::test_support::run_main(
            [
                PackageSource::new(
                    "gleam_erlang",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "gleam/erlang/process",
                        "producer.gleam",
                        "pub type Pid",
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["gleam_erlang"],
                    [ModuleSource::new(
                        "main",
                        "main.gleam",
                        r#"
import gleam/erlang/process as p
@external(erlang, "host", "make") fn make() -> p.Pid
@external(erlang, "host", "round_trip") fn round_trip(value: p.Pid, items: List(p.Pid)) -> p.Pid
pub fn main() {
  let value = make()
  value == round_trip(value, [value, value])
}
"#,
                    )],
                ),
            ],
            [producer, consumer],
        );
        assert_eq!(result, geam_core::Value::Bool(true));
    }

    fn make<'call>(
        call: Call<'call>,
        constructions: Constructions<'call>,
    ) -> Result<HostCallCompletion<'call, HostPid>, HostCallError> {
        let value = Pid::new(call.execution_unit().unwrap());
        value
            .clone()
            .complete(call, &ProviderConstructions::new(&constructions))
    }

    fn round_trip<'call>(
        mut call: Call<'call>,
        constructions: Constructions<'call>,
        original: HostExternal<'call, HostPid>,
        items: HostList<'call, HostPid>,
    ) -> Result<HostCallCompletion<'call, HostPid>, HostCallError> {
        let expected_hash = call.source_hash::<HostPid>(original);
        let expected_inspection = call.inspect::<HostPid>(original);
        let retained = <Pid>::from_host(&mut call, original);
        let alias = retained.clone();
        drop(retained);
        assert_eq!(
            alias.execution_unit().id(),
            call.execution_unit().unwrap().id()
        );
        let decoder = <Pid as ProviderListInputCodec<
            GleamErlangProfile,
            Component<GleamErlangProfile>,
        >>::decoder_with(&call, &ProviderConstructions::none());
        let alias_decoder = decoder.clone();
        drop(decoder);
        let items = call.provider_retained_list::<Pid, _, _>(items, alias_decoder);
        assert_eq!(items.len(), 2);
        assert!(items.get(2).is_none());
        let first = items.get(0).unwrap();
        let second = items.get(1).unwrap();
        drop(items);
        let constructions = ProviderConstructions::new(&constructions);
        for item in [first, second, alias] {
            let restored = item.into_host_infallible(&mut call, &constructions);
            assert!(call.equal::<HostPid>(original, restored));
            assert_eq!(call.source_hash::<HostPid>(restored), expected_hash);
            assert_eq!(call.inspect::<HostPid>(restored), expected_inspection);
        }
        <Pid>::from_host(&mut call, original).complete(call, &constructions)
    }
}
