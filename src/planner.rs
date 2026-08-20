mod bit_array;
mod context;
#[cfg(test)]
mod dsl;
mod error;
mod expression;
mod function;
mod host_requirement;
mod module;
mod pattern;
mod statement;
mod type_parameter;
mod value_shape;

pub use error::{
    ExternalTypeProviderLinkReason, HostProviderLinkReason, InvalidBitArraySegmentOptionsReason,
    InvalidCallShapeReason, InvalidCaseShapeReason, InvalidCustomTypeReason,
    InvalidExpressionShapeKind, InvalidExpressionType, InvalidFunctionShapeReason,
    InvalidModuleReferenceReason, InvalidPatternShapeReason, InvalidPipelineShapeReason,
    InvalidRecordUpdateShapeReason, InvalidTypedAstReason, InvalidUseShapeReason, PatternKind,
    PlanError, RecordUpdateArgumentOrigin, UnsupportedBitArraySegmentReason,
    UnsupportedFunctionReason, UnsupportedPatternKind, UnsupportedTopLevelKind,
};
pub use host_requirement::{RequiredHostFunction, required_host_functions};
pub use module::{plan_host_program, plan_module, plan_module_with_source, plan_program};

#[cfg(test)]
mod support {
    use crate::frontend::compile_typed_module;
    use crate::planner::error::PlanError;
    use crate::planner::plan_module;
    use gleam_core::ast::{SrcSpan, TypedModule};

    pub(in crate::planner) fn compile(src: &str) -> TypedModule {
        compile_typed_module("main", "main.gleam", src).expect("source should compile")
    }

    pub(in crate::planner) fn expect_plan_error(src: &str) -> PlanError {
        plan_module(compile(src)).expect_err("source should fail planning")
    }

    // Minimal valid typed module used as a mutable fixture in planner margin tests.
    pub(in crate::planner) fn compile_minimal_module() -> TypedModule {
        compile(
            r#"
pub fn main() {
  1
}
"#,
        )
    }

    // Dummy location for hand-built typed AST nodes in planner margin tests.
    pub(in crate::planner) fn dummy_span() -> SrcSpan {
        SrcSpan::new(0, 0)
    }
}
