use super::super::{
    conversion::{expect_expression, validate_expression_value_type},
    plan_expr, plan_expr_with_expected_source_stop_type, record_access,
};
use crate::plan::{CustomConstructor, CustomExpr, CustomType, Expr, ValueType};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidRecordUpdateShapeReason, InvalidTypedAstReason, PlanError, RecordUpdateArgumentOrigin,
};
use ecow::EcoString;
use gleam_core::ast::{CallArg, ImplicitCallArgOrigin, TypedExpr};
use gleam_core::type_::error::VariableOrigin;
use gleam_core::type_::{ModuleValueConstructor, Type, ValueConstructor, ValueConstructorVariant};
use std::sync::Arc;

pub(super) struct ValidatedRecordUpdate {
    source: CustomExpr,
    constructor: CustomConstructor,
    arguments: ValidatedRecordUpdateArguments,
}

pub(super) struct ValidatedRecordUpdateArguments {
    values: Vec<ValidatedRecordUpdateArgument>,
}

impl ValidatedRecordUpdate {
    pub(super) fn into_parts(
        self,
    ) -> (
        CustomExpr,
        CustomConstructor,
        ValidatedRecordUpdateArguments,
    ) {
        (self.source, self.constructor, self.arguments)
    }
}

impl ValidatedRecordUpdateArguments {
    pub(super) fn plan(
        self,
        source: CustomExpr,
        context: &mut PlanContext<'_>,
    ) -> Result<Vec<Expr>, PlanError> {
        self.values
            .into_iter()
            .map(|argument| match argument {
                ValidatedRecordUpdateArgument::Explicit {
                    expression,
                    expected_type,
                } => {
                    let expression = plan_expr_with_expected_source_stop_type(
                        *expression,
                        expected_type.clone(),
                        context,
                    )?;
                    validate_expression_value_type(&expected_type, &expression.value_type())?;
                    Ok(expression)
                }
                ValidatedRecordUpdateArgument::Implicit(access) => record_access::plan_from_expr(
                    access.type_,
                    access.label,
                    access.index,
                    Expr::custom(source.clone()),
                    context,
                ),
            })
            .collect()
    }
}

pub(super) fn validate(
    type_: Arc<Type>,
    updated_record: TypedExpr,
    updated_record_assigned_name: Option<EcoString>,
    constructor: TypedExpr,
    arguments: Vec<CallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
) -> Result<ValidatedRecordUpdate, PlanError> {
    let source_type = source_custom_type(context.value_type(updated_record.type_().as_ref()))?;
    let implicit_target = implicit_target(
        &updated_record,
        updated_record_assigned_name.as_ref(),
        &source_type,
    )?;
    let constructor = record_constructor(constructor, context)?;
    validate_constructor_result_type(&constructor, context.value_type(type_.as_ref()))?;
    let arguments = validate_arguments(arguments, &constructor, &implicit_target, context)?;
    let source = plan_updated_source(updated_record, &source_type, context)?;

    Ok(ValidatedRecordUpdate {
        source,
        constructor,
        arguments,
    })
}

fn source_custom_type(actual: ValueType) -> Result<CustomType, PlanError> {
    match actual {
        ValueType::Custom(type_) => Ok(type_),
        actual => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::RecordUpdateShape {
                reason: InvalidRecordUpdateShapeReason::UpdatedSourceFamily { actual },
            },
        }),
    }
}

fn validate_constructor_result_type(
    constructor: &CustomConstructor,
    actual: ValueType,
) -> Result<(), PlanError> {
    let expected = ValueType::Custom(constructor.type_().clone());
    match actual {
        actual if expected == actual => Ok(()),
        actual => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::RecordUpdateShape {
                reason: InvalidRecordUpdateShapeReason::ConstructorResultType { expected, actual },
            },
        }),
    }
}

fn plan_updated_source(
    expression: TypedExpr,
    expected: &CustomType,
    context: &mut PlanContext<'_>,
) -> Result<CustomExpr, PlanError> {
    let expression = plan_expr(expression, context)?;
    let expected_type = ValueType::Custom(expected.clone());
    let actual = expression.value_type();
    if actual != expected_type {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::RecordUpdateShape {
                reason: InvalidRecordUpdateShapeReason::UpdatedSourceType {
                    expected: expected_type,
                    actual,
                },
            },
        });
    }
    expect_expression(expression)
}

