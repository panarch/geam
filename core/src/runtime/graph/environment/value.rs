use super::BlockEnvironment;
use crate::StringValue;
use crate::plan::execution::graph::{
    BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId, BoolFunctionLocalId,
    BoolListLocalId, BoolLocalId, CustomFunctionLocal, CustomListLocalId, CustomLocal,
    ExternalFunctionLocal, ExternalListLocalId, ExternalLocal, FloatFunctionLocalId,
    FloatListLocalId, FloatLocalId, FunctionFunctionLocal, FunctionListLocalId, FunctionLocal,
    GenericFunctionLocal, IntFunctionLocalId, IntListLocalId, IntLocalId, ListFunctionLocal,
    ListListLocalId, NeverFunctionLocal, NilFunctionLocalId, NilListLocalId, NilLocalId,
    ParameterListListLocalId, ParameterListLocalId, StringFunctionLocalId, StringListLocalId,
    StringLocalId, TupleFunctionLocalId, TupleListLocalId, TupleLocalId,
    UtfCodepointFunctionLocalId, UtfCodepointListLocalId, UtfCodepointLocalId,
};
use crate::runtime::evaluated::{
    EvaluatedBitArray, EvaluatedCustomValue, EvaluatedExternalValue, EvaluatedFunctionValue,
    EvaluatedValue,
};
use crate::runtime::state::list::{
    BitArrayListValueId, BoolListValueId, CustomListValueId, ExternalListValueId, FloatListValueId,
    FunctionListValueId, IntListValueId, ListListValueId, NilListValueId, ParameterListListValueId,
    ParameterListValueId, StringListValueId, TupleListValueId, UtfCodepointListValueId,
};
use crate::runtime::{
    EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCustomFunction,
    EvaluatedExternalFunction, EvaluatedFloatFunction, EvaluatedFunctionFunction,
    EvaluatedGenericFunction, EvaluatedIntFunction, EvaluatedListFunction, EvaluatedNeverFunction,
    EvaluatedNilFunction, EvaluatedStringFunction, EvaluatedTupleFunction,
    EvaluatedUtfCodepointFunction,
};
use num_bigint::BigInt;
use std::convert::Infallible;

pub(in crate::runtime) trait GraphValue: Sync {
    type Evaluated: Send + 'static;

    fn take(&self, environment: BlockEnvironment) -> Self::Evaluated;
}

impl GraphValue for Infallible {
    type Evaluated = Infallible;

    fn take(&self, _environment: BlockEnvironment) -> Self::Evaluated {
        match *self {}
    }
}

impl GraphValue for NilLocalId {
    type Evaluated = ();

    fn take(&self, _environment: BlockEnvironment) {}
}

macro_rules! local_value {
    ($local:ty, $value:ty, $field:ident) => {
        impl GraphValue for $local {
            type Evaluated = $value;

            fn take(&self, mut environment: BlockEnvironment) -> Self::Evaluated {
                environment.values.$field.swap_remove(self.0)
            }
        }
    };
}

local_value!(IntLocalId, BigInt, ints);
local_value!(FloatLocalId, f64, floats);
local_value!(StringLocalId, StringValue, strings);
local_value!(BitArrayLocalId, EvaluatedBitArray, bit_arrays);
local_value!(UtfCodepointLocalId, char, utf_codepoints);
local_value!(BoolLocalId, bool, bools);
local_value!(TupleLocalId, Vec<EvaluatedValue>, tuples);
local_value!(ParameterListLocalId, ParameterListValueId, parameter_lists);
local_value!(
    ParameterListListLocalId,
    ParameterListListValueId,
    parameter_list_lists
);
local_value!(IntListLocalId, IntListValueId, int_lists);
local_value!(StringListLocalId, StringListValueId, string_lists);
local_value!(BitArrayListLocalId, BitArrayListValueId, bit_array_lists);
local_value!(
    UtfCodepointListLocalId,
    UtfCodepointListValueId,
    utf_codepoint_lists
);
local_value!(CustomListLocalId, CustomListValueId, custom_lists);
local_value!(ExternalListLocalId, ExternalListValueId, external_lists);
local_value!(FloatListLocalId, FloatListValueId, float_lists);
local_value!(BoolListLocalId, BoolListValueId, bool_lists);
local_value!(NilListLocalId, NilListValueId, nil_lists);
local_value!(TupleListLocalId, TupleListValueId, tuple_lists);
local_value!(ListListLocalId, ListListValueId, list_lists);
local_value!(FunctionListLocalId, FunctionListValueId, function_lists);
local_value!(IntFunctionLocalId, EvaluatedIntFunction, int_functions);
local_value!(
    FloatFunctionLocalId,
    EvaluatedFloatFunction,
    float_functions
);
local_value!(
    StringFunctionLocalId,
    EvaluatedStringFunction,
    string_functions
);
local_value!(
    BitArrayFunctionLocalId,
    EvaluatedBitArrayFunction,
    bit_array_functions
);
local_value!(
    UtfCodepointFunctionLocalId,
    EvaluatedUtfCodepointFunction,
    utf_codepoint_functions
);
local_value!(BoolFunctionLocalId, EvaluatedBoolFunction, bool_functions);
local_value!(NilFunctionLocalId, EvaluatedNilFunction, nil_functions);
local_value!(
    TupleFunctionLocalId,
    EvaluatedTupleFunction,
    tuple_functions
);

