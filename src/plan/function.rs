use super::expression::Expr;
use super::id::{FunctionId, LocalId};
use super::step::Step;
use ecow::EcoString;

#[derive(Debug, PartialEq)]
pub struct FunctionPlan {
    id: FunctionId,
    name: EcoString,
    params: Vec<Param>,
    steps: Vec<Step>,
    return_: Expr,
}

#[derive(Debug, PartialEq)]
pub struct Param {
    local: LocalId,
    name: EcoString,
}

pub(crate) struct RuntimeFunction<Return> {
    steps: Vec<Step>,
    return_: Return,
}

impl FunctionPlan {
    pub(crate) fn new(
        id: FunctionId,
        name: EcoString,
        params: Vec<Param>,
        steps: Vec<Step>,
        return_: Expr,
    ) -> Self {
        Self {
            id,
            name,
            params,
            steps,
            return_,
        }
    }

    pub fn id(&self) -> FunctionId {
        self.id
    }

    pub fn name(&self) -> &EcoString {
        &self.name
    }

    pub fn params(&self) -> &[Param] {
        &self.params
    }

    pub fn steps(&self) -> &[Step] {
        &self.steps
    }

    pub fn return_(&self) -> &Expr {
        &self.return_
    }
}

impl<Return> RuntimeFunction<Return> {
    pub(crate) fn new(steps: Vec<Step>, return_: Return) -> Self {
        Self { steps, return_ }
    }

    pub(crate) fn steps(&self) -> &[Step] {
        &self.steps
    }

    pub(crate) fn return_(&self) -> &Return {
        &self.return_
    }
}

impl Param {
    pub(crate) fn new(local: LocalId, name: EcoString) -> Self {
        Self { local, name }
    }

    pub fn name(&self) -> &EcoString {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::{FunctionPlan, Param, RuntimeFunction};
    use crate::plan::{Expr, FunctionId, IntExpr, IntLocalId, LocalId};
    use num_bigint::BigInt;

    #[test]
    fn function_plan_accessors() {
        let param = Param::new(LocalId::Int(IntLocalId(0)), "x".into());
        let return_ = Expr::int(IntExpr::value(BigInt::from(1)));
        let function = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            vec![param],
            Vec::new(),
            return_,
        );

        assert_eq!(function.id(), FunctionId::new(0));
        assert_eq!(function.name(), "main");
        assert_eq!(function.params().len(), 1);
        assert_eq!(function.params()[0].name(), "x");
        assert_eq!(function.steps(), &[]);
        assert_eq!(
            function.return_(),
            &Expr::int(IntExpr::value(BigInt::from(1)))
        );
    }

    #[test]
    fn param_name_accessor() {
        let param = Param::new(LocalId::Int(IntLocalId(0)), "x".into());

        assert_eq!(param.name(), "x");
    }

    #[test]
    fn runtime_function_accessors() {
        let return_ = IntExpr::value(BigInt::from(1));
        let function = RuntimeFunction::new(Vec::new(), return_);

        assert_eq!(function.steps(), &[]);
        assert_eq!(function.return_(), &IntExpr::value(BigInt::from(1)));
    }
}
