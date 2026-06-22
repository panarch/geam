mod error;
mod expression;
mod frame;
mod function;

pub use crate::plan::Value;
pub use error::RuntimeError;

use crate::plan::ModulePlan;

pub fn run_main(plan: &ModulePlan) -> Result<Value, RuntimeError> {
    run_function(plan, "main", Vec::new())
}

pub fn run_function(
    plan: &ModulePlan,
    name: &str,
    args: Vec<Value>,
) -> Result<Value, RuntimeError> {
    function::run_function(plan, name, args)
}

#[cfg(test)]
fn run_src(src: &str) -> Value {
    let plan = plan_src(src);
    run_main(&plan).expect("source should run")
}

#[cfg(test)]
fn run_function_src(src: &str, name: &str, args: Vec<Value>) -> Result<Value, RuntimeError> {
    let plan = plan_src(src);
    run_function(&plan, name, args)
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
    use super::{RuntimeError, int, run_function_src, run_src};

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

    #[test]
    fn report_missing_function() {
        assert_eq!(
            run_function_src(
                r#"
pub fn main() {
  1
}
"#,
                "missing",
                Vec::new(),
            ),
            Err(RuntimeError::MissingFunction {
                name: "missing".into(),
            }),
        );
    }
}