impl GraphValue for CustomLocal {
    type Evaluated = EvaluatedCustomValue;

    fn take(&self, mut environment: BlockEnvironment) -> Self::Evaluated {
        environment.values.customs.swap_remove(self.id().0)
    }
}

impl GraphValue for ExternalLocal {
    type Evaluated = EvaluatedExternalValue;

    fn take(&self, mut environment: BlockEnvironment) -> Self::Evaluated {
        environment.values.externals.swap_remove(self.id().0)
    }
}

impl GraphValue for CustomFunctionLocal {
    type Evaluated = EvaluatedCustomFunction;

    fn take(&self, mut environment: BlockEnvironment) -> Self::Evaluated {
        environment.values.custom_functions.swap_remove(self.id().0)
    }
}

impl GraphValue for ExternalFunctionLocal {
    type Evaluated = EvaluatedExternalFunction;

    fn take(&self, mut environment: BlockEnvironment) -> Self::Evaluated {
        environment
            .values
            .external_functions
            .swap_remove(self.id().0)
    }
}

impl GraphValue for GenericFunctionLocal {
    type Evaluated = EvaluatedGenericFunction;

    fn take(&self, mut environment: BlockEnvironment) -> Self::Evaluated {
        environment
            .values
            .generic_functions
            .swap_remove(self.id().0)
    }
}

impl GraphValue for NeverFunctionLocal {
    type Evaluated = EvaluatedNeverFunction;

    fn take(&self, mut environment: BlockEnvironment) -> Self::Evaluated {
        environment.values.never_functions.swap_remove(self.id().0)
    }
}

impl GraphValue for ListFunctionLocal {
    type Evaluated = EvaluatedListFunction;

    fn take(&self, mut environment: BlockEnvironment) -> Self::Evaluated {
        match self {
            Self::Parameter { local, .. } => environment
                .values
                .parameter_list_functions
                .swap_remove(local.0),
            Self::ParameterList { local, .. } => environment
                .values
                .parameter_list_list_functions
                .swap_remove(local.0),
            Self::Int { local, .. } => environment.values.int_list_functions.swap_remove(local.0),
            Self::String { local, .. } => environment
                .values
                .string_list_functions
                .swap_remove(local.0),
            Self::BitArray { local, .. } => environment
                .values
                .bit_array_list_functions
                .swap_remove(local.0),
            Self::UtfCodepoint { local, .. } => environment
                .values
                .utf_codepoint_list_functions
                .swap_remove(local.0),
            Self::Custom { local, .. } => environment
                .values
                .custom_list_functions
                .swap_remove(local.0),
            Self::External { local, .. } => environment
                .values
                .external_list_functions
                .swap_remove(local.0)
                .map_runtime_id(crate::plan::execution::function::RuntimeListFunctionId::External),
            Self::Float { local, .. } => {
                environment.values.float_list_functions.swap_remove(local.0)
            }
            Self::Bool { local, .. } => environment.values.bool_list_functions.swap_remove(local.0),
            Self::Nil { local, .. } => environment.values.nil_list_functions.swap_remove(local.0),
            Self::Tuple { local, .. } => {
                environment.values.tuple_list_functions.swap_remove(local.0)
            }
            Self::List { local, .. } => environment.values.list_list_functions.swap_remove(local.0),
            Self::Function { local, .. } => environment
                .values
                .function_list_functions
                .swap_remove(local.0),
        }
    }
}

impl GraphValue for FunctionFunctionLocal {
    type Evaluated = EvaluatedFunctionFunction;

    fn take(&self, mut environment: BlockEnvironment) -> Self::Evaluated {
        match self {
            Self::Core(local) => EvaluatedFunctionFunction::Core(
                environment
                    .values
                    .core_function_functions
                    .swap_remove(local.id().0),
            ),
            Self::External(local) => EvaluatedFunctionFunction::External(
                environment
                    .values
                    .external_function_functions
                    .swap_remove(local.id().0),
            ),
        }
    }
}

impl GraphValue for FunctionLocal {
    type Evaluated = EvaluatedFunctionValue;

