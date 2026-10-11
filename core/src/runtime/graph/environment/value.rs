use super::BlockEnvironment;
use crate::StringValue;
use crate::plan::execution::function::FunctionReturnFamily;
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
use crate::runtime::compiled::calls::{
    BitArrayCallable, BoolCallable, CallBitArray, CallInteger, CallOutput, CallTuple,
    FloatCallable, IntCallable, NilCallable, StringCallable, UtfCodepointCallable,
};
use crate::runtime::error::{ExecutionResult, InvariantError};
use crate::runtime::evaluated::{
    EvaluatedBitArray, EvaluatedCustomValue, EvaluatedExternalValue, EvaluatedFunctionValue,
    EvaluatedValue,
};
use crate::runtime::integer::IntegerValue;
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
use std::convert::Infallible;

pub(in crate::runtime) trait GraphValue: Sync {
    type Evaluated: Send + 'static;
    const RETURN_FAMILY: FunctionReturnFamily;

    fn from_call_output(output: CallOutput) -> ExecutionResult<Self::Evaluated> {
        Err(InvariantError::FunctionReturnFamilyMismatch {
            expected: Self::RETURN_FAMILY,
            actual: output.family(),
        }
        .into())
    }

    fn take(&self, environment: &mut BlockEnvironment) -> Self::Evaluated;
}

impl GraphValue for Infallible {
    type Evaluated = Infallible;
    const RETURN_FAMILY: FunctionReturnFamily = FunctionReturnFamily::Never;

    fn take(&self, _environment: &mut BlockEnvironment) -> Self::Evaluated {
        match *self {}
    }
}

impl GraphValue for NilLocalId {
    type Evaluated = ();
    const RETURN_FAMILY: FunctionReturnFamily = FunctionReturnFamily::Nil;

    fn from_call_output(output: CallOutput) -> ExecutionResult<()> {
        match output {
            CallOutput::Nil(value) => Ok(value),
            output => Err(InvariantError::FunctionReturnFamilyMismatch {
                expected: Self::RETURN_FAMILY,
                actual: output.family(),
            }
            .into()),
        }
    }

    fn take(&self, _environment: &mut BlockEnvironment) {}
}

macro_rules! local_value {
    ($local:ty, $value:ty, $field:ident, $family:ident) => {
        impl GraphValue for $local {
            type Evaluated = $value;
            const RETURN_FAMILY: FunctionReturnFamily = FunctionReturnFamily::$family;

            fn take(&self, environment: &mut BlockEnvironment) -> Self::Evaluated {
                environment.values.$field.swap_remove(self.0)
            }
        }
    };
}

// Admission seals the family. Generated completion decodes the actual result;
// ordinary operands and canonical returns keep their original local columns.
macro_rules! call_return {
    ($local:ty, $value:ty, $field:ident, $family:ident, $variant:ident, $decode:expr) => {
        impl GraphValue for $local {
            type Evaluated = $value;
            const RETURN_FAMILY: FunctionReturnFamily = FunctionReturnFamily::$family;

            fn from_call_output(output: CallOutput) -> ExecutionResult<Self::Evaluated> {
                match output {
                    CallOutput::$variant(value) => Ok(($decode)(value)),
                    output => Err(InvariantError::FunctionReturnFamilyMismatch {
                        expected: Self::RETURN_FAMILY,
                        actual: output.family(),
                    }
                    .into()),
                }
            }

            fn take(&self, environment: &mut BlockEnvironment) -> Self::Evaluated {
                environment.values.$field.swap_remove(self.0)
            }
        }
    };
}

