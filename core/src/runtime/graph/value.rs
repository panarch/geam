use super::environment::BlockEnvironment;
use crate::runtime::evaluated::{EvaluatedFunctionValue, EvaluatedValue};
use crate::runtime::{LocalValues, RuntimeValueProfile};
use ecow::EcoString;
use num_bigint::BigInt;
use std::convert::Infallible;

pub(in crate::runtime) trait GraphValue<Profile: RuntimeValueProfile = LocalValues> {
    type Evaluated;

    fn read(&self, environment: &BlockEnvironment<Profile>) -> Self::Evaluated;
}

impl<Profile: RuntimeValueProfile> GraphValue<Profile> for Infallible {
    type Evaluated = Infallible;

    fn read(&self, _environment: &BlockEnvironment<Profile>) -> Self::Evaluated {
        match *self {}
    }
}

impl<Profile: RuntimeValueProfile> GraphValue<Profile>
    for crate::plan::execution::graph::IntLocalId
{
    type Evaluated = BigInt;

    fn read(&self, environment: &BlockEnvironment<Profile>) -> Self::Evaluated {
        environment.int(*self)
    }
}

impl<Profile: RuntimeValueProfile> GraphValue<Profile>
    for crate::plan::execution::graph::FloatLocalId
{
    type Evaluated = f64;

    fn read(&self, environment: &BlockEnvironment<Profile>) -> Self::Evaluated {
        environment.float(*self)
    }
}

impl<Profile: RuntimeValueProfile> GraphValue<Profile>
    for crate::plan::execution::graph::StringLocalId
{
    type Evaluated = EcoString;

    fn read(&self, environment: &BlockEnvironment<Profile>) -> Self::Evaluated {
        environment.string(*self)
    }
}

impl<Profile: RuntimeValueProfile> GraphValue<Profile>
    for crate::plan::execution::graph::BitArrayLocalId
{
    type Evaluated = crate::runtime::EvaluatedBitArray;

    fn read(&self, environment: &BlockEnvironment<Profile>) -> Self::Evaluated {
        environment.bit_array(*self)
    }
}

impl<Profile: RuntimeValueProfile> GraphValue<Profile>
    for crate::plan::execution::graph::UtfCodepointLocalId
{
    type Evaluated = char;

    fn read(&self, environment: &BlockEnvironment<Profile>) -> Self::Evaluated {
        environment.utf_codepoint(*self)
    }
}

impl<Profile: RuntimeValueProfile> GraphValue<Profile>
    for crate::plan::execution::graph::CustomLocal
{
    type Evaluated = crate::runtime::EvaluatedCustomValue<Profile>;

    fn read(&self, environment: &BlockEnvironment<Profile>) -> Self::Evaluated {
        environment.custom(*self)
    }
}

impl<Profile: RuntimeValueProfile> GraphValue<Profile>
    for crate::plan::execution::graph::ExternalLocal
{
    type Evaluated = crate::runtime::EvaluatedExternalValue<Profile>;

    fn read(&self, environment: &BlockEnvironment<Profile>) -> Self::Evaluated {
        environment.external(*self)
    }
}

impl<Profile: RuntimeValueProfile> GraphValue<Profile>
    for crate::plan::execution::graph::BoolLocalId
{
    type Evaluated = bool;

    fn read(&self, environment: &BlockEnvironment<Profile>) -> Self::Evaluated {
        environment.bool(*self)
    }
}

impl<Profile: RuntimeValueProfile> GraphValue<Profile>
    for crate::plan::execution::graph::NilLocalId
{
    type Evaluated = ();

    fn read(&self, environment: &BlockEnvironment<Profile>) -> Self::Evaluated {
        environment.nil(*self)
    }
}

impl<Profile: RuntimeValueProfile> GraphValue<Profile>
    for crate::plan::execution::graph::TupleLocalId
{
    type Evaluated = Vec<EvaluatedValue<Profile>>;

    fn read(&self, environment: &BlockEnvironment<Profile>) -> Self::Evaluated {
        environment.tuple(*self)
    }
}

