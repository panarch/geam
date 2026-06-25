use crate::plan::{
    BoolExpr, BoolFunctionId, BoolLocalId, CallArg, Expr, FunctionExpr, FunctionType,
    FunctionValue, IntExpr, IntFunctionId, IntLocalId, LocalId, NilExpr, NilFunctionId, NilLocalId,
    ReturnExpr, RuntimeFunctionId, Step, StringExpr, StringFunctionId, StringLocalId,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub(crate) struct Int(IntExpr);

pub(crate) struct String(StringExpr);

pub(crate) struct Bool(BoolExpr);

pub(crate) struct Nil(NilExpr);

pub(crate) struct Function(FunctionExpr);

pub(crate) fn int(value: i64) -> Int {
    Int(IntExpr::value(BigInt::from(value)))
}

pub(crate) fn string(value: impl Into<EcoString>) -> String {
    String(StringExpr::value(value.into()))
}

pub(crate) fn bool_(value: bool) -> Bool {
    Bool(BoolExpr::value(value))
}

pub(crate) fn nil() -> Nil {
    Nil(NilExpr::value())
}

pub(crate) fn function_ref(
    runtime_id: RuntimeFunctionId,
    type_: FunctionType,
    params: impl IntoIterator<Item = LocalId>,
) -> Function {
    Function(FunctionExpr::value(FunctionValue::new(
        type_,
        runtime_id,
        params.into_iter().collect(),
    )))
}

pub(crate) fn equal(left: impl Into<Expr>, right: impl Into<Expr>) -> Bool {
    Bool(BoolExpr::equal(left.into(), right.into()))
}

pub(crate) fn not_equal(left: impl Into<Expr>, right: impl Into<Expr>) -> Bool {
    Bool(BoolExpr::not_equal(left.into(), right.into()))
}

pub(crate) fn bool_case_int(subject: Bool, true_: Int, false_: Int) -> Int {
    Int(IntExpr::bool_case(
        subject.into(),
        true_.into(),
        false_.into(),
    ))
}

pub(crate) fn bool_case_string(subject: Bool, true_: String, false_: String) -> String {
    String(StringExpr::bool_case(
        subject.into(),
        true_.into(),
        false_.into(),
    ))
}

pub(crate) fn bool_case_bool(subject: Bool, true_: Bool, false_: Bool) -> Bool {
    Bool(BoolExpr::bool_case(
        subject.into(),
        true_.into(),
        false_.into(),
    ))
}

pub(crate) fn bool_case_nil(subject: Bool, true_: Nil, false_: Nil) -> Nil {
    Nil(NilExpr::bool_case(
        subject.into(),
        true_.into(),
        false_.into(),
    ))
}

pub(crate) fn int_case_int(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, Int)>,
    fallback: Int,
) -> Int {
    Int(IntExpr::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn int_case_string(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, String)>,
    fallback: String,
) -> String {
    String(StringExpr::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn int_case_bool(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, Bool)>,
    fallback: Bool,
) -> Bool {
    Bool(BoolExpr::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn int_case_nil(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, Nil)>,
    fallback: Nil,
) -> Nil {
    Nil(NilExpr::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn block_int(steps: impl IntoIterator<Item = Step>, return_: Int) -> Int {
    Int(IntExpr::block(steps.into_iter().collect(), return_.into()))
}

pub(crate) fn block_string(steps: impl IntoIterator<Item = Step>, return_: String) -> String {
    String(StringExpr::block(
        steps.into_iter().collect(),
        return_.into(),
    ))
}

pub(crate) fn block_bool(steps: impl IntoIterator<Item = Step>, return_: Bool) -> Bool {
    Bool(BoolExpr::block(steps.into_iter().collect(), return_.into()))
}

pub(crate) fn block_nil(steps: impl IntoIterator<Item = Step>, return_: Nil) -> Nil {
    Nil(NilExpr::block(steps.into_iter().collect(), return_.into()))
}

pub(crate) fn block_function(steps: impl IntoIterator<Item = Step>, return_: Function) -> Function {
    Function(FunctionExpr::block(
        steps.into_iter().collect(),
        return_.into(),
    ))
}

pub(crate) fn let_int_step(local: usize, name: impl Into<EcoString>, value: Int) -> Step {
    Step::let_int(IntLocalId(local), name.into(), value.into())
}

pub(crate) fn let_string_step(local: usize, name: impl Into<EcoString>, value: String) -> Step {
    Step::let_string(StringLocalId(local), name.into(), value.into())
}

pub(crate) fn let_bool_step(local: usize, name: impl Into<EcoString>, value: Bool) -> Step {
    Step::let_bool(BoolLocalId(local), name.into(), value.into())
}

pub(crate) fn let_nil_step(local: usize, name: impl Into<EcoString>, value: Nil) -> Step {
    Step::let_nil(NilLocalId(local), name.into(), value.into())
}

pub(crate) fn let_function_step(name: impl Into<EcoString>, value: Function) -> Step {
    Step::let_function(name.into(), value.into())
}

pub(crate) fn evaluate_step(value: impl Into<Expr>) -> Step {
    Step::evaluate(value.into())
}

pub(crate) fn local_int(index: usize, name: impl Into<EcoString>) -> Int {
    Int(IntExpr::local_get(IntLocalId(index), name.into()))
}

pub(crate) fn local_string(index: usize, name: impl Into<EcoString>) -> String {
    String(StringExpr::local_get(StringLocalId(index), name.into()))
}

pub(crate) fn local_bool(index: usize, name: impl Into<EcoString>) -> Bool {
    Bool(BoolExpr::local_get(BoolLocalId(index), name.into()))
}

pub(crate) fn local_nil(index: usize, name: impl Into<EcoString>) -> Nil {
    Nil(NilExpr::local_get(NilLocalId(index), name.into()))
}

pub(crate) fn call_int(function: usize, args: impl IntoIterator<Item = CallArg>) -> Int {
    Int(IntExpr::call(
        IntFunctionId(function),
        args.into_iter().collect(),
    ))
}

pub(crate) fn call_string(function: usize, args: impl IntoIterator<Item = CallArg>) -> String {
    String(StringExpr::call(
        StringFunctionId(function),
        args.into_iter().collect(),
    ))
}

pub(crate) fn call_bool(function: usize, args: impl IntoIterator<Item = CallArg>) -> Bool {
    Bool(BoolExpr::call(
        BoolFunctionId(function),
        args.into_iter().collect(),
    ))
}

pub(crate) fn call_nil(function: usize, args: impl IntoIterator<Item = CallArg>) -> Nil {
    Nil(NilExpr::call(
        NilFunctionId(function),
        args.into_iter().collect(),
    ))
}

pub(crate) fn int_arg(local: usize, value: Int) -> CallArg {
    CallArg::int(IntLocalId(local), value.into())
}

pub(crate) fn string_arg(local: usize, value: String) -> CallArg {
    CallArg::string(StringLocalId(local), value.into())
}

pub(crate) fn bool_arg(local: usize, value: Bool) -> CallArg {
    CallArg::bool(BoolLocalId(local), value.into())
}

pub(crate) fn nil_arg(local: usize, value: Nil) -> CallArg {
    CallArg::nil(NilLocalId(local), value.into())
}

impl Int {
    pub(crate) fn add_int(self, right: Self) -> Self {
        Self(IntExpr::add(self.into(), right.into()))
    }

    pub(crate) fn sub_int(self, right: Self) -> Self {
        Self(IntExpr::sub(self.into(), right.into()))
    }

    pub(crate) fn mult_int(self, right: Self) -> Self {
        Self(IntExpr::mult(self.into(), right.into()))
    }

    pub(crate) fn div_int(self, right: Self) -> Self {
        Self(IntExpr::div(self.into(), right.into()))
    }

    pub(crate) fn remainder_int(self, right: Self) -> Self {
        Self(IntExpr::remainder(self.into(), right.into()))
    }

    pub(crate) fn lt_int(self, right: Self) -> Bool {
        Bool(BoolExpr::lt_int(self.into(), right.into()))
    }

    pub(crate) fn lte_int(self, right: Self) -> Bool {
        Bool(BoolExpr::lte_int(self.into(), right.into()))
    }

    pub(crate) fn gt_int(self, right: Self) -> Bool {
        Bool(BoolExpr::gt_int(self.into(), right.into()))
    }

    pub(crate) fn gte_int(self, right: Self) -> Bool {
        Bool(BoolExpr::gte_int(self.into(), right.into()))
    }

    pub(crate) fn negate_int(self) -> Self {
        Self(IntExpr::negate(self.into()))
    }
}

impl String {
    pub(crate) fn concatenate(self, right: Self) -> Self {
        Self(StringExpr::concatenate(self.into(), right.into()))
    }
}

impl Bool {
    pub(crate) fn and_bool(self, right: Self) -> Self {
        Self(BoolExpr::and(self.into(), right.into()))
    }

    pub(crate) fn or_bool(self, right: Self) -> Self {
        Self(BoolExpr::or(self.into(), right.into()))
    }

    pub(crate) fn negate_bool(self) -> Self {
        Self(BoolExpr::not(self.into()))
    }
}

impl From<Int> for Expr {
    fn from(value: Int) -> Self {
        Self::int(value.into())
    }
}

impl From<Int> for ReturnExpr {
    fn from(value: Int) -> Self {
        Self::int(value.into())
    }
}

impl From<String> for Expr {
    fn from(value: String) -> Self {
        Self::string(value.into())
    }
}

impl From<String> for ReturnExpr {
    fn from(value: String) -> Self {
        Self::string(value.into())
    }
}

impl From<Bool> for Expr {
    fn from(value: Bool) -> Self {
        Self::bool(value.into())
    }
}

impl From<Bool> for ReturnExpr {
    fn from(value: Bool) -> Self {
        Self::bool(value.into())
    }
}

impl From<Nil> for Expr {
    fn from(value: Nil) -> Self {
        Self::nil(value.into())
    }
}

impl From<Nil> for ReturnExpr {
    fn from(value: Nil) -> Self {
        Self::nil(value.into())
    }
}

impl From<Function> for Expr {
    fn from(value: Function) -> Self {
        Self::function(value.into())
    }
}

impl From<Int> for IntExpr {
    fn from(value: Int) -> Self {
        value.0
    }
}

impl From<String> for StringExpr {
    fn from(value: String) -> Self {
        value.0
    }
}

impl From<Bool> for BoolExpr {
    fn from(value: Bool) -> Self {
        value.0
    }
}

impl From<Nil> for NilExpr {
    fn from(value: Nil) -> Self {
        value.0
    }
}

impl From<Function> for FunctionExpr {
    fn from(value: Function) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::{
        BoolExprKind, CallArgKind, ExprKind, FunctionExprKind, FunctionType, IntExprKind,
        NilExprKind, RuntimeFunctionId, StepKind, StringExprKind, ValueType,
    };

    #[test]
    fn int_dsl() {
        assert!(matches!(Expr::from(int(1)).kind(), ExprKind::Int(_)));
        assert!(matches!(
            int(1).add_int(int(2)).0.kind(),
            IntExprKind::Add { .. },
        ));
        assert!(matches!(
            int(1).sub_int(int(2)).0.kind(),
            IntExprKind::Sub { .. },
        ));
        assert!(matches!(
            int(1).mult_int(int(2)).0.kind(),
            IntExprKind::Mult { .. },
        ));
        assert!(matches!(
            int(1).div_int(int(2)).0.kind(),
            IntExprKind::Div { .. },
        ));
        assert!(matches!(
            int(1).remainder_int(int(2)).0.kind(),
            IntExprKind::Remainder { .. },
        ));
        assert!(matches!(
            int(1).negate_int().0.kind(),
            IntExprKind::Negate(_)
        ));
        assert!(matches!(
            bool_case_int(bool_(true), int(1), int(0)).0.kind(),
            IntExprKind::BoolCase { .. },
        ));
        assert!(matches!(
            int_case_int(int(1), [(1, int(10))], int(0)).0.kind(),
            IntExprKind::IntCase { .. },
        ));
        assert!(matches!(
            block_int([let_int_step(0, "x", int(1))], local_int(0, "x"))
                .0
                .kind(),
            IntExprKind::Block { .. },
        ));
    }

    #[test]
    fn string_dsl() {
        assert!(matches!(
            Expr::from(string("a")).kind(),
            ExprKind::String(_),
        ));
        assert!(matches!(
            string("a").concatenate(string("b")).0.kind(),
            StringExprKind::Concatenate { .. },
        ));
        assert!(matches!(
            bool_case_string(bool_(true), string("a"), string("b"))
                .0
                .kind(),
            StringExprKind::BoolCase { .. },
        ));
        assert!(matches!(
            int_case_string(int(1), [(1, string("one"))], string("other"))
                .0
                .kind(),
            StringExprKind::IntCase { .. },
        ));
        assert!(matches!(
            block_string([let_string_step(0, "x", string("a"))], local_string(0, "x"))
                .0
                .kind(),
            StringExprKind::Block { .. },
        ));
    }

    #[test]
    fn bool_dsl() {
        assert!(matches!(
            int(1).lt_int(int(2)).0.kind(),
            BoolExprKind::LtInt { .. },
        ));
        assert!(matches!(
            int(1).lte_int(int(2)).0.kind(),
            BoolExprKind::LtEqInt { .. },
        ));
        assert!(matches!(
            int(2).gt_int(int(1)).0.kind(),
            BoolExprKind::GtInt { .. },
        ));
        assert!(matches!(
            int(2).gte_int(int(1)).0.kind(),
            BoolExprKind::GtEqInt { .. },
        ));
        assert!(matches!(
            equal(int(1), int(1)).0.kind(),
            BoolExprKind::Equal { .. },
        ));
        assert!(matches!(
            not_equal(bool_(true), bool_(false)).0.kind(),
            BoolExprKind::NotEqual { .. },
        ));
        assert!(matches!(
            bool_(true).and_bool(bool_(false)).0.kind(),
            BoolExprKind::And { .. },
        ));
        assert!(matches!(
            bool_(true).or_bool(bool_(false)).0.kind(),
            BoolExprKind::Or { .. },
        ));
        assert!(matches!(
            bool_(true).negate_bool().0.kind(),
            BoolExprKind::Not(_)
        ));
        assert!(matches!(
            bool_case_bool(bool_(true), bool_(true), bool_(false))
                .0
                .kind(),
            BoolExprKind::BoolCase { .. },
        ));
        assert!(matches!(
            int_case_bool(int(1), [(1, bool_(true))], bool_(false))
                .0
                .kind(),
            BoolExprKind::IntCase { .. },
        ));
        assert!(matches!(
            block_bool([let_bool_step(0, "x", bool_(true))], local_bool(0, "x"))
                .0
                .kind(),
            BoolExprKind::Block { .. },
        ));
    }

    #[test]
    fn nil_dsl() {
        assert!(matches!(Expr::from(nil()).kind(), ExprKind::Nil(_),));
        assert!(matches!(
            bool_case_nil(bool_(true), nil(), nil()).0.kind(),
            NilExprKind::BoolCase { .. },
        ));
        assert!(matches!(
            int_case_nil(int(1), [(1, nil())], nil()).0.kind(),
            NilExprKind::IntCase { .. },
        ));
        assert!(matches!(
            block_nil([let_nil_step(0, "x", nil())], local_nil(0, "x"))
                .0
                .kind(),
            NilExprKind::Block { .. },
        ));
    }

    #[test]
    fn function_dsl() {
        assert!(matches!(
            Expr::from(function_ref(
                RuntimeFunctionId::Int(crate::plan::IntFunctionId(0)),
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
                [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
            ))
            .kind(),
            ExprKind::Function(_),
        ));
        assert!(matches!(
            let_function_step(
                "f",
                function_ref(
                    RuntimeFunctionId::Int(crate::plan::IntFunctionId(0)),
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                    [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
                ),
            )
            .kind(),
            StepKind::LetFunction { .. },
        ));
        assert!(matches!(
            FunctionExpr::from(function_ref(
                RuntimeFunctionId::Int(crate::plan::IntFunctionId(0)),
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
                [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
            ))
            .kind(),
            FunctionExprKind::Value(_),
        ));
        assert!(matches!(
            FunctionExpr::from(block_function(
                [],
                function_ref(
                    RuntimeFunctionId::Int(crate::plan::IntFunctionId(0)),
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                    [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
                ),
            ))
            .kind(),
            FunctionExprKind::Block { .. },
        ));
    }

    #[test]
    fn local_dsl() {
        assert!(matches!(
            local_int(0, "x").0.kind(),
            IntExprKind::LocalGet { .. }
        ));
        assert!(matches!(
            local_string(0, "x").0.kind(),
            StringExprKind::LocalGet { .. },
        ));
        assert!(matches!(
            local_bool(0, "x").0.kind(),
            BoolExprKind::LocalGet { .. },
        ));
        assert!(matches!(
            local_nil(0, "x").0.kind(),
            NilExprKind::LocalGet { .. },
        ));
    }

    #[test]
    fn call_dsl() {
        assert!(matches!(
            call_int(0, [int_arg(0, int(1))]).0.kind(),
            IntExprKind::Call { .. },
        ));
        assert!(matches!(
            call_string(0, [string_arg(0, string("a"))]).0.kind(),
            StringExprKind::Call { .. },
        ));
        assert!(matches!(
            call_bool(0, []).0.kind(),
            BoolExprKind::Call { .. },
        ));
        assert!(matches!(call_nil(0, []).0.kind(), NilExprKind::Call { .. },));
    }

    #[test]
    fn call_arg_dsl() {
        assert!(matches!(int_arg(0, int(1)).kind(), CallArgKind::Int { .. },));
        assert!(matches!(
            string_arg(0, string("a")).kind(),
            CallArgKind::String { .. },
        ));
        assert!(matches!(
            bool_arg(0, bool_(true)).kind(),
            CallArgKind::Bool { .. },
        ));
        assert!(matches!(nil_arg(0, nil()).kind(), CallArgKind::Nil { .. },));
    }

    #[test]
    fn step_dsl() {
        assert!(matches!(
            evaluate_step(int(1)).kind(),
            StepKind::Evaluate(_),
        ));
    }
}
