mod error;
mod expression;
mod frame;
mod function;

pub use crate::plan::Value;
pub use error::RuntimeError;

use crate::plan::ModulePlan;

pub fn run_main(plan: &ModulePlan) -> Result<Value, RuntimeError> {
    function::run_function(plan, plan.main, Vec::new())
}

#[cfg(test)]
fn run_src(src: &str) -> Value {
    let plan = plan_src(src);
    run_main(&plan).expect("source should run")
}

#[cfg(test)]
fn plan_src(src: &str) -> crate::ModulePlan {
    let module =
        crate::compile_typed_module("main", "main.gleam", src).expect("source should compile");
    crate::plan_module(module).expect("source should plan")
}

#[cfg(test)]
fn int(value: i64) -> Value {
    Value::Int(num_bigint::BigInt::from(value))
}

#[cfg(test)]
mod tests {
    use super::{int, run_src};

    #[test]
    fn run_main() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  1
}
"#,
            ),
            int(1),
        );
    }
}
