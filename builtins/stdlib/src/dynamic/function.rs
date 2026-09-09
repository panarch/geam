use super::storage::{DynamicRepresentation, DynamicValue};
use super::{Dynamic, DynamicSchema};
use crate::{
    Component, GleamStdlibRunState, HostConstruction, HostExternal, HostProvider, HostType,
    HostTypeListEnd,
};
use ecow::EcoString;
use geam_core::provider::advanced::{
    Equality, Hashing, Inspection, NativeValue, RetainedExternalPayload,
};
use geam_core::provider::{Call, Value};
use num_bigint::BigInt;

#[geam_macros::module(
    path = "gleam/dynamic",
    crate_path = geam_core,
    profile = crate::GleamStdlibHostProfile,
    component = crate::Component<Profile::Io>,
    stores = dynamic,
)]
pub(super) mod provider {
    use super::{
        BigInt, Call, DynamicRepresentation, DynamicValue, EcoString, Equality,
        GleamStdlibRunState, Hashing, Inspection, NativeValue, RetainedExternalPayload, Value,
    };

    #[geam_macros::external(name = "Dynamic", retained)]
    pub struct DynamicPayload {
        pub(in crate::dynamic) value: DynamicValue,
    }

    impl DynamicPayload {
        pub(crate) fn stored(
            value: geam_core::provider::advanced::StoredDynamic<DynamicPayload>,
        ) -> Self {
            Self {
                value: DynamicValue::stored(value),
            }
        }

        pub(crate) fn representation(&self) -> DynamicRepresentation {
            self.value.representation()
        }

        /// Creates a Gleam Dynamic value from a declared native representation.
        pub fn from_native(value: NativeValue) -> Self {
            Self {
                value: DynamicValue::native(value),
            }
        }

        /// Borrows the native representation without changing its retained source types.
        pub fn native_value(&self) -> &NativeValue {
            self.value.view()
        }
    }

    impl RetainedExternalPayload for DynamicPayload {
        fn source_equal(&self, context: &Equality<'_>, other: &Self) -> bool {
            self.native_value()
                .source_equal(context, other.native_value())
        }

        fn source_hash(&self, context: &Hashing<'_>) -> u64 {
            self.native_value().source_hash(context)
        }

        fn inspect(&self, context: &Inspection<'_>) -> EcoString {
            self.native_value().inspect(context)
        }

        fn native_view(&self) -> Option<NativeValue> {
            Some(self.native_value().clone())
        }
    }

    #[geam_macros::function]
    fn classify(value: &DynamicPayload) -> EcoString {
        value.representation().name().into()
    }

    #[geam_macros::function(profile = Profile)]
    fn bool(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        value: bool,
    ) -> DynamicPayload {
        DynamicPayload::stored(call.store_dynamic(value))
    }

    #[geam_macros::function(profile = Profile)]
    fn string(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        value: EcoString,
    ) -> DynamicPayload {
        DynamicPayload::stored(call.store_dynamic(value))
    }

    #[geam_macros::function(profile = Profile)]
    fn float(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        value: f64,
    ) -> DynamicPayload {
        DynamicPayload::stored(call.store_dynamic(value))
    }

    #[geam_macros::function(profile = Profile)]
    fn int(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        value: BigInt,
    ) -> DynamicPayload {
        DynamicPayload::stored(call.store_dynamic(value))
    }

    #[geam_macros::function(profile = Profile)]
    fn bit_array(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        value: geam_core::BitArrayValue,
    ) -> DynamicPayload {
        DynamicPayload::stored(call.store_dynamic(value))
    }

    #[geam_macros::function(profile = Profile)]
    fn list(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        values: List<DynamicPayload>,
    ) -> DynamicPayload {
        DynamicPayload::stored(call.store_dynamic(values))
    }

    #[geam_macros::function(profile = Profile)]
    fn array(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        values: List<DynamicPayload>,
    ) -> DynamicPayload {
        DynamicPayload::from_native(call.native_tuple(values))
    }

    #[geam_macros::function(profile = Profile)]
    fn cast<Item>(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        value: Value<Item>,
    ) -> DynamicPayload {
        DynamicPayload::stored(call.store_dynamic(value))
    }
}

pub(super) fn host_provider<Profile>()
-> Result<crate::HostProviderModule<Profile>, crate::HostRegistrationError>
where
    Profile: crate::GleamStdlibProviderProfile,
{
    provider::__geam_module::<Profile>()
}

