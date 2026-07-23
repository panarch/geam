use super::CompletedGraph;
use crate::runtime::evaluated::{EvaluatedFunctionValue, EvaluatedValue};
use ecow::EcoString;
use num_bigint::BigInt;
use std::convert::Infallible;

pub(in crate::runtime) trait GraphValue {
    type Evaluated;

    fn read(&self, completed: &CompletedGraph) -> Self::Evaluated;
}

impl GraphValue for Infallible {
    type Evaluated = Infallible;

    fn read(&self, _completed: &CompletedGraph) -> Self::Evaluated {
        match *self {}
    }
}

impl GraphValue for crate::plan::execution::graph::IntLocalId {
    type Evaluated = BigInt;

    fn read(&self, completed: &CompletedGraph) -> Self::Evaluated {
        completed.environment.int(*self)
    }
}

impl GraphValue for crate::plan::execution::graph::FloatLocalId {
    type Evaluated = f64;

    fn read(&self, completed: &CompletedGraph) -> Self::Evaluated {
        completed.environment.float(*self)
    }
}

impl GraphValue for crate::plan::execution::graph::StringLocalId {
    type Evaluated = EcoString;

    fn read(&self, completed: &CompletedGraph) -> Self::Evaluated {
        completed.environment.string(*self)
    }
}

impl GraphValue for crate::plan::execution::graph::BitArrayLocalId {
    type Evaluated = crate::runtime::EvaluatedBitArray;

    fn read(&self, completed: &CompletedGraph) -> Self::Evaluated {
        completed.environment.bit_array(*self)
    }
}

impl GraphValue for crate::plan::execution::graph::UtfCodepointLocalId {
    type Evaluated = char;

    fn read(&self, completed: &CompletedGraph) -> Self::Evaluated {
        completed.environment.utf_codepoint(*self)
    }
}

impl GraphValue for crate::plan::execution::graph::CustomLocal {
    type Evaluated = crate::runtime::EvaluatedCustomValue;

    fn read(&self, completed: &CompletedGraph) -> Self::Evaluated {
        completed.environment.custom(*self)
    }
}

impl GraphValue for crate::plan::execution::graph::BoolLocalId {
    type Evaluated = bool;

    fn read(&self, completed: &CompletedGraph) -> Self::Evaluated {
        completed.environment.bool(*self)
    }
}

impl GraphValue for crate::plan::execution::graph::NilLocalId {
    type Evaluated = ();

    fn read(&self, completed: &CompletedGraph) -> Self::Evaluated {
        completed.environment.nil(*self)
    }
}

impl GraphValue for crate::plan::execution::graph::TupleLocalId {
    type Evaluated = Vec<EvaluatedValue>;

    fn read(&self, completed: &CompletedGraph) -> Self::Evaluated {
        completed.environment.tuple(*self)
    }
}

macro_rules! list_graph_value {
    ($local:ty, $value:ty, $method:ident) => {
        impl GraphValue for $local {
            type Evaluated = $value;

            fn read(&self, completed: &CompletedGraph) -> Self::Evaluated {
                completed.environment.$method(*self)
            }
        }
    };
}

list_graph_value!(
    crate::plan::execution::graph::ParameterListLocalId,
    crate::runtime::state::ParameterListValueId,
    parameter_list
);
list_graph_value!(
    crate::plan::execution::graph::IntListLocalId,
    crate::runtime::state::IntListValueId,
    int_list
);
list_graph_value!(
    crate::plan::execution::graph::StringListLocalId,
    crate::runtime::state::StringListValueId,
    string_list
);
list_graph_value!(
    crate::plan::execution::graph::BitArrayListLocalId,
    crate::runtime::state::BitArrayListValueId,
    bit_array_list
);
list_graph_value!(
    crate::plan::execution::graph::UtfCodepointListLocalId,
    crate::runtime::state::UtfCodepointListValueId,
    utf_codepoint_list
);
list_graph_value!(
    crate::plan::execution::graph::CustomListLocalId,
    crate::runtime::state::CustomListValueId,
    custom_list
);
list_graph_value!(
    crate::plan::execution::graph::FloatListLocalId,
    crate::runtime::state::FloatListValueId,
    float_list
);
list_graph_value!(
    crate::plan::execution::graph::BoolListLocalId,
    crate::runtime::state::BoolListValueId,
    bool_list
);
list_graph_value!(
    crate::plan::execution::graph::NilListLocalId,
    crate::runtime::state::NilListValueId,
    nil_list
);
list_graph_value!(
    crate::plan::execution::graph::TupleListLocalId,
    crate::runtime::state::TupleListValueId,
    tuple_list
);
list_graph_value!(
    crate::plan::execution::graph::ParameterListListLocalId,
    crate::runtime::state::ParameterListListValueId,
    parameter_list_list
);
list_graph_value!(
    crate::plan::execution::graph::ListListLocalId,
    crate::runtime::state::ListListValueId,
    list_list
);
list_graph_value!(
    crate::plan::execution::graph::FunctionListLocalId,
    crate::runtime::state::FunctionListValueId,
    function_list
);

