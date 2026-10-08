use super::{DictOf, ExactDynamicDictOutput, create_dynamic_dict_with};
use crate::dynamic::{Dynamic, DynamicPayload};
use crate::{GleamStdlibProviderProfile, HostProvider, HostType};
use geam_core::provider::advanced::NativeMap;
use geam_core::provider::{
    ProviderConstruction, ProviderConstructionIndex0, ProviderConstructionIndexNext,
    ProviderConstructionList, ProviderConstructions, ProviderConvertedStorage,
    ProviderNoConstructions, ProviderOutputValue, ProviderValue, ProviderValueForms,
};
use geam_core::{HostCall, HostCallError};

pub struct DynamicDictOutput {
    value: Output,
}

enum Output {
    Exact(ExactDynamicDictOutput),
    Native(NativeMap),
}

impl DynamicDictOutput {
    pub(crate) fn exact(value: ExactDynamicDictOutput) -> Self {
        Self {
            value: Output::Exact(value),
        }
    }

    pub(crate) fn native(value: NativeMap) -> Self {
        Self {
            value: Output::Native(value),
        }
    }
}

impl ProviderValue for DynamicDictOutput {
    type Host = DictOf<Dynamic, Dynamic>;
    type OutputRequirements = ProviderConstructionList<
        ProviderConstruction<Self::Host>,
        ProviderConstructionList<ProviderConstruction<Dynamic>, ProviderNoConstructions>,
    >;
    type RootRequirements = Self::OutputRequirements;
}

impl ProviderValueForms for DynamicDictOutput {
    type InvocationRequirements = ();
    type ImmediateListDecoder = geam_core::provider::MissingListContext;
    type OwnedListDecoder = geam_core::provider::MissingListContext;
    type Runtime<Profile: geam_core::HostProfile> =
        geam_core::provider::ProviderStaticValueForms<Self>;
    type Output = Self;
    type ImmediateInput = Self;
    type ImmediateListInput = Self;
    type OwnedInput = Self;
    type OwnedListInput = Self;
}