pub fn create_value<'call, Profile, Provider, Return, Type>(
    call: &mut geam_core::host::HostCall<'call, Profile, Provider, Return>,
    construction: HostConstruction<'call, Dynamic>,
    value: Type::Value<'call>,
) -> HostExternal<'call, Dynamic>
where
    Profile: crate::GleamStdlibProviderProfile,
    Profile::RunState: Send,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Type: HostType,
{
    let value = geam_core::__macro_support::retain_constructed_dynamic::<
        _,
        _,
        _,
        _,
        _,
        provider::DynamicPayload,
        Type,
    >(call, &construction, value);
    call.construct_external_with_binding::<provider::__GeamProvider, DynamicSchema, HostTypeListEnd>(
        construction, provider::DynamicPayload::stored(value),
    )
}

#[cfg(test)]
mod tests {
    use super::super::Dynamic;
    use super::super::host_provider;
    use super::provider::__GeamProvider as DynamicProvider;
    use crate::{GleamStdlibProfile, GleamStdlibRunState};
    use crate::{
        HostCall, HostCallCompletion, HostCallError, HostExternal, HostModule, HostProvider,
        HostProviderSet, HostedExecution, ModuleSource, PackageSource, Value,
        compile_typed_host_program, plan_host_program,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;

    mod native_payload {
        use super::super::provider::DynamicPayload;
        use crate::dict::{DictExternalStorage, DictSchema};
        use crate::dynamic::{Dynamic, DynamicExternalStorage, DynamicSchema};
        use crate::{
            Component, GleamStdlibHostProfile, GleamStdlibRunState, GleamStdlibStores, IoOutput,
        };
        use ecow::EcoString;
        use geam_core::provider::advanced::{
            Equality, Hashing, Inspection, NativeValue, RetainedExternalPayload,
        };
        use geam_core::{
            HostCall, HostCallCompletion, HostCallError, HostComponentProfile, HostExternal,
            HostExternalBinding, HostExternalSchema, HostExternalStorage, HostExternalStore,
            HostExternalType, HostProfile, HostProvider, HostProviderModule, HostProviderSet,
            HostedExecution, ModuleSource, PackageSource, compile_typed_host_program,
            plan_host_program,
        };
        use num_bigint::BigInt;

        struct Profile;
        struct Snapshot;
        struct SnapshotStorage;
        type SnapshotType = HostExternalType<Snapshot>;

        impl HostProfile for Profile {
            type RunState = GleamStdlibRunState;
            type ExternalStores = (GleamStdlibStores, HostExternalStore<DynamicPayload>);
        }

        impl HostComponentProfile<Component> for Profile {
            fn component_stores(stores: &Self::ExternalStores) -> &GleamStdlibStores {
                &stores.0
            }
            fn component_state(state: &mut Self::RunState) -> &mut GleamStdlibRunState {
                state
            }
        }

        impl GleamStdlibHostProfile for Profile {
            type Io = Vec<IoOutput>;
        }

        impl HostProvider<Profile> for Snapshot {
            type State = GleamStdlibRunState;
            fn project(state: &mut Self::State) -> &mut Self::State {
                state
            }
        }

        impl HostExternalSchema for Snapshot {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "main";
            const NAME: &'static str = "Snapshot";
            const PARAMETER_COUNT: usize = 0;
        }

        impl HostExternalBinding<Profile, Snapshot> for Snapshot {
            type Storage = SnapshotStorage;
        }

        impl HostExternalBinding<Profile, DynamicSchema> for Snapshot {
            type Storage = DynamicExternalStorage;
        }

        impl HostExternalBinding<Profile, DictSchema> for Snapshot {
            type Storage = DictExternalStorage;
        }

        impl HostExternalStorage<Profile, Snapshot> for SnapshotStorage {
            type Payload = DynamicPayload;
            fn store(
                stores: &<Profile as HostProfile>::ExternalStores,
            ) -> &HostExternalStore<DynamicPayload> {
                &stores.1
            }
            fn source_equal(
                context: &Equality<'_>,
                left: &DynamicPayload,
                right: &DynamicPayload,
            ) -> bool {
                left.source_equal(context, right)
            }
            fn source_hash(context: &Hashing<'_>, value: &DynamicPayload) -> u64 {
                value.source_hash(context)
            }
            fn inspect(context: &Inspection<'_>, value: &DynamicPayload) -> EcoString {
                value.inspect(context)
            }
        }

        fn native<'call>(
            mut call: HostCall<'call, Profile, Snapshot, Dynamic>,
            name: EcoString,
        ) -> Result<HostCallCompletion<'call, Dynamic>, HostCallError> {
            let value =
                call.create_external(DynamicPayload::from_native(NativeValue::symbol(name)));
            Ok(call.return_value(value))
        }