fn validate_arguments(
    arguments: Vec<CallArg<TypedExpr>>,
    constructor: &CustomConstructor,
    implicit_target: &ImplicitTarget,
    context: &mut PlanContext<'_>,
) -> Result<ValidatedRecordUpdateArguments, PlanError> {
    if arguments.len() != constructor.fields().len() {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::RecordUpdateShape {
                reason: InvalidRecordUpdateShapeReason::ArgumentCount {
                    expected: constructor.fields().len(),
                    actual: arguments.len(),
                },
            },
        });
    }

    let mut values = Vec::with_capacity(arguments.len());
    for (index, (argument, field)) in arguments.into_iter().zip(constructor.fields()).enumerate() {
        if argument.label.as_ref() != field.label() {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::RecordUpdateShape {
                    reason: InvalidRecordUpdateShapeReason::ArgumentLabel {
                        index,
                        expected: field.label().cloned(),
                        actual: argument.label,
                    },
                },
            });
        }

        let origin = argument.implicit.map(record_update_argument_origin);
        let value = match origin {
            None => ValidatedRecordUpdateArgument::Explicit {
                expression: Box::new(argument.value),
                expected_type: field.type_().clone(),
            },
            Some(RecordUpdateArgumentOrigin::RecordUpdate) => {
                ValidatedRecordUpdateArgument::Implicit(validate_implicit_argument(
                    argument.value,
                    index,
                    field.label().cloned(),
                    field.type_(),
                    implicit_target,
                    context,
                )?)
            }
            Some(actual) => {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::RecordUpdateShape {
                        reason: InvalidRecordUpdateShapeReason::ImplicitArgumentOrigin {
                            index,
                            actual,
                        },
                    },
                });
            }
        };
        values.push(value);
    }

    Ok(ValidatedRecordUpdateArguments { values })
}

fn record_update_argument_origin(origin: ImplicitCallArgOrigin) -> RecordUpdateArgumentOrigin {
    match origin {
        ImplicitCallArgOrigin::IncorrectArityUse => RecordUpdateArgumentOrigin::IncorrectArityUse,
        ImplicitCallArgOrigin::PatternFieldSpread => RecordUpdateArgumentOrigin::PatternFieldSpread,
        ImplicitCallArgOrigin::Pipe => RecordUpdateArgumentOrigin::Pipe,
        ImplicitCallArgOrigin::RecordUpdate => RecordUpdateArgumentOrigin::RecordUpdate,
        ImplicitCallArgOrigin::Use => RecordUpdateArgumentOrigin::Use,
    }
}

fn validate_implicit_argument(
    expression: TypedExpr,
    argument: usize,
    expected_label: Option<EcoString>,
    expected_type: &ValueType,
    implicit_target: &ImplicitTarget,
    context: &mut PlanContext<'_>,
) -> Result<ValidatedImplicitFieldAccess, PlanError> {
    let (type_, label, index, record) = match expression {
        TypedExpr::RecordAccess {
            type_,
            label,
            index,
            record,
            ..
        } => (type_, Some(label), index, *record),
        TypedExpr::PositionalAccess {
            type_,
            index,
            record,
            ..
        } => (type_, None, index, *record),
        _ => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::RecordUpdateShape {
                    reason: InvalidRecordUpdateShapeReason::ImplicitFieldExpression { argument },
                },
            });
        }
    };
    if index != argument as u64 {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::RecordUpdateShape {
                reason: InvalidRecordUpdateShapeReason::ImplicitFieldIndex {
                    argument,
                    expected: argument,
                    actual: index,
                },
            },
        });
    }
    if label != expected_label {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::RecordUpdateShape {
                reason: InvalidRecordUpdateShapeReason::ImplicitFieldLabel {
                    argument,
                    expected: expected_label,
                    actual: label,
                },
            },
        });
    }
    let actual_type = context.value_type(type_.as_ref());
    if actual_type != *expected_type {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::RecordUpdateShape {
                reason: InvalidRecordUpdateShapeReason::ImplicitFieldType {
                    argument,
                    expected: expected_type.clone(),
                    actual: actual_type,
                },
            },
        });
    }
    validate_implicit_target(&record, argument, implicit_target, context)?;

    Ok(ValidatedImplicitFieldAccess {
        type_,
        label,
        index,
    })
}

fn validate_implicit_target(
    expression: &TypedExpr,
    argument: usize,
    expected: &ImplicitTarget,
    context: &mut PlanContext<'_>,
) -> Result<(), PlanError> {
    let TypedExpr::Var {
        name, constructor, ..
    } = expression
    else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::RecordUpdateShape {
                reason: InvalidRecordUpdateShapeReason::ImplicitTargetExpression { argument },
            },
        });
    };

    let expected_name = expected.name();
    if name != expected_name {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::RecordUpdateShape {
                reason: InvalidRecordUpdateShapeReason::ImplicitTargetName {
                    argument,
                    expected: expected_name.clone(),
                    actual: name.clone(),
                },
            },
        });
    }

    match expected {
        ImplicitTarget::OriginalVariable {
            constructor: expected,
            ..
        } => {
            if constructor != expected.as_ref() {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::RecordUpdateShape {
                        reason: InvalidRecordUpdateShapeReason::ImplicitOriginalTargetConstructor {
                            argument,
                        },
                    },
                });
            }
        }
        ImplicitTarget::GeneratedVariable {
            type_: expected, ..
        } => {
            let actual = context
                .value_shape_in_scope(constructor.type_.as_ref())
                .value_type();
            if actual != *expected {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::RecordUpdateShape {
                        reason: InvalidRecordUpdateShapeReason::ImplicitGeneratedTargetType {
                            argument,
                            expected: expected.clone(),
                            actual,
                        },
                    },
                });
            }
            let ValueConstructorVariant::LocalVariable { origin, .. } = &constructor.variant else {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::RecordUpdateShape {
                        reason: InvalidRecordUpdateShapeReason::ImplicitGeneratedTargetKind {
                            argument,
                        },
                    },
                });
            };
            if origin != &VariableOrigin::generated() {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::RecordUpdateShape {
                        reason: InvalidRecordUpdateShapeReason::ImplicitGeneratedTargetOrigin {
                            argument,
                        },
                    },
                });
            }
        }
    }

    Ok(())
}