macro_rules! function_graph_value {
    ($local:ty, $value:ty, $method:ident) => {
        impl GraphValue for $local {
            type Evaluated = $value;

            fn read(&self, completed: &CompletedGraph) -> Self::Evaluated {
                completed.environment.$method(self.clone())
            }
        }
    };
}

function_graph_value!(
    crate::plan::execution::graph::IntFunctionLocalId,
    crate::runtime::EvaluatedIntFunction,
    int_function
);
function_graph_value!(
    crate::plan::execution::graph::FloatFunctionLocalId,
    crate::runtime::EvaluatedFloatFunction,
    float_function
);
function_graph_value!(
    crate::plan::execution::graph::StringFunctionLocalId,
    crate::runtime::EvaluatedStringFunction,
    string_function
);
function_graph_value!(
    crate::plan::execution::graph::BitArrayFunctionLocalId,
    crate::runtime::EvaluatedBitArrayFunction,
    bit_array_function
);
function_graph_value!(
    crate::plan::execution::graph::UtfCodepointFunctionLocalId,
    crate::runtime::EvaluatedUtfCodepointFunction,
    utf_codepoint_function
);
function_graph_value!(
    crate::plan::execution::graph::BoolFunctionLocalId,
    crate::runtime::EvaluatedBoolFunction,
    bool_function
);
function_graph_value!(
    crate::plan::execution::graph::NilFunctionLocalId,
    crate::runtime::EvaluatedNilFunction,
    nil_function
);
function_graph_value!(
    crate::plan::execution::graph::TupleFunctionLocalId,
    crate::runtime::EvaluatedTupleFunction,
    tuple_function
);

impl GraphValue for crate::plan::execution::graph::GenericFunctionLocal {
    type Evaluated = crate::runtime::EvaluatedGenericFunction;

    fn read(&self, completed: &CompletedGraph) -> Self::Evaluated {
        completed.environment.generic_function(self)
    }
}

impl GraphValue for crate::plan::execution::graph::NeverFunctionLocal {
    type Evaluated = crate::runtime::EvaluatedNeverFunction;

    fn read(&self, completed: &CompletedGraph) -> Self::Evaluated {
        completed.environment.never_function(self)
    }
}

impl GraphValue for crate::plan::execution::graph::CustomFunctionLocal {
    type Evaluated = crate::runtime::EvaluatedCustomFunction;

    fn read(&self, completed: &CompletedGraph) -> Self::Evaluated {
        completed.environment.custom_function(self)
    }
}

impl GraphValue for crate::plan::execution::graph::ListFunctionLocal {
    type Evaluated = crate::runtime::EvaluatedListFunction;

    fn read(&self, completed: &CompletedGraph) -> Self::Evaluated {
        completed.environment.list_function(self)
    }
}

impl GraphValue for crate::plan::execution::graph::FunctionFunctionLocal {
    type Evaluated = crate::runtime::EvaluatedFunctionFunction;

    fn read(&self, completed: &CompletedGraph) -> Self::Evaluated {
        completed.environment.function_function(self)
    }
}

impl GraphValue for crate::plan::execution::graph::FunctionLocal {
    type Evaluated = EvaluatedFunctionValue;

    fn read(&self, completed: &CompletedGraph) -> Self::Evaluated {
        match self {
            Self::Generic(local) => completed.environment.generic_function(local).into(),
            Self::Never(local) => completed.environment.never_function(local).into(),
            Self::Int(local) => completed.environment.int_function(*local).into(),
            Self::Float(local) => completed.environment.float_function(*local).into(),
            Self::String(local) => completed.environment.string_function(*local).into(),
            Self::BitArray(local) => completed.environment.bit_array_function(*local).into(),
            Self::UtfCodepoint(local) => {
                completed.environment.utf_codepoint_function(*local).into()
            }
            Self::Custom(local) => completed.environment.custom_function(local).into(),
            Self::Bool(local) => completed.environment.bool_function(*local).into(),
            Self::Nil(local) => completed.environment.nil_function(*local).into(),
            Self::Tuple(local) => completed.environment.tuple_function(*local).into(),
            Self::List(local) => completed.environment.list_function(local).into(),
            Self::Function(local) => completed.environment.function_function(local).into(),
        }
    }
}