        fn snapshot<'call>(
            mut call: HostCall<'call, Profile, Snapshot, SnapshotType>,
            name: EcoString,
        ) -> Result<HostCallCompletion<'call, SnapshotType>, HostCallError> {
            let value =
                call.create_external(DynamicPayload::from_native(NativeValue::symbol(name)));
            Ok(call.return_value(value))
        }

        fn hash<'call>(
            call: HostCall<'call, Profile, Snapshot, BigInt>,
            value: HostExternal<'call, SnapshotType>,
        ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
            let value = call.source_hash::<SnapshotType>(value).into();
            Ok(call.return_value(value))
        }

        #[test]
        fn dynamic_payload_semantics_survive_opaque_storage_and_keep_native_classification() {
            let native = HostProviderModule::new("application", "main")
                .unwrap()
                .with_external_type::<Snapshot, Snapshot>()
                .unwrap()
                .with_scoped_function::<Snapshot, (EcoString,), Dynamic, _>("native", native)
                .unwrap()
                .with_scoped_function::<Snapshot, (EcoString,), SnapshotType, _>(
                    "snapshot", snapshot,
                )
                .unwrap()
                .with_scoped_function::<Snapshot, (SnapshotType,), BigInt, _>("hash", hash)
                .unwrap();
            let source = compile_typed_host_program(
                "application",
                "main",
                [
                    PackageSource::new(
                        "gleam_stdlib",
                        Vec::<String>::new(),
                        [
                            ModuleSource::new(
                                "gleam/dict",
                                "src/gleam/dict.gleam",
                                "pub type Dict(key, value)",
                            ),
                            ModuleSource::new(
                                "gleam/option",
                                "src/gleam/option.gleam",
                                "pub type Option(a) { Some(a) None }",
                            ),
                            ModuleSource::new(
                                "gleam/dynamic",
                                "src/gleam/dynamic.gleam",
                                super::DYNAMIC_DECLARATIONS,
                            ),
                            ModuleSource::new(
                                "gleam/dynamic/decode",
                                "src/gleam/dynamic/decode.gleam",
                                r#"
import gleam/dynamic.{type Dynamic}
import gleam/dict.{type Dict}
import gleam/option.{type Option}
pub type DecodeError {
  DecodeError(expected: String, found: String, path: List(String))
}
@external(erlang, "gleam_stdlib", "index")
fn bare_index(data: Dynamic, key: key) -> Result(Option(Dynamic), String)
@external(erlang, "gleam_stdlib", "string")
fn dynamic_string(data: Dynamic) -> Result(String, String)
@external(erlang, "gleam_stdlib", "int")
fn dynamic_int(data: Dynamic) -> Result(Int, Int)
@external(erlang, "gleam_stdlib", "float")
fn dynamic_float(data: Dynamic) -> Result(Float, Float)
@external(erlang, "gleam_stdlib", "bit_array")
fn dynamic_bit_array(data: Dynamic) -> Result(BitArray, BitArray)
@external(erlang, "gleam_stdlib", "list")
fn decode_list(
  data: Dynamic,
  item: fn(Dynamic) -> #(item, List(DecodeError)),
  push_path: fn(#(item, List(DecodeError)), key) -> #(item, List(DecodeError)),
  index: Int,
  accumulator: List(item),
) -> #(List(item), List(DecodeError))
@external(erlang, "gleam_stdlib", "dict")
fn decode_dict(data: Dynamic) -> Result(Dict(Dynamic, Dynamic), Nil)
@external(erlang, "gleam_stdlib", "identity")
fn cast(value: value) -> Dynamic
@external(erlang, "gleam_stdlib", "is_null")
pub fn is_null(value: Dynamic) -> Bool
"#,
                            ),
                        ],
                    ),
                    PackageSource::new(
                        "application",
                        ["gleam_stdlib"],
                        [ModuleSource::new(
                            "main",
                            "src/main.gleam",
                            r#"
import gleam/dynamic.{type Dynamic}
import gleam/dynamic/decode
pub type Snapshot
@external(erlang, "native", "native")
fn native(name: String) -> Dynamic
@external(erlang, "native", "snapshot")
fn snapshot(name: String) -> Snapshot
@external(erlang, "native", "hash")
fn hash(value: Snapshot) -> Int
pub fn main() {
  let first = snapshot("alpha")
  let same = snapshot("alpha")
  assert first == same
  assert first != snapshot("beta")
  assert hash(first) == hash(same)
  assert dynamic.classify(native("alpha")) == "Atom"
  assert dynamic.classify(native("null")) == "Nil"
  assert dynamic.classify(native("undefined")) == "Nil"
  assert dynamic.classify(dynamic.cast(first)) == "External"
  assert decode.is_null(native("null"))
  assert decode.is_null(native("undefined"))
  assert !decode.is_null(native("alpha"))
  first
}
"#,
                        )],
                    ),
                ],
                HostProviderSet::from_providers([
                    HostProviderModule::new("gleam_stdlib", "gleam/dict")
                        .unwrap()
                        .with_external_type::<Snapshot, DictSchema>()
                        .unwrap(),
                    crate::dynamic::host_provider::<Profile>().unwrap(),
                    crate::dynamic_decode::host_provider::<Profile>().unwrap(),
                    native,
                ])
                .unwrap(),
            )
            .unwrap();
            let execution =
                HostedExecution::try_from_module_plan(plan_host_program(source).unwrap()).unwrap();
            let mut state = GleamStdlibRunState::from_seed([0; 32]);
            let state_address = std::ptr::from_mut(&mut state);
            assert!(std::ptr::eq(
                state_address,
                <Snapshot as HostProvider<Profile>>::project(&mut state),
            ));
            assert!(std::ptr::eq(
                state_address,
                <Profile as HostComponentProfile<Component>>::component_state(&mut state),
            ));
            let value = execution.run_main(&mut state, &mut Vec::new()).unwrap();
            drop(execution);
            assert_eq!(value.inspect().to_string(), "Alpha");
        }
    }

    struct HashProvider;

    impl HostProvider<GleamStdlibProfile> for HashProvider {
        type State = GleamStdlibRunState;

        fn project(state: &mut GleamStdlibRunState) -> &mut Self::State {
            state
        }
    }

    fn source_hash<'call>(
        call: HostCall<'call, GleamStdlibProfile, HashProvider, BigInt>,
        value: HostExternal<'call, Dynamic>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let hash = BigInt::from(call.source_hash::<Dynamic>(value));
        Ok(call.return_value(hash))
    }

    const DYNAMIC_DECLARATIONS: &str = r#"