call_return!(
    IntLocalId,
    IntegerValue,
    ints,
    Int,
    Int,
    |value: CallInteger| value.0
);
call_return!(FloatLocalId, f64, floats, Float, Float, |value| value);
call_return!(
    StringLocalId,
    StringValue,
    strings,
    String,
    String,
    |value| value
);
call_return!(
    BitArrayLocalId,
    EvaluatedBitArray,
    bit_arrays,
    BitArray,
    BitArray,
    |value: CallBitArray| value.0
);
call_return!(
    UtfCodepointLocalId,
    char,
    utf_codepoints,
    UtfCodepoint,
    UtfCodepoint,
    |value| value
);
call_return!(BoolLocalId, bool, bools, Bool, Bool, |value| value);
call_return!(
    TupleLocalId,
    Vec<EvaluatedValue>,
    tuples,
    Tuple,
    Tuple,
    |value: CallTuple| value.0
);
local_value!(
    ParameterListLocalId,
    ParameterListValueId,
    parameter_lists,
    List
);
local_value!(
    ParameterListListLocalId,
    ParameterListListValueId,
    parameter_list_lists,
    List
);
local_value!(IntListLocalId, IntListValueId, int_lists, List);
local_value!(StringListLocalId, StringListValueId, string_lists, List);
local_value!(
    BitArrayListLocalId,
    BitArrayListValueId,
    bit_array_lists,
    List
);
local_value!(
    UtfCodepointListLocalId,
    UtfCodepointListValueId,
    utf_codepoint_lists,
    List
);
local_value!(CustomListLocalId, CustomListValueId, custom_lists, List);
local_value!(
    ExternalListLocalId,
    ExternalListValueId,
    external_lists,
    List
);
local_value!(FloatListLocalId, FloatListValueId, float_lists, List);
local_value!(BoolListLocalId, BoolListValueId, bool_lists, List);
local_value!(NilListLocalId, NilListValueId, nil_lists, List);
local_value!(TupleListLocalId, TupleListValueId, tuple_lists, List);
local_value!(ListListLocalId, ListListValueId, list_lists, List);
local_value!(
    FunctionListLocalId,
    FunctionListValueId,
    function_lists,
    List
);
call_return!(
    IntFunctionLocalId,
    EvaluatedIntFunction,
    int_functions,
    Function,
    IntFunction,
    |value: IntCallable| value.0
);
call_return!(
    FloatFunctionLocalId,
    EvaluatedFloatFunction,
    float_functions,
    Function,
    FloatFunction,
    |value: FloatCallable| value.0
);
call_return!(
    StringFunctionLocalId,
    EvaluatedStringFunction,
    string_functions,
    Function,
    StringFunction,
    |value: StringCallable| value.0
);
call_return!(
    BitArrayFunctionLocalId,
    EvaluatedBitArrayFunction,
    bit_array_functions,
    Function,
    BitArrayFunction,
    |value: BitArrayCallable| value.0
);
call_return!(
    UtfCodepointFunctionLocalId,
    EvaluatedUtfCodepointFunction,
    utf_codepoint_functions,
    Function,
    UtfCodepointFunction,
    |value: UtfCodepointCallable| value.0
);
call_return!(
    BoolFunctionLocalId,
    EvaluatedBoolFunction,
    bool_functions,
    Function,
    BoolFunction,
    |value: BoolCallable| value.0
);
call_return!(
    NilFunctionLocalId,
    EvaluatedNilFunction,
    nil_functions,
    Function,
    NilFunction,
    |value: NilCallable| value.0
);
local_value!(
    TupleFunctionLocalId,
    EvaluatedTupleFunction,
    tuple_functions,
    Function
);

impl GraphValue for CustomLocal {
    fn from_call_output(output: CallOutput) -> ExecutionResult<Self::Evaluated> {
        match output {
            CallOutput::Custom(value) => Ok(value.0),
            output => Err(InvariantError::FunctionReturnFamilyMismatch {
                expected: Self::RETURN_FAMILY,
                actual: output.family(),
            }
            .into()),
        }
    }
    type Evaluated = EvaluatedCustomValue;
    const RETURN_FAMILY: FunctionReturnFamily = FunctionReturnFamily::Custom;

    fn take(&self, environment: &mut BlockEnvironment) -> Self::Evaluated {
        environment.values.customs.swap_remove(self.id().0)
    }
}

impl GraphValue for ExternalLocal {
    type Evaluated = EvaluatedExternalValue;
    const RETURN_FAMILY: FunctionReturnFamily = FunctionReturnFamily::External;

    fn take(&self, environment: &mut BlockEnvironment) -> Self::Evaluated {
        environment.values.externals.swap_remove(self.id().0)
    }
}

