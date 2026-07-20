mod bit_array;
mod bool;
mod custom;
mod float;
mod generic;
mod int;
mod list;
mod never;
mod nil;
mod returning_function;
mod string;
mod tuple;
mod utf_codepoint;

pub(in crate::plan::execution::lowering) use bit_array::bit_array_function_expr;
pub(in crate::plan::execution::lowering) use bool::bool_function_expr;
pub(in crate::plan::execution::lowering) use custom::{
    custom_function_expr, custom_function_expr_kind, generic_custom_function_expr,
    generic_custom_function_expr_kind,
};
pub(in crate::plan::execution::lowering) use float::float_function_expr;
pub(in crate::plan::execution::lowering) use generic::symbolic_custom_function_expr_kind;
pub(in crate::plan::execution::lowering) use generic::symbolic_function_function_expr_kind;
pub(in crate::plan::execution::lowering) use generic::{
    generic_bit_array_function_expr, generic_bool_function_expr, generic_float_function_expr,
    generic_int_function_expr, generic_nil_function_expr, generic_string_function_expr,
    generic_symbolic_function_expr, generic_tuple_function_expr,
    generic_utf_codepoint_function_expr, symbolic_bit_array_function_expr,
    symbolic_bool_function_expr, symbolic_custom_function_expr, symbolic_float_function_expr,
    symbolic_function_function_expr, symbolic_int_function_expr, symbolic_list_function_expr,
    symbolic_nil_function_expr, symbolic_string_function_expr, symbolic_tuple_function_expr,
    symbolic_utf_codepoint_function_expr,
};
pub(in crate::plan::execution::lowering) use int::int_function_expr;
pub(in crate::plan::execution::lowering) use list::{
    generic_list_function_expr, list_function_expr,
};
pub(in crate::plan::execution::lowering) use never::{
    custom_never_function_expr, custom_never_function_expr_kind, generic_never_function_expr,
    generic_value_never_function_expr, tuple_never_function_expr,
};
pub(in crate::plan::execution::lowering) use nil::nil_function_expr;
pub(in crate::plan::execution::lowering) use returning_function::{
    function_function_expr, function_function_expr_kind, generic_function_function_expr,
    generic_function_function_expr_kind,
};
pub(in crate::plan::execution::lowering) use string::string_function_expr;
pub(in crate::plan::execution::lowering) use tuple::tuple_function_expr;
pub(in crate::plan::execution::lowering) use utf_codepoint::utf_codepoint_function_expr;

use super::super::specialization::Representability;
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) enum SpecializedFunctionBinding {
    Int {
        local: execution::IntFunctionLocalId,
        value: execution::TypedFunctionExpr<execution::IntFunctionExpr>,
    },
    Float {
        local: execution::FloatFunctionLocalId,
        value: execution::TypedFunctionExpr<execution::FloatFunctionExpr>,
    },
    String {
        local: execution::StringFunctionLocalId,
        value: execution::TypedFunctionExpr<execution::StringFunctionExpr>,
    },
    BitArray {
        local: execution::BitArrayFunctionLocalId,
        value: execution::TypedFunctionExpr<execution::BitArrayFunctionExpr>,
    },
    UtfCodepoint {
        local: execution::UtfCodepointFunctionLocalId,
        value: execution::TypedFunctionExpr<execution::UtfCodepointFunctionExpr>,
    },
    Generic {
        local: execution::GenericFunctionLocal,
        value: execution::TypedFunctionExpr<execution::GenericFunctionExpr>,
    },
    Never {
        local: execution::NeverFunctionLocal,
        value: execution::TypedFunctionExpr<execution::NeverFunctionExpr>,
    },
    Custom {
        local: execution::CustomFunctionLocal,
        value: execution::TypedFunctionExpr<execution::CustomFunctionExpr>,
    },
    Bool {
        local: execution::BoolFunctionLocalId,
        value: execution::TypedFunctionExpr<execution::BoolFunctionExpr>,
    },
    Nil {
        local: execution::NilFunctionLocalId,
        value: execution::TypedFunctionExpr<execution::NilFunctionExpr>,
    },
    Tuple {
        local: execution::TupleFunctionLocalId,
        value: execution::TypedFunctionExpr<execution::TupleFunctionExpr>,
    },
    List {
        local: execution::ListFunctionLocal,
        value: execution::TypedFunctionExpr<execution::ListFunctionExpr>,
    },
    Function {
        local: execution::FunctionFunctionLocal,
        value: execution::TypedFunctionExpr<execution::FunctionFunctionExpr>,
    },
}

pub(in crate::plan::execution::lowering) fn typed_function_expr<ModuleExpr, ExecutionExpr>(
    expression: &module::TypedFunctionExpr<ModuleExpr>,
    context: &mut super::super::LoweringContext,
    lower: impl FnOnce(
        &ModuleExpr,
        &mut super::super::LoweringContext,
    ) -> Representability<ExecutionExpr>,
) -> Representability<execution::TypedFunctionExpr<ExecutionExpr>> {
    let shape = context.function_shape(expression.shape().clone());
    lower(expression.expression(), context)
        .map(|expression| execution::TypedFunctionExpr::new(shape, expression))
}

pub(super) fn symbolic_typed_function_binding<ModuleExpr>(
    index: usize,
    expression: &module::TypedFunctionExpr<ModuleExpr>,
    concrete: super::super::specialization::SpecializedFunctionShape,
    context: &mut super::super::LoweringContext,
    lower: impl FnOnce(
        &ModuleExpr,
        &super::super::specialization::SpecializedFunctionShape,
        &mut super::super::LoweringContext,
    ) -> Representability<execution::GenericFunctionExpr>,
) -> Representability<SpecializedFunctionBinding> {
    let shape = context.lower_concrete_function_shape(&concrete);
    lower(expression.expression(), &concrete, context).map(|value| {
        SpecializedFunctionBinding::Generic {
            local: execution::GenericFunctionLocal::new(
                execution::GenericFunctionLocalId(index),
                value.generic_function_type().clone(),
            ),
            value: execution::TypedFunctionExpr::new(shape, value),
        }
    })
}