@external(erlang, "host", "Dynamic")
pub type Dynamic

@external(erlang, "host", "classify")
pub fn classify(value: Dynamic) -> String

@external(erlang, "host", "bool")
pub fn bool(value: Bool) -> Dynamic

@external(erlang, "host", "string")
pub fn string(value: String) -> Dynamic

@external(erlang, "host", "float")
pub fn float(value: Float) -> Dynamic

@external(erlang, "host", "int")
pub fn int(value: Int) -> Dynamic

@external(erlang, "host", "bit_array")
pub fn bit_array(value: BitArray) -> Dynamic

@external(erlang, "host", "list")
pub fn list(value: List(Dynamic)) -> Dynamic

@external(erlang, "host", "array")
pub fn array(value: List(Dynamic)) -> Dynamic

@external(erlang, "host", "cast")
pub fn cast(value: value) -> Dynamic
"#;

    fn execution(
        source: &str,
        modules: impl IntoIterator<Item = HostModule<GleamStdlibProfile>>,
    ) -> HostedExecution<GleamStdlibProfile> {
        let source = format!("{DYNAMIC_DECLARATIONS}\n{source}");
        let provider = host_provider::<GleamStdlibProfile>()
            .expect("official dynamic provider should register");
        let typed = compile_typed_host_program(
            "gleam_stdlib",
            "gleam/dynamic",
            [PackageSource::new(
                "gleam_stdlib",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "gleam/dynamic",
                    "src/gleam/dynamic.gleam",
                    source,
                )],
            )],
            HostProviderSet::with_providers(modules, [provider])
                .expect("dynamic provider module should be unique"),
        )
        .expect("synthetic dynamic source should compile");
        let plan = plan_host_program(typed).expect("synthetic dynamic source should plan");
        HostedExecution::try_from_module_plan(plan)
            .expect("synthetic dynamic execution should seal")
    }

    #[test]
    fn provider_projects_the_complete_run_state() {
        let mut state = GleamStdlibRunState::from_seed([0; 32]);
        let projected = <DynamicProvider as HostProvider<GleamStdlibProfile>>::project(&mut state);

        assert!(std::ptr::eq(projected, &state));

        let projected = <HashProvider as HostProvider<GleamStdlibProfile>>::project(&mut state);

        assert!(std::ptr::eq(projected, &state));
    }

    #[test]
    fn generic_cast_classifies_recursive_and_callable_families() {
        let source = r#"
pub type Boxed {
  Boxed(Int)
}

fn increment(value: Int) {
  value + 1
}

pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  #(
    classify(cast(codepoint)),
    classify(cast(#(1, "one"))),
    classify(cast([1, 2])),
    classify(cast(increment)),
    classify(cast(Boxed(1))),
    classify(cast(Nil)),
  )
}
"#;
        let actual = execution(source, Vec::<HostModule<GleamStdlibProfile>>::new())
            .run_main(
                &mut GleamStdlibRunState::from_seed([0; 32]),
                &mut Vec::new(),
            )
            .expect("generic dynamic casts should run");

        assert_eq!(
            actual,
            Value::Tuple(vec![
                Value::String("Int".into()),
                Value::String("Array".into()),
                Value::String("List".into()),
                Value::String("Function".into()),
                Value::String("Array".into()),
                Value::String("Nil".into()),
            ]),
        );
    }

    #[test]
    fn public_constructors_preserve_representation_equality_hash_and_inspection() {
        let hash =
            HostModule::<GleamStdlibProfile>::new_for_profile("gleam_stdlib", "host/dynamic_hash")
                .expect("hash module should be valid")
                .with_scoped_function::<HashProvider, (Dynamic,), BigInt, _>(
                    "source_hash",
                    source_hash,
                )
                .expect("hash function should be valid");
        let source = r#"
import host/dynamic_hash

pub fn main() {
  let bool_value = bool(True)
  let string_value = string("one")
  let float_value = float(1.5)
  let first_int = int(42)
  let second_int = int(42)
  let second_string = string("one")
  let bits = bit_array(<<1, 2>>)
  let list_value = list([first_int, string_value])
  let array_value = array([first_int, string_value])
  let second_array = array([second_int, second_string])
  let empty_array = array([])

  assert classify(bool_value) == "Bool"
  assert classify(string_value) == "String"
  assert classify(float_value) == "Float"
  assert classify(first_int) == "Int"
  assert classify(bits) == "String"
  assert classify(bit_array(<<1:size(1)>>)) == "BitArray"
  assert classify(list_value) == "List"
  assert classify(array_value) == "Array"
  assert classify(cast(first_int)) == "Int"
  assert cast(first_int) == first_int
  assert cast(#(42, "one")) == array_value
  assert array_value == cast(#(42, "one"))
  assert cast([42]) == list([first_int])
  assert dynamic_hash.source_hash(cast(#(42, "one")))
    == dynamic_hash.source_hash(array_value)
  assert first_int == second_int
  assert list_value != array_value
  assert array_value == second_array
  assert dynamic_hash.source_hash(first_int)
    == dynamic_hash.source_hash(second_int)
  assert dynamic_hash.source_hash(array_value)
    == dynamic_hash.source_hash(second_array)

  #(
    bool_value,
    string_value,
    float_value,
    first_int,
    bits,
    list_value,
    array_value,
    empty_array,
  )
}
"#;
        let actual = execution(source, [hash])
            .run_main(
                &mut GleamStdlibRunState::from_seed([0; 32]),
                &mut Vec::new(),
            )
            .expect("public dynamic constructors should run");

        assert_eq!(
            actual.inspect().to_string(),
            r#"#(True, "one", 1.5, 42, "\u{1}\u{2}", [42, "one"], #(42, "one"), #())"#,
        );
    }
}