impl<Profile, Provider, Return> ProviderOutputValue<Profile, Provider, Return> for DynamicDictOutput
where
    Profile: GleamStdlibProviderProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    type Error = HostCallError;
    type Storage = ProviderConvertedStorage;

    fn into_host<'call>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        constructions: &ProviderConstructions<'call, Self::OutputRequirements>,
    ) -> Result<<Self::Host as HostType>::Value<'call>, Self::Error> {
        let dict = constructions.select::<ProviderConstructionIndex0>();
        match self.value {
            Output::Exact(value) => value.into_host(call, &dict),
            Output::Native(value) => {
                let dynamic = constructions
                    .select::<ProviderConstructionIndexNext<ProviderConstructionIndex0>>();
                Ok(create_dynamic_dict_with(
                    call,
                    dict.token(),
                    value.entries(),
                    |call, entry| {
                        let key = DynamicPayload::from_native(entry.key)
                            .into_host_infallible(call, &dynamic);
                        let value = DynamicPayload::from_native(entry.value)
                            .into_host_infallible(call, &dynamic);
                        (key, value)
                    },
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DynamicDictOutput;
    use crate::dict::function::provider::DictValue;
    use crate::dict::storage::DictPayload;
    use crate::dict::{DictExternalStorage, DictOf, DictSchema, ExactDynamicDictOutput};
    use crate::dynamic::{Dynamic, DynamicExternalStorage, DynamicPayload, DynamicSchema};
    use crate::{GleamStdlibProfile, GleamStdlibRunState};
    use geam_core::host::{
        HostCall, HostCallCompletion, HostCallError, HostConstructions, HostExternal,
        HostExternalBinding, HostProvider, HostProviderModule, HostProviderSet, HostTypeList,
        HostTypeListEnd,
    };
    use geam_core::provider::{
        ProviderConstructions, ProviderInputValue, ProviderOutputValue, ProviderRootOutputValue,
        ProviderValueForms,
    };
    use geam_core::{
        HostedExecution, ModuleSource, PackageSource, Value, compile_typed_host_program,
        plan_host_program,
    };
    use std::sync::{Arc, Mutex};

    struct Consumer;
    impl HostProvider<GleamStdlibProfile> for Consumer {
        type State = GleamStdlibRunState;
        fn project(state: &mut GleamStdlibRunState) -> &mut Self::State {
            state
        }
    }
    impl HostExternalBinding<GleamStdlibProfile, DictSchema> for Consumer {
        type Storage = DictExternalStorage;
    }
    impl HostExternalBinding<GleamStdlibProfile, DynamicSchema> for Consumer {
        type Storage = DynamicExternalStorage;
    }

    type Dict = DictOf<Dynamic, Dynamic>;
    type Permissions = HostTypeList<Dict, HostTypeList<Dynamic, HostTypeListEnd>>;
    type Input = <ExactDynamicDictOutput as ProviderValueForms>::OwnedInput;

    type Call<'call> = HostCall<'call, GleamStdlibProfile, Consumer, Dict>;
    type Completion<'call> = Result<HostCallCompletion<'call, Dict>, HostCallError>;

    #[test]
    fn exact_dict_outputs_preserve_identity_and_reject_an_earlier_execution() {
        let previous = Arc::new(Mutex::new(None));
        let consumer = HostProviderModule::new("application", "main")
            .unwrap()
            .with_scoped_function_and_constructions::<Consumer, (Dict,), Dict, Permissions, _>(
                "keep",
                retaining_dict(previous),
            )
            .unwrap();
        let typed = compile_typed_host_program(
            "application", "main",
            [
                PackageSource::new("gleam_stdlib", Vec::<String>::new(), [
                    ModuleSource::new("gleam/dict", "dict.gleam", "pub type Dict(key, value)
type TransientDict(key, value)"),
                    ModuleSource::new("gleam/dynamic", "dynamic.gleam", "pub type Dynamic"),
                ]),
                PackageSource::new("application", ["gleam_stdlib"], [ModuleSource::new("main", "main.gleam", r#"
import gleam/dict
import gleam/dynamic
@external(erlang, "host", "empty") fn empty() -> dict.Dict(dynamic.Dynamic, dynamic.Dynamic)
@external(erlang, "host", "keep") fn keep(value: dict.Dict(dynamic.Dynamic, dynamic.Dynamic)) -> dict.Dict(dynamic.Dynamic, dynamic.Dynamic)
pub fn main() { let value = empty() value == keep(value) }
"#)]),
            ],
            HostProviderSet::from_providers([
                HostProviderModule::new("gleam_stdlib", "gleam/dict").unwrap().with_external_type::<Consumer, DictSchema>().unwrap(), HostProviderModule::new("gleam_stdlib", "gleam/dynamic").unwrap().with_external_type::<Consumer, DynamicSchema>().unwrap(),
                consumer.with_scoped_function::<Consumer, (), Dict, _>("empty", empty).unwrap(),
            ]).unwrap(),
        ).unwrap();
        let mut execution =
            HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
        let mut state = GleamStdlibRunState::from_seed([0; 32]);
        let state_pointer = &mut state as *mut GleamStdlibRunState;
        assert_eq!(
            Consumer::project(&mut state) as *mut GleamStdlibRunState,
            state_pointer
        );
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut state, &mut echo),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut state, &mut echo).map_err(|error| error.to_string()),
            Err("host function application::main.keep failed: retained value belongs to another execution".into()),
        );
        assert!(echo.is_empty());
    }

    fn empty<'call>(call: Call<'call>) -> Completion<'call> {
        let value = DictValue::<DynamicPayload, DynamicPayload>::from_payload(DictPayload {
            storage: Default::default(),
        });
        value.complete(call, &ProviderConstructions::none())
    }

    fn retaining_dict(
        previous: Arc<Mutex<Option<(ExactDynamicDictOutput, ExactDynamicDictOutput)>>>,
    ) -> impl for<'call> Fn(
        Call<'call>,
        HostConstructions<'call, Permissions>,
        HostExternal<'call, Dict>,
    ) -> Completion<'call> {
        move |mut call, permissions, original| {
            let first = Input::from_host(&mut call, original).into_value();
            let second = Input::from_host(&mut call, original).into_value();
            let previous = previous.lock().unwrap().replace((first, second));
            let constructions = ProviderConstructions::new(&permissions);
            match previous {
                None => {
                    let first = Input::from_host(&mut call, original).into_value();
                    let restored = DynamicDictOutput::exact(first)
                        .into_host(&mut call, &constructions)
                        .expect("exact current dict can be restored");
                    assert!(call.equal::<Dict>(original, restored));
                    Input::from_host(&mut call, original)
                        .into_value()
                        .complete(call, &ProviderConstructions::none())
                }
                Some((first, second)) => {
                    assert_eq!(
                        DynamicDictOutput::exact(first)
                            .into_host(&mut call, &constructions)
                            .err()
                            .map(|error| error.to_string()),
                        Some("retained value belongs to another execution".into())
                    );
                    second.complete(call, &ProviderConstructions::none())
                }
            }
        }
    }
}