macro_rules! list_graph_value {
    ($local:ty, $value:ty, $method:ident) => {
        impl<Profile: RuntimeValueProfile> GraphValue<Profile> for $local {
            type Evaluated = $value;

            fn read(&self, environment: &BlockEnvironment<Profile>) -> Self::Evaluated {
                environment.$method(*self)
            }
        }
    };
}

list_graph_value!(
    crate::plan::execution::graph::ParameterListLocalId,
    crate::runtime::state::list::ParameterListValueId<Profile>,
    parameter_list
);
list_graph_value!(
    crate::plan::execution::graph::IntListLocalId,
    crate::runtime::state::list::IntListValueId<Profile>,
    int_list
);
list_graph_value!(
    crate::plan::execution::graph::StringListLocalId,
    crate::runtime::state::list::StringListValueId<Profile>,
    string_list
);
list_graph_value!(
    crate::plan::execution::graph::BitArrayListLocalId,
    crate::runtime::state::list::BitArrayListValueId<Profile>,
    bit_array_list
);
list_graph_value!(
    crate::plan::execution::graph::UtfCodepointListLocalId,
    crate::runtime::state::list::UtfCodepointListValueId<Profile>,
    utf_codepoint_list
);
list_graph_value!(
    crate::plan::execution::graph::CustomListLocalId,
    crate::runtime::state::list::CustomListValueId<Profile>,
    custom_list
);
list_graph_value!(
    crate::plan::execution::graph::ExternalListLocalId,
    crate::runtime::state::list::ExternalListValueId<Profile>,
    external_list
);
list_graph_value!(
    crate::plan::execution::graph::FloatListLocalId,
    crate::runtime::state::list::FloatListValueId<Profile>,
    float_list
);
list_graph_value!(
    crate::plan::execution::graph::BoolListLocalId,
    crate::runtime::state::list::BoolListValueId<Profile>,
    bool_list
);
list_graph_value!(
    crate::plan::execution::graph::NilListLocalId,
    crate::runtime::state::list::NilListValueId<Profile>,
    nil_list
);
list_graph_value!(
    crate::plan::execution::graph::TupleListLocalId,
    crate::runtime::state::list::TupleListValueId<Profile>,
    tuple_list
);
list_graph_value!(
    crate::plan::execution::graph::ParameterListListLocalId,
    crate::runtime::state::list::ParameterListListValueId<Profile>,
    parameter_list_list
);
list_graph_value!(
    crate::plan::execution::graph::ListListLocalId,
    crate::runtime::state::list::ListListValueId<Profile>,
    list_list
);
list_graph_value!(
    crate::plan::execution::graph::FunctionListLocalId,
    crate::runtime::state::list::FunctionListValueId<Profile>,
    function_list
);

macro_rules! function_graph_value {
    ($local:ty, $value:ty, $method:ident) => {
        impl<Profile: RuntimeValueProfile> GraphValue<Profile> for $local {
            type Evaluated = $value;

            fn read(&self, environment: &BlockEnvironment<Profile>) -> Self::Evaluated {
                environment.$method(self.clone())
            }
        }
    };
}

function_graph_value!(
    crate::plan::execution::graph::IntFunctionLocalId,
    crate::runtime::EvaluatedIntFunction<Profile>,
    int_function
);
function_graph_value!(
    crate::plan::execution::graph::FloatFunctionLocalId,
    crate::runtime::EvaluatedFloatFunction<Profile>,
    float_function
);
function_graph_value!(
    crate::plan::execution::graph::StringFunctionLocalId,
    crate::runtime::EvaluatedStringFunction<Profile>,
    string_function
);
function_graph_value!(
    crate::plan::execution::graph::BitArrayFunctionLocalId,
    crate::runtime::EvaluatedBitArrayFunction<Profile>,
    bit_array_function
);
function_graph_value!(
    crate::plan::execution::graph::UtfCodepointFunctionLocalId,
    crate::runtime::EvaluatedUtfCodepointFunction<Profile>,
    utf_codepoint_function
);
function_graph_value!(
    crate::plan::execution::graph::BoolFunctionLocalId,
    crate::runtime::EvaluatedBoolFunction<Profile>,
    bool_function
);
function_graph_value!(
    crate::plan::execution::graph::NilFunctionLocalId,
    crate::runtime::EvaluatedNilFunction<Profile>,
    nil_function
);
function_graph_value!(
    crate::plan::execution::graph::TupleFunctionLocalId,
    crate::runtime::EvaluatedTupleFunction<Profile>,
    tuple_function
);

