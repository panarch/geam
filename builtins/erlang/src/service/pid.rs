use crate::{Component, GleamErlangHostProfile, Pid as HostPid, PidSchema};
use geam_core::execution::ExecutionUnit;
use geam_core::host::{
    HostCall, HostCallCompletion, HostCallError, HostExternal, HostProvider, HostType,
    HostTypeListEnd,
};
use geam_core::provider::{
    ProviderConstruction, ProviderConstructions, ProviderConvertedStorage,
    ProviderExternalPayloadAccess, ProviderInputValue, ProviderListInputCodec,
    ProviderListInputValue, ProviderListItemDecoder, ProviderListItemValue,
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
    type Error = HostCallError;
    type Storage = ProviderConvertedStorage;

    fn into_host<'call>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        constructions: &ProviderConstructions<'call, Self::OutputRequirements>,
    ) -> Result<HostExternal<'call, HostPid>, Self::Error> {
        Ok(match self.value {
            Representation::Source(value) => call
                .provider_external_from_item::<PidSchema, HostTypeListEnd, ExecutionUnit>(value)?,
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
        let value = self.into_host(&mut call, constructions)?;
        Ok(call.return_value(value))
    }
}

#[cfg(test)]
mod tests {
    use super::Pid;
    use crate::{Component, GleamErlangProfile, Pid as HostPid, PidSchema};
    use geam_core::host::HostProviderSet;
    use geam_core::host::{
        HostCall, HostCallCompletion, HostCallError, HostConstructions, HostExternal, HostList,
        HostListType, HostProviderModule, HostTypeList, HostTypeListEnd,
    };
    use geam_core::provider::{
        ProviderConstructions, ProviderInputValue, ProviderListInputCodec, ProviderOutputValue,
        ProviderRootOutputValue,
    };
    use geam_core::{
        HostedExecution, ModuleSource, PackageSource, compile_typed_host_program, plan_host_program,
    };
    use std::sync::{Arc, Mutex};

    type Call<'call> = HostCall<'call, GleamErlangProfile, Component<GleamErlangProfile>, HostPid>;
    type Constructions<'call> = HostConstructions<'call, HostTypeList<HostPid, HostTypeListEnd>>;

    type Source<'call> = HostExternal<'call, HostPid>;
    type Items<'call> = HostList<'call, HostPid>;
    type Completion<'call> = Result<HostCallCompletion<'call, HostPid>, HostCallError>;

    #[test]
    fn retained_values_preserve_identity_and_reject_a_previous_execution() {
        let producer = HostProviderModule::new("gleam_erlang", "gleam/erlang/process")
            .unwrap()
            .with_external_type::<Component<GleamErlangProfile>, PidSchema>()
            .unwrap();
        let previous = Arc::new(Mutex::new(None::<Pid>));
        let consumer = HostProviderModule::new("application", "main")
            .unwrap()
            .with_scoped_function_and_constructions::<Component<GleamErlangProfile>, (), HostPid, HostTypeList<HostPid, HostTypeListEnd>, _>("make", make)
            .unwrap()
            .with_scoped_function_and_constructions::<Component<GleamErlangProfile>, (HostPid, HostListType<HostPid>), HostPid, HostTypeList<HostPid, HostTypeListEnd>, _>("round_trip", retaining_round_trip(previous))
            .unwrap();
        let typed = compile_typed_host_program(
            "application",
            "main",
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
            HostProviderSet::from_providers([producer, consumer]).unwrap(),
        )
        .unwrap();
        let mut execution =
            HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
        let mut state = crate::GleamErlangRunState {
            stdlib: geam_stdlib::GleamStdlibRunState::from_seed([0; 32]),
            erlang: crate::Configuration::default(),
        };
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut state, &mut echo),
            Ok(geam_core::Value::Bool(true))
        );
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut state, &mut echo).map_err(|error| error.to_string()),
            Err("host function application::main.round_trip failed: retained value belongs to another execution".into()),
        );
        assert!(echo.is_empty());
    }

    fn retaining_round_trip(
        previous: Arc<Mutex<Option<Pid>>>,
    ) -> impl for<'call> Fn(
        Call<'call>,
        Constructions<'call>,
        Source<'call>,
        Items<'call>,
    ) -> Completion<'call> {
        move |mut call, constructions, original, items| {
            let retained = <Pid>::from_host(&mut call, original);
            let previous = previous.lock().unwrap().replace(retained);
            match previous {
                None => round_trip(call, constructions, original, items),
                Some(previous) => {
                    let constructions = ProviderConstructions::new(&constructions);
                    assert_eq!(
                        previous
                            .clone()
                            .into_host(&mut call, &constructions)
                            .err()
                            .map(|error| error.to_string()),
                        Some("retained value belongs to another execution".into()),
                    );
                    previous.complete(call, &constructions)
                }
            }
        }
    }

    fn make<'call>(call: Call<'call>, constructions: Constructions<'call>) -> Completion<'call> {
        let value = Pid::new(call.execution_unit().unwrap());
        value
            .clone()
            .complete(call, &ProviderConstructions::new(&constructions))
    }

    fn round_trip<'call>(
        mut call: Call<'call>,
        constructions: Constructions<'call>,
        original: Source<'call>,
        items: Items<'call>,
    ) -> Completion<'call> {
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
            let restored = item
                .into_host(&mut call, &constructions)
                .expect("current execution retains the exact producer value");
            assert!(call.equal::<HostPid>(original, restored));
            assert_eq!(call.source_hash::<HostPid>(restored), expected_hash);
            assert_eq!(call.inspect::<HostPid>(restored), expected_inspection);
        }
        <Pid>::from_host(&mut call, original).complete(call, &constructions)
    }
}
