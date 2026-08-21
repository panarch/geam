use crate::plan::{CustomConstructor, Expr, ValueShape};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidModuleReferenceReason, InvalidTypedAstReason, PlanError};
use ecow::EcoString;
use gleam_compiler_core::type_::Type;
use std::sync::Arc;

pub(super) struct ResolvedRecordConstructor {
    module: EcoString,
    name: EcoString,
    constructor: CustomConstructor,
}

impl ResolvedRecordConstructor {
    pub(super) fn direct(
        module: EcoString,
        name: EcoString,
        constructor: CustomConstructor,
    ) -> Self {
        Self {
            module,
            name,
            constructor,
        }
    }

    pub(super) fn selected(
        context: &PlanContext<'_>,
        module: EcoString,
        name: EcoString,
        constructor_name: EcoString,
        type_: Arc<Type>,
        variant_index: usize,
        arity: usize,
    ) -> Result<Self, PlanError> {
        let _linked_module = context.resolve_module_reference(&module, &name)?;
        if constructor_name != name {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module,
                    name,
                    reason: InvalidModuleReferenceReason::RecordConstructorName {
                        actual: constructor_name,
                    },
                },
            });
        }
        let constructor = context.module_custom_constructor(
            type_.as_ref(),
            constructor_name,
            &module,
            variant_index,
            arity,
        )?;
        Ok(Self {
            module,
            name,
            constructor,
        })
    }

    pub(super) fn into_constructor(self) -> CustomConstructor {
        self.constructor
    }

    pub(super) fn plan_reference(self, shape: ValueShape) -> Result<Expr, PlanError> {
        crate::plan::module::custom_constructor_expr(self.constructor)
            .with_shape(shape)
            .ok_or_else(|| PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: self.module,
                    name: self.name,
                    reason: InvalidModuleReferenceReason::RecordConstructorResultShape,
                },
            })
    }
}

#[cfg(test)]
mod tests {
    use super::ResolvedRecordConstructor;
    use crate::plan::{
        CustomConstructor, CustomConstructorField, CustomType, CustomTypeName, ValueShape,
        ValueType,
    };
    use crate::planner::context::{AnonymousFunctions, PlanContext};
    use crate::planner::{InvalidModuleReferenceReason, InvalidTypedAstReason, PlanError};
    use ecow::EcoString;
    use std::collections::HashMap;

    #[test]
    fn selected_record_constructor_rejects_mismatched_name() {
        let module = EcoString::from("main");
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let context = PlanContext::new(&module, &functions, &mut anonymous);

        assert_eq!(
            ResolvedRecordConstructor::selected(
                &context,
                module.clone(),
                "Expected".into(),
                "Actual".into(),
                gleam_compiler_core::type_::int(),
                0,
                0,
            )
            .map(ResolvedRecordConstructor::into_constructor),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "Expected".into(),
                    reason: InvalidModuleReferenceReason::RecordConstructorName {
                        actual: "Actual".into(),
                    },
                },
            }),
        );
    }

    #[test]
    fn record_constructor_reference_rejects_conflicting_result_shape() {
        let constructor = CustomConstructor::new(
            CustomType::new(
                CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
                Vec::new(),
            ),
            "Boxed".into(),
            0,
            vec![CustomConstructorField::new(None, ValueType::Int)],
        );

        assert_eq!(
            ResolvedRecordConstructor::direct("main".into(), "Boxed".into(), constructor)
                .plan_reference(ValueShape::Int),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "Boxed".into(),
                    reason: InvalidModuleReferenceReason::RecordConstructorResultShape,
                },
            }),
        );
    }
}
