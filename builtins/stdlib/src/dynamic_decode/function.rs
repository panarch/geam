use crate::dict::DictDeclaration;
use crate::dynamic::DynamicPayload;
use crate::{Component, GleamStdlibRunState};
use ecow::EcoString;
use geam_core::provider::{Call, Callback, List, Value};
use num_bigint::BigInt;
use num_traits::ToPrimitive;

#[geam_macros::module(
    path = "gleam/dynamic/decode",
    crate_path = geam_core,
    profile = crate::GleamStdlibHostProfile,
    component = crate::Component<Profile::Io>,
)]
pub(super) mod provider {
    use super::{
        BigInt, Call, Callback, DictDeclaration, DynamicPayload, EcoString, GleamStdlibRunState,
        List, ToPrimitive, Value,
    };
    use geam_core::provider::HostResult;
    use geam_core::provider::advanced::NativeKind;

    #[geam_macros::custom(input = DecodeErrorInput)]
    pub enum DecodeError {
        DecodeError {
            expected: EcoString,
            found: EcoString,
            path: Vec<EcoString>,
        },
    }

    #[geam_macros::function(profile = Profile)]
    fn bare_index<Key>(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        data: geam_core::provider::advanced::External<DynamicPayload>,
        key: Value<Key>,
    ) -> Result<Option<crate::dynamic::DynamicPayload>, EcoString> {
        let value = data.native_value().clone();
        drop(data);
        let key = call.store_dynamic::<_, DynamicPayload>(key).native_view();
        if let Some(dict) = value.as_map() {
            return Ok(dict
                .get(call.native_hash(&key), &key, |left, right| {
                    call.native_equal(left, right)
                })
                .map(DynamicPayload::from_native));
        }

        let Some(index) = key.as_int() else {
            return Err("Dict".into());
        };
        let kind = value.kind();
        if !matches!(kind, NativeKind::Tuple | NativeKind::List) {
            return Err("Indexable".into());
        }
        let Some(index) = index.to_usize() else {
            return if kind == NativeKind::Tuple {
                Ok(None)
            } else {
                Err("Indexable".into())
            };
        };
        if kind == NativeKind::List && index >= 8 {
            return Err("Indexable".into());
        }
        match value.index(index) {
            Some(value) => Ok(Some(DynamicPayload::from_native(value))),
            None if kind == NativeKind::Tuple => Ok(None),
            None => Err("Indexable".into()),
        }
    }

    #[geam_macros::function]
    fn dynamic_string(
        data: geam_core::provider::advanced::External<DynamicPayload>,
    ) -> Result<EcoString, EcoString> {
        data.native_value()
            .as_string()
            .ok_or_else(EcoString::default)
    }

    #[geam_macros::function]
    fn dynamic_int(
        data: geam_core::provider::advanced::External<DynamicPayload>,
    ) -> Result<BigInt, BigInt> {
        data.native_value().as_int().ok_or_else(|| BigInt::from(0))
    }

    #[geam_macros::function]
    fn dynamic_float(
        data: geam_core::provider::advanced::External<DynamicPayload>,
    ) -> Result<f64, f64> {
        data.native_value().as_float().ok_or(0.0)
    }

    #[geam_macros::function]
    fn dynamic_bit_array(
        data: geam_core::provider::advanced::External<DynamicPayload>,
    ) -> Result<geam_core::BitArrayValue, geam_core::BitArrayValue> {
        data.native_value()
            .as_bit_array()
            .ok_or_else(|| geam_core::BitArrayValue::from_bytes(Vec::new()))
    }

