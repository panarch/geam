use super::EmbeddingOutput;
use crate::embedding::CallError;
use crate::host::HostProfile;
use crate::plan::execution::function::{
    BitArrayFunctionId, BoolFunctionId, CustomFunctionId, ExternalFunctionId, FloatFunctionId,
    IntFunctionId, LibraryListFunctionId, NilFunctionId, StringFunctionId, TupleFunctionId,
    UtfCodepointFunctionId,
};
use crate::runtime::execution::EntryContext;
use crate::runtime::state::list::ListValueId;
use crate::runtime::{HostCallOrigin, RetainedInputs};
use std::future::Future;

pub(crate) trait EmbeddingEntry: Send + Copy {
    type Output: Send;

    fn call<Profile: HostProfile>(
        self,
        context: &EntryContext<Profile>,
        inputs: RetainedInputs,
    ) -> impl Future<Output = Result<Self::Output, CallError>> + Send;
}

macro_rules! scalar {
    ($entry:ty, $output:ty, $map:expr) => {
        impl EmbeddingEntry for $entry {
            type Output = $output;
            async fn call<Profile: HostProfile>(
                self,
                context: &EntryContext<Profile>,
                inputs: RetainedInputs,
            ) -> Result<Self::Output, CallError> {
                context
                    .call(self, HostCallOrigin::Entry, inputs.into_retained())
                    .await
                    .map_err(|_| CallError::Cancelled)?
                    .map($map)
                    .map_err(CallError::Execution)
            }
        }
    };
}

scalar!(IntFunctionId, num_bigint::BigInt, Into::into);
scalar!(FloatFunctionId, f64, std::convert::identity);
scalar!(StringFunctionId, crate::StringValue, std::convert::identity);
scalar!(
    BitArrayFunctionId,
    crate::BitArrayValue,
    crate::runtime::EvaluatedBitArray::into_value
);
scalar!(UtfCodepointFunctionId, char, std::convert::identity);
scalar!(BoolFunctionId, bool, std::convert::identity);
scalar!(NilFunctionId, (), std::convert::identity);
scalar!(
    TupleFunctionId,
    EmbeddingOutput,
    EmbeddingOutput::from_tuple
);
scalar!(
    CustomFunctionId,
    EmbeddingOutput,
    EmbeddingOutput::from_custom
);
scalar!(
    ExternalFunctionId,
    crate::runtime::EvaluatedExternalValue,
    std::convert::identity
);

impl EmbeddingEntry for LibraryListFunctionId {
    type Output = EmbeddingOutput;

    async fn call<Profile: HostProfile>(
        self,
        context: &EntryContext<Profile>,
        inputs: RetainedInputs,
    ) -> Result<Self::Output, CallError> {
        macro_rules! call {
            ($id:expr, $family:ident) => {
                context
                    .call($id, HostCallOrigin::Entry, inputs.into_retained())
                    .await
                    .map_err(|_| CallError::Cancelled)?
                    .map(|value| EmbeddingOutput::from_value(ListValueId::$family(value).into()))
                    .map_err(CallError::Execution)
            };
        }
        match self {
            Self::Int(id) => call!(id, Int),
            Self::Float(id) => call!(id, Float),
            Self::String(id) => call!(id, String),
            Self::BitArray(id) => call!(id, BitArray),
            Self::UtfCodepoint(id) => call!(id, UtfCodepoint),
            Self::Bool(id) => call!(id, Bool),
            Self::Nil(id) => call!(id, Nil),
            Self::Custom(id) => call!(id, Custom),
            Self::External(id) => call!(id, External),
            Self::Tuple(id) => call!(id, Tuple),
            Self::List(id) => call!(id, List),
            Self::Function(id) => call!(id, Function),
        }
    }
}

