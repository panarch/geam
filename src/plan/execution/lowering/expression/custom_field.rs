use super::custom_expr;
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn custom_field_access(
    access: &module::CustomFieldAccess,
    context: &mut super::super::LoweringContext,
) -> execution::CustomFieldAccess {
    execution::CustomFieldAccess::from_parts(custom_expr(access.source(), context), access.index())
}