    #[geam_macros::function(profile = Profile, await)]
    async fn decode_list<Item, PathKey>(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        data: geam_core::provider::advanced::External<DynamicPayload>,
        item: Callback<fn(crate::dynamic::DynamicPayload) -> (Value<Item>, List<DecodeErrorInput>)>,
        _push_path: Value<
            fn((Item, List<DecodeErrorInput>), PathKey) -> (Item, List<DecodeErrorInput>),
        >,
        mut index: BigInt,
        accumulator: Value<List<Item>>,
    ) -> HostResult<(Vec<Value<Item>>, Vec<DecodeError>)> {
        let (representation, values) =
            data.with(|data| (data.representation(), data.native_value().clone()));
        drop(data);
        let mut decoded = call
            .with_call(move |call| {
                let mut decoded = Vec::with_capacity(call.list_len(&accumulator));
                let mut index = 0;
                while let Some(value) = call.list_get::<_, Item, _>(&accumulator, index) {
                    decoded.push(value);
                    index += 1;
                }
                decoded
            })
            .await?;
        if !matches!(values.kind(), NativeKind::List | NativeKind::Tuple) && decoded.is_empty() {
            return Ok((
                Vec::new(),
                vec![DecodeError::DecodeError {
                    expected: "List".into(),
                    found: representation.name().into(),
                    path: Vec::new(),
                }],
            ));
        }

        decoded.reverse();

        let mut value_index = 0;
        while let Some(value) = values.index(value_index) {
            let (value, errors) = call
                .invoke(&item, (DynamicPayload::from_native(value),))
                .await?;
            if errors.len() != 0 {
                let mut updated_errors = Vec::with_capacity(errors.len());
                let mut error_index = 0;
                while let Some(error) = errors.get(error_index) {
                    match error {
                        DecodeErrorInput::DecodeError {
                            expected,
                            found,
                            path,
                        } => {
                            let mut updated_path = Vec::with_capacity(path.len() + 1);
                            updated_path.push(index.to_string().into());
                            let mut path_index = 0;
                            while let Some(segment) = path.get(path_index) {
                                updated_path.push(segment);
                                path_index += 1;
                            }
                            updated_errors.push(DecodeError::DecodeError {
                                expected,
                                found,
                                path: updated_path,
                            });
                        }
                    }
                    error_index += 1;
                }
                return Ok((Vec::new(), updated_errors));
            }
            decoded.push(value);
            value_index += 1;
            index += 1;
        }

        Ok((decoded, Vec::new()))
    }

    #[geam_macros::function(profile = Profile)]
    fn decode_dict(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        data: geam_core::provider::advanced::External<DynamicPayload>,
    ) -> Result<crate::dict::DynamicDictOutput, ()> {
        let value = data.native_value().clone();
        drop(data);
        if let Some(dict) =
            call.restore_native::<DictDeclaration<DynamicPayload, DynamicPayload>>(&value)
        {
            return Ok(crate::dict::DynamicDictOutput::exact(dict.into_value()));
        }
        value
            .as_map()
            .map(crate::dict::DynamicDictOutput::native)
            .ok_or(())
    }

    #[geam_macros::function(profile = Profile)]
    fn cast<Item>(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        value: Value<Item>,
    ) -> crate::dynamic::DynamicPayload {
        DynamicPayload::stored(call.store_dynamic(value))
    }

    #[geam_macros::function]
    fn is_null(value: geam_core::provider::advanced::External<DynamicPayload>) -> bool {
        matches!(
            value.native_value().as_symbol().as_deref(),
            Some("nil" | "null" | "undefined")
        )
    }
}

pub(super) fn host_provider<Profile>()
-> Result<crate::HostProviderModule<Profile>, crate::HostRegistrationError>
where
    Profile: crate::GleamStdlibProviderProfile,
{
    provider::__geam_module::<Profile>()
}

#[cfg(test)]
mod tests {
    use super::provider::__GeamProvider as DynamicDecodeProvider;
    use crate::{
        ExecutionError, HostModule, HostProvider, HostProviderSet, HostedExecution, ModuleSource,
        PackageSource, PanicKind, PanicMessage, Value, compile_typed_host_program,
        plan_host_program,
    };
    use crate::{GleamStdlibProfile, GleamStdlibRunState};
    use ecow::EcoString;

    const OPTION_SOURCE: &str = r#"
pub type Option(value) {
  Some(value)
  None
}
"#;