    fn take(&self, environment: BlockEnvironment) -> Self::Evaluated {
        match self {
            Self::Generic(local) => local.take(environment).into(),
            Self::Never(local) => local.take(environment).into(),
            Self::Int(local) => local.take(environment).into(),
            Self::Float(local) => local.take(environment).into(),
            Self::String(local) => local.take(environment).into(),
            Self::BitArray(local) => local.take(environment).into(),
            Self::UtfCodepoint(local) => local.take(environment).into(),
            Self::Custom(local) => local.take(environment).into(),
            Self::External(local) => local.take(environment).into(),
            Self::Bool(local) => local.take(environment).into(),
            Self::Nil(local) => local.take(environment).into(),
            Self::Tuple(local) => local.take(environment).into(),
            Self::List(local) => local.take(environment).into(),
            Self::Function(local) => local.take(environment).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Value;

    #[test]
    fn owned_returns_preserve_every_plain_value_and_callable_family() {
        let declarations = r#"
pub type Box { Box(Int) }
fn identity(value) { value }
fn stop(_value: Int) -> a { panic }
fn empty() -> List(a) { [] }
fn nested_empty() -> List(List(a)) { [[]] }
fn codepoint() {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}
fn integer() { 42 }
fn decimal() { 1.5 }
fn text() { "text" }
fn bits() { <<42>> }
fn boxed() { Box(42) }
fn flag() { True }
fn nil() { Nil }
fn pair() { #(42, True) }
fn integers() { [42] }
fn decimals() { [1.5] }
fn texts() { ["text"] }
fn arrays() { [<<42>>] }
fn boxes() { [Box(42)] }
fn flags() { [True] }
fn nils() { [Nil] }
fn pairs() { [#(42, True)] }
fn nested() { [[42]] }
fn codepoints() { [codepoint()] }
fn callbacks() { [integer] }
fn callback() { integer }
fn select(flag, first, second) {
  case flag { True -> second False -> first }
}
fn retained(value) { identity(value) }
"#;
        for expression in [
            "42",
            "1.5",
            "\"text\"",
            "<<42>>",
            "codepoint()",
            "Box(42)",
            "True",
            "Nil",
            "#(42, True)",
            "empty()",
            "nested_empty()",
            "[42]",
            "[1.5]",
            "[\"text\"]",
            "[<<42>>]",
            "[codepoint()]",
            "[Box(42)]",
            "[True]",
            "[Nil]",
            "[#(42, True)]",
            "[[42]]",
            "[integer]",
            "integer",
            "decimal",
            "text",
            "bits",
            "codepoint",
            "boxed",
            "flag",
            "nil",
            "pair",
            "empty",
            "nested_empty",
            "integers",
            "decimals",
            "texts",
            "arrays",
            "codepoints",
            "boxes",
            "flags",
            "nils",
            "pairs",
            "nested",
            "callbacks",
            "callback",
            "identity",
            "stop",
        ] {
            let source = format!(
                r#"
{declarations}
pub fn main() {{
  let original = {expression}
  let returned = retained(original)
  let selected = select(True, original, returned)
  #(returned == original, selected == original)
}}
"#
            );
            assert_eq!(
                crate::runtime::run_src(&source),
                Value::Tuple(vec![Value::Bool(true), Value::Bool(true)]),
                "{expression}",
            );
        }
    }

    #[test]
    fn owned_external_returns_preserve_payloads_lists_and_callable_identity() {
        use crate::host::{ExternalTestProfile, ExternalTestRunState};
        use crate::runtime::profile::external_test::{
            RuntimeCounterProvider, RuntimeCounterSchema, RuntimeHostCounter,
        };
        use crate::{HostCall, HostCallCompletion, HostCallError, HostModule, HostProviderModule};
        use num_bigint::BigInt;

        fn counter<'call>(
            mut call: HostCall<
                'call,
                ExternalTestProfile,
                RuntimeCounterProvider,
                RuntimeHostCounter,
            >,
            value: BigInt,
        ) -> Result<HostCallCompletion<'call, RuntimeHostCounter>, HostCallError> {
            let value = call.create_external(value);
            Ok(call.return_value(value))
        }

        let provider = HostProviderModule::<ExternalTestProfile>::new("application", "main")
            .unwrap()
            .with_external_type::<RuntimeCounterProvider, RuntimeCounterSchema>()
            .unwrap()
            .with_scoped_function::<RuntimeCounterProvider, (BigInt,), RuntimeHostCounter, _>(
                "counter", counter,
            )
            .unwrap();
        let source = r#"
@external(erlang, "host", "Counter")
pub type Counter
@external(erlang, "host", "counter")
fn counter(value: Int) -> Counter
fn identity(value) { value }
fn retained(value) { identity(value) }
fn select(flag, first, second) {
  case flag { True -> second False -> first }
}
fn counters() { [counter(10), counter(20)] }
fn callback() { counter }
pub fn main() {
  let value = counter(42)
  let values = counters()
  let make = retained(counter)
  let make_list = retained(counters)
  let make_function = retained(callback)
  #(
    select(True, value, retained(value)) == counter(42),
    select(True, values, retained(values)) == [counter(10), counter(20)],
    select(True, counter, make)(42) == value,
    select(True, counters, make_list)() == values,
    select(True, callback, make_function)()(42) == value,
    make == counter,
    make_list == counters,
    make_function == callback,
  )
}
"#;
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [crate::PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [crate::ModuleSource::new("main", "main.gleam", source)],
            )],
            crate::HostProviderSet::with_providers(
                Vec::<HostModule<ExternalTestProfile>>::new(),
                [provider],
            )
            .unwrap(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(
                &mut execution,
                &mut ExternalTestRunState::default(),
                &mut echo,
            ),
            Ok(Value::Tuple(vec![Value::Bool(true); 8])),
        );
        assert!(echo.is_empty());
    }
}
