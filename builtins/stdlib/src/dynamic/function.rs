use super::storage::{DynamicRepresentation, DynamicValue};
use crate::{Component, GleamStdlibRunState};
use ecow::EcoString;
use geam_core::provider::advanced::{Equality, Hashing, Inspection, RetainedExternalPayload};
use geam_core::provider::{Call, Value};
use num_bigint::BigInt;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[geam_macros::module(
    path = "gleam/dynamic",
    crate_path = geam_core,
    profile = crate::GleamStdlibHostProfile,
    component = crate::Component<Profile::Io>,
    stores = crate::dynamic::stores,
)]
pub(super) mod provider {
    use super::{
        BigInt, Call, DefaultHasher, DynamicRepresentation, DynamicValue, EcoString, Equality,
        GleamStdlibRunState, Hash, Hasher, Hashing, Inspection, RetainedExternalPayload, Value,
    };

    #[geam_macros::external(name = "Dynamic", retained)]
    pub struct DynamicPayload {
        pub(in crate::dynamic) value: DynamicValue,
    }

    impl DynamicPayload {
        pub(crate) fn stored(value: geam_core::provider::advanced::StoredDynamic<Self>) -> Self {
            Self {
                value: DynamicValue::stored(value),
            }
        }

        pub(crate) fn representation(&self) -> DynamicRepresentation {
            self.value.representation()
        }

        pub(crate) fn stored_value(&self) -> &geam_core::provider::advanced::StoredDynamic<Self> {
            self.value.value()
        }
    }

    impl RetainedExternalPayload for DynamicPayload {
        fn source_equal(&self, context: &Equality<'_>, other: &Self) -> bool {
            self.representation() == other.representation()
                && self
                    .stored_value()
                    .source_equal(context, other.stored_value())
        }

        fn source_hash(&self, context: &Hashing<'_>) -> u64 {
            let mut hasher = DefaultHasher::new();
            self.representation().hash(&mut hasher);
            self.stored_value().source_hash(context).hash(&mut hasher);
            hasher.finish()
        }

        fn inspect(&self, context: &Inspection<'_>) -> EcoString {
            match &self.value {
                DynamicValue::Stored { value, .. } => value.inspect(context),
                DynamicValue::Array { elements, .. } => {
                    let values = elements
                        .iter()
                        .map(|item| item.inspect(context))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("#({values})").into()
                }
            }
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
        let mut elements = Vec::with_capacity(values.len());
        let mut index = 0;
        while let Some(value) = values.get(index) {
            elements.push(call.store_dynamic(value));
            index += 1;
        }
        DynamicPayload {
            value: DynamicValue::Array {
                value: call.store_dynamic(values),
                elements: elements.into_boxed_slice(),
            },
        }
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
    Profile: crate::GleamStdlibHostProfile,
{
    provider::__geam_module::<Profile>()
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
                Value::String("UtfCodepoint".into()),
                Value::String("Array".into()),
                Value::String("List".into()),
                Value::String("Function".into()),
                Value::String("Custom".into()),
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
  assert classify(bits) == "BitArray"
  assert classify(list_value) == "List"
  assert classify(array_value) == "Array"
  assert classify(cast(first_int)) == "External"
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
            r#"#(True, "one", 1.5, 42, <<1, 2>>, [42, "one"], #(42, "one"), #())"#,
        );
    }
}