    const DICT_SOURCE: &str = r#"
pub type Dict(key, value)

pub type TransientDict(key, value)

@external(erlang, "gleam_stdlib", "identity")
pub fn to_transient(dict: Dict(key, value)) -> TransientDict(key, value)

@external(erlang, "gleam_stdlib", "identity")
pub fn from_transient(transient: TransientDict(key, value)) -> Dict(key, value)

@external(erlang, "maps", "size")
pub fn size(dict: Dict(key, value)) -> Int

@external(erlang, "maps", "is_key")
pub fn do_has_key(key: key, dict: Dict(key, value)) -> Bool

@external(erlang, "maps", "new")
pub fn new() -> Dict(key, value)

@external(erlang, "gleam_stdlib", "map_get")
pub fn get(dict: Dict(key, value), key: key) -> Result(value, Nil)

@external(erlang, "maps", "put")
pub fn do_insert(
  key: key,
  value: value,
  dict: Dict(key, value),
) -> Dict(key, value)

@external(erlang, "maps", "put")
pub fn transient_insert(
  key: key,
  value: value,
  dict: TransientDict(key, value),
) -> TransientDict(key, value)

@external(erlang, "maps", "map")
pub fn do_map_values(
  function: fn(key, value) -> mapped,
  dict: Dict(key, value),
) -> Dict(key, mapped)

@external(erlang, "maps", "remove")
pub fn transient_delete(
  key: key,
  dict: TransientDict(key, value),
) -> TransientDict(key, value)

@external(erlang, "maps", "fold")
pub fn do_fold(
  function: fn(key, value, accumulator) -> accumulator,
  initial: accumulator,
  dict: Dict(key, value),
) -> accumulator

@external(erlang, "maps", "update_with")
pub fn transient_update_with(
  key: key,
  function: fn(value) -> value,
  initial: value,
  dict: TransientDict(key, value),
) -> TransientDict(key, value)
"#;

    const DYNAMIC_SOURCE: &str = r#"
@external(erlang, "gleam_stdlib", "Dynamic")
pub type Dynamic

@external(erlang, "gleam_stdlib", "classify_dynamic")
pub fn classify(value: Dynamic) -> String

@external(erlang, "gleam_stdlib", "identity")
pub fn bool(value: Bool) -> Dynamic

@external(erlang, "gleam_stdlib", "identity")
pub fn string(value: String) -> Dynamic

@external(erlang, "gleam_stdlib", "identity")
pub fn float(value: Float) -> Dynamic

@external(erlang, "gleam_stdlib", "identity")
pub fn int(value: Int) -> Dynamic

@external(erlang, "gleam_stdlib", "identity")
pub fn bit_array(value: BitArray) -> Dynamic

@external(erlang, "gleam_stdlib", "identity")
pub fn list(value: List(Dynamic)) -> Dynamic

@external(erlang, "erlang", "list_to_tuple")
pub fn array(value: List(Dynamic)) -> Dynamic

@external(erlang, "gleam_stdlib", "identity")
pub fn cast(value: value) -> Dynamic
"#;

    const DECODE_SOURCE: &str = r#"
import gleam/dict
import gleam/dynamic
import gleam/option

pub type DecodeError {
  DecodeError(expected: String, found: String, path: List(String))
}

@external(erlang, "gleam_stdlib", "index")
fn bare_index(
  data: dynamic.Dynamic,
  key: key,
) -> Result(option.Option(dynamic.Dynamic), String)

@external(erlang, "gleam_stdlib", "string")
fn dynamic_string(data: dynamic.Dynamic) -> Result(String, String)

@external(erlang, "gleam_stdlib", "int")
fn dynamic_int(data: dynamic.Dynamic) -> Result(Int, Int)

@external(erlang, "gleam_stdlib", "float")
fn dynamic_float(data: dynamic.Dynamic) -> Result(Float, Float)

@external(erlang, "gleam_stdlib", "bit_array")
fn dynamic_bit_array(data: dynamic.Dynamic) -> Result(BitArray, BitArray)

@external(erlang, "gleam_stdlib", "list")
fn decode_list(
  data: dynamic.Dynamic,
  item: fn(dynamic.Dynamic) -> #(item, List(DecodeError)),
  push_path: fn(#(item, List(DecodeError)), key) -> #(item, List(DecodeError)),
  index: Int,
  accumulator: List(item),
) -> #(List(item), List(DecodeError))

@external(erlang, "gleam_stdlib", "dict")
fn decode_dict(
  data: dynamic.Dynamic,
) -> Result(dict.Dict(dynamic.Dynamic, dynamic.Dynamic), Nil)

@external(erlang, "gleam_stdlib", "identity")
fn cast(value: value) -> dynamic.Dynamic

@external(erlang, "gleam_stdlib", "is_null")
fn is_null(value: dynamic.Dynamic) -> Bool

fn decode_int_item(data: dynamic.Dynamic) -> #(Int, List(DecodeError)) {
  case dynamic_int(data) {
    Ok(value) -> #(value, [])
    Error(_) -> #(0, [DecodeError("Int", "Other", ["inner"])])
  }
}

fn keep_path(
  layer: #(item, List(DecodeError)),
  _key: key,
) -> #(item, List(DecodeError)) {
  layer
}

