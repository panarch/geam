use super::custom_expr;
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn custom_field_access(
    access: module::CustomFieldAccess,
    context: &mut super::super::LoweringContext,
) -> execution::CustomFieldAccess {
    let (source, index, _label, constructors) = access.into_parts();
    let constructors = constructors
        .into_iter()
        .map(|constructor| context.custom_constructor(constructor))
        .collect();
    execution::CustomFieldAccess::from_parts(custom_expr(source, context), index, constructors)
}