impl GraphValue for CustomFunctionLocal {
    type Evaluated = EvaluatedCustomFunction;
    const RETURN_FAMILY: FunctionReturnFamily = FunctionReturnFamily::Function;

    fn take(&self, environment: &mut BlockEnvironment) -> Self::Evaluated {
        environment.values.custom_functions.swap_remove(self.id().0)
    }
}

impl GraphValue for ExternalFunctionLocal {
    type Evaluated = EvaluatedExternalFunction;
    const RETURN_FAMILY: FunctionReturnFamily = FunctionReturnFamily::Function;

    fn take(&self, environment: &mut BlockEnvironment) -> Self::Evaluated {
        environment
            .values
            .external_functions
            .swap_remove(self.id().0)
    }
}

impl GraphValue for GenericFunctionLocal {
    type Evaluated = EvaluatedGenericFunction;
    const RETURN_FAMILY: FunctionReturnFamily = FunctionReturnFamily::Function;

    fn take(&self, environment: &mut BlockEnvironment) -> Self::Evaluated {
        environment
            .values
            .generic_functions
            .swap_remove(self.id().0)
    }
}

impl GraphValue for NeverFunctionLocal {
    type Evaluated = EvaluatedNeverFunction;
    const RETURN_FAMILY: FunctionReturnFamily = FunctionReturnFamily::Function;

    fn take(&self, environment: &mut BlockEnvironment) -> Self::Evaluated {
        environment.values.never_functions.swap_remove(self.id().0)
    }
}

impl GraphValue for ListFunctionLocal {
    type Evaluated = EvaluatedListFunction;
    const RETURN_FAMILY: FunctionReturnFamily = FunctionReturnFamily::Function;

