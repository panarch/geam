use super::super::specialization::Representability;
use super::custom_expr;
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn custom_field_access(
    access: &module::CustomFieldAccess,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::CustomFieldAccess> {
    custom_expr(access.source(), context)
        .map(|source| execution::CustomFieldAccess::from_parts(source, access.index()))
}
