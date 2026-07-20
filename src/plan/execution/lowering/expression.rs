mod bit_array;
mod bool;
mod custom;
mod custom_field;
mod float;
mod function;
mod generic;
mod int;
mod list;
mod never;
mod nil;
mod string;
mod tuple;
mod utf_codepoint;

use super::id::list_function_local_at_target;
use super::specialization::Representability;
use crate::plan::module;

pub(super) use bool::bool_expr;
pub(super) use custom::{custom_expr, custom_expr_kind};
pub(super) use custom_field::custom_field_access;
pub(super) use float::float_expr;
pub(super) use function::{
    bit_array_function_expr, bool_function_expr, custom_function_expr, custom_function_expr_kind,
    custom_never_function_expr, custom_never_function_expr_kind, float_function_expr,
    function_expr, function_function_expr, function_function_expr_kind,
    generic_bit_array_function_expr, generic_bool_function_expr, generic_custom_function_expr,
    generic_custom_function_expr_kind, generic_float_function_expr, generic_function_function_expr,
    generic_function_function_expr_kind, generic_int_function_expr, generic_list_function_expr,
    generic_never_function_expr, generic_nil_function_expr, generic_string_function_expr,
    generic_symbolic_function_expr, generic_tuple_function_expr,
    generic_utf_codepoint_function_expr, generic_value_never_function_expr, int_function_expr,
    list_function_expr, nil_function_expr, specialized_typed_custom_function_binding,
    specialized_typed_tuple_function_binding, string_function_expr,
    symbolic_bit_array_function_expr, symbolic_bool_function_expr, symbolic_custom_function_expr,
    symbolic_custom_function_expr_kind, symbolic_float_function_expr,
    symbolic_function_function_expr, symbolic_function_function_expr_kind,
    symbolic_int_function_expr, symbolic_list_function_expr, symbolic_nil_function_expr,
    symbolic_string_function_expr, symbolic_tuple_function_expr,
    symbolic_utf_codepoint_function_expr, tuple_function_expr, tuple_never_function_expr,
    typed_function_expr, utf_codepoint_function_expr,
};
pub(super) use generic::{
    generic_bit_array_expr, generic_bool_expr, generic_custom_expr, generic_custom_expr_kind,
    generic_expr, generic_float_expr, generic_function_value_expr, generic_int_expr,
    generic_list_value_expr, generic_nil_expr, generic_string_expr, generic_tuple_expr,
    generic_utf_codepoint_expr, generic_value_bit_array_function_expr,
    generic_value_bit_array_list_expr, generic_value_bool_function_expr,
    generic_value_bool_list_expr, generic_value_custom_function_expr_kind,
    generic_value_custom_list_expr, generic_value_float_function_expr,
    generic_value_float_list_expr, generic_value_function_function_expr_kind,
    generic_value_function_list_expr, generic_value_generic_function_expr,
    generic_value_int_function_expr, generic_value_int_list_expr, generic_value_list_function_expr,
    generic_value_nil_function_expr, generic_value_nil_list_expr,
    generic_value_parameter_list_list_expr, generic_value_stored_list_expr,
    generic_value_stored_nested_list_expr, generic_value_string_function_expr,
    generic_value_string_list_expr, generic_value_tuple_function_expr,
    generic_value_tuple_list_expr, generic_value_utf_codepoint_function_expr,
    generic_value_utf_codepoint_list_expr, never_expr,
};
pub(super) use int::int_expr;
pub(super) use list::{
    bit_array_list_expr, bool_list_expr, concrete_parameter_list_list_expr, custom_list_expr,
    float_list_expr, function_list_expr, generic_bit_array_list_expr, generic_bool_list_expr,
    generic_custom_list_expr, generic_float_list_expr, generic_function_list_expr,
    generic_int_list_expr, generic_nil_list_expr, generic_parameter_list_list_expr,
    generic_stored_nested_list_expr, generic_string_list_expr, generic_tuple_list_expr,
    generic_utf_codepoint_list_expr, int_list_expr, list_expr, list_list_expr, list_local_expr,
    nil_list_expr, parameter_list_expr, parameter_list_value_expr, string_list_expr,
    tuple_list_expr, unresolved_parameter_list_list_expr, utf_codepoint_list_expr,
};
pub(super) use never::{
    custom_inhabitation, custom_never_expr, custom_never_expr_kind, tuple_inhabitation,
    tuple_never_expr,
};
pub(super) use nil::nil_expr;
pub(super) use string::string_expr;
pub(super) use tuple::tuple_expr;
pub(super) use utf_codepoint::utf_codepoint_expr;

pub(super) trait WithLoweredSteps<Output>: Sized {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> Output;
}

enum LoweredBoolSubject {
    True(Vec<execution::Step>),
    False(Vec<execution::Step>),
    Dynamic(execution::BoolExpr),
}

pub(super) fn bool_case<Output: WithLoweredSteps<Output>>(
    subject: &module::BoolExpr,
    context: &mut super::LoweringContext,
    lower_true: impl FnOnce(&mut super::LoweringContext) -> Representability<Output>,
    lower_false: impl FnOnce(&mut super::LoweringContext) -> Representability<Output>,
    combine: impl FnOnce(execution::BoolExpr, Output, Output) -> Output,
) -> Representability<Output> {
    bool_expr(subject, context).and_then(|subject| match lower_bool_subject(subject) {
        LoweredBoolSubject::True(steps) => {
            lower_true(context).map(|value| prepend_steps(value, steps, |value| value))
        }
        LoweredBoolSubject::False(steps) => {
            lower_false(context).map(|value| prepend_steps(value, steps, |value| value))
        }
        LoweredBoolSubject::Dynamic(subject) => lower_true(context)
            .and_then(|true_| lower_false(context).map(|false_| combine(subject, true_, false_))),
    })
}

pub(super) fn bool_case_into<Branch: WithLoweredSteps<Output>, Output>(
    subject: &module::BoolExpr,
    context: &mut super::LoweringContext,
    lower_true: impl FnOnce(&mut super::LoweringContext) -> Representability<Branch>,
    lower_false: impl FnOnce(&mut super::LoweringContext) -> Representability<Branch>,
    into_output: impl FnOnce(Branch) -> Output,
    combine: impl FnOnce(execution::BoolExpr, Branch, Branch) -> Output,
) -> Representability<Output> {
    bool_expr(subject, context).and_then(|subject| match lower_bool_subject(subject) {
        LoweredBoolSubject::True(steps) => {
            lower_true(context).map(|value| prepend_steps(value, steps, into_output))
        }
        LoweredBoolSubject::False(steps) => {
            lower_false(context).map(|value| prepend_steps(value, steps, into_output))
        }
        LoweredBoolSubject::Dynamic(subject) => lower_true(context)
            .and_then(|true_| lower_false(context).map(|false_| combine(subject, true_, false_))),
    })
}

fn lower_bool_subject(subject: execution::BoolExpr) -> LoweredBoolSubject {
    use execution::BoolExprKind as E;

    match subject.into_kind() {
        E::Value(true) => LoweredBoolSubject::True(Vec::new()),
        E::Value(false) => LoweredBoolSubject::False(Vec::new()),
        E::Block { steps, return_ } => match return_.into_kind() {
            E::Value(true) => LoweredBoolSubject::True(steps),
            E::Value(false) => LoweredBoolSubject::False(steps),
            return_ => LoweredBoolSubject::Dynamic(execution::BoolExpr::from_kind(E::Block {
                steps,
                return_: Box::new(execution::BoolExpr::from_kind(return_)),
            })),
        },
        kind => LoweredBoolSubject::Dynamic(execution::BoolExpr::from_kind(kind)),
    }
}

fn prepend_steps<Value, Output>(
    value: Value,
    steps: Vec<execution::Step>,
    into_output: impl FnOnce(Value) -> Output,
) -> Output
where
    Value: WithLoweredSteps<Output>,
{
    if steps.is_empty() {
        into_output(value)
    } else {
        value.with_lowered_steps(steps)
    }
}

