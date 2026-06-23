mod context;
#[cfg(test)]
mod dsl;
mod error;
mod expression;
mod function;
mod module;
mod statement;

pub use error::{
    InvalidCallShapeReason, InvalidCaseShapeReason, InvalidExpressionShapeKind,
    InvalidExpressionType, InvalidFunctionShapeReason, InvalidPipelineShapeReason,
    InvalidTypedAstReason, PlanError, UnsupportedArgumentReason, UnsupportedAssignmentKind,
    UnsupportedBinOpKind, UnsupportedCallReason, UnsupportedCaseReason, UnsupportedExpressionKind,
    UnsupportedFunctionReason, UnsupportedPatternKind, UnsupportedPipelineReason,
    UnsupportedStatementKind, UnsupportedTopLevelKind,
};
pub use module::plan_module;

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