pub(super) fn specialized_function_binding(
    index: usize,
    expression: &module::FunctionExpr,
    context: &mut super::super::LoweringContext,
) -> Representability<SpecializedFunctionBinding> {
    let concrete = context.concrete_function_shape(expression.shape());
    specialized_function_binding_for_shape(index, expression, concrete, context)
}

pub(super) fn specialized_function_binding_for_shape(
    index: usize,
    expression: &module::FunctionExpr,
    concrete: super::super::specialization::SpecializedFunctionShape,
    context: &mut super::super::LoweringContext,
) -> Representability<SpecializedFunctionBinding> {
    let shape = context.lower_concrete_function_shape(&concrete);
    let return_ = match context.function_representation(&concrete) {
        super::super::specialization::FunctionRepresentation::Symbolic => {
            return symbolic_function_expr(expression, &concrete, context).map(|value| {
                SpecializedFunctionBinding::Generic {
                    local: execution::GenericFunctionLocal::new(
                        execution::GenericFunctionLocalId(index),
                        value.generic_function_type().clone(),
                    ),
                    value: execution::TypedFunctionExpr::new(shape, value),
                }
            });
        }
        super::super::specialization::FunctionRepresentation::Never(_) => {
            return never_function_expr(expression, context).map(|value| {
                SpecializedFunctionBinding::Never {
                    local: execution::NeverFunctionLocal::new(
                        execution::NeverFunctionLocalId(index),
                        value.type_().clone(),
                    ),
                    value: execution::TypedFunctionExpr::new(shape, value),
                }
            });
        }
        super::super::specialization::FunctionRepresentation::Executable(return_) => return_,
    };

    match expression.kind() {
        module::FunctionExprKind::Generic(expression) => {
            specialized_executable_generic_function_binding(
                index, expression, concrete, return_, context,
            )
        }
        module::FunctionExprKind::Int(expression) => {
            int_function_expr(expression, context).map(|value| SpecializedFunctionBinding::Int {
                local: execution::IntFunctionLocalId(index),
                value: execution::TypedFunctionExpr::new(shape, value),
            })
        }
        module::FunctionExprKind::Float(expression) => float_function_expr(expression, context)
            .map(|value| SpecializedFunctionBinding::Float {
                local: execution::FloatFunctionLocalId(index),
                value: execution::TypedFunctionExpr::new(shape, value),
            }),
        module::FunctionExprKind::String(expression) => string_function_expr(expression, context)
            .map(|value| SpecializedFunctionBinding::String {
                local: execution::StringFunctionLocalId(index),
                value: execution::TypedFunctionExpr::new(shape, value),
            }),
        module::FunctionExprKind::BitArray(expression) => {
            bit_array_function_expr(expression, context).map(|value| {
                SpecializedFunctionBinding::BitArray {
                    local: execution::BitArrayFunctionLocalId(index),
                    value: execution::TypedFunctionExpr::new(shape, value),
                }
            })
        }
        module::FunctionExprKind::UtfCodepoint(expression) => {
            utf_codepoint_function_expr(expression, context).map(|value| {
                SpecializedFunctionBinding::UtfCodepoint {
                    local: execution::UtfCodepointFunctionLocalId(index),
                    value: execution::TypedFunctionExpr::new(shape, value),
                }
            })
        }
        module::FunctionExprKind::Custom(expression) => custom_function_expr(expression, context)
            .map(|value| {
                let local = execution::CustomFunctionLocal::new(
                    execution::CustomFunctionLocalId(index),
                    value.custom_function_type().clone(),
                );
                SpecializedFunctionBinding::Custom {
                    local,
                    value: execution::TypedFunctionExpr::new(shape, value),
                }
            }),
        module::FunctionExprKind::Bool(expression) => {
            bool_function_expr(expression, context).map(|value| SpecializedFunctionBinding::Bool {
                local: execution::BoolFunctionLocalId(index),
                value: execution::TypedFunctionExpr::new(shape, value),
            })
        }
        module::FunctionExprKind::Nil(expression) => {
            nil_function_expr(expression, context).map(|value| SpecializedFunctionBinding::Nil {
                local: execution::NilFunctionLocalId(index),
                value: execution::TypedFunctionExpr::new(shape, value),
            })
        }
        module::FunctionExprKind::Tuple(expression) => tuple_function_expr(expression, context)
            .map(|value| SpecializedFunctionBinding::Tuple {
                local: execution::TupleFunctionLocalId(index),
                value: execution::TypedFunctionExpr::new(shape, value),
            }),
        module::FunctionExprKind::List(expression) => {
            let item = context.concrete_value_shape(&crate::plan::ValueShape::from_value_type(
                expression.return_item_type(),
            ));
            let type_ = shape.type_().clone();
            list_function_expr(expression, context).map(|value| SpecializedFunctionBinding::List {
                local: super::super::frame::list_function_local_at(&item, type_, index, context),
                value: execution::TypedFunctionExpr::new(shape, value),
            })
        }
        module::FunctionExprKind::Function(expression) => {
            function_function_expr(expression, context).map(|value| {
                let local = execution::FunctionFunctionLocal::new(
                    execution::FunctionFunctionLocalId(index),
                    value.function_function_type().clone(),
                );
                SpecializedFunctionBinding::Function {
                    local,
                    value: execution::TypedFunctionExpr::new(shape, value),
                }
            })
        }
    }
}