impl<Profile: RuntimeValueProfile> GraphValue<Profile>
    for crate::plan::execution::graph::GenericFunctionLocal
{
    type Evaluated = crate::runtime::EvaluatedGenericFunction<Profile>;

    fn read(&self, environment: &BlockEnvironment<Profile>) -> Self::Evaluated {
        environment.generic_function(self)
    }
}

impl<Profile: RuntimeValueProfile> GraphValue<Profile>
    for crate::plan::execution::graph::NeverFunctionLocal
{
    type Evaluated = crate::runtime::EvaluatedNeverFunction<Profile>;

    fn read(&self, environment: &BlockEnvironment<Profile>) -> Self::Evaluated {
        environment.never_function(self)
    }
}

impl<Profile: RuntimeValueProfile> GraphValue<Profile>
    for crate::plan::execution::graph::CustomFunctionLocal
{
    type Evaluated = crate::runtime::EvaluatedCustomFunction<Profile>;

    fn read(&self, environment: &BlockEnvironment<Profile>) -> Self::Evaluated {
        environment.custom_function(self)
    }
}

impl<Profile: RuntimeValueProfile> GraphValue<Profile>
    for crate::plan::execution::graph::ExternalFunctionLocal
{
    type Evaluated = crate::runtime::EvaluatedExternalFunction<Profile>;

    fn read(&self, environment: &BlockEnvironment<Profile>) -> Self::Evaluated {
        environment.external_function(self)
    }
}

impl<Profile: RuntimeValueProfile> GraphValue<Profile>
    for crate::plan::execution::graph::ListFunctionLocal
{
    type Evaluated = crate::runtime::EvaluatedListFunction<Profile>;

    fn read(&self, environment: &BlockEnvironment<Profile>) -> Self::Evaluated {
        environment.list_function(self)
    }
}

impl<Profile: RuntimeValueProfile> GraphValue<Profile>
    for crate::plan::execution::graph::FunctionFunctionLocal
{
    type Evaluated = crate::runtime::EvaluatedFunctionFunction<Profile>;

    fn read(&self, environment: &BlockEnvironment<Profile>) -> Self::Evaluated {
        environment.function_function(self)
    }
}

impl<Profile: RuntimeValueProfile> GraphValue<Profile>
    for crate::plan::execution::graph::FunctionLocal
{
    type Evaluated = EvaluatedFunctionValue<Profile>;

    fn read(&self, environment: &BlockEnvironment<Profile>) -> Self::Evaluated {
        match self {
            Self::Generic(local) => environment.generic_function(local).into(),
            Self::Never(local) => environment.never_function(local).into(),
            Self::Int(local) => environment.int_function(*local).into(),
            Self::Float(local) => environment.float_function(*local).into(),
            Self::String(local) => environment.string_function(*local).into(),
            Self::BitArray(local) => environment.bit_array_function(*local).into(),
            Self::UtfCodepoint(local) => environment.utf_codepoint_function(*local).into(),
            Self::Custom(local) => environment.custom_function(local).into(),
            Self::External(local) => environment.external_function(local).into(),
            Self::Bool(local) => environment.bool_function(*local).into(),
            Self::Nil(local) => environment.nil_function(*local).into(),
            Self::Tuple(local) => environment.tuple_function(*local).into(),
            Self::List(local) => environment.list_function(local).into(),
            Self::Function(local) => environment.function_function(local).into(),
        }
    }
}
