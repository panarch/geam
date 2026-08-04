mod collection;
mod index;
mod scalar;

pub(super) use collection::{decode_dict, decode_list};
pub(super) use index::bare_index;
pub(super) use scalar::{
    cast, dynamic_bit_array, dynamic_float, dynamic_int, dynamic_string, is_null,
};

use crate::gleam_stdlib::{
    DictExternalStorage, DictSchema, DynamicExternalStorage, DynamicSchema, GleamStdlibHostProfile,
};
use crate::{HostExternalBinding, HostProvider};
use std::marker::PhantomData;

pub(super) struct DynamicDecodeProvider<Profile>(PhantomData<Profile>);

impl<Profile> HostProvider<Profile> for DynamicDecodeProvider<Profile>
where
    Profile: GleamStdlibHostProfile,
{
    type State = Profile::RunState;

    fn project(state: &mut Profile::RunState) -> &mut Self::State {
        state
    }
}

impl<Profile> HostExternalBinding<Profile, DynamicSchema> for DynamicDecodeProvider<Profile>
where
    Profile: GleamStdlibHostProfile,
{
    type Storage = DynamicExternalStorage;
}

impl<Profile> HostExternalBinding<Profile, DictSchema> for DynamicDecodeProvider<Profile>
where
    Profile: GleamStdlibHostProfile,
{
    type Storage = DictExternalStorage;
}

#[cfg(test)]
mod tests {
    use super::DynamicDecodeProvider;
    use crate::gleam_stdlib::{GleamStdlibProfile, GleamStdlibRunState};
    use crate::{
        ExecutionError, HostModule, HostProvider, HostProviderSet, HostedExecution, ModuleSource,
        PackageSource, PanicKind, PanicMessage, Value, compile_typed_host_program,
        plan_host_program,
    };
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
  assert decode_dict(dynamic_dict) == Ok(dict)
  assert decode_dict(one) == Error(Nil)

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

  assert decode_list(list, decode_int_item, keep_path, 0, []) == #([1, 2], [])
  assert decode_list(array, decode_int_item, keep_path, 5, [4, 3]) ==
    #([3, 4, 1, 2], [])
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
            crate::gleam_stdlib::dict::host_provider::<GleamStdlibProfile>()
                .expect("official dict provider should register"),
            crate::gleam_stdlib::dynamic::host_provider::<GleamStdlibProfile>()
                .expect("official dynamic provider should register"),
            crate::gleam_stdlib::dynamic_decode::host_provider::<GleamStdlibProfile>()
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
        let execution = HostedExecution::try_from_module_plan(plan)
            .expect("synthetic dynamic decode execution should seal");
        let actual = execution
            .run_main(
                &mut GleamStdlibRunState::from_seed([0; 32]),
                &mut Vec::new(),
            )
            .expect("every dynamic decode provider should execute");

        assert_eq!(actual, Value::Bool(true));
    }

    #[test]
    fn preserves_nested_source_panic_from_the_list_decoder_callback() {
        let providers = [
            crate::gleam_stdlib::dict::host_provider::<GleamStdlibProfile>()
                .expect("official dict provider should register"),
            crate::gleam_stdlib::dynamic::host_provider::<GleamStdlibProfile>()
                .expect("official dynamic provider should register"),
            crate::gleam_stdlib::dynamic_decode::host_provider::<GleamStdlibProfile>()
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
        let execution = HostedExecution::try_from_module_plan(plan)
            .expect("callback failure execution should seal");
        let error = execution
            .run_main(
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
        let projected = <DynamicDecodeProvider<GleamStdlibProfile> as HostProvider<
            GleamStdlibProfile,
        >>::project(&mut state);

        assert!(std::ptr::eq(projected, &state));
    }
}