fn never_function_expr(
    expression: &module::FunctionExpr,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::NeverFunctionExpr> {
    match expression.kind() {
        module::FunctionExprKind::Generic(expression) => {
            generic_never_function_expr(expression, context)
        }
        module::FunctionExprKind::Tuple(expression) => {
            tuple_never_function_expr(expression, context)
        }
        module::FunctionExprKind::Custom(expression) => {
            custom_never_function_expr(expression, context)
        }
        _ => Representability::Uninhabited,
    }
}

fn symbolic_function_expr(
    expression: &module::FunctionExpr,
    shape: &super::super::specialization::SpecializedFunctionShape,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::GenericFunctionExpr> {
    match expression.kind() {
        module::FunctionExprKind::Generic(expression) => {
            generic_symbolic_function_expr(expression, context)
        }
        module::FunctionExprKind::Int(expression) => {
            symbolic_int_function_expr(expression, shape, context)
        }
        module::FunctionExprKind::Float(expression) => {
            symbolic_float_function_expr(expression, shape, context)
        }
        module::FunctionExprKind::String(expression) => {
            symbolic_string_function_expr(expression, shape, context)
        }
        module::FunctionExprKind::BitArray(expression) => {
            symbolic_bit_array_function_expr(expression, shape, context)
        }
        module::FunctionExprKind::UtfCodepoint(expression) => {
            symbolic_utf_codepoint_function_expr(expression, shape, context)
        }
        module::FunctionExprKind::Custom(expression) => {
            symbolic_custom_function_expr(expression, shape, context)
        }
        module::FunctionExprKind::Bool(expression) => {
            symbolic_bool_function_expr(expression, shape, context)
        }
        module::FunctionExprKind::Nil(expression) => {
            symbolic_nil_function_expr(expression, shape, context)
        }
        module::FunctionExprKind::Tuple(expression) => {
            symbolic_tuple_function_expr(expression, shape, context)
        }
        module::FunctionExprKind::List(expression) => {
            symbolic_list_function_expr(expression, shape, context)
        }
        module::FunctionExprKind::Function(expression) => {
            symbolic_function_function_expr(expression, shape, context)
        }
    }
}

macro_rules! evaluated_primitive_function_expr {
    ($name:ident, $module:ident, $symbolic:ident) => {
        pub(in crate::plan::execution::lowering) fn $name(
            expression: &module::$module,
            context: &mut super::super::LoweringContext,
        ) -> Representability<execution::FunctionExpr> {
            let concrete = context.concrete_function_shape(
                &crate::plan::FunctionShape::from_function_type(expression.type_().clone()),
            );
            let shape = context.lower_concrete_function_shape(&concrete);
            $symbolic(expression, &concrete, context).map(|expression| {
                execution::FunctionExpr::from_parts(
                    shape,
                    execution::FunctionExprKind::Generic(expression),
                )
            })
        }
    };
}

evaluated_primitive_function_expr!(
    evaluated_int_function_expr,
    IntFunctionExpr,
    symbolic_int_function_expr
);
evaluated_primitive_function_expr!(
    evaluated_float_function_expr,
    FloatFunctionExpr,
    symbolic_float_function_expr
);
evaluated_primitive_function_expr!(
    evaluated_string_function_expr,
    StringFunctionExpr,
    symbolic_string_function_expr
);
evaluated_primitive_function_expr!(
    evaluated_bit_array_function_expr,
    BitArrayFunctionExpr,
    symbolic_bit_array_function_expr
);
evaluated_primitive_function_expr!(
    evaluated_utf_codepoint_function_expr,
    UtfCodepointFunctionExpr,
    symbolic_utf_codepoint_function_expr
);
evaluated_primitive_function_expr!(
    evaluated_bool_function_expr,
    BoolFunctionExpr,
    symbolic_bool_function_expr
);
evaluated_primitive_function_expr!(
    evaluated_nil_function_expr,
    NilFunctionExpr,
    symbolic_nil_function_expr
);
pub(in crate::plan::execution::lowering) fn evaluated_tuple_function_expr(
    expression: &module::TupleFunctionExpr,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::FunctionExpr> {
    let concrete = context.concrete_function_shape(
        &crate::plan::FunctionShape::from_function_type(expression.type_().clone()),
    );
    let shape = context.lower_concrete_function_shape(&concrete);
    symbolic_tuple_function_expr(expression, &concrete, context).map(|expression| {
        execution::FunctionExpr::from_parts(shape, execution::FunctionExprKind::Generic(expression))
    })
}

pub(in crate::plan::execution::lowering) fn evaluated_custom_function_expr(
    expression: &module::CustomFunctionExpr,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::FunctionExpr> {
    let concrete =
        context.concrete_function_shape(&crate::plan::FunctionShape::from_function_type(
            expression.custom_function_type().to_function_type(),
        ));
    let shape = context.lower_concrete_function_shape(&concrete);
    symbolic_custom_function_expr(expression, &concrete, context).map(|expression| {
        execution::FunctionExpr::from_parts(shape, execution::FunctionExprKind::Generic(expression))
    })
}

pub(in crate::plan::execution::lowering) fn evaluated_list_function_expr(
    expression: &module::ListFunctionExpr,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::FunctionExpr> {
    let concrete = context.concrete_function_shape(
        &crate::plan::FunctionShape::from_function_type(expression.type_().clone()),
    );
    let shape = context.lower_concrete_function_shape(&concrete);
    symbolic_list_function_expr(expression, &concrete, context).map(|expression| {
        execution::FunctionExpr::from_parts(shape, execution::FunctionExprKind::Generic(expression))
    })
}

pub(in crate::plan::execution::lowering) fn evaluated_function_function_expr(
    expression: &module::FunctionFunctionExpr,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::FunctionExpr> {
    let concrete =
        context.concrete_function_shape(&crate::plan::FunctionShape::from_function_type(
            expression.function_function_type().to_function_type(),
        ));
    let shape = context.lower_concrete_function_shape(&concrete);
    symbolic_function_function_expr(expression, &concrete, context).map(|expression| {
        execution::FunctionExpr::from_parts(shape, execution::FunctionExprKind::Generic(expression))
    })
}

pub(in crate::plan::execution::lowering) fn evaluated_generic_function_expr(
    expression: &module::GenericFunctionExpr,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::FunctionExpr> {
    let concrete = context.concrete_function_shape(&expression.shape());
    let shape = context.lower_concrete_function_shape(&concrete);
    generic_symbolic_function_expr(expression, context).map(|expression| {
        execution::FunctionExpr::from_parts(shape, execution::FunctionExprKind::Generic(expression))
    })
}

pub(super) fn specialized_typed_generic_function_binding(
    index: usize,
    expression: &module::TypedFunctionExpr<module::GenericFunctionExpr>,
    context: &mut super::super::LoweringContext,
) -> Representability<SpecializedFunctionBinding> {
    specialized_generic_function_binding(index, expression.expression(), context)
}

pub(super) fn specialized_typed_generic_function_binding_for_shape(
    index: usize,
    expression: &module::TypedFunctionExpr<module::GenericFunctionExpr>,
    concrete: super::super::specialization::SpecializedFunctionShape,
    context: &mut super::super::LoweringContext,
) -> Representability<SpecializedFunctionBinding> {
    specialized_generic_function_binding_for_shape(
        index,
        expression.expression(),
        concrete,
        context,
    )
}

pub(in crate::plan::execution::lowering) fn specialized_typed_tuple_function_binding(
    index: usize,
    expression: &module::TypedFunctionExpr<module::TupleFunctionExpr>,
    context: &mut super::super::LoweringContext,
) -> Representability<SpecializedFunctionBinding> {
    let concrete = context.concrete_function_shape(expression.shape());
    let shape = context.lower_concrete_function_shape(&concrete);
    match context.function_representation(&concrete) {
        super::super::specialization::FunctionRepresentation::Symbolic => {
            symbolic_tuple_function_expr(expression.expression(), &concrete, context).map(|value| {
                SpecializedFunctionBinding::Generic {
                    local: execution::GenericFunctionLocal::new(
                        execution::GenericFunctionLocalId(index),
                        value.generic_function_type().clone(),
                    ),
                    value: execution::TypedFunctionExpr::new(shape, value),
                }
            })
        }
        super::super::specialization::FunctionRepresentation::Never(_) => {
            tuple_never_function_expr(expression.expression(), context).map(|value| {
                SpecializedFunctionBinding::Never {
                    local: execution::NeverFunctionLocal::new(
                        execution::NeverFunctionLocalId(index),
                        value.type_().clone(),
                    ),
                    value: execution::TypedFunctionExpr::new(shape, value),
                }
            })
        }
        super::super::specialization::FunctionRepresentation::Executable(_) => {
            tuple_function_expr(expression.expression(), context).map(|value| {
                SpecializedFunctionBinding::Tuple {
                    local: execution::TupleFunctionLocalId(index),
                    value: execution::TypedFunctionExpr::new(shape, value),
                }
            })
        }
    }
}

pub(in crate::plan::execution::lowering) fn specialized_typed_custom_function_binding(
    index: usize,
    expression: &module::TypedFunctionExpr<module::CustomFunctionExpr>,
    context: &mut super::super::LoweringContext,
) -> Representability<SpecializedFunctionBinding> {
    let concrete = context.concrete_function_shape(expression.shape());
    let shape = context.lower_concrete_function_shape(&concrete);
    match context.function_representation(&concrete) {
        super::super::specialization::FunctionRepresentation::Symbolic => {
            symbolic_custom_function_expr(expression.expression(), &concrete, context).map(
                |value| SpecializedFunctionBinding::Generic {
                    local: execution::GenericFunctionLocal::new(
                        execution::GenericFunctionLocalId(index),
                        value.generic_function_type().clone(),
                    ),
                    value: execution::TypedFunctionExpr::new(shape, value),
                },
            )
        }
        super::super::specialization::FunctionRepresentation::Never(_) => {
            custom_never_function_expr(expression.expression(), context).map(|value| {
                SpecializedFunctionBinding::Never {
                    local: execution::NeverFunctionLocal::new(
                        execution::NeverFunctionLocalId(index),
                        value.type_().clone(),
                    ),
                    value: execution::TypedFunctionExpr::new(shape, value),
                }
            })
        }
        super::super::specialization::FunctionRepresentation::Executable(_) => {
            custom_function_expr(expression.expression(), context).map(|value| {
                SpecializedFunctionBinding::Custom {
                    local: execution::CustomFunctionLocal::new(
                        execution::CustomFunctionLocalId(index),
                        value.custom_function_type().clone(),
                    ),
                    value: execution::TypedFunctionExpr::new(shape, value),
                }
            })
        }
    }
}

fn specialized_generic_function_binding(
    index: usize,
    expression: &module::GenericFunctionExpr,
    context: &mut super::super::LoweringContext,
) -> Representability<SpecializedFunctionBinding> {
    let concrete = context.concrete_function_shape(&expression.shape());
    specialized_generic_function_binding_for_shape(index, expression, concrete, context)
}

fn specialized_generic_function_binding_for_shape(
    index: usize,
    expression: &module::GenericFunctionExpr,
    concrete: super::super::specialization::SpecializedFunctionShape,
    context: &mut super::super::LoweringContext,
) -> Representability<SpecializedFunctionBinding> {
    use super::super::specialization::FunctionRepresentation;

    let shape = context.lower_concrete_function_shape(&concrete);
    match context.function_representation(&concrete) {
        FunctionRepresentation::Symbolic => generic_symbolic_function_expr(expression, context)
            .map(|value| SpecializedFunctionBinding::Generic {
                local: execution::GenericFunctionLocal::new(
                    execution::GenericFunctionLocalId(index),
                    value.generic_function_type().clone(),
                ),
                value: execution::TypedFunctionExpr::new(shape, value),
            }),
        FunctionRepresentation::Never(_) => {
            generic_never_function_expr(expression, context).map(|value| {
                SpecializedFunctionBinding::Never {
                    local: execution::NeverFunctionLocal::new(
                        execution::NeverFunctionLocalId(index),
                        value.type_().clone(),
                    ),
                    value: execution::TypedFunctionExpr::new(shape, value),
                }
            })
        }
        FunctionRepresentation::Executable(return_) => {
            specialized_executable_generic_function_binding(
                index, expression, concrete, return_, context,
            )
        }
    }
}

fn specialized_executable_generic_function_binding(
    index: usize,
    expression: &module::GenericFunctionExpr,
    concrete: super::super::specialization::SpecializedFunctionShape,
    return_: super::super::specialization::StoredValueShape,
    context: &mut super::super::LoweringContext,
) -> Representability<SpecializedFunctionBinding> {
    use super::super::specialization::StoredValueShape as S;

    let shape = context.lower_concrete_function_shape(&concrete);
    match return_ {
        S::Int => generic_int_function_expr(expression, context).map(|value| {
            SpecializedFunctionBinding::Int {
                local: execution::IntFunctionLocalId(index),
                value: execution::TypedFunctionExpr::new(shape, value),
            }
        }),
        S::Float => generic_float_function_expr(expression, context).map(|value| {
            SpecializedFunctionBinding::Float {
                local: execution::FloatFunctionLocalId(index),
                value: execution::TypedFunctionExpr::new(shape, value),
            }
        }),
        S::String => generic_string_function_expr(expression, context).map(|value| {
            SpecializedFunctionBinding::String {
                local: execution::StringFunctionLocalId(index),
                value: execution::TypedFunctionExpr::new(shape, value),
            }
        }),
        S::BitArray => generic_bit_array_function_expr(expression, context).map(|value| {
            SpecializedFunctionBinding::BitArray {
                local: execution::BitArrayFunctionLocalId(index),
                value: execution::TypedFunctionExpr::new(shape, value),
            }
        }),
        S::UtfCodepoint => generic_utf_codepoint_function_expr(expression, context).map(|value| {
            SpecializedFunctionBinding::UtfCodepoint {
                local: execution::UtfCodepointFunctionLocalId(index),
                value: execution::TypedFunctionExpr::new(shape, value),
            }
        }),
        S::Custom(return_) => {
            generic_custom_function_expr(expression, &return_, context).map(|value| {
                let local = execution::CustomFunctionLocal::new(
                    execution::CustomFunctionLocalId(index),
                    value.custom_function_type().clone(),
                );
                SpecializedFunctionBinding::Custom {
                    local,
                    value: execution::TypedFunctionExpr::new(shape, value),
                }
            })
        }
        S::Bool => generic_bool_function_expr(expression, context).map(|value| {
            SpecializedFunctionBinding::Bool {
                local: execution::BoolFunctionLocalId(index),
                value: execution::TypedFunctionExpr::new(shape, value),
            }
        }),
        S::Nil => generic_nil_function_expr(expression, context).map(|value| {
            SpecializedFunctionBinding::Nil {
                local: execution::NilFunctionLocalId(index),
                value: execution::TypedFunctionExpr::new(shape, value),
            }
        }),
        S::Tuple(_) => generic_tuple_function_expr(expression, context).map(|value| {
            SpecializedFunctionBinding::Tuple {
                local: execution::TupleFunctionLocalId(index),
                value: execution::TypedFunctionExpr::new(shape, value),
            }
        }),
        S::List(item) => {
            let type_ = shape.type_().clone();
            generic_list_function_expr(expression, &item, context).map(|value| {
                SpecializedFunctionBinding::List {
                    local: super::super::frame::list_function_local_at(
                        &item, type_, index, context,
                    ),
                    value: execution::TypedFunctionExpr::new(shape, value),
                }
            })
        }
        S::Function(return_) => {
            generic_function_function_expr(expression, &return_, context).map(|value| {
                let local = execution::FunctionFunctionLocal::new(
                    execution::FunctionFunctionLocalId(index),
                    value.function_function_type().clone(),
                );
                SpecializedFunctionBinding::Function {
                    local,
                    value: execution::TypedFunctionExpr::new(shape, value),
                }
            })
        }
    }
}

fn function_reference<ModuleFunction, ExecutionFunction>(
    reference: &module::TypedFunctionReference<ModuleFunction>,
    context: &mut super::super::LoweringContext,
    lower_function: impl FnOnce(
        &module::FunctionInstantiation,
        &mut super::super::LoweringContext,
    ) -> Representability<ExecutionFunction>,
) -> Representability<execution::FunctionReference<ExecutionFunction>> {
    let params = reference
        .params()
        .iter()
        .map(|param| {
            crate::plan::execution::lowering::param::target_param_slot(
                reference.instantiation(),
                param,
                context,
            )
        })
        .collect();
    lower_function(reference.instantiation(), context)
        .map(|function| execution::FunctionReference::new(function, params))
}

fn closure_template<ExecutionFunction>(
    function: &module::FunctionInstantiation,
    params: &[module::ParamSlot],
    captures: &[module::CaptureArg],
    context: &mut super::super::LoweringContext,
    lower_function: impl FnOnce(
        &module::FunctionInstantiation,
        &mut super::super::LoweringContext,
    ) -> Representability<ExecutionFunction>,
) -> Representability<execution::ClosureTemplate<ExecutionFunction>> {
    let params = params
        .iter()
        .map(|param| {
            crate::plan::execution::lowering::param::target_param_slot(function, param, context)
        })
        .collect();
    super::capture_args(function, captures, context).and_then(|captures| {
        lower_function(function, context)
            .map(|function| execution::ClosureTemplate::new(function, params, captures))
    })
}

pub(in crate::plan::execution::lowering) fn function_expr(
    expression: &module::FunctionExpr,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::FunctionExpr> {
    let concrete = context.concrete_function_shape(expression.shape());
    let shape = context.lower_concrete_function_shape(&concrete);
    let return_ = match context.function_representation(&concrete) {
        super::super::specialization::FunctionRepresentation::Symbolic => {
            return symbolic_function_expr(expression, &concrete, context).map(|expression| {
                execution::FunctionExpr::from_parts(
                    shape,
                    execution::FunctionExprKind::Generic(expression),
                )
            });
        }
        super::super::specialization::FunctionRepresentation::Never(_) => {
            return never_function_expr(expression, context).map(|expression| {
                execution::FunctionExpr::from_parts(
                    shape,
                    execution::FunctionExprKind::Never(expression),
                )
            });
        }
        super::super::specialization::FunctionRepresentation::Executable(return_) => return_,
    };

    let kind = match expression.kind() {
        module::FunctionExprKind::Int(expression) => {
            int_function_expr(expression, context).map(execution::FunctionExprKind::Int)
        }
        module::FunctionExprKind::String(expression) => {
            string_function_expr(expression, context).map(execution::FunctionExprKind::String)
        }
        module::FunctionExprKind::BitArray(expression) => {
            bit_array_function_expr(expression, context).map(execution::FunctionExprKind::BitArray)
        }
        module::FunctionExprKind::UtfCodepoint(expression) => {
            utf_codepoint_function_expr(expression, context)
                .map(execution::FunctionExprKind::UtfCodepoint)
        }
        module::FunctionExprKind::Custom(expression) => {
            custom_function_expr(expression, context).map(execution::FunctionExprKind::Custom)
        }
        module::FunctionExprKind::Float(expression) => {
            float_function_expr(expression, context).map(execution::FunctionExprKind::Float)
        }
        module::FunctionExprKind::Bool(expression) => {
            bool_function_expr(expression, context).map(execution::FunctionExprKind::Bool)
        }
        module::FunctionExprKind::Nil(expression) => {
            nil_function_expr(expression, context).map(execution::FunctionExprKind::Nil)
        }
        module::FunctionExprKind::Tuple(expression) => {
            tuple_function_expr(expression, context).map(execution::FunctionExprKind::Tuple)
        }
        module::FunctionExprKind::List(expression) => {
            list_function_expr(expression, context).map(execution::FunctionExprKind::List)
        }
        module::FunctionExprKind::Function(expression) => {
            function_function_expr(expression, context).map(execution::FunctionExprKind::Function)
        }
        module::FunctionExprKind::Generic(expression) => {
            return lower_generic_function_expr(expression, return_, context);
        }
    };
    kind.map(|kind| execution::FunctionExpr::from_parts(shape, kind))
}

fn lower_generic_function_expr(
    expression: &module::GenericFunctionExpr,
    return_: super::super::specialization::StoredValueShape,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::FunctionExpr> {
    use super::super::specialization::StoredValueShape as S;

    let shape = context.concrete_function_shape(&expression.shape());
    let lowered_shape = context.lower_concrete_function_shape(&shape);
    let kind = match return_ {
        S::Int => {
            generic_int_function_expr(expression, context).map(execution::FunctionExprKind::Int)
        }
        S::String => generic_string_function_expr(expression, context)
            .map(execution::FunctionExprKind::String),
        S::BitArray => generic_bit_array_function_expr(expression, context)
            .map(execution::FunctionExprKind::BitArray),
        S::UtfCodepoint => generic_utf_codepoint_function_expr(expression, context)
            .map(execution::FunctionExprKind::UtfCodepoint),
        S::Custom(return_shape) => generic_custom_function_expr(expression, &return_shape, context)
            .map(execution::FunctionExprKind::Custom),
        S::Float => {
            generic_float_function_expr(expression, context).map(execution::FunctionExprKind::Float)
        }
        S::Bool => {
            generic_bool_function_expr(expression, context).map(execution::FunctionExprKind::Bool)
        }
        S::Nil => {
            generic_nil_function_expr(expression, context).map(execution::FunctionExprKind::Nil)
        }
        S::Tuple(_) => {
            generic_tuple_function_expr(expression, context).map(execution::FunctionExprKind::Tuple)
        }
        S::List(item) => generic_list_function_expr(expression, &item, context)
            .map(execution::FunctionExprKind::List),
        S::Function(return_shape) => {
            generic_function_function_expr(expression, &return_shape, context)
                .map(execution::FunctionExprKind::Function)
        }
    };
    kind.map(|kind| execution::FunctionExpr::from_parts(lowered_shape, kind))
}

#[cfg(test)]
mod tests {
    use super::super::super::super::{
        CallArg, CallArgKind, CaptureArg, CaptureArgKind, ClosureTemplate, ExecutionPlan,
        FunctionCall, FunctionReference, IntExpr, IntExprKind, IntFunctionExpr,
        IntFunctionExprKind, IntFunctionFunctionId, IntFunctionId, IntFunctionLocalId, IntLocalId,
        ParamLocal, ReturnBlock, ReturnGraph, Step, StepKind, StringExpr, StringExprKind,
        StringLocalId,
    };
    use super::super::super::specialization::{
        Representability, RepresentationContext, SpecializationKey, UninhabitedValueShape,
    };
    use super::super::super::{FunctionTemplates, LoweringContext};
    use num_bigint::BigInt;
    use std::collections::HashSet;

    #[test]
    fn never_dispatch_rejects_storable_expression_families() {
        let main_id = crate::plan::FunctionTemplateId::new(0);
        let main = crate::plan::FunctionTemplate::new(
            main_id,
            "main".into(),
            Vec::new(),
            Vec::new(),
            crate::plan::ReturnExpr::int(
                crate::plan::IntFunctionId(0),
                crate::plan::IntExpr::value(0.into()),
            ),
        );
        let templates = FunctionTemplates::new(main, Vec::new(), Vec::new());
        let mut context = LoweringContext::new(
            &templates,
            SpecializationKey::monomorphic(main_id),
            RepresentationContext::new(Vec::new()),
            crate::plan::ConstantTemplates::from_entries(Vec::new()),
            HashSet::new(),
        );
        let function = crate::plan::FunctionExpr::int(crate::plan::IntFunctionExpr::panic(
            crate::plan::PanicExpr::panic_at(None, crate::plan::PanicSite::unknown()),
            crate::plan::FunctionType::new(Vec::new(), crate::plan::ValueType::Int),
        ));
        let value = crate::plan::Expr::int(crate::plan::IntExpr::value(1.into()));

        assert_eq!(
            super::never_function_expr(&function, &mut context).map(|_| ()),
            Representability::Uninhabited,
        );
        assert_eq!(
            super::super::never::uninhabited_expr(
                &value,
                &UninhabitedValueShape::Parameter(crate::plan::TypeParameterId(0)),
                &mut context,
            )
            .map(|_| ()),
            Representability::Uninhabited,
        );
    }

    #[test]
    fn lowering_separates_int_function_reference_and_closure_lifecycles() {
        let plan = reference_closure_execution_plan();
        let main = plan.int_function(IntFunctionId(0));

        assert_eq!(main.steps().len(), 3);

        let (captured_local, captured_value) = expect_int_binding(&main.steps()[0]);
        assert_eq!(captured_local, IntLocalId(0));
        assert_eq!(expect_int_value(captured_value), &BigInt::from(1));

        let (reference_local, reference_value) = expect_int_function_binding(&main.steps()[1]);
        assert_eq!(reference_local, IntFunctionLocalId(0));
        let reference = expect_int_function_reference(reference_value);
        assert_eq!(reference.function(), &IntFunctionId(1));
        assert_eq!(reference.params().len(), 1);
        assert_eq!(
            reference.params()[0].local(),
            &ParamLocal::Int(IntLocalId(0))
        );
        assert_eq!(
            plan.value_type(&plan.shape_value_type(reference.params()[0].shape())),
            crate::plan::ValueType::Int,
        );

        let (closure_local, closure_value) = expect_int_function_binding(&main.steps()[2]);
        assert_eq!(closure_local, IntFunctionLocalId(1));
        let closure = expect_int_function_closure(closure_value);
        assert_eq!(closure.function(), &IntFunctionId(2));
        assert_eq!(closure.params().len(), 1);
        assert_eq!(closure.params()[0].local(), &ParamLocal::Int(IntLocalId(0)));
        assert_eq!(
            plan.value_type(&plan.shape_value_type(closure.params()[0].shape())),
            crate::plan::ValueType::Int,
        );
        assert_eq!(closure.captures().len(), 1);
        let (capture_local, capture_value) = expect_int_capture(&closure.captures()[0]);
        assert_eq!(capture_local, IntLocalId(1));
        assert_eq!(expect_int_local_get(capture_value), IntLocalId(0));

        let returned = expect_expression_return(main.return_());
        let (reference_call, reference_args) = expect_int_function_call(returned);
        assert_eq!(
            expect_int_function_local_get(reference_call),
            IntFunctionLocalId(0)
        );
        assert_eq!(reference_args.len(), 1);
        let (reference_arg_local, reference_argument) = expect_int_call_arg(&reference_args[0]);
        assert_eq!(reference_arg_local, IntLocalId(0));
        let (closure_call, closure_args) = expect_int_function_call(reference_argument);
        assert_eq!(
            expect_int_function_local_get(closure_call),
            IntFunctionLocalId(1)
        );
        assert_eq!(closure_args.len(), 1);
        let (closure_arg_local, closure_argument) = expect_int_call_arg(&closure_args[0]);
        assert_eq!(closure_arg_local, IntLocalId(0));
        assert_eq!(expect_int_value(closure_argument), &BigInt::from(40));
    }

    #[test]
    #[should_panic(expected = "expected an Int binding step")]
    fn int_binding_fixture_guard_rejects_function_binding() {
        let plan = reference_closure_execution_plan();
        let _ = expect_int_binding(&plan.int_function(IntFunctionId(0)).steps()[1]);
    }

    #[test]
    #[should_panic(expected = "expected an Int function binding step")]
    fn int_function_binding_fixture_guard_rejects_int_binding() {
        let plan = reference_closure_execution_plan();
        let _ = expect_int_function_binding(&plan.int_function(IntFunctionId(0)).steps()[0]);
    }

    #[test]
    #[should_panic(expected = "expected an Int function reference")]
    fn int_function_reference_fixture_guard_rejects_closure() {
        let plan = reference_closure_execution_plan();
        let (_, expression) =
            expect_int_function_binding(&plan.int_function(IntFunctionId(0)).steps()[2]);
        let _ = expect_int_function_reference(expression);
    }

    #[test]
    #[should_panic(expected = "expected an Int function closure")]
    fn int_function_closure_fixture_guard_rejects_reference() {
        let plan = reference_closure_execution_plan();
        let (_, expression) =
            expect_int_function_binding(&plan.int_function(IntFunctionId(0)).steps()[1]);
        let _ = expect_int_function_closure(expression);
    }

    #[test]
    #[should_panic(expected = "expected an Int capture")]
    fn int_capture_fixture_guard_rejects_function_capture() {
        let source = r#"
pub fn main() {
  let captured = fn() { 1 }
  let closure = fn() { captured() }
  closure()
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);
        let (_, closure_expression) =
            expect_int_function_binding(&plan.int_function(IntFunctionId(0)).steps()[1]);
        let closure = expect_int_function_closure(closure_expression);

        let _ = expect_int_capture(&closure.captures()[0]);
    }

    #[test]
    #[should_panic(expected = "expected an Int function call")]
    fn int_function_call_fixture_guard_rejects_value() {
        let plan = reference_closure_execution_plan();
        let (_, value) = expect_int_binding(&plan.int_function(IntFunctionId(0)).steps()[0]);
        let _ = expect_int_function_call(value);
    }

    #[test]
    #[should_panic(expected = "expected an Int call argument")]
    fn int_call_argument_fixture_guard_rejects_string_argument() {
        let argument = CallArg::from_kind(CallArgKind::String {
            local: StringLocalId(0),
            value: StringExpr::from_kind(StringExprKind::Value("value".into())),
        });

        let _ = expect_int_call_arg(&argument);
    }

    #[test]
    #[should_panic(expected = "expected an Int function local get")]
    fn int_function_local_get_fixture_guard_rejects_reference() {
        let plan = reference_closure_execution_plan();
        let (_, reference) =
            expect_int_function_binding(&plan.int_function(IntFunctionId(0)).steps()[1]);
        let _ = expect_int_function_local_get(reference);
    }

    #[test]
    #[should_panic(expected = "expected an Int local get")]
    fn int_local_get_fixture_guard_rejects_value() {
        let plan = reference_closure_execution_plan();
        let (_, value) = expect_int_binding(&plan.int_function(IntFunctionId(0)).steps()[0]);
        let _ = expect_int_local_get(value);
    }

    #[test]
    #[should_panic(expected = "expected an Int value")]
    fn int_value_fixture_guard_rejects_local_get() {
        let plan = reference_closure_execution_plan();
        let (_, closure_expression) =
            expect_int_function_binding(&plan.int_function(IntFunctionId(0)).steps()[2]);
        let capture = expect_int_function_closure(closure_expression)
            .captures()
            .first()
            .expect("fixture should contain one capture");
        let (_, value) = expect_int_capture(capture);
        let _ = expect_int_value(value);
    }

    #[test]
    #[should_panic(expected = "expected an expression return body")]
    fn int_expression_return_fixture_guard_rejects_case_return() {
        let source = r#"
pub fn main() {
  case 1 == 1 {
    True -> 1
    False -> 0
  }
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        let _ = expect_expression_return(plan.int_function(IntFunctionId(0)).return_());
    }

    #[test]
    #[should_panic(expected = "expected an expression return body")]
    fn int_function_expression_return_fixture_guard_rejects_case_return() {
        let source = r#"
fn identity(value: Int) { value }

fn choose() {
  case 1 == 1 {
    True -> identity
    False -> identity
  }
}

pub fn main() { choose()(1) }
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        let _ = expect_expression_return(
            plan.int_function_function(IntFunctionFunctionId(0))
                .return_()
                .body(),
        );
    }

    fn reference_closure_execution_plan() -> ExecutionPlan {
        let source = r#"
fn identity(value: Int) { value }

pub fn main() {
  let captured = 1
  let reference = identity
  let closure = fn(value) { value + captured }
  reference(closure(40))
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        ExecutionPlan::from_module_plan(module_plan)
    }

    fn expect_int_binding(step: &Step) -> (IntLocalId, &IntExpr) {
        match step.kind() {
            StepKind::LetInt { local, value } => (*local, value),
            _ => panic!("expected an Int binding step"),
        }
    }

    fn expect_int_function_binding(step: &Step) -> (IntFunctionLocalId, &IntFunctionExpr) {
        match step.kind() {
            StepKind::LetIntFunction { local, value } => (*local, value.expression()),
            _ => panic!("expected an Int function binding step"),
        }
    }

    fn expect_int_function_reference(
        expression: &IntFunctionExpr,
    ) -> &FunctionReference<IntFunctionId> {
        match expression.kind() {
            IntFunctionExprKind::Reference(reference) => reference,
            _ => panic!("expected an Int function reference"),
        }
    }

    fn expect_int_function_closure(
        expression: &IntFunctionExpr,
    ) -> &ClosureTemplate<IntFunctionId> {
        match expression.kind() {
            IntFunctionExprKind::Closure(closure) => closure,
            _ => panic!("expected an Int function closure"),
        }
    }

    fn expect_int_capture(capture: &CaptureArg) -> (IntLocalId, &IntExpr) {
        match capture.kind() {
            CaptureArgKind::Int { local, value } => (*local, value),
            _ => panic!("expected an Int capture"),
        }
    }

    fn expect_int_function_call(expression: &IntExpr) -> (&IntFunctionExpr, &[CallArg]) {
        match expression.kind() {
            IntExprKind::FunctionCall(FunctionCall::Executable { function, args }) => {
                (function, args)
            }
            _ => panic!("expected an Int function call"),
        }
    }

    fn expect_int_call_arg(argument: &CallArg) -> (IntLocalId, &IntExpr) {
        match argument.kind() {
            CallArgKind::Int { local, value } => (*local, value),
            _ => panic!("expected an Int call argument"),
        }
    }

    fn expect_int_function_local_get(expression: &IntFunctionExpr) -> IntFunctionLocalId {
        match expression.kind() {
            IntFunctionExprKind::LocalGet { local } => *local,
            _ => panic!("expected an Int function local get"),
        }
    }

    fn expect_int_local_get(expression: &IntExpr) -> IntLocalId {
        match expression.kind() {
            IntExprKind::LocalGet { local } => *local,
            _ => panic!("expected an Int local get"),
        }
    }

    fn expect_int_value(expression: &IntExpr) -> &BigInt {
        match expression.kind() {
            IntExprKind::Value(value) => value,
            _ => panic!("expected an Int value"),
        }
    }

    fn expect_expression_return<Expression, Function>(
        graph: &ReturnGraph<Expression, Function>,
    ) -> &Expression {
        match graph.block(graph.entry()) {
            ReturnBlock::Return { expression } => graph.expression(*expression),
            _ => panic!("expected an expression return body"),
        }
    }
}
