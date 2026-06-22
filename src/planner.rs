mod context;
#[cfg(test)]
mod dsl;
mod error;
mod expression;
mod function;
mod module;
mod statement;

pub use error::PlanError;
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

    pub(in crate::planner) fn compile_base_module() -> TypedModule {
        compile(
            r#"
pub fn main() {
  1
}
"#,
        )
    }

    pub(in crate::planner) fn empty_span() -> SrcSpan {
        SrcSpan::new(0, 0)
    }
}