    fn take(&self, environment: &mut BlockEnvironment) -> Self::Evaluated {
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
    const RETURN_FAMILY: FunctionReturnFamily = FunctionReturnFamily::Function;

    fn take(&self, environment: &mut BlockEnvironment) -> Self::Evaluated {
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
    const RETURN_FAMILY: FunctionReturnFamily = FunctionReturnFamily::Function;

    fn take(&self, environment: &mut BlockEnvironment) -> Self::Evaluated {
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
    use super::super::{BlockEnvironment, RetainedValues};
    use super::GraphValue;
    use crate::plan::execution::function::{
        BitArrayFunctionId, BoolFunctionId, FloatFunctionId, FunctionReturnFamily, IntFunctionId,
        NilFunctionId, StringFunctionId, UtfCodepointFunctionId,
    };
    use crate::plan::execution::graph::{
        BitArrayFunctionLocalId, BitArrayLocalId, BoolFunctionLocalId, BoolLocalId, CustomLocal,
        FloatFunctionLocalId, FloatLocalId, IntFunctionLocalId, IntLocalId, NilFunctionLocalId,
        NilLocalId, ParameterListLocalId, StringFunctionLocalId, StringLocalId, TupleLocalId,
        UtfCodepointFunctionLocalId, UtfCodepointLocalId,
    };
    use crate::plan::execution::type_::{FunctionType, ValueType};
    use crate::runtime::compiled::calls::{
        CallBitArray, CallCapture, CallInteger, CallOps, CallOutput,
    };
    use crate::runtime::compiled::numeric::NumericValues;
    use crate::runtime::error::{ExecutionError, InvariantError};
    use crate::runtime::integer::IntegerValue;
    use crate::runtime::state::list::RuntimeListStorage;
    use crate::runtime::{CaptureStorage, EvaluatedCustomValue, EvaluatedValue};
    use crate::{BitArrayValue, StringValue, Value};

    #[test]
    fn typed_extraction_moves_the_result_and_leaves_environment_cleanup_to_its_owner() {
        let tuple = vec![EvaluatedValue::Int(42.into())];
        let tuple_buffer = tuple.as_ptr();
        let mut retained = RetainedValues::empty();
        retained.push_tuple(tuple);
        retained.push_tuple(vec![EvaluatedValue::Bool(true)]);
        retained.push_int(7.into());
        let mut environment = BlockEnvironment::from_retained(retained);
        let tuple_capacity = environment.values.tuples.capacity();
        let returned = TupleLocalId(0).take(&mut environment);
        assert_eq!(returned, vec![EvaluatedValue::Int(42.into())]);
        assert_eq!(returned.as_ptr(), tuple_buffer);
        assert_eq!(environment.values.tuples.capacity(), tuple_capacity);
        assert_eq!(
            environment.values.tuples,
            vec![vec![EvaluatedValue::Bool(true)]]
        );
        assert_eq!(environment.int(IntLocalId(0)), num_bigint::BigInt::from(7));
        NilLocalId(0).take(&mut environment);
        assert_eq!(
            environment.values.tuples,
            vec![vec![EvaluatedValue::Bool(true)]]
        );
        drop(environment);
        assert_eq!(returned, vec![EvaluatedValue::Int(42.into())]);
    }

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

    #[test]
    fn generated_returns_consume_only_the_result_in_every_supported_family() {
        let integer = IntegerValue::from(1_i128 << 100);
        assert_eq!(
            IntLocalId::from_call_output(CallOutput::Int(CallInteger(integer.clone()))).unwrap(),
            integer
        );
        assert!(BoolLocalId::from_call_output(CallOutput::Bool(true)).unwrap());
        assert_eq!(
            FloatLocalId::from_call_output(CallOutput::Float(-0.0))
                .unwrap()
                .to_bits(),
            (-0.0_f64).to_bits()
        );
        let string =
            StringValue::from("actual generated result with an independently owned long backing");
        let string_owner = string.as_bytes().as_ptr();
        let returned = StringLocalId::from_call_output(CallOutput::String(string)).unwrap();
        assert_eq!(
            returned.as_bytes(),
            b"actual generated result with an independently owned long backing"
        );
        assert_eq!(returned.as_bytes().as_ptr(), string_owner);
        let bit_array =
            CallBitArray::from(BitArrayValue::from_bytes(vec![0xa5, 0x60]).slice_in_bounds(2, 11));
        let bit_alias = bit_array.0.clone();
        assert_eq!(
            BitArrayLocalId::from_call_output(CallOutput::BitArray(bit_array)).unwrap(),
            bit_alias
        );
        assert_eq!(
            UtfCodepointLocalId::from_call_output(CallOutput::UtfCodepoint('λ')).unwrap(),
            'λ'
        );
        NilLocalId::from_call_output(CallOutput::Nil(())).unwrap();

        let captures = CaptureStorage::default();
        let mut numeric = NumericValues::default();
        let mut string_scratch = None;
        let mut bit_scratch = None;
        let lists = RuntimeListStorage::default();
        let ops = CallOps::new(
            &captures,
            &mut numeric,
            &lists,
            &mut string_scratch,
            &mut bit_scratch,
        );
        let integer_function = ops.int_closure(
            IntFunctionId(2),
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
            vec![CallCapture::int(IntLocalId(0), 7)],
        );
        let alias = integer_function.clone();
        let returned =
            IntFunctionLocalId::from_call_output(CallOutput::IntFunction(integer_function))
                .unwrap();
        assert_eq!(returned, alias.0);
        let boolean_function = ops.bool_closure(
            BoolFunctionId(3),
            FunctionType::new(vec![ValueType::Bool], ValueType::Bool),
            vec![CallCapture::bool(BoolLocalId(0), false)],
        );
        let alias = boolean_function.clone();
        assert_eq!(
            BoolFunctionLocalId::from_call_output(CallOutput::BoolFunction(boolean_function))
                .unwrap(),
            alias.0
        );
        let float_function = ops.float_closure(
            FloatFunctionId(4),
            FunctionType::new(vec![ValueType::Float], ValueType::Float),
            vec![CallCapture::float(FloatLocalId(0), -0.0)],
        );
        let alias = float_function.clone();
        assert_eq!(
            FloatFunctionLocalId::from_call_output(CallOutput::FloatFunction(float_function))
                .unwrap(),
            alias.0
        );
        let string_function = ops.string_closure(
            StringFunctionId(5),
            FunctionType::new(vec![ValueType::String], ValueType::String),
            vec![CallCapture::string(
                StringLocalId(0),
                StringValue::from("capture"),
            )],
        );
        let alias = string_function.clone();
        assert_eq!(
            StringFunctionLocalId::from_call_output(CallOutput::StringFunction(string_function))
                .unwrap(),
            alias.0
        );
        let bit_function = ops.bit_array_closure(
            BitArrayFunctionId(6),
            FunctionType::new(vec![ValueType::BitArray], ValueType::BitArray),
            vec![CallCapture::bit_array(
                BitArrayLocalId(0),
                CallBitArray::from(BitArrayValue::from_bytes(vec![0x42])),
            )],
        );
        let alias = bit_function.clone();
        assert_eq!(
            BitArrayFunctionLocalId::from_call_output(CallOutput::BitArrayFunction(bit_function))
                .unwrap(),
            alias.0
        );
        let codepoint_function = ops.utf_codepoint_closure(
            UtfCodepointFunctionId(7),
            FunctionType::new(vec![ValueType::UtfCodepoint], ValueType::UtfCodepoint),
            vec![CallCapture::utf_codepoint(UtfCodepointLocalId(0), 'λ')],
        );
        let alias = codepoint_function.clone();
        assert_eq!(
            UtfCodepointFunctionLocalId::from_call_output(CallOutput::UtfCodepointFunction(
                codepoint_function
            ))
            .unwrap(),
            alias.0
        );
        let nil_function = ops.nil_closure(
            NilFunctionId(8),
            FunctionType::new(vec![ValueType::Nil], ValueType::Nil),
            Vec::new(),
        );
        let alias = nil_function.clone();
        assert_eq!(
            NilFunctionLocalId::from_call_output(CallOutput::NilFunction(nil_function)).unwrap(),
            alias.0
        );
    }

    #[test]
    fn generated_completion_rejects_mismatched_and_unsupported_return_destinations() {
        macro_rules! wrong_family {
            ($local:ty, $family:ident) => {
                assert_eq!(
                    <$local>::from_call_output(CallOutput::Int(CallInteger(42.into())))
                        .unwrap_err(),
                    ExecutionError::from(InvariantError::FunctionReturnFamilyMismatch {
                        expected: FunctionReturnFamily::$family,
                        actual: FunctionReturnFamily::Int,
                    })
                );
            };
        }
        wrong_family!(BoolLocalId, Bool);
        wrong_family!(FloatLocalId, Float);
        wrong_family!(StringLocalId, String);
        wrong_family!(BitArrayLocalId, BitArray);
        wrong_family!(UtfCodepointLocalId, UtfCodepoint);
        wrong_family!(NilLocalId, Nil);
        wrong_family!(IntFunctionLocalId, Function);
        wrong_family!(BoolFunctionLocalId, Function);
        wrong_family!(FloatFunctionLocalId, Function);
        wrong_family!(StringFunctionLocalId, Function);
        wrong_family!(BitArrayFunctionLocalId, Function);
        wrong_family!(UtfCodepointFunctionLocalId, Function);
        wrong_family!(NilFunctionLocalId, Function);
        wrong_family!(TupleLocalId, Tuple);
        wrong_family!(CustomLocal, Custom);
        wrong_family!(ParameterListLocalId, List);
        assert_eq!(
            IntLocalId::from_call_output(CallOutput::Bool(true)).unwrap_err(),
            ExecutionError::from(InvariantError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::Int,
                actual: FunctionReturnFamily::Bool,
            })
        );
    }

    #[test]
    fn generated_custom_completion_retains_the_exact_constructor_and_owned_fields() {
        use crate::plan::execution::type_::{CustomConstructorId, CustomTypeId};
        use crate::runtime::compiled::calls::CallCustom;

        let original = EvaluatedCustomValue::from_fields(
            CustomConstructorId {
                type_id: CustomTypeId(7),
                index: 2,
            },
            vec![
                EvaluatedValue::String("kept".into()),
                EvaluatedValue::Int(42.into()),
            ]
            .into_boxed_slice(),
        );
        let alias = original.clone();
        let actual =
            CustomLocal::from_call_output(CallOutput::Custom(CallCustom(original))).unwrap();
        assert_eq!(actual, alias);
        assert_eq!(actual.fields().as_ptr(), alias.fields().as_ptr());
    }
}