pub fn decode_items(
  data: dynamic.Dynamic,
  item: fn(dynamic.Dynamic) -> #(item, List(DecodeError)),
) -> #(List(item), List(DecodeError)) {
  decode_list(data, item, keep_path, 0, [])
}

pub fn main() {
  let one = dynamic.int(1)
  let two = dynamic.int(2)
  let text = dynamic.string("two")
  let bits = dynamic.bit_array(<<1, 2>>)

  assert dynamic_string(text) == Ok("two")
  assert dynamic_string(one) == Error("")
  assert dynamic_int(one) == Ok(1)
  assert dynamic_int(dynamic.float(1.0)) == Error(0)
  assert dynamic_float(dynamic.float(1.5)) == Ok(1.5)
  assert dynamic_float(one) == Error(0.0)
  assert dynamic_bit_array(bits) == Ok(<<1, 2>>)
  assert dynamic_bit_array(one) == Error(<<>>)

  assert is_null(cast(Nil))
  assert !is_null(cast(1))

  let dict = dict.do_insert(dynamic.string("key"), one, dict.new())
  let dynamic_dict = dynamic.cast(dict)
  assert bare_index(dynamic_dict, "key") == Ok(option.Some(one))
  assert bare_index(dynamic_dict, "missing") == Ok(option.None)
  assert bare_index(dynamic_dict, 1) == Ok(option.None)
  assert decode_dict(dynamic_dict) == Ok(dict)
  assert decode_dict(one) == Error(Nil)
  let raw_dict = dynamic.cast(dict.do_insert("key", 1, dict.new()))
  assert bare_index(raw_dict, "key") == Ok(option.Some(one))
    as "raw dict indexing"
  assert decode_dict(raw_dict) == Ok(dict) as "raw dict decoding"
  assert raw_dict == dynamic_dict as "native dictionary equality"

  let list = dynamic.list([one, two])
  let array = dynamic.array([one, two])
  assert bare_index(list, 0) == Ok(option.Some(one))
  assert bare_index(list, 7) == Error("Indexable")
  assert bare_index(list, 8) == Error("Indexable")
  assert bare_index(list, -1) == Error("Indexable")
  assert bare_index(array, 1) == Ok(option.Some(two))
  assert bare_index(array, 2) == Ok(option.None)
  assert bare_index(one, 0) == Error("Indexable")
  assert bare_index(one, "key") == Error("Dict")
  assert bare_index(dynamic.cast([1, 2]), 0) == Ok(option.Some(one))
  assert bare_index(array, -1) == Ok(option.None)
  assert bare_index(array, 999999999999999999999999999999999) == Ok(option.None)
  assert bare_index(dynamic.cast(#(1, 2)), 1) == Ok(option.Some(two))

  assert decode_list(list, decode_int_item, keep_path, 0, []) == #([1, 2], [])
  assert decode_list(array, decode_int_item, keep_path, 5, [4, 3]) ==
    #([3, 4, 1, 2], [])
  assert decode_list(dynamic.cast([1, 2]), decode_int_item, keep_path, 0, []) ==
    #([1, 2], [])
  assert decode_list(dynamic.cast(#(1, 2)), decode_int_item, keep_path, 0, []) ==
    #([1, 2], [])
  assert decode_list(one, decode_int_item, keep_path, 0, [4, 3]) ==
    #([3, 4], [])
  assert decode_list(
    dynamic.list([one, text]),
    decode_int_item,
    keep_path,
    5,
    [],
  ) == #([], [DecodeError("Int", "Other", ["6", "inner"])])
  assert decode_list(one, decode_int_item, keep_path, 0, []) ==
    #([], [DecodeError("List", "Int", [])])

  True
}
"#;

    const CALLBACK_FAILURE_SOURCE: &str = r#"
import gleam/dynamic
import gleam/dynamic/decode

fn fail(
  _value: dynamic.Dynamic,
) -> #(Int, List(decode.DecodeError)) {
  panic as "callback failed"
}

pub fn main() {
  decode.decode_items(dynamic.list([dynamic.int(1)]), fail)
}
"#;

    #[test]
    fn executes_every_dynamic_decode_provider_through_the_hosted_pipeline() {
        let providers = [
            crate::dict::host_provider::<GleamStdlibProfile>()
                .expect("official dict provider should register"),
            crate::dynamic::host_provider::<GleamStdlibProfile>()
                .expect("official dynamic provider should register"),
            crate::dynamic_decode::host_provider::<GleamStdlibProfile>()
                .expect("official dynamic decode provider should register"),
        ];
        let typed = compile_typed_host_program(
            "gleam_stdlib",
            "gleam/dynamic/decode",
            [PackageSource::new(
                "gleam_stdlib",
                Vec::<EcoString>::new(),
                [
                    ModuleSource::new("gleam/option", "src/gleam/option.gleam", OPTION_SOURCE),
                    ModuleSource::new("gleam/dict", "src/gleam/dict.gleam", DICT_SOURCE),
                    ModuleSource::new("gleam/dynamic", "src/gleam/dynamic.gleam", DYNAMIC_SOURCE),
                    ModuleSource::new(
                        "gleam/dynamic/decode",
                        "src/gleam/dynamic/decode.gleam",
                        DECODE_SOURCE,
                    ),
                ],
            )],
            HostProviderSet::with_providers(
                Vec::<HostModule<GleamStdlibProfile>>::new(),
                providers,
            )
            .expect("official providers should be unique"),
        )
        .expect("synthetic dynamic decode source should compile");
        let plan = plan_host_program(typed).expect("synthetic dynamic decode source should plan");
        let mut execution = HostedExecution::try_from_module_plan(plan)
            .expect("synthetic dynamic decode execution should seal");
        let actual = crate::execution_fixture::run(
            &mut execution,
            &mut GleamStdlibRunState::from_seed([0; 32]),
            &mut Vec::new(),
        )
        .expect("every dynamic decode provider should execute");

        assert_eq!(actual, Value::Bool(true));
    }

    #[test]
    fn preserves_nested_source_panic_from_the_list_decoder_callback() {
        let providers = [
            crate::dict::host_provider::<GleamStdlibProfile>()
                .expect("official dict provider should register"),
            crate::dynamic::host_provider::<GleamStdlibProfile>()
                .expect("official dynamic provider should register"),
            crate::dynamic_decode::host_provider::<GleamStdlibProfile>()
                .expect("official dynamic decode provider should register"),
        ];
        let typed = compile_typed_host_program(
            "gleam_stdlib",
            "main",
            [PackageSource::new(
                "gleam_stdlib",
                Vec::<EcoString>::new(),
                [
                    ModuleSource::new("gleam/option", "src/gleam/option.gleam", OPTION_SOURCE),
                    ModuleSource::new("gleam/dict", "src/gleam/dict.gleam", DICT_SOURCE),
                    ModuleSource::new("gleam/dynamic", "src/gleam/dynamic.gleam", DYNAMIC_SOURCE),
                    ModuleSource::new(
                        "gleam/dynamic/decode",
                        "src/gleam/dynamic/decode.gleam",
                        DECODE_SOURCE,
                    ),
                    ModuleSource::new("main", "src/main.gleam", CALLBACK_FAILURE_SOURCE),
                ],
            )],
            HostProviderSet::with_providers(
                Vec::<HostModule<GleamStdlibProfile>>::new(),
                providers,
            )
            .expect("official providers should be unique"),
        )
        .expect("callback failure source should compile");
        let plan = plan_host_program(typed).expect("callback failure source should plan");
        let mut execution = HostedExecution::try_from_module_plan(plan)
            .expect("callback failure execution should seal");
        let error = crate::execution_fixture::run(
            &mut execution,
            &mut GleamStdlibRunState::from_seed([0; 32]),
            &mut Vec::new(),
        )
        .expect_err("callback should preserve its source panic");
        assert!(matches!(
            error,
            ExecutionError::Panic(ref panic)
                if panic.kind() == PanicKind::Panic
                    && panic.message()
                        == &PanicMessage::Explicit(EcoString::from("callback failed"))
                    && panic.site().module() == "main"
                    && panic.site().function() == "fail"
        ));
    }

    #[test]
    fn provider_projects_the_complete_run_state() {
        let mut state = GleamStdlibRunState::from_seed([0; 32]);
        let projected =
            <DynamicDecodeProvider as HostProvider<GleamStdlibProfile>>::project(&mut state);

        assert!(std::ptr::eq(projected, &state));
    }
}
