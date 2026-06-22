mod expression;
mod function;
mod locals;
mod module;
mod step;

pub(in crate::planner) use expression::{ExprBuilder, bool_, call, int, local, nil, string};
pub(in crate::planner) use function::{FunctionBuilder, function};
pub(in crate::planner) use module::{ModuleBuilder, module};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::{Expr, FunctionId, FunctionPlan, ModulePlan, Value};
    use num_bigint::BigInt;

    #[test]
    fn dsl_surface_build() {
        let actual = module("main")
            .function(function("main").return_(int(1)))
            .build();

        assert_eq!(
            actual,
            ModulePlan {
                module: "main".into(),
                main: FunctionId(0),
                functions: vec![FunctionPlan {
                    id: FunctionId(0),
                    name: "main".into(),
                    params: vec![],
                    steps: vec![],
                    return_: Expr::Value(Value::Int(BigInt::from(1))),
                }],
            }
        );
    }

    #[test]
    fn dsl_surface_re_exports() {
        let _: ModuleBuilder = module("main");
        let _: FunctionBuilder = function("main").param("x").return_(nil());
        let _: ExprBuilder = bool_(true).negate_bool();
        let _: ExprBuilder = string("a").concatenate(string("b"));
        let _: ExprBuilder = call("helper", [local("x"), nil()]);
    }
}