impl crate::plan::execution::LibraryCallableEntry {
    pub(crate) async fn call<Profile: HostProfile>(
        &self,
        context: &EntryContext<Profile>,
        inputs: RetainedInputs,
    ) -> Result<EmbeddingOutput, CallError> {
        use crate::plan::execution::function::{
            ProfiledFunctionFunctionId, RuntimeFunctionFunctionTarget,
        };
        macro_rules! call {
            ($id:expr) => {
                context
                    .call($id, HostCallOrigin::Entry, inputs.into_retained())
                    .await
                    .map_err(|_| CallError::Cancelled)?
                    .map(|value| {
                        EmbeddingOutput::from_value(crate::runtime::EvaluatedValue::Function(
                            value.into(),
                        ))
                    })
                    .map_err(CallError::Execution)
            };
        }
        match &self.function {
            RuntimeFunctionFunctionTarget::Core(id) => match id {
                ProfiledFunctionFunctionId::External(id) => match *id {},
                ProfiledFunctionFunctionId::Generic(id) => match *id {},
                ProfiledFunctionFunctionId::Never(id) => call!(id.clone()),
                ProfiledFunctionFunctionId::Int(id) => call!(*id),
                ProfiledFunctionFunctionId::Float(id) => call!(*id),
                ProfiledFunctionFunctionId::String(id) => call!(*id),
                ProfiledFunctionFunctionId::BitArray(id) => call!(*id),
                ProfiledFunctionFunctionId::UtfCodepoint(id) => call!(*id),
                ProfiledFunctionFunctionId::Parameter(id) => match *id {},
                ProfiledFunctionFunctionId::Custom(id) => call!(id.clone()),
                ProfiledFunctionFunctionId::Bool(id) => call!(*id),
                ProfiledFunctionFunctionId::Nil(id) => call!(*id),
                ProfiledFunctionFunctionId::Tuple(id) => call!(*id),
                ProfiledFunctionFunctionId::List(id) => call!(id.value_target()),
                ProfiledFunctionFunctionId::Function(id) => call!(id.clone()),
            },
            RuntimeFunctionFunctionTarget::External(id) => match id {
                crate::plan::execution::graph::ExternalFunctionCallTarget::Function(id) => {
                    call!(id.clone())
                }
                crate::plan::execution::graph::ExternalFunctionCallTarget::ListFunction {
                    id,
                    ..
                } => call!(*id),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::embedding::{
        BigInt, BitArrayValue, CallableType, FunctionDeclaration, HostedModuleBuilder, List,
        StringValue,
    };
    use crate::{HostProviderSet, ModuleSource, PackageSource, StatelessHostProfile};

    struct Profile;
    impl crate::HostProfile for Profile {
        type RunState = ();
        type ExternalStores = crate::host::HostFutureStore;
        type ExecutionState = ();
    }
    impl crate::host::HostWorkProfile for Profile {
        type Work = crate::work_fixture::WorkComponent;
    }
    impl crate::HostComponentProfile<crate::work_fixture::WorkComponent> for Profile {
        fn component_stores(
            stores: &crate::host::HostFutureStore,
        ) -> &crate::host::HostFutureStore {
            stores
        }
        fn component_state(state: &mut ()) -> &mut () {
            state
        }
    }

    #[test]
    fn named_function_entries_preserve_each_returned_callable_family() {
        use crate::embedding::{CustomType, NamedTypeSchema};
        use crate::work_fixture::{WorkComponent, WorkType};
        struct Empty;
        impl NamedTypeSchema for Empty {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "library";
            const NAME: &'static str = "Empty";
        }
        let mut state = ();
        assert_eq!(
            <Profile as crate::HostComponentProfile<WorkComponent>>::component_state(&mut state)
                as *mut (),
            &mut state as *mut (),
        );
        let program = crate::compile_typed_host_program(
            "application",
            "library",
            [
                PackageSource::new(
                    "work_fixture",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "fixture/work",
                        "work.gleam",
                        WorkComponent::SOURCE,
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["work_fixture"],
                    [ModuleSource::new(
                        "library",
                        "src/library.gleam",
                        r#"
import fixture/work as future
pub type Empty { Again(Empty) }
pub fn never() -> fn() -> Empty { fn() { panic as "unused" } }
pub fn works() { fn(value: Int) { future.ready(value) } }
pub fn work_lists() { fn(value: Int) { [future.ready(value), future.ready(value + 1)] } }
pub fn ints() { fn(value: Int) { value + 2 } }
pub fn floats() { fn(value: Float) { value +. 0.5 } }
pub fn strings() { fn(value: String) { value <> "!" } }
pub fn bits() { fn(value: BitArray) { value } }
pub fn codepoints() { fn(value: UtfCodepoint) { value } }
pub fn bools() { fn(value: Bool) { !value } }
pub fn nils() { fn(value: Nil) { value } }
pub fn tuples() { fn(value: #(Int, String)) { value } }
pub fn choices() { fn(value: Result(Int, Bool)) { value } }
pub fn lists() { fn(value: List(Int)) { value } }
pub fn functions() { fn() { fn(value: Int) { value + 3 } } }
pub fn float_lists() { fn(value: Float) { [value] } }
pub fn string_lists() { fn(value: String) { [value] } }
pub fn bit_lists() { fn(value: BitArray) { [value] } }
pub fn codepoint_lists() { fn(value: UtfCodepoint) { [value] } }
pub fn bool_lists() { fn(value: Bool) { [value] } }
pub fn nil_lists() { fn(value: Nil) { [value] } }
pub fn tuple_lists() { fn(value: #(Int, String)) { [value] } }
pub fn choice_lists() { fn(value: Result(Int, Bool)) { [value] } }
pub fn nested_lists() { fn(value: List(Int)) { [value] } }
pub fn zero_args() { fn() { 42 } }
pub fn function_lists() { fn(value: fn() -> Int) { [value] } }
"#,
                    )],
                ),
            ],
            HostProviderSet::from_providers(WorkComponent::providers::<Profile>().unwrap())
                .unwrap(),
        )
        .unwrap();
        let (mut bindings, ints) = HostedModuleBuilder::new(program)
            .unwrap()
            .function(FunctionDeclaration::<(), CallableType<(BigInt,), BigInt>>::new("ints"))
            .unwrap();
        macro_rules! bind {
            ($name:ident, $type:ty) => {
                let $name = bindings
                    .function(
                        FunctionDeclaration::<(), CallableType<($type,), $type>>::new(stringify!(
                            $name
                        )),
                    )
                    .unwrap();
            };
        }
        bind!(floats, f64);
        bind!(strings, StringValue);
        bind!(bits, BitArrayValue);
        bind!(codepoints, char);
        bind!(bools, bool);
        bind!(nils, ());
        bind!(tuples, (BigInt, StringValue));
        bind!(choices, Result<BigInt, bool>);
        bind!(lists, List<BigInt>);
        macro_rules! bind_list {
            ($name:ident, $type:ty) => {
                let $name = bindings
                    .function(
                        FunctionDeclaration::<(), CallableType<($type,), List<$type>>>::new(
                            stringify!($name),
                        ),
                    )
                    .unwrap();
            };
        }
        bind_list!(float_lists, f64);
        bind_list!(string_lists, StringValue);
        bind_list!(bit_lists, BitArrayValue);
        bind_list!(codepoint_lists, char);
        bind_list!(bool_lists, bool);
        bind_list!(nil_lists, ());
        bind_list!(tuple_lists, (BigInt, StringValue));
        bind_list!(choice_lists, Result<BigInt, bool>);
        bind_list!(nested_lists, List<BigInt>);
        bind_list!(function_lists, CallableType<(), BigInt>);
        let zero_args = bindings
            .function(FunctionDeclaration::<(), CallableType<(), BigInt>>::new(
                "zero_args",
            ))
            .unwrap();
        let functions = bindings
            .function(FunctionDeclaration::<
                (),
                CallableType<(), CallableType<(BigInt,), BigInt>>,
            >::new("functions"))
            .unwrap();
        let never = bindings
            .function(FunctionDeclaration::<(), CallableType<(), CustomType<Empty>>>::new("never"))
            .unwrap();
        let works = bindings
            .function(FunctionDeclaration::<
                (),
                CallableType<(BigInt,), WorkType<BigInt>>,
            >::new("works"))
            .unwrap();
        let work_lists = bindings
            .function(FunctionDeclaration::<
                (),
                CallableType<(BigInt,), List<WorkType<BigInt>>>,
            >::new("work_lists"))
            .unwrap();
        let mut module = bindings.seal().unwrap();
        let host = crate::execution_fixture::TestHost::default();
        let mut echo = Vec::new();
        host.block_on(
            module.with_execution(&host, &mut (), &mut echo, async |scope| {
                drop(scope.call(&never, ()).await.unwrap());
                let work_function = scope.call(&works, ()).await.unwrap();
                let work = scope
                    .invoke(&work_function, (BigInt::from(42),))
                    .await
                    .unwrap();
                assert_eq!(
                    scope.observe(&work).await.unwrap().read(Clone::clone),
                    BigInt::from(42)
                );
                let list_function = scope.call(&work_lists, ()).await.unwrap();
                let works = scope
                    .invoke(&list_function, (BigInt::from(42),))
                    .await
                    .unwrap();
                let second = works.read_item(1, std::convert::identity).unwrap();
                drop(works);
                assert_eq!(
                    scope.observe(&second).await.unwrap().read(Clone::clone),
                    BigInt::from(43)
                );
                macro_rules! invoke {
                    ($entry:ident, $input:expr, $expected:expr) => {
                        let callback = scope.call(&$entry, ()).await.unwrap();
                        assert_eq!(scope.invoke(&callback, ($input,)).await.unwrap(), $expected);
                    };
                }
                invoke!(ints, BigInt::from(40), BigInt::from(42));
                invoke!(floats, 3.0, 3.5);
                invoke!(
                    strings,
                    StringValue::from("hello"),
                    StringValue::from("hello!")
                );
                invoke!(
                    bits,
                    BitArrayValue::from_bytes(vec![0x80, 0x42]),
                    BitArrayValue::from_bytes(vec![0x80, 0x42])
                );
                invoke!(codepoints, '한', '한');
                invoke!(bools, false, true);
                invoke!(nils, (), ());
                invoke!(
                    tuples,
                    (BigInt::from(42), StringValue::from("tuple")),
                    (BigInt::from(42), StringValue::from("tuple"))
                );
                invoke!(
                    choices,
                    Ok::<BigInt, bool>(BigInt::from(42)),
                    Ok(BigInt::from(42))
                );
                invoke!(choices, Err::<BigInt, bool>(false), Err(false));
                let callback = scope.call(&lists, ()).await.unwrap();
                let values = scope
                    .invoke(&callback, (vec![BigInt::from(40), BigInt::from(42)],))
                    .await
                    .unwrap();
                assert_eq!(values.len(), 2);
                assert_eq!(values.read_item(0, Clone::clone), Some(BigInt::from(40)));
                assert_eq!(values.read_item(1, Clone::clone), Some(BigInt::from(42)));
                macro_rules! invoke_list {
                    ($entry:ident, $input:expr, $read:expr, $expected:expr) => {
                        let callback = scope.call(&$entry, ()).await.unwrap();
                        let values = scope.invoke(&callback, ($input,)).await.unwrap();
                        assert_eq!(values.len(), 1);
                        assert_eq!(values.read_item(0, $read), Some($expected));
                    };
                }
                invoke_list!(float_lists, 3.5, std::convert::identity, 3.5);
                invoke_list!(
                    string_lists,
                    StringValue::from("list"),
                    Clone::clone,
                    StringValue::from("list")
                );
                invoke_list!(
                    bit_lists,
                    BitArrayValue::from_bytes(vec![0x42]),
                    Clone::clone,
                    BitArrayValue::from_bytes(vec![0x42])
                );
                invoke_list!(codepoint_lists, '한', std::convert::identity, '한');
                invoke_list!(bool_lists, true, std::convert::identity, true);
                invoke_list!(nil_lists, (), std::convert::identity, ());
                invoke_list!(
                    tuple_lists,
                    (BigInt::from(42), StringValue::from("tuple")),
                    |(int, string)| (int.clone(), string.clone()),
                    (BigInt::from(42), StringValue::from("tuple"))
                );
                invoke_list!(
                    choice_lists,
                    Err::<BigInt, bool>(false),
                    |value| value.cloned(),
                    Err::<BigInt, bool>(false)
                );
                invoke_list!(
                    nested_lists,
                    vec![BigInt::from(42)],
                    |list| list.read_item(0, Clone::clone),
                    Some(BigInt::from(42))
                );
                let factory = scope.call(&functions, ()).await.unwrap();
                let callback = scope.invoke(&factory, ()).await.unwrap();
                assert_eq!(
                    scope.invoke(&callback, (BigInt::from(39),)).await.unwrap(),
                    BigInt::from(42)
                );
                let list_factory = scope.call(&function_lists, ()).await.unwrap();
                let zero_args = scope.call(&zero_args, ()).await.unwrap();
                let functions = scope.invoke(&list_factory, (zero_args,)).await.unwrap();
                let retained = functions.read_item(0, std::convert::identity).unwrap();
                drop(functions);
                assert_eq!(scope.invoke(&retained, ()).await.unwrap(), BigInt::from(42));
            }),
        )
        .unwrap()
        .try_into_value()
        .unwrap();
        assert!(echo.is_empty());
    }

    #[test]
    fn an_invocable_function_with_an_uninhabited_return_preserves_the_source_panic() {
        use crate::embedding::{CallError, CustomType, NamedTypeSchema};
        use crate::{ExecutionError, PanicKind, PanicSite, SourceContext, SourceSpan};

        struct Empty;
        impl NamedTypeSchema for Empty {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "library";
            const NAME: &'static str = "Empty";
        }
        let source = r#"
pub type Empty { Again(Empty) }
pub fn stops() -> fn() -> Empty { fn() { panic as "stopped" } }
"#;
        let program = crate::compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("library", "src/library.gleam", source)],
            )],
            HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        let (bindings, stops) = HostedModuleBuilder::new(program)
            .unwrap()
            .function(FunctionDeclaration::<(), CallableType<(), CustomType<Empty>>>::new("stops"))
            .unwrap();
        let mut module = bindings.seal().unwrap();
        let host = crate::execution_fixture::TestHost::default();
        let actual = host
            .block_on(
                module.with_execution(&host, &mut (), &mut Vec::new(), async |scope| {
                    let callback = scope.call(&stops, ()).await.unwrap();
                    scope.invoke(&callback, ()).await.err().unwrap()
                }),
            )
            .unwrap()
            .try_into_value()
            .unwrap();
        let expression = "panic as \"stopped\"";
        let start = source.find(expression).unwrap();
        assert_eq!(
            actual,
            CallError::Execution(ExecutionError::source_panic(
                Some(&SourceContext::new("src/library.gleam", source)),
                PanicKind::Panic,
                Some("stopped".into()),
                PanicSite::new(
                    "library".into(),
                    "<anonymous:0>".into(),
                    SourceSpan::new(start, start + expression.len())
                ),
            ))
        );
    }
}