enum ValidatedRecordUpdateArgument {
    Explicit {
        expression: Box<TypedExpr>,
        expected_type: ValueType,
    },
    Implicit(ValidatedImplicitFieldAccess),
}

struct ValidatedImplicitFieldAccess {
    type_: Arc<Type>,
    label: Option<EcoString>,
    index: u64,
}

enum RecordUpdateConstructorSource {
    Direct(Box<ValueConstructor>),
    Selected {
        module_name: EcoString,
        label: EcoString,
        name: EcoString,
        variant_index: usize,
        arity: usize,
        type_: Arc<Type>,
    },
}

fn record_constructor(
    expression: TypedExpr,
    context: &PlanContext<'_>,
) -> Result<CustomConstructor, PlanError> {
    let source = match expression {
        TypedExpr::Var {
            name, constructor, ..
        } => match &constructor.variant {
            ValueConstructorVariant::Record { name: actual, .. } => {
                validate_constructor_name(name, actual)?;
                Some(RecordUpdateConstructorSource::Direct(Box::new(constructor)))
            }
            ValueConstructorVariant::LocalVariable { .. }
            | ValueConstructorVariant::ModuleConstant { .. }
            | ValueConstructorVariant::ModuleFn { .. } => None,
        },
        TypedExpr::ModuleSelect {
            module_name,
            label,
            constructor,
            ..
        } => match constructor {
            ModuleValueConstructor::Record {
                name,
                variant_index,
                arity,
                type_,
                ..
            } => Some(RecordUpdateConstructorSource::Selected {
                module_name,
                label,
                name,
                variant_index: usize::from(variant_index),
                arity: usize::from(arity),
                type_,
            }),
            ModuleValueConstructor::Constant { .. } | ModuleValueConstructor::Fn { .. } => None,
        },
        _ => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::RecordUpdateShape {
                    reason: InvalidRecordUpdateShapeReason::ConstructorExpression,
                },
            });
        }
    };
    let Some(source) = source else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::RecordUpdateShape {
                reason: InvalidRecordUpdateShapeReason::ConstructorKind,
            },
        });
    };

    match source {
        RecordUpdateConstructorSource::Direct(constructor) => {
            context.custom_constructor(constructor.as_ref())
        }
        RecordUpdateConstructorSource::Selected {
            module_name,
            label,
            name,
            variant_index,
            arity,
            type_,
        } => {
            validate_constructor_name(label, &name)?;
            context.module_custom_constructor(
                type_.as_ref(),
                name,
                &module_name,
                variant_index,
                arity,
            )
        }
    }
}

fn validate_constructor_name(expected: EcoString, actual: &EcoString) -> Result<(), PlanError> {
    if actual != &expected {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::RecordUpdateShape {
                reason: InvalidRecordUpdateShapeReason::ConstructorName {
                    expected,
                    actual: actual.clone(),
                },
            },
        });
    }

    Ok(())
}

enum ImplicitTarget {
    OriginalVariable {
        name: EcoString,
        constructor: Box<ValueConstructor>,
    },
    GeneratedVariable {
        name: EcoString,
        type_: ValueType,
    },
}

impl ImplicitTarget {
    fn name(&self) -> &EcoString {
        match self {
            Self::OriginalVariable { name, .. } | Self::GeneratedVariable { name, .. } => name,
        }
    }
}

fn implicit_target(
    updated_record: &TypedExpr,
    assigned_name: Option<&EcoString>,
    source_type: &CustomType,
) -> Result<ImplicitTarget, PlanError> {
    match (updated_record, assigned_name) {
        (
            TypedExpr::Var {
                name, constructor, ..
            },
            None,
        ) => Ok(ImplicitTarget::OriginalVariable {
            name: name.clone(),
            constructor: Box::new(constructor.clone()),
        }),
        (expression, Some(name)) if !matches!(expression, TypedExpr::Var { .. }) => {
            Ok(ImplicitTarget::GeneratedVariable {
                name: name.clone(),
                type_: ValueType::Custom(source_type.clone()),
            })
        }
        (expression, assigned_name) => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::RecordUpdateShape {
                reason: InvalidRecordUpdateShapeReason::BaseAssignment {
                    requires_assignment: !matches!(expression, TypedExpr::Var { .. }),
                    has_assignment: assigned_name.is_some(),
                },
            },
        }),
    }
}