impl WithLoweredSteps<execution::IntExprKind> for execution::IntExpr {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> execution::IntExprKind {
        execution::IntExprKind::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl WithLoweredSteps<execution::FloatExprKind> for execution::FloatExpr {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> execution::FloatExprKind {
        execution::FloatExprKind::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl WithLoweredSteps<execution::StringExprKind> for execution::StringExpr {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> execution::StringExprKind {
        execution::StringExprKind::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl WithLoweredSteps<execution::BitArrayExprKind> for execution::BitArrayExpr {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> execution::BitArrayExprKind {
        execution::BitArrayExprKind::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl WithLoweredSteps<execution::UtfCodepointExprKind> for execution::UtfCodepointExpr {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> execution::UtfCodepointExprKind {
        execution::UtfCodepointExprKind::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl WithLoweredSteps<execution::BoolExprKind> for execution::BoolExpr {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> execution::BoolExprKind {
        execution::BoolExprKind::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl WithLoweredSteps<execution::NilExprKind> for execution::NilExpr {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> execution::NilExprKind {
        execution::NilExprKind::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl WithLoweredSteps<execution::TupleExprKind> for execution::TupleExpr {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> execution::TupleExprKind {
        execution::TupleExprKind::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl WithLoweredSteps<execution::NeverExprKind> for execution::NeverExpr {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> execution::NeverExprKind {
        execution::NeverExprKind::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl WithLoweredSteps<execution::IntFunctionExprKind> for execution::IntFunctionExpr {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> execution::IntFunctionExprKind {
        execution::IntFunctionExprKind::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl WithLoweredSteps<execution::FloatFunctionExprKind> for execution::FloatFunctionExpr {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> execution::FloatFunctionExprKind {
        execution::FloatFunctionExprKind::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl WithLoweredSteps<execution::StringFunctionExprKind> for execution::StringFunctionExpr {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> execution::StringFunctionExprKind {
        execution::StringFunctionExprKind::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl WithLoweredSteps<execution::BitArrayFunctionExprKind> for execution::BitArrayFunctionExpr {
    fn with_lowered_steps(
        self,
        steps: Vec<execution::Step>,
    ) -> execution::BitArrayFunctionExprKind {
        execution::BitArrayFunctionExprKind::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl WithLoweredSteps<execution::UtfCodepointFunctionExprKind>
    for execution::UtfCodepointFunctionExpr
{
    fn with_lowered_steps(
        self,
        steps: Vec<execution::Step>,
    ) -> execution::UtfCodepointFunctionExprKind {
        execution::UtfCodepointFunctionExprKind::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl WithLoweredSteps<execution::BoolFunctionExprKind> for execution::BoolFunctionExpr {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> execution::BoolFunctionExprKind {
        execution::BoolFunctionExprKind::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl WithLoweredSteps<execution::NilFunctionExprKind> for execution::NilFunctionExpr {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> execution::NilFunctionExprKind {
        execution::NilFunctionExprKind::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl WithLoweredSteps<execution::TupleFunctionExprKind> for execution::TupleFunctionExpr {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> execution::TupleFunctionExprKind {
        execution::TupleFunctionExprKind::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl WithLoweredSteps<execution::ListFunctionExprKind> for execution::ListFunctionExpr {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> execution::ListFunctionExprKind {
        execution::ListFunctionExprKind::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl WithLoweredSteps<execution::GenericFunctionExprKind> for execution::GenericFunctionExpr {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> execution::GenericFunctionExprKind {
        execution::GenericFunctionExprKind::Block {
            steps,
            return_: Box::new(self.into_kind()),
        }
    }
}

impl WithLoweredSteps<execution::NeverFunctionExprKind> for execution::NeverFunctionExpr {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> execution::NeverFunctionExprKind {
        execution::NeverFunctionExprKind::Block {
            steps,
            return_: Box::new(self.into_kind()),
        }
    }
}

impl WithLoweredSteps<execution::CustomExprKind> for execution::CustomExprKind {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> Self {
        Self::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl WithLoweredSteps<execution::CustomFunctionExprKind> for execution::CustomFunctionExprKind {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> Self {
        Self::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl WithLoweredSteps<execution::FunctionFunctionExprKind> for execution::FunctionFunctionExprKind {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> Self {
        Self::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl WithLoweredSteps<execution::GenericFunctionExprKind> for execution::GenericFunctionExprKind {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> Self {
        Self::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl WithLoweredSteps<execution::NeverFunctionExprKind> for execution::NeverFunctionExprKind {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> Self {
        Self::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl WithLoweredSteps<execution::ParameterListExprKind> for execution::ParameterListExprKind {
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> Self {
        Self::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl<Item> WithLoweredSteps<execution::TypedListExprKind<Item>>
    for execution::TypedListExprKind<Item>
where
    Item: execution::ListItem,
{
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> Self {
        Self::Block {
            steps,
            return_: Box::new(self),
        }
    }
}

impl<Expression, Function> WithLoweredSteps<execution::ReturnBodyKind<Expression, Function>>
    for execution::ReturnBodyKind<Expression, Function>
{
    fn with_lowered_steps(self, steps: Vec<execution::Step>) -> Self {
        Self::Block {
            steps,
            return_: Box::new(execution::ReturnBody::from_kind(self)),
        }
    }
}

use super::super as execution;

enum SpecializedValueBinding {
    Int {
        local: execution::IntLocalId,
        value: execution::IntExpr,
    },
    Float {
        local: execution::FloatLocalId,
        value: execution::FloatExpr,
    },
    String {
        local: execution::StringLocalId,
        value: execution::StringExpr,
    },
    BitArray {
        local: execution::BitArrayLocalId,
        value: execution::BitArrayExpr,
    },
    UtfCodepoint {
        local: execution::UtfCodepointLocalId,
        value: execution::UtfCodepointExpr,
    },
    Custom(execution::CustomLocalExpr),
    Bool {
        local: execution::BoolLocalId,
        value: execution::BoolExpr,
    },
    Nil {
        local: execution::NilLocalId,
        value: execution::NilExpr,
    },
    Tuple {
        local: execution::TupleLocalId,
        value: execution::TupleExpr,
    },
    List(execution::ListLocalExpr),
    Function(Box<function::SpecializedFunctionBinding>),
}

enum LoweredCallArguments {
    Complete(Vec<execution::CallArg>),
    Diverging {
        prefix: Vec<execution::CallArg>,
        expression: execution::NeverExpr,
    },
}

enum LoweredCallArg {
    Stored(Box<execution::CallArg>),
    Diverging(execution::NeverExpr),
}

enum PotentialCallArg {
    Stored(super::specialization::StoredValueShape),
    Diverging(execution::NeverExpr),
}

fn specialized_value_binding_for_shape(
    index: usize,
    value: &module::Expr,
    shape: &super::specialization::StoredValueShape,
    context: &mut super::LoweringContext,
) -> Representability<SpecializedValueBinding> {
    match value.kind() {
        module::ExprKind::Generic(value) => {
            specialized_stored_generic_value_binding(index, value, shape, context)
        }
        module::ExprKind::Int(value) => {
            int_expr(value, context).map(|value| SpecializedValueBinding::Int {
                local: execution::IntLocalId(index),
                value,
            })
        }
        module::ExprKind::Float(value) => {
            float_expr(value, context).map(|value| SpecializedValueBinding::Float {
                local: execution::FloatLocalId(index),
                value,
            })
        }
        module::ExprKind::String(value) => {
            string_expr(value, context).map(|value| SpecializedValueBinding::String {
                local: execution::StringLocalId(index),
                value,
            })
        }
        module::ExprKind::BitArray(value) => {
            bit_array_expr(value, context).map(|value| SpecializedValueBinding::BitArray {
                local: execution::BitArrayLocalId(index),
                value,
            })
        }
        module::ExprKind::UtfCodepoint(value) => {
            utf_codepoint_expr(value, context).map(|value| SpecializedValueBinding::UtfCodepoint {
                local: execution::UtfCodepointLocalId(index),
                value,
            })
        }
        module::ExprKind::Custom(value) => custom_expr(value, context).map(|expression| {
            SpecializedValueBinding::Custom(execution::CustomLocalExpr::new(
                execution::CustomLocal::new(
                    execution::CustomLocalId(index),
                    context.custom_value_shape(value.shape().clone()),
                ),
                expression,
            ))
        }),
        module::ExprKind::Bool(value) => {
            bool_expr(value, context).map(|value| SpecializedValueBinding::Bool {
                local: execution::BoolLocalId(index),
                value,
            })
        }
        module::ExprKind::Nil(value) => {
            nil_expr(value, context).map(|value| SpecializedValueBinding::Nil {
                local: execution::NilLocalId(index),
                value,
            })
        }
        module::ExprKind::Tuple(value) => {
            tuple_expr(value, context).map(|value| SpecializedValueBinding::Tuple {
                local: execution::TupleLocalId(index),
                value,
            })
        }
        module::ExprKind::List(value) => {
            list::specialized_list_local_expr(index, list_expr(value, context))
                .map(SpecializedValueBinding::List)
        }
        module::ExprKind::Function(value) => {
            function::specialized_function_binding(index, value, context)
                .map(|value| SpecializedValueBinding::Function(Box::new(value)))
        }
    }
}

fn specialized_stored_generic_value_binding(
    index: usize,
    value: &module::GenericExpr,
    shape: &super::specialization::StoredValueShape,
    context: &mut super::LoweringContext,
) -> Representability<SpecializedValueBinding> {
    use super::specialization::StoredValueShape as S;

    match shape {
        S::Int => generic_int_expr(value, context).map(|value| SpecializedValueBinding::Int {
            local: execution::IntLocalId(index),
            value,
        }),
        S::Float => {
            generic_float_expr(value, context).map(|value| SpecializedValueBinding::Float {
                local: execution::FloatLocalId(index),
                value,
            })
        }
        S::String => {
            generic_string_expr(value, context).map(|value| SpecializedValueBinding::String {
                local: execution::StringLocalId(index),
                value,
            })
        }
        S::BitArray => {
            generic_bit_array_expr(value, context).map(|value| SpecializedValueBinding::BitArray {
                local: execution::BitArrayLocalId(index),
                value,
            })
        }
        S::UtfCodepoint => generic_utf_codepoint_expr(value, context).map(|value| {
            SpecializedValueBinding::UtfCodepoint {
                local: execution::UtfCodepointLocalId(index),
                value,
            }
        }),
        S::Custom(shape) => generic_custom_expr(value, shape, context).map(|expression| {
            SpecializedValueBinding::Custom(execution::CustomLocalExpr::new(
                execution::CustomLocal::new(
                    execution::CustomLocalId(index),
                    context.lower_concrete_custom_shape(shape),
                ),
                expression,
            ))
        }),
        S::Bool => generic_bool_expr(value, context).map(|value| SpecializedValueBinding::Bool {
            local: execution::BoolLocalId(index),
            value,
        }),
        S::Nil => generic_nil_expr(value, context).map(|value| SpecializedValueBinding::Nil {
            local: execution::NilLocalId(index),
            value,
        }),
        S::Tuple(elements) => generic_tuple_expr(value, elements, context).map(|value| {
            SpecializedValueBinding::Tuple {
                local: execution::TupleLocalId(index),
                value,
            }
        }),
        S::List(item) => {
            list::specialized_list_local_expr(index, generic_list_value_expr(value, item, context))
                .map(SpecializedValueBinding::List)
        }
        S::Function(shape) => generic::generic_function_value_binding(index, value, shape, context)
            .map(|value| SpecializedValueBinding::Function(Box::new(value))),
    }
}

fn specialized_call_arg(binding: SpecializedValueBinding) -> execution::CallArg {
    use execution::CallArgKind as E;

    execution::CallArg::from_kind(match binding {
        SpecializedValueBinding::Int { local, value } => E::Int { local, value },
        SpecializedValueBinding::Float { local, value } => E::Float { local, value },
        SpecializedValueBinding::String { local, value } => E::String { local, value },
        SpecializedValueBinding::BitArray { local, value } => E::BitArray { local, value },
        SpecializedValueBinding::UtfCodepoint { local, value } => E::UtfCodepoint { local, value },
        SpecializedValueBinding::Custom(value) => E::Custom(value),
        SpecializedValueBinding::Bool { local, value } => E::Bool { local, value },
        SpecializedValueBinding::Nil { local, value } => E::Nil { local, value },
        SpecializedValueBinding::Tuple { local, value } => E::Tuple { local, value },
        SpecializedValueBinding::List(value) => E::List(value),
        SpecializedValueBinding::Function(value) => specialized_function_call_arg(*value),
    })
}

fn specialized_function_call_arg(
    binding: function::SpecializedFunctionBinding,
) -> execution::CallArgKind {
    use execution::CallArgKind as E;
    use function::SpecializedFunctionBinding as B;

    match binding {
        B::Int { local, value } => E::IntFunction { local, value },
        B::Float { local, value } => E::FloatFunction { local, value },
        B::String { local, value } => E::StringFunction { local, value },
        B::BitArray { local, value } => E::BitArrayFunction { local, value },
        B::UtfCodepoint { local, value } => E::UtfCodepointFunction { local, value },
        B::Generic { local, value } => E::GenericFunction { local, value },
        B::Never { local, value } => E::NeverFunction { local, value },
        B::Custom { local, value } => E::CustomFunction { local, value },
        B::Bool { local, value } => E::BoolFunction { local, value },
        B::Nil { local, value } => E::NilFunction { local, value },
        B::Tuple { local, value } => E::TupleFunction { local, value },
        B::List { local, value } => E::ListFunction { local, value },
        B::Function { local, value } => E::FunctionFunction { local, value },
    }
}

fn specialized_capture_arg(
    binding: SpecializedValueBinding,
) -> Representability<execution::CaptureArg> {
    use execution::CaptureArgKind as E;

    let kind = match binding {
        SpecializedValueBinding::Int { local, value } => E::Int { local, value },
        SpecializedValueBinding::Float { local, value } => E::Float { local, value },
        SpecializedValueBinding::String { local, value } => E::String { local, value },
        SpecializedValueBinding::BitArray { local, value } => E::BitArray { local, value },
        SpecializedValueBinding::UtfCodepoint { local, value } => E::UtfCodepoint { local, value },
        SpecializedValueBinding::Custom(value) => E::Custom(value),
        SpecializedValueBinding::Bool { local, value } => E::Bool { local, value },
        SpecializedValueBinding::Nil { local, value } => E::Nil { local, value },
        SpecializedValueBinding::Tuple { local, value } => E::Tuple { local, value },
        SpecializedValueBinding::List(value) => E::List(value),
        SpecializedValueBinding::Function(value) => specialized_function_capture_arg(*value),
    };
    Representability::Inhabited(execution::CaptureArg::from_kind(kind))
}

fn specialized_function_capture_arg(
    binding: function::SpecializedFunctionBinding,
) -> execution::CaptureArgKind {
    use execution::CaptureArgKind as E;
    use function::SpecializedFunctionBinding as B;

    match binding {
        B::Int { local, value } => E::IntFunction { local, value },
        B::Float { local, value } => E::FloatFunction { local, value },
        B::String { local, value } => E::StringFunction { local, value },
        B::BitArray { local, value } => E::BitArrayFunction { local, value },
        B::UtfCodepoint { local, value } => E::UtfCodepointFunction { local, value },
        B::Generic { local, value } => E::GenericFunction { local, value },
        B::Never { local, value } => E::NeverFunction { local, value },
        B::Custom { local, value } => E::CustomFunction { local, value },
        B::Bool { local, value } => E::BoolFunction { local, value },
        B::Nil { local, value } => E::NilFunction { local, value },
        B::Tuple { local, value } => E::TupleFunction { local, value },
        B::List { local, value } => E::ListFunction { local, value },
        B::Function { local, value } => E::FunctionFunction { local, value },
    }
}

pub(super) fn generic_step(
    local: crate::plan::GenericLocal,
    value: &module::GenericExpr,
    context: &mut super::LoweringContext,
) -> Representability<execution::StepKind> {
    let shape = context.concrete_parameter(value.parameter());
    match context.representations.representation(&shape) {
        super::specialization::ValueRepresentation::Uninhabited(_) => {
            generic::never_expr(value, context).map(|value| {
                execution::StepKind::Evaluate(execution::Expr::from_kind(
                    execution::ExprKind::Never(value),
                ))
            })
        }
        super::specialization::ValueRepresentation::Stored(shape) => {
            context.generic_local_index(local.id()).and_then(|index| {
                specialized_stored_generic_value_binding(index, value, &shape, context)
                    .map(specialized_step)
            })
        }
    }
}

pub(super) fn generic_function_step(
    local: &crate::plan::GenericFunctionLocal,
    value: &module::TypedFunctionExpr<module::GenericFunctionExpr>,
    context: &mut super::LoweringContext,
) -> Representability<execution::StepKind> {
    function::specialized_typed_generic_function_binding(
        context.generic_function_local_index(local.id()),
        value,
        context,
    )
    .map(specialized_function_step)
}

pub(super) fn symbolic_function_step<ModuleExpression>(
    key: super::frame::LocalKey,
    shape: &crate::plan::FunctionShape,
    expression: &ModuleExpression,
    context: &mut super::LoweringContext,
    lower: impl FnOnce(
        &ModuleExpression,
        &super::specialization::SpecializedFunctionShape,
        &mut super::LoweringContext,
    ) -> Representability<execution::GenericFunctionExpr>,
) -> Representability<execution::StepKind> {
    let shape = context.concrete_function_shape(shape);
    let lowered_shape = context.lower_concrete_function_shape(&shape);
    lower(expression, &shape, context).map(|value| {
        let local = execution::GenericFunctionLocal::new(
            execution::GenericFunctionLocalId(context.local_index(key)),
            value.generic_function_type().clone(),
        );
        specialized_function_step(function::SpecializedFunctionBinding::Generic {
            local,
            value: execution::TypedFunctionExpr::new(lowered_shape, value),
        })
    })
}

fn specialized_step(binding: SpecializedValueBinding) -> execution::StepKind {
    use execution::StepKind as E;

    match binding {
        SpecializedValueBinding::Int { local, value } => E::LetInt { local, value },
        SpecializedValueBinding::Float { local, value } => E::LetFloat { local, value },
        SpecializedValueBinding::String { local, value } => E::LetString { local, value },
        SpecializedValueBinding::BitArray { local, value } => E::LetBitArray { local, value },
        SpecializedValueBinding::UtfCodepoint { local, value } => {
            E::LetUtfCodepoint { local, value }
        }
        SpecializedValueBinding::Custom(value) => E::LetCustom(value),
        SpecializedValueBinding::Bool { local, value } => E::LetBool { local, value },
        SpecializedValueBinding::Nil { local, value } => E::LetNil { local, value },
        SpecializedValueBinding::Tuple { local, value } => E::LetTuple { local, value },
        SpecializedValueBinding::List(value) => E::LetList { value },
        SpecializedValueBinding::Function(value) => specialized_function_step(*value),
    }
}

pub(super) fn specialized_function_step(
    binding: function::SpecializedFunctionBinding,
) -> execution::StepKind {
    use execution::StepKind as E;
    use function::SpecializedFunctionBinding as B;

    match binding {
        B::Int { local, value } => E::LetIntFunction { local, value },
        B::Float { local, value } => E::LetFloatFunction { local, value },
        B::String { local, value } => E::LetStringFunction { local, value },
        B::BitArray { local, value } => E::LetBitArrayFunction { local, value },
        B::UtfCodepoint { local, value } => E::LetUtfCodepointFunction { local, value },
        B::Generic { local, value } => E::LetGenericFunction { local, value },
        B::Never { local, value } => E::LetNeverFunction { local, value },
        B::Custom { local, value } => E::LetCustomFunction { local, value },
        B::Bool { local, value } => E::LetBoolFunction { local, value },
        B::Nil { local, value } => E::LetNilFunction { local, value },
        B::Tuple { local, value } => E::LetTupleFunction { local, value },
        B::List { local, value } => E::LetListFunction { local, value },
        B::Function { local, value } => E::LetFunctionFunction { local, value },
    }
}

pub(super) fn expr(
    expression: &module::Expr,
    context: &mut super::LoweringContext,
) -> Representability<execution::Expr> {
    let kind = match expression.kind() {
        module::ExprKind::Generic(expression) => return generic_expr(expression, context),
        module::ExprKind::Int(expression) => {
            int_expr(expression, context).map(execution::ExprKind::Int)
        }
        module::ExprKind::String(expression) => {
            string_expr(expression, context).map(execution::ExprKind::String)
        }
        module::ExprKind::BitArray(expression) => {
            bit_array_expr(expression, context).map(execution::ExprKind::BitArray)
        }
        module::ExprKind::UtfCodepoint(expression) => {
            utf_codepoint_expr(expression, context).map(execution::ExprKind::UtfCodepoint)
        }
        module::ExprKind::Custom(expression) => {
            custom_expr(expression, context).map(execution::ExprKind::Custom)
        }
        module::ExprKind::Float(expression) => {
            float_expr(expression, context).map(execution::ExprKind::Float)
        }
        module::ExprKind::Bool(expression) => {
            bool_expr(expression, context).map(execution::ExprKind::Bool)
        }
        module::ExprKind::Nil(expression) => {
            nil_expr(expression, context).map(execution::ExprKind::Nil)
        }
        module::ExprKind::Tuple(expression) => {
            tuple_expr(expression, context).map(execution::ExprKind::Tuple)
        }
        module::ExprKind::List(expression) => {
            list_expr(expression, context).map(execution::ExprKind::List)
        }
        module::ExprKind::Function(expression) => {
            function_expr(expression, context).map(execution::ExprKind::Function)
        }
    };
    kind.map(execution::Expr::from_kind)
}

pub(super) fn panic_expr(
    expression: &module::PanicExpr,
    context: &mut super::LoweringContext,
) -> Representability<execution::PanicExpr> {
    let kind = match expression.kind() {
        module::PanicExprKind::Panic { message } => Representability::transpose_option(
            message
                .as_deref()
                .map(|message| string_expr(message, context)),
        )
        .map(|message| execution::PanicExprKind::Panic {
            message: message.map(Box::new),
        }),
        module::PanicExprKind::Todo { message } => Representability::transpose_option(
            message
                .as_deref()
                .map(|message| string_expr(message, context)),
        )
        .map(|message| execution::PanicExprKind::Todo {
            message: message.map(Box::new),
        }),
        module::PanicExprKind::EmptyFunction => {
            Representability::Inhabited(execution::PanicExprKind::EmptyFunction)
        }
        module::PanicExprKind::EmptyBlock => {
            Representability::Inhabited(execution::PanicExprKind::EmptyBlock)
        }
        module::PanicExprKind::IncompleteUse => {
            Representability::Inhabited(execution::PanicExprKind::IncompleteUse)
        }
    };
    kind.map(|kind| execution::PanicExpr::from_parts(expression.site(), kind))
}

fn lower_call_args(
    args: &[module::CallArg],
    context: &mut super::LoweringContext,
) -> Representability<LoweredCallArguments> {
    let mut prefix = super::frame::ParameterPrefix::default();
    let mut lowered = Vec::with_capacity(args.len());
    for arg in args {
        let arg = match arg.storage() {
            module::CallArgStorage::Stored(shape) => {
                let shape = context.stored_value_shape(&shape);
                let (index, shape) = prefix.allocate_stored(shape, &context.representations);
                let target = context.current_stored_target(index, shape);
                stored_call_arg_at(&target, arg, context)
                    .map(Box::new)
                    .map(LoweredCallArg::Stored)
            }
            module::CallArgStorage::PotentiallyUninhabited(value) => {
                match potentially_uninhabited_call_arg(value, context) {
                    Representability::Inhabited(PotentialCallArg::Stored(shape)) => {
                        let (index, shape) =
                            prefix.allocate_stored(shape, &context.representations);
                        let target = context.current_stored_target(index, shape);
                        stored_call_arg_at(&target, arg, context)
                            .map(Box::new)
                            .map(LoweredCallArg::Stored)
                    }
                    Representability::Inhabited(PotentialCallArg::Diverging(value)) => {
                        Representability::Inhabited(LoweredCallArg::Diverging(value))
                    }
                    Representability::Uninhabited => Representability::Uninhabited,
                }
            }
        };
        let arg = match arg {
            Representability::Inhabited(arg) => arg,
            Representability::Uninhabited => return Representability::Uninhabited,
        };
        match arg {
            LoweredCallArg::Diverging(expression) => {
                return Representability::Inhabited(LoweredCallArguments::Diverging {
                    prefix: lowered,
                    expression,
                });
            }
            LoweredCallArg::Stored(arg) => lowered.push(*arg),
        }
    }
    Representability::Inhabited(LoweredCallArguments::Complete(lowered))
}

fn lower_direct_call_args(
    function: &module::FunctionInstantiation,
    args: &[module::CallArg],
    context: &mut super::LoweringContext,
) -> Representability<LoweredCallArguments> {
    let mut lowered = Vec::with_capacity(args.len());
    for arg in args {
        let arg = match direct_call_arg(function, arg, context) {
            Representability::Inhabited(arg) => arg,
            Representability::Uninhabited => return Representability::Uninhabited,
        };
        match arg {
            LoweredCallArg::Diverging(expression) => {
                return Representability::Inhabited(LoweredCallArguments::Diverging {
                    prefix: lowered,
                    expression,
                });
            }
            LoweredCallArg::Stored(arg) => lowered.push(*arg),
        }
    }
    Representability::Inhabited(LoweredCallArguments::Complete(lowered))
}

pub(super) fn direct_call<Function>(
    function: &module::FunctionInstantiation,
    args: &[module::CallArg],
    context: &mut super::LoweringContext,
    executable: impl FnOnce(
        &module::FunctionInstantiation,
        &mut super::LoweringContext,
    ) -> Representability<Function>,
) -> Representability<execution::DirectCall<Function>> {
    lower_direct_call_args(function, args, context).and_then(|args| match args {
        LoweredCallArguments::Complete(args) => executable(function, context)
            .map(|function| execution::DirectCall::Executable { function, args }),
        LoweredCallArguments::Diverging { prefix, expression } => {
            Representability::Inhabited(execution::DirectCall::Diverging(
                execution::NeverExpr::from_kind(execution::NeverExprKind::Arguments {
                    prefix,
                    diverging: Box::new(expression),
                }),
            ))
        }
    })
}

pub(super) fn function_call<Function>(
    args: &[module::CallArg],
    context: &mut super::LoweringContext,
    executable: impl FnOnce(&mut super::LoweringContext) -> Representability<Function>,
    evaluated: impl FnOnce(&mut super::LoweringContext) -> Representability<execution::FunctionExpr>,
) -> Representability<execution::FunctionCall<Function>> {
    lower_call_args(args, context).and_then(|args| match args {
        LoweredCallArguments::Complete(args) => {
            executable(context).map(|function| execution::FunctionCall::Executable {
                function: Box::new(function),
                args,
            })
        }
        LoweredCallArguments::Diverging { prefix, expression } => {
            evaluated(context).map(|function| {
                execution::FunctionCall::Diverging(execution::NeverExpr::from_kind(
                    execution::NeverExprKind::FunctionArguments {
                        function: Box::new(function),
                        prefix,
                        diverging: Box::new(expression),
                    },
                ))
            })
        }
    })
}

fn direct_call_arg(
    function: &module::FunctionInstantiation,
    arg: &module::CallArg,
    context: &mut super::LoweringContext,
) -> Representability<LoweredCallArg> {
    let key = call_arg_local_key(arg);
    match arg.storage() {
        module::CallArgStorage::Stored(_) => {
            let target = context.stored_symbolic_target_local(function, key);
            stored_call_arg_at(&target, arg, context)
                .map(Box::new)
                .map(LoweredCallArg::Stored)
        }
        module::CallArgStorage::PotentiallyUninhabited(value) => {
            match potentially_uninhabited_call_arg(value, context) {
                Representability::Inhabited(PotentialCallArg::Stored(_)) => {
                    let target = context.stored_symbolic_target_local(function, key);
                    stored_call_arg_at(&target, arg, context)
                        .map(Box::new)
                        .map(LoweredCallArg::Stored)
                }
                Representability::Inhabited(PotentialCallArg::Diverging(value)) => {
                    Representability::Inhabited(LoweredCallArg::Diverging(value))
                }
                Representability::Uninhabited => Representability::Uninhabited,
            }
        }
    }
}

fn potentially_uninhabited_call_arg(
    value: module::PotentiallyUninhabitedCallArg<'_>,
    context: &mut super::LoweringContext,
) -> Representability<PotentialCallArg> {
    use super::specialization::{CompoundInhabitation, ValueInhabitation};
    use module::PotentiallyUninhabitedCallArg as V;

    match value {
        V::Generic(value) => {
            let shape = context.concrete_parameter(value.parameter());
            match context.representations.inhabitation(&shape) {
                ValueInhabitation::Inhabited(shape) => {
                    Representability::Inhabited(PotentialCallArg::Stored(shape))
                }
                ValueInhabitation::Uninhabited(_) => {
                    generic::never_expr(value, context).map(PotentialCallArg::Diverging)
                }
            }
        }
        V::Tuple(value) => {
            let elements = value
                .shape()
                .iter()
                .map(|shape| context.concrete_value_shape(shape))
                .collect::<Vec<_>>();
            match context.representations.tuple_inhabitation(&elements) {
                CompoundInhabitation::Inhabited => {
                    Representability::Inhabited(PotentialCallArg::Stored(
                        super::specialization::StoredValueShape::Tuple(elements.into_boxed_slice()),
                    ))
                }
                CompoundInhabitation::Uninhabited(proof) => {
                    tuple_never_expr(value, &proof, context).map(PotentialCallArg::Diverging)
                }
            }
        }
        V::Custom(value) => {
            let shape = context.concrete_custom_value_shape(value.shape());
            match context.representations.custom_inhabitation(&shape) {
                CompoundInhabitation::Inhabited => {
                    Representability::Inhabited(PotentialCallArg::Stored(
                        super::specialization::StoredValueShape::Custom(shape),
                    ))
                }
                CompoundInhabitation::Uninhabited(proof) => {
                    custom_never_expr(value, &proof, context).map(PotentialCallArg::Diverging)
                }
            }
        }
    }
}

fn stored_call_arg_at(
    target: &super::StoredTargetLocal,
    arg: &module::CallArg,
    context: &mut super::LoweringContext,
) -> Representability<execution::CallArg> {
    use execution::CallArgKind as E;
    use module::CallArgKind as M;

    let index = target.index();

    let kind = match arg.kind() {
        M::Parametric { slot: _, value } => {
            return specialized_value_binding_for_shape(index, value, target.shape(), context)
                .map(specialized_call_arg);
        }
        M::Int { local: _, value } => int_expr(value, context).map(|value| E::Int {
            local: execution::IntLocalId(index),
            value,
        }),
        M::String { local: _, value } => string_expr(value, context).map(|value| E::String {
            local: execution::StringLocalId(index),
            value,
        }),
        M::BitArray { local: _, value } => {
            bit_array_expr(value, context).map(|value| E::BitArray {
                local: execution::BitArrayLocalId(index),
                value,
            })
        }
        M::UtfCodepoint { local: _, value } => {
            utf_codepoint_expr(value, context).map(|value| E::UtfCodepoint {
                local: execution::UtfCodepointLocalId(index),
                value,
            })
        }
        M::Custom(binding) => {
            let local = execution::CustomLocal::new(
                execution::CustomLocalId(index),
                context.lower_concrete_custom_shape(&target.custom_shape(binding.local().shape())),
            );
            custom_expr(binding.value(), context)
                .map(|value| E::Custom(execution::CustomLocalExpr::new(local, value)))
        }
        M::Float { local: _, value } => float_expr(value, context).map(|value| E::Float {
            local: execution::FloatLocalId(index),
            value,
        }),
        M::Bool { local: _, value } => bool_expr(value, context).map(|value| E::Bool {
            local: execution::BoolLocalId(index),
            value,
        }),
        M::Nil { local: _, value } => nil_expr(value, context).map(|value| E::Nil {
            local: execution::NilLocalId(index),
            value,
        }),
        M::Tuple { local: _, value } => tuple_expr(value, context).map(|value| E::Tuple {
            local: execution::TupleLocalId(index),
            value,
        }),
        M::List(value) => list::list_local_expr_at(index, value, context).map(E::List),
        M::IntFunction { local: _, value } => {
            let shape = target.function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                super::specialization::FunctionRepresentation::Symbolic
            ) {
                return symbolic_function_call_arg(
                    index,
                    value,
                    shape,
                    context,
                    symbolic_int_function_expr,
                );
            }
            typed_function_expr(value, context, int_function_expr).map(|value| E::IntFunction {
                local: execution::IntFunctionLocalId(index),
                value,
            })
        }
        M::StringFunction { local: _, value } => {
            let shape = target.function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                super::specialization::FunctionRepresentation::Symbolic
            ) {
                return symbolic_function_call_arg(
                    index,
                    value,
                    shape,
                    context,
                    symbolic_string_function_expr,
                );
            }
            typed_function_expr(value, context, string_function_expr).map(|value| {
                E::StringFunction {
                    local: execution::StringFunctionLocalId(index),
                    value,
                }
            })
        }
        M::BitArrayFunction { local: _, value } => {
            let shape = target.function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                super::specialization::FunctionRepresentation::Symbolic
            ) {
                return symbolic_function_call_arg(
                    index,
                    value,
                    shape,
                    context,
                    symbolic_bit_array_function_expr,
                );
            }
            typed_function_expr(value, context, bit_array_function_expr).map(|value| {
                E::BitArrayFunction {
                    local: execution::BitArrayFunctionLocalId(index),
                    value,
                }
            })
        }
        M::UtfCodepointFunction { local: _, value } => {
            let shape = target.function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                super::specialization::FunctionRepresentation::Symbolic
            ) {
                return symbolic_function_call_arg(
                    index,
                    value,
                    shape,
                    context,
                    symbolic_utf_codepoint_function_expr,
                );
            }
            typed_function_expr(value, context, utf_codepoint_function_expr).map(|value| {
                E::UtfCodepointFunction {
                    local: execution::UtfCodepointFunctionLocalId(index),
                    value,
                }
            })
        }
        M::CustomFunction { local, value } => {
            let shape = target.function_shape(value.shape());
            match shape.arguments_representation(&context.representations) {
                super::specialization::FunctionArgumentsRepresentation::Symbolic => {
                    return symbolic_function_call_arg(
                        index,
                        value,
                        shape,
                        context,
                        symbolic_custom_function_expr,
                    );
                }
                super::specialization::FunctionArgumentsRepresentation::Inhabited => {}
            }
            let local = execution::CustomFunctionLocal::new(
                execution::CustomFunctionLocalId(index),
                context
                    .custom_function_type_with_substitution(local.type_(), target.substitution()),
            );
            typed_function_expr(value, context, custom_function_expr)
                .map(|value| E::CustomFunction { local, value })
        }
        M::FloatFunction { local: _, value } => {
            let shape = target.function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                super::specialization::FunctionRepresentation::Symbolic
            ) {
                return symbolic_function_call_arg(
                    index,
                    value,
                    shape,
                    context,
                    symbolic_float_function_expr,
                );
            }
            typed_function_expr(value, context, float_function_expr).map(|value| E::FloatFunction {
                local: execution::FloatFunctionLocalId(index),
                value,
            })
        }
        M::BoolFunction { local: _, value } => {
            let shape = target.function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                super::specialization::FunctionRepresentation::Symbolic
            ) {
                return symbolic_function_call_arg(
                    index,
                    value,
                    shape,
                    context,
                    symbolic_bool_function_expr,
                );
            }
            typed_function_expr(value, context, bool_function_expr).map(|value| E::BoolFunction {
                local: execution::BoolFunctionLocalId(index),
                value,
            })
        }
        M::NilFunction { local: _, value } => {
            let shape = target.function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                super::specialization::FunctionRepresentation::Symbolic
            ) {
                return symbolic_function_call_arg(
                    index,
                    value,
                    shape,
                    context,
                    symbolic_nil_function_expr,
                );
            }
            typed_function_expr(value, context, nil_function_expr).map(|value| E::NilFunction {
                local: execution::NilFunctionLocalId(index),
                value,
            })
        }
        M::TupleFunction { local: _, value } => {
            let shape = target.function_shape(value.shape());
            match shape.arguments_representation(&context.representations) {
                super::specialization::FunctionArgumentsRepresentation::Symbolic => {
                    return symbolic_function_call_arg(
                        index,
                        value,
                        shape,
                        context,
                        symbolic_tuple_function_expr,
                    );
                }
                super::specialization::FunctionArgumentsRepresentation::Inhabited => {}
            }
            typed_function_expr(value, context, tuple_function_expr).map(|value| E::TupleFunction {
                local: execution::TupleFunctionLocalId(index),
                value,
            })
        }
        M::ListFunction { local, value } => {
            let shape = target.function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                super::specialization::FunctionRepresentation::Symbolic
            ) {
                return symbolic_function_call_arg(
                    index,
                    value,
                    shape,
                    context,
                    symbolic_list_function_expr,
                );
            }
            let local = list_function_local_at_target(index, local, target, context);
            typed_function_expr(value, context, list_function_expr)
                .map(|value| E::ListFunction { local, value })
        }
        M::FunctionFunction { local, value } => {
            let shape = target.function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                super::specialization::FunctionRepresentation::Symbolic
            ) {
                return symbolic_function_call_arg(
                    index,
                    value,
                    shape,
                    context,
                    symbolic_function_function_expr,
                );
            }
            let local = execution::FunctionFunctionLocal::new(
                execution::FunctionFunctionLocalId(index),
                context
                    .function_function_type_with_substitution(local.type_(), target.substitution()),
            );
            typed_function_expr(value, context, function_function_expr)
                .map(|value| E::FunctionFunction { local, value })
        }
        M::GenericFunction { local: _, value } => {
            return function::specialized_function_binding_for_shape(
                index,
                value,
                target.function_shape(value.shape()),
                context,
            )
            .map(specialized_function_call_arg)
            .map(execution::CallArg::from_kind);
        }
    };
    kind.map(execution::CallArg::from_kind)
}

fn symbolic_function_call_arg<ModuleExpr>(
    index: usize,
    value: &module::TypedFunctionExpr<ModuleExpr>,
    shape: super::specialization::SpecializedFunctionShape,
    context: &mut super::LoweringContext,
    lower: impl FnOnce(
        &ModuleExpr,
        &super::specialization::SpecializedFunctionShape,
        &mut super::LoweringContext,
    ) -> Representability<execution::GenericFunctionExpr>,
) -> Representability<execution::CallArg> {
    function::symbolic_typed_function_binding(index, value, shape, context, lower)
        .map(specialized_function_call_arg)
        .map(execution::CallArg::from_kind)
}

fn call_arg_local_key(arg: &module::CallArg) -> super::frame::LocalKey {
    use super::frame::{LocalKey, LocalKind};
    use module::CallArgKind as A;

    match arg.kind() {
        A::Parametric { slot, .. } => super::frame::param_local_key(slot.local()),
        A::Int { local, .. } => LocalKey::new(LocalKind::Int, local.0),
        A::String { local, .. } => LocalKey::new(LocalKind::String, local.0),
        A::BitArray { local, .. } => LocalKey::new(LocalKind::BitArray, local.0),
        A::UtfCodepoint { local, .. } => LocalKey::new(LocalKind::UtfCodepoint, local.0),
        A::Custom(binding) => LocalKey::new(LocalKind::Custom, binding.local().id().0),
        A::Float { local, .. } => LocalKey::new(LocalKind::Float, local.0),
        A::Bool { local, .. } => LocalKey::new(LocalKind::Bool, local.0),
        A::Nil { local, .. } => LocalKey::new(LocalKind::Nil, local.0),
        A::Tuple { local, .. } => LocalKey::new(LocalKind::Tuple, local.0),
        A::List(local) => list_local_expr_key(local),
        A::IntFunction { local, .. } => LocalKey::new(LocalKind::IntFunction, local.0),
        A::StringFunction { local, .. } => LocalKey::new(LocalKind::StringFunction, local.0),
        A::BitArrayFunction { local, .. } => LocalKey::new(LocalKind::BitArrayFunction, local.0),
        A::UtfCodepointFunction { local, .. } => {
            LocalKey::new(LocalKind::UtfCodepointFunction, local.0)
        }
        A::CustomFunction { local, .. } => LocalKey::new(LocalKind::CustomFunction, local.id().0),
        A::FloatFunction { local, .. } => LocalKey::new(LocalKind::FloatFunction, local.0),
        A::BoolFunction { local, .. } => LocalKey::new(LocalKind::BoolFunction, local.0),
        A::NilFunction { local, .. } => LocalKey::new(LocalKind::NilFunction, local.0),
        A::TupleFunction { local, .. } => LocalKey::new(LocalKind::TupleFunction, local.0),
        A::ListFunction { local, .. } => super::frame::list_function_local_key(local),
        A::FunctionFunction { local, .. } => {
            LocalKey::new(LocalKind::FunctionFunction, local.id().0)
        }
        A::GenericFunction { local, .. } => LocalKey::new(LocalKind::GenericFunction, local.id().0),
    }
}

pub(super) fn list_local_expr_key(local: &module::ListLocalExpr) -> super::frame::LocalKey {
    use super::frame::{LocalKey, LocalKind};

    match local {
        module::ListLocalExpr::Generic { local, .. } => {
            LocalKey::new(LocalKind::GenericList, local.0)
        }
        module::ListLocalExpr::ParameterList { local, .. } => {
            LocalKey::new(LocalKind::ListList, local.0)
        }
        module::ListLocalExpr::Int { local, .. } => LocalKey::new(LocalKind::IntList, local.0),
        module::ListLocalExpr::String { local, .. } => {
            LocalKey::new(LocalKind::StringList, local.0)
        }
        module::ListLocalExpr::BitArray { local, .. } => {
            LocalKey::new(LocalKind::BitArrayList, local.0)
        }
        module::ListLocalExpr::UtfCodepoint { local, .. } => {
            LocalKey::new(LocalKind::UtfCodepointList, local.0)
        }
        module::ListLocalExpr::Custom { local, .. } => {
            LocalKey::new(LocalKind::CustomList, local.0)
        }
        module::ListLocalExpr::Float { local, .. } => LocalKey::new(LocalKind::FloatList, local.0),
        module::ListLocalExpr::Bool { local, .. } => LocalKey::new(LocalKind::BoolList, local.0),
        module::ListLocalExpr::Nil { local, .. } => LocalKey::new(LocalKind::NilList, local.0),
        module::ListLocalExpr::Tuple { local, .. } => LocalKey::new(LocalKind::TupleList, local.0),
        module::ListLocalExpr::List { local, .. } => LocalKey::new(LocalKind::ListList, local.0),
        module::ListLocalExpr::Function { local, .. } => {
            LocalKey::new(LocalKind::FunctionList, local.0)
        }
    }
}

pub(super) fn capture_args(
    function: &module::FunctionInstantiation,
    args: &[module::CaptureArg],
    context: &mut super::LoweringContext,
) -> Representability<Vec<execution::CaptureArg>> {
    let mut lowered = Vec::with_capacity(args.len());
    for arg in args {
        let target = context.stored_symbolic_target_local(function, capture_arg_local_key(arg));
        let arg = match capture_arg_at(&target, arg, context) {
            Representability::Inhabited(arg) => arg,
            Representability::Uninhabited => return Representability::Uninhabited,
        };
        lowered.push(arg);
    }
    Representability::Inhabited(lowered)
}

pub(super) fn symbolic_capture_args(
    function: &module::FunctionInstantiation,
    args: &[module::CaptureArg],
    context: &mut super::LoweringContext,
) -> Representability<Vec<execution::CaptureArg>> {
    capture_args(function, args, context)
}

fn capture_arg_at(
    target: &super::StoredTargetLocal,
    arg: &module::CaptureArg,
    context: &mut super::LoweringContext,
) -> Representability<execution::CaptureArg> {
    use execution::CaptureArgKind as E;
    use module::CaptureArgKind as M;

    let index = target.index();

    let kind = match arg.kind() {
        M::Generic { local: _, value } => {
            return specialized_stored_generic_value_binding(index, value, target.shape(), context)
                .and_then(specialized_capture_arg);
        }
        M::Int { local: _, value } => int_expr(value, context).map(|value| E::Int {
            local: execution::IntLocalId(index),
            value,
        }),
        M::String { local: _, value } => string_expr(value, context).map(|value| E::String {
            local: execution::StringLocalId(index),
            value,
        }),
        M::BitArray { local: _, value } => {
            bit_array_expr(value, context).map(|value| E::BitArray {
                local: execution::BitArrayLocalId(index),
                value,
            })
        }
        M::UtfCodepoint { local: _, value } => {
            utf_codepoint_expr(value, context).map(|value| E::UtfCodepoint {
                local: execution::UtfCodepointLocalId(index),
                value,
            })
        }
        M::Custom(binding) => {
            let local = execution::CustomLocal::new(
                execution::CustomLocalId(index),
                context.lower_concrete_custom_shape(&target.custom_shape(binding.local().shape())),
            );
            custom_expr(binding.value(), context)
                .map(|value| E::Custom(execution::CustomLocalExpr::new(local, value)))
        }
        M::Float { local: _, value } => float_expr(value, context).map(|value| E::Float {
            local: execution::FloatLocalId(index),
            value,
        }),
        M::Bool { local: _, value } => bool_expr(value, context).map(|value| E::Bool {
            local: execution::BoolLocalId(index),
            value,
        }),
        M::Nil { local: _, value } => nil_expr(value, context).map(|value| E::Nil {
            local: execution::NilLocalId(index),
            value,
        }),
        M::Tuple { local: _, value } => tuple_expr(value, context).map(|value| E::Tuple {
            local: execution::TupleLocalId(index),
            value,
        }),
        M::List(value) => list::list_local_expr_at(index, value, context).map(E::List),
        M::IntFunction { local: _, value } => {
            let shape = target.function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                super::specialization::FunctionRepresentation::Symbolic
            ) {
                return symbolic_function_capture_arg(
                    index,
                    value,
                    shape,
                    context,
                    symbolic_int_function_expr,
                );
            }
            typed_function_expr(value, context, int_function_expr).map(|value| E::IntFunction {
                local: execution::IntFunctionLocalId(index),
                value,
            })
        }
        M::StringFunction { local: _, value } => {
            let shape = target.function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                super::specialization::FunctionRepresentation::Symbolic
            ) {
                return symbolic_function_capture_arg(
                    index,
                    value,
                    shape,
                    context,
                    symbolic_string_function_expr,
                );
            }
            typed_function_expr(value, context, string_function_expr).map(|value| {
                E::StringFunction {
                    local: execution::StringFunctionLocalId(index),
                    value,
                }
            })
        }
        M::BitArrayFunction { local: _, value } => {
            let shape = target.function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                super::specialization::FunctionRepresentation::Symbolic
            ) {
                return symbolic_function_capture_arg(
                    index,
                    value,
                    shape,
                    context,
                    symbolic_bit_array_function_expr,
                );
            }
            typed_function_expr(value, context, bit_array_function_expr).map(|value| {
                E::BitArrayFunction {
                    local: execution::BitArrayFunctionLocalId(index),
                    value,
                }
            })
        }
        M::UtfCodepointFunction { local: _, value } => {
            let shape = target.function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                super::specialization::FunctionRepresentation::Symbolic
            ) {
                return symbolic_function_capture_arg(
                    index,
                    value,
                    shape,
                    context,
                    symbolic_utf_codepoint_function_expr,
                );
            }
            typed_function_expr(value, context, utf_codepoint_function_expr).map(|value| {
                E::UtfCodepointFunction {
                    local: execution::UtfCodepointFunctionLocalId(index),
                    value,
                }
            })
        }
        M::CustomFunction { local, value } => {
            let shape = target.function_shape(value.shape());
            match shape.arguments_representation(&context.representations) {
                super::specialization::FunctionArgumentsRepresentation::Symbolic => {
                    return symbolic_function_capture_arg(
                        index,
                        value,
                        shape,
                        context,
                        symbolic_custom_function_expr,
                    );
                }
                super::specialization::FunctionArgumentsRepresentation::Inhabited => {}
            }
            let local = execution::CustomFunctionLocal::new(
                execution::CustomFunctionLocalId(index),
                context
                    .custom_function_type_with_substitution(local.type_(), target.substitution()),
            );
            typed_function_expr(value, context, custom_function_expr)
                .map(|value| E::CustomFunction { local, value })
        }
        M::FloatFunction { local: _, value } => {
            let shape = target.function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                super::specialization::FunctionRepresentation::Symbolic
            ) {
                return symbolic_function_capture_arg(
                    index,
                    value,
                    shape,
                    context,
                    symbolic_float_function_expr,
                );
            }
            typed_function_expr(value, context, float_function_expr).map(|value| E::FloatFunction {
                local: execution::FloatFunctionLocalId(index),
                value,
            })
        }
        M::BoolFunction { local: _, value } => {
            let shape = target.function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                super::specialization::FunctionRepresentation::Symbolic
            ) {
                return symbolic_function_capture_arg(
                    index,
                    value,
                    shape,
                    context,
                    symbolic_bool_function_expr,
                );
            }
            typed_function_expr(value, context, bool_function_expr).map(|value| E::BoolFunction {
                local: execution::BoolFunctionLocalId(index),
                value,
            })
        }
        M::NilFunction { local: _, value } => {
            let shape = target.function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                super::specialization::FunctionRepresentation::Symbolic
            ) {
                return symbolic_function_capture_arg(
                    index,
                    value,
                    shape,
                    context,
                    symbolic_nil_function_expr,
                );
            }
            typed_function_expr(value, context, nil_function_expr).map(|value| E::NilFunction {
                local: execution::NilFunctionLocalId(index),
                value,
            })
        }
        M::TupleFunction { local: _, value } => {
            let shape = target.function_shape(value.shape());
            match shape.arguments_representation(&context.representations) {
                super::specialization::FunctionArgumentsRepresentation::Symbolic => {
                    return symbolic_function_capture_arg(
                        index,
                        value,
                        shape,
                        context,
                        symbolic_tuple_function_expr,
                    );
                }
                super::specialization::FunctionArgumentsRepresentation::Inhabited => {}
            }
            typed_function_expr(value, context, tuple_function_expr).map(|value| E::TupleFunction {
                local: execution::TupleFunctionLocalId(index),
                value,
            })
        }
        M::ListFunction { local, value } => {
            let shape = target.function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                super::specialization::FunctionRepresentation::Symbolic
            ) {
                return symbolic_function_capture_arg(
                    index,
                    value,
                    shape,
                    context,
                    symbolic_list_function_expr,
                );
            }
            let local = list_function_local_at_target(index, local, target, context);
            typed_function_expr(value, context, list_function_expr)
                .map(|value| E::ListFunction { local, value })
        }
        M::FunctionFunction { local, value } => {
            let shape = target.function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                super::specialization::FunctionRepresentation::Symbolic
            ) {
                return symbolic_function_capture_arg(
                    index,
                    value,
                    shape,
                    context,
                    symbolic_function_function_expr,
                );
            }
            let local = execution::FunctionFunctionLocal::new(
                execution::FunctionFunctionLocalId(index),
                context
                    .function_function_type_with_substitution(local.type_(), target.substitution()),
            );
            typed_function_expr(value, context, function_function_expr)
                .map(|value| E::FunctionFunction { local, value })
        }
        M::GenericFunction { local: _, value } => {
            return function::specialized_typed_generic_function_binding_for_shape(
                index,
                value,
                target.function_shape(value.shape()),
                context,
            )
            .map(specialized_function_capture_arg)
            .map(execution::CaptureArg::from_kind);
        }
    };
    kind.map(execution::CaptureArg::from_kind)
}

fn symbolic_function_capture_arg<ModuleExpr>(
    index: usize,
    value: &module::TypedFunctionExpr<ModuleExpr>,
    shape: super::specialization::SpecializedFunctionShape,
    context: &mut super::LoweringContext,
    lower: impl FnOnce(
        &ModuleExpr,
        &super::specialization::SpecializedFunctionShape,
        &mut super::LoweringContext,
    ) -> Representability<execution::GenericFunctionExpr>,
) -> Representability<execution::CaptureArg> {
    function::symbolic_typed_function_binding(index, value, shape, context, lower)
        .map(specialized_function_capture_arg)
        .map(execution::CaptureArg::from_kind)
}

fn capture_arg_local_key(arg: &module::CaptureArg) -> super::frame::LocalKey {
    use super::frame::{LocalKey, LocalKind};
    use module::CaptureArgKind as A;

    match arg.kind() {
        A::Generic { local, .. } => LocalKey::new(LocalKind::Generic, local.id().0),
        A::Int { local, .. } => LocalKey::new(LocalKind::Int, local.0),
        A::String { local, .. } => LocalKey::new(LocalKind::String, local.0),
        A::BitArray { local, .. } => LocalKey::new(LocalKind::BitArray, local.0),
        A::UtfCodepoint { local, .. } => LocalKey::new(LocalKind::UtfCodepoint, local.0),
        A::Custom(binding) => LocalKey::new(LocalKind::Custom, binding.local().id().0),
        A::Float { local, .. } => LocalKey::new(LocalKind::Float, local.0),
        A::Bool { local, .. } => LocalKey::new(LocalKind::Bool, local.0),
        A::Nil { local, .. } => LocalKey::new(LocalKind::Nil, local.0),
        A::Tuple { local, .. } => LocalKey::new(LocalKind::Tuple, local.0),
        A::List(local) => list_local_expr_key(local),
        A::IntFunction { local, .. } => LocalKey::new(LocalKind::IntFunction, local.0),
        A::StringFunction { local, .. } => LocalKey::new(LocalKind::StringFunction, local.0),
        A::BitArrayFunction { local, .. } => LocalKey::new(LocalKind::BitArrayFunction, local.0),
        A::UtfCodepointFunction { local, .. } => {
            LocalKey::new(LocalKind::UtfCodepointFunction, local.0)
        }
        A::CustomFunction { local, .. } => LocalKey::new(LocalKind::CustomFunction, local.id().0),
        A::FloatFunction { local, .. } => LocalKey::new(LocalKind::FloatFunction, local.0),
        A::BoolFunction { local, .. } => LocalKey::new(LocalKind::BoolFunction, local.0),
        A::NilFunction { local, .. } => LocalKey::new(LocalKind::NilFunction, local.0),
        A::TupleFunction { local, .. } => LocalKey::new(LocalKind::TupleFunction, local.0),
        A::ListFunction { local, .. } => super::frame::list_function_local_key(local),
        A::FunctionFunction { local, .. } => {
            LocalKey::new(LocalKind::FunctionFunction, local.id().0)
        }
        A::GenericFunction { local, .. } => LocalKey::new(LocalKind::GenericFunction, local.id().0),
    }
}
pub(super) use bit_array::bit_array_expr;
