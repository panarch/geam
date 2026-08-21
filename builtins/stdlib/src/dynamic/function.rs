use super::DynamicSequence;
use super::schema::{Dynamic, DynamicList, DynamicSchema};
use super::storage::{DynamicExternalStorage, DynamicPayload, DynamicRepresentation};
use crate::{GleamStdlibHostProfile, GleamStdlibRunState, stdlib_state};
use crate::{
    HostCall, HostCallCompletion, HostCallError, HostConstruction, HostExternal,
    HostExternalBinding, HostList, HostProvider, HostType,
};
use ecow::EcoString;
use std::marker::PhantomData;

pub(crate) struct DynamicProvider<Profile>(PhantomData<Profile>);

impl<Profile> HostProvider<Profile> for DynamicProvider<Profile>
where
    Profile: GleamStdlibHostProfile,
{
    type State = GleamStdlibRunState<Profile::Io>;

    fn project(state: &mut Profile::RunState) -> &mut Self::State {
        stdlib_state::<Profile>(state)
    }
}

impl<Profile> HostExternalBinding<Profile, DynamicSchema> for DynamicProvider<Profile>
where
    Profile: GleamStdlibHostProfile,
{
    type Storage = DynamicExternalStorage;
}

pub(super) fn classify<'call, Profile>(
    call: HostCall<'call, Profile, DynamicProvider<Profile>, EcoString>,
    value: HostExternal<'call, Dynamic>,
) -> Result<HostCallCompletion<'call, EcoString>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let name = call.external_payload(value).representation().name().into();
    Ok(call.return_value(name))
}

pub(super) fn cast<'call, Profile, Type>(
    mut call: HostCall<'call, Profile, DynamicProvider<Profile>, Dynamic>,
    value: Type::Value<'call>,
) -> Result<HostCallCompletion<'call, Dynamic>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
    Type: HostType,
{
    let dynamic = create_return_value::<Profile, DynamicProvider<Profile>, Type>(&mut call, value);
    Ok(call.return_value(dynamic))
}

pub(super) fn array<'call, Profile>(
    mut call: HostCall<'call, Profile, DynamicProvider<Profile>, Dynamic>,
    values: HostList<'call, Dynamic>,
) -> Result<HostCallCompletion<'call, Dynamic>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    let mut index = 0;
    let mut sequence = Vec::new();
    while let Some(value) = call.list_item(values, index) {
        sequence.push(value);
        index += 1;
    }
    let dynamic = call.create_external_with(move |builder| DynamicPayload::Array {
        value: builder.store_dynamic::<DynamicList>(values),
        elements: sequence
            .into_iter()
            .map(|value| builder.store_dynamic::<Dynamic>(value))
            .collect(),
    });
    Ok(call.return_value(dynamic))
}

pub(crate) fn classification<'call, Profile, Provider, Return>(
    call: &HostCall<'call, Profile, Provider, Return>,
    value: HostExternal<'call, Dynamic>,
) -> EcoString
where
    Profile: GleamStdlibHostProfile,
    Provider: HostExternalBinding<Profile, DynamicSchema, Storage = DynamicExternalStorage>,
    Return: HostType,
{
    call.external_payload(value).representation().name().into()
}

pub(crate) fn create_return_value<'call, Profile, Provider, Type>(
    call: &mut HostCall<'call, Profile, Provider, Dynamic>,
    value: Type::Value<'call>,
) -> HostExternal<'call, Dynamic>
where
    Profile: GleamStdlibHostProfile,
    Provider: HostExternalBinding<Profile, DynamicSchema, Storage = DynamicExternalStorage>,
    Type: HostType,
{
    call.create_external_with(|builder| {
        let value = builder.store_dynamic::<Type>(value);
        DynamicPayload::Stored {
            representation: DynamicRepresentation::from_value(&value),
            value,
        }
    })
}

pub fn create_value<'call, Profile, Provider, Return, Type>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    construction: HostConstruction<'call, Dynamic>,
    value: Type::Value<'call>,
) -> HostExternal<'call, Dynamic>
where
    Profile: GleamStdlibHostProfile,
    Provider: HostExternalBinding<Profile, DynamicSchema, Storage = DynamicExternalStorage>,
    Return: HostType,
    Type: HostType,
{
    call.construct_external_with::<DynamicSchema, crate::HostTypeListEnd>(construction, |builder| {
        let value = builder.store_dynamic::<Type>(value);
        DynamicPayload::Stored {
            representation: DynamicRepresentation::from_value(&value),
            value,
        }
    })
}

pub(crate) fn decode_value<'call, Profile, Provider, Return, Type>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    value: HostExternal<'call, Dynamic>,
) -> Option<Type::Value<'call>>
where
    Profile: GleamStdlibHostProfile,
    Provider: HostExternalBinding<Profile, DynamicSchema, Storage = DynamicExternalStorage>,
    Return: HostType,
    Type: HostType,
{
    let payload = call.external_payload(value);
    payload.decode::<Profile, Provider, Return, Type>(call, DynamicPayload::value)
}

pub(crate) fn sequence<'call, Profile, Provider, Return>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    value: HostExternal<'call, Dynamic>,
) -> Option<DynamicSequence<'call>>
where
    Profile: GleamStdlibHostProfile,
    Provider: HostExternalBinding<Profile, DynamicSchema, Storage = DynamicExternalStorage>,
    Return: HostType,
{
    let payload = call.external_payload(value);
    let sequence = match payload.representation() {
        DynamicRepresentation::List => DynamicSequence::List,
        DynamicRepresentation::Array => DynamicSequence::Array,
        _ => return None,
    };
    payload
        .decode::<Profile, Provider, Return, DynamicList>(call, DynamicPayload::value)
        .map(sequence)
}

#[cfg(test)]
mod tests {
    use super::super::host_provider;
    use super::{Dynamic, DynamicProvider};
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
        let projected =
            <DynamicProvider<GleamStdlibProfile> as HostProvider<GleamStdlibProfile>>::project(
                &mut state,
            );

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
