use crate::plan::{
    BoolExpr, BoolFunctionExpr, BoolFunctionId, BoolFunctionLocalId, BoolFunctionValue,
    BoolLocalId, CallArg, Expr, FunctionCallArg, FunctionExpr, FunctionExprKind, FunctionType,
    FunctionValue, IntExpr, IntFunctionExpr, IntFunctionId, IntFunctionLocalId, IntFunctionValue,
    IntLocalId, LocalId, NilExpr, NilFunctionExpr, NilFunctionId, NilFunctionLocalId,
    NilFunctionValue, NilLocalId, ParamLocal, ReturnExpr, RuntimeFunctionId, Step, StringExpr,
    StringFunctionExpr, StringFunctionId, StringFunctionLocalId, StringFunctionValue,
    StringLocalId, ValueType,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub(crate) struct Int(IntExpr);

pub(crate) struct String(StringExpr);

pub(crate) struct Bool(BoolExpr);

pub(crate) struct Nil(NilExpr);

pub(crate) struct Function(FunctionExpr);

pub(crate) struct IntFunction(IntFunctionExpr);

pub(crate) struct StringFunction(StringFunctionExpr);

pub(crate) struct BoolFunction(BoolFunctionExpr);

pub(crate) struct NilFunction(NilFunctionExpr);

pub(crate) trait IntoValueType {
    fn into_value_type(self) -> ValueType;
}

pub(crate) trait IntoParamLocal {
    fn into_param_local(self) -> ParamLocal;
}

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
    params: impl IntoIterator<Item = impl IntoParamLocal>,
) -> Function {
    Function(FunctionExpr::value(FunctionValue::new(
        runtime_id,
        params
            .into_iter()
            .map(IntoParamLocal::into_param_local)
            .collect(),
    )))
}

pub(crate) fn int_function_ref(
    runtime_id: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
) -> IntFunction {
    IntFunction(IntFunctionExpr::value(IntFunctionValue::new(
        IntFunctionId(runtime_id),
        params
            .into_iter()
            .map(IntoParamLocal::into_param_local)
            .collect(),
    )))
}

pub(crate) fn string_function_ref(
    runtime_id: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
) -> StringFunction {
    StringFunction(StringFunctionExpr::value(StringFunctionValue::new(
        StringFunctionId(runtime_id),
        params
            .into_iter()
            .map(IntoParamLocal::into_param_local)
            .collect(),
    )))
}

pub(crate) fn bool_function_ref(
    runtime_id: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
) -> BoolFunction {
    BoolFunction(BoolFunctionExpr::value(BoolFunctionValue::new(
        BoolFunctionId(runtime_id),
        params
            .into_iter()
            .map(IntoParamLocal::into_param_local)
            .collect(),
    )))
}

pub(crate) fn nil_function_ref(
    runtime_id: usize,
    params: impl IntoIterator<Item = impl IntoParamLocal>,
) -> NilFunction {
    NilFunction(NilFunctionExpr::value(NilFunctionValue::new(
        NilFunctionId(runtime_id),
        params
            .into_iter()
            .map(IntoParamLocal::into_param_local)
            .collect(),
    )))
}

pub(crate) fn local_int_function(
    local: usize,
    name: impl Into<EcoString>,
    params: impl IntoIterator<Item = impl IntoValueType>,
) -> IntFunction {
    IntFunction(IntFunctionExpr::local_get(
        IntFunctionLocalId(local),
        name.into(),
        int_function_type(params),
    ))
}

pub(crate) fn local_string_function(
    local: usize,
    name: impl Into<EcoString>,
    params: impl IntoIterator<Item = impl IntoValueType>,
) -> StringFunction {
    StringFunction(StringFunctionExpr::local_get(
        StringFunctionLocalId(local),
        name.into(),
        string_function_type(params),
    ))
}

pub(crate) fn local_bool_function(
    local: usize,
    name: impl Into<EcoString>,
    params: impl IntoIterator<Item = impl IntoValueType>,
) -> BoolFunction {
    BoolFunction(BoolFunctionExpr::local_get(
        BoolFunctionLocalId(local),
        name.into(),
        bool_function_type(params),
    ))
}

pub(crate) fn local_nil_function(
    local: usize,
    name: impl Into<EcoString>,
    params: impl IntoIterator<Item = impl IntoValueType>,
) -> NilFunction {
    NilFunction(NilFunctionExpr::local_get(
        NilFunctionLocalId(local),
        name.into(),
        nil_function_type(params),
    ))
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

pub(crate) fn bool_case_int_function(
    subject: Bool,
    true_: IntFunction,
    false_: IntFunction,
) -> IntFunction {
    IntFunction(IntFunctionExpr::bool_case(
        subject.into(),
        true_.into(),
        false_.into(),
    ))
}

pub(crate) fn bool_case_string_function(
    subject: Bool,
    true_: StringFunction,
    false_: StringFunction,
) -> StringFunction {
    StringFunction(StringFunctionExpr::bool_case(
        subject.into(),
        true_.into(),
        false_.into(),
    ))
}

pub(crate) fn bool_case_bool_function(
    subject: Bool,
    true_: BoolFunction,
    false_: BoolFunction,
) -> BoolFunction {
    BoolFunction(BoolFunctionExpr::bool_case(
        subject.into(),
        true_.into(),
        false_.into(),
    ))
}

pub(crate) fn bool_case_nil_function(
    subject: Bool,
    true_: NilFunction,
    false_: NilFunction,
) -> NilFunction {
    NilFunction(NilFunctionExpr::bool_case(
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

pub(crate) fn int_case_int_function(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, IntFunction)>,
    fallback: IntFunction,
) -> IntFunction {
    IntFunction(IntFunctionExpr::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn int_case_string_function(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, StringFunction)>,
    fallback: StringFunction,
) -> StringFunction {
    StringFunction(StringFunctionExpr::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn int_case_bool_function(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, BoolFunction)>,
    fallback: BoolFunction,
) -> BoolFunction {
    BoolFunction(BoolFunctionExpr::int_case(
        subject.into(),
        clauses
            .into_iter()
            .map(|(value, branch)| (BigInt::from(value), branch.into()))
            .collect(),
        fallback.into(),
    ))
}

pub(crate) fn int_case_nil_function(
    subject: Int,
    clauses: impl IntoIterator<Item = (i64, NilFunction)>,
    fallback: NilFunction,
) -> NilFunction {
    NilFunction(NilFunctionExpr::int_case(
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
    let steps = steps.into_iter().collect();
    Function(match FunctionExpr::from(return_).into_kind() {
        FunctionExprKind::Int(return_) => FunctionExpr::int(IntFunctionExpr::block(steps, return_)),
        FunctionExprKind::String(return_) => {
            FunctionExpr::string(StringFunctionExpr::block(steps, return_))
        }
        FunctionExprKind::Bool(return_) => {
            FunctionExpr::bool(BoolFunctionExpr::block(steps, return_))
        }
        FunctionExprKind::Nil(return_) => FunctionExpr::nil(NilFunctionExpr::block(steps, return_)),
    })
}

pub(crate) fn block_int_function(
    steps: impl IntoIterator<Item = Step>,
    return_: IntFunction,
) -> IntFunction {
    IntFunction(IntFunctionExpr::block(
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

pub(crate) fn let_int_function_step(
    local: usize,
    name: impl Into<EcoString>,
    value: IntFunction,
) -> Step {
    Step::let_int_function(IntFunctionLocalId(local), name.into(), value.into())
}

pub(crate) fn let_string_function_step(
    local: usize,
    name: impl Into<EcoString>,
    value: StringFunction,
) -> Step {
    Step::let_string_function(StringFunctionLocalId(local), name.into(), value.into())
}

pub(crate) fn let_bool_function_step(
    local: usize,
    name: impl Into<EcoString>,
    value: BoolFunction,
) -> Step {
    Step::let_bool_function(BoolFunctionLocalId(local), name.into(), value.into())
}

pub(crate) fn let_nil_function_step(
    local: usize,
    name: impl Into<EcoString>,
    value: NilFunction,
) -> Step {
    Step::let_nil_function(NilFunctionLocalId(local), name.into(), value.into())
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

pub(crate) fn call_int_function(
    function: IntFunction,
    args: impl IntoIterator<Item = FunctionCallArg>,
) -> Int {
    Int(IntExpr::function_call(
        function.into(),
        args.into_iter().collect(),
    ))
}

pub(crate) fn int_arg(local: usize, value: Int) -> CallArg {
    CallArg::int(IntLocalId(local), value.into())
}

pub(crate) fn int_function_call_arg(value: Int) -> FunctionCallArg {
    FunctionCallArg::int(value.into())
}

pub(crate) fn int_function_arg(local: usize, value: IntFunction) -> CallArg {
    CallArg::int_function(IntFunctionLocalId(local), value.into())
}

pub(crate) fn string_arg(local: usize, value: String) -> CallArg {
    CallArg::string(StringLocalId(local), value.into())
}

pub(crate) fn string_function_arg(local: usize, value: StringFunction) -> CallArg {
    CallArg::string_function(StringFunctionLocalId(local), value.into())
}

pub(crate) fn bool_arg(local: usize, value: Bool) -> CallArg {
    CallArg::bool(BoolLocalId(local), value.into())
}

pub(crate) fn bool_function_arg(local: usize, value: BoolFunction) -> CallArg {
    CallArg::bool_function(BoolFunctionLocalId(local), value.into())
}

pub(crate) fn nil_arg(local: usize, value: Nil) -> CallArg {
    CallArg::nil(NilLocalId(local), value.into())
}

pub(crate) fn nil_function_arg(local: usize, value: NilFunction) -> CallArg {
    CallArg::nil_function(NilFunctionLocalId(local), value.into())
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

impl From<IntFunction> for Function {
    fn from(value: IntFunction) -> Self {
        Function(FunctionExpr::int(value.into()))
    }
}

impl From<IntFunction> for FunctionExpr {
    fn from(value: IntFunction) -> Self {
        FunctionExpr::int(value.into())
    }
}

impl From<IntFunction> for IntFunctionExpr {
    fn from(value: IntFunction) -> Self {
        value.0
    }
}

impl From<StringFunction> for StringFunctionExpr {
    fn from(value: StringFunction) -> Self {
        value.0
    }
}

impl From<BoolFunction> for BoolFunctionExpr {
    fn from(value: BoolFunction) -> Self {
        value.0
    }
}

impl From<NilFunction> for NilFunctionExpr {
    fn from(value: NilFunction) -> Self {
        value.0
    }
}

fn int_function_type(params: impl IntoIterator<Item = impl IntoValueType>) -> FunctionType {
    function_type(params, ValueType::Int)
}

fn string_function_type(params: impl IntoIterator<Item = impl IntoValueType>) -> FunctionType {
    function_type(params, ValueType::String)
}

fn bool_function_type(params: impl IntoIterator<Item = impl IntoValueType>) -> FunctionType {
    function_type(params, ValueType::Bool)
}

fn nil_function_type(params: impl IntoIterator<Item = impl IntoValueType>) -> FunctionType {
    function_type(params, ValueType::Nil)
}

fn function_type(
    params: impl IntoIterator<Item = impl IntoValueType>,
    return_: ValueType,
) -> FunctionType {
    FunctionType::new(
        params
            .into_iter()
            .map(IntoValueType::into_value_type)
            .collect(),
        return_,
    )
}

impl IntoValueType for ValueType {
    fn into_value_type(self) -> ValueType {
        self
    }
}

impl IntoValueType for LocalId {
    fn into_value_type(self) -> ValueType {
        match self {
            LocalId::Int(_) => ValueType::Int,
            LocalId::String(_) => ValueType::String,
            LocalId::Bool(_) => ValueType::Bool,
            LocalId::Nil(_) => ValueType::Nil,
        }
    }
}

impl IntoValueType for ParamLocal {
    fn into_value_type(self) -> ValueType {
        self.value_type()
    }
}

impl IntoParamLocal for LocalId {
    fn into_param_local(self) -> ParamLocal {
        match self {
            LocalId::Int(local) => ParamLocal::int(local),
            LocalId::String(local) => ParamLocal::string(local),
            LocalId::Bool(local) => ParamLocal::bool(local),
            LocalId::Nil(local) => ParamLocal::nil(local),
        }
    }
}

impl IntoParamLocal for ParamLocal {
    fn into_param_local(self) -> ParamLocal {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::{
        BoolExprKind, CallArgKind, ExprKind, FunctionExprKind, IntExprKind, IntFunctionExprKind,
        NilExprKind, RuntimeFunctionId, StepKind, StringExprKind,
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
                [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
            ))
            .kind(),
            ExprKind::Function(_),
        ));
        assert!(matches!(
            FunctionExpr::from(function_ref(
                RuntimeFunctionId::Int(crate::plan::IntFunctionId(0)),
                [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
            ))
            .kind(),
            FunctionExprKind::Int(_),
        ));
        assert!(matches!(
            FunctionExpr::from(block_function(
                [],
                function_ref(
                    RuntimeFunctionId::Int(crate::plan::IntFunctionId(0)),
                    [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
                ),
            ))
            .kind(),
            FunctionExprKind::Int(_),
        ));
        assert!(matches!(
            FunctionExpr::from(block_function(
                [],
                function_ref(
                    RuntimeFunctionId::String(crate::plan::StringFunctionId(0)),
                    [crate::plan::LocalId::String(crate::plan::StringLocalId(0))],
                ),
            ))
            .kind(),
            FunctionExprKind::String(_),
        ));
        assert!(matches!(
            FunctionExpr::from(block_function(
                [],
                function_ref(
                    RuntimeFunctionId::Bool(crate::plan::BoolFunctionId(0)),
                    [crate::plan::LocalId::Bool(crate::plan::BoolLocalId(0))],
                ),
            ))
            .kind(),
            FunctionExprKind::Bool(_),
        ));
        assert!(matches!(
            FunctionExpr::from(block_function(
                [],
                function_ref(
                    RuntimeFunctionId::Nil(crate::plan::NilFunctionId(0)),
                    [crate::plan::LocalId::Nil(crate::plan::NilLocalId(0))],
                ),
            ))
            .kind(),
            FunctionExprKind::Nil(_),
        ));
        assert!(matches!(
            FunctionExpr::from(Function::from(int_function_ref(
                0,
                [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
            )))
            .kind(),
            FunctionExprKind::Int(_),
        ));
        assert!(matches!(
            block_int_function(
                [],
                int_function_ref(0, [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))]),
            )
            .0
            .kind(),
            IntFunctionExprKind::Block { .. },
        ));
        assert!(matches!(
            bool_case_string_function(
                bool_(true),
                string_function_ref(
                    0,
                    [crate::plan::LocalId::String(crate::plan::StringLocalId(0))],
                ),
                string_function_ref(
                    1,
                    [crate::plan::LocalId::String(crate::plan::StringLocalId(0))],
                ),
            )
            .0
            .kind(),
            crate::plan::StringFunctionExprKind::BoolCase { .. },
        ));
        assert!(matches!(
            bool_case_bool_function(
                bool_(true),
                bool_function_ref(0, [crate::plan::LocalId::Bool(crate::plan::BoolLocalId(0))]),
                bool_function_ref(1, [crate::plan::LocalId::Bool(crate::plan::BoolLocalId(0))]),
            )
            .0
            .kind(),
            crate::plan::BoolFunctionExprKind::BoolCase { .. },
        ));
        assert!(matches!(
            bool_case_nil_function(
                bool_(true),
                nil_function_ref(0, [crate::plan::LocalId::Nil(crate::plan::NilLocalId(0))]),
                nil_function_ref(1, [crate::plan::LocalId::Nil(crate::plan::NilLocalId(0))]),
            )
            .0
            .kind(),
            crate::plan::NilFunctionExprKind::BoolCase { .. },
        ));
        assert!(matches!(
            int_case_string_function(
                int(1),
                [(
                    1,
                    string_function_ref(
                        0,
                        [crate::plan::LocalId::String(crate::plan::StringLocalId(0))],
                    ),
                )],
                string_function_ref(
                    1,
                    [crate::plan::LocalId::String(crate::plan::StringLocalId(0))],
                ),
            )
            .0
            .kind(),
            crate::plan::StringFunctionExprKind::IntCase { .. },
        ));
        assert!(matches!(
            int_case_bool_function(
                int(1),
                [(
                    1,
                    bool_function_ref(
                        0,
                        [crate::plan::LocalId::Bool(crate::plan::BoolLocalId(0))],
                    ),
                )],
                bool_function_ref(1, [crate::plan::LocalId::Bool(crate::plan::BoolLocalId(0))]),
            )
            .0
            .kind(),
            crate::plan::BoolFunctionExprKind::IntCase { .. },
        ));
        assert!(matches!(
            int_case_nil_function(
                int(1),
                [(
                    1,
                    nil_function_ref(0, [crate::plan::LocalId::Nil(crate::plan::NilLocalId(0))]),
                )],
                nil_function_ref(1, [crate::plan::LocalId::Nil(crate::plan::NilLocalId(0))]),
            )
            .0
            .kind(),
            crate::plan::NilFunctionExprKind::IntCase { .. },
        ));
    }

    #[test]
    fn value_type_dsl() {
        assert_eq!(ValueType::Int.into_value_type(), ValueType::Int);
        assert_eq!(
            ParamLocal::int(crate::plan::IntLocalId(0)).into_value_type(),
            ValueType::Int,
        );
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
        assert!(matches!(
            local_int_function(
                0,
                "f",
                [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))]
            )
            .0
            .kind(),
            IntFunctionExprKind::LocalGet { .. },
        ));
        assert!(matches!(
            local_string_function(
                0,
                "f",
                [crate::plan::LocalId::String(crate::plan::StringLocalId(0))],
            )
            .0
            .kind(),
            crate::plan::StringFunctionExprKind::LocalGet { .. },
        ));
        assert!(matches!(
            local_bool_function(
                0,
                "f",
                [crate::plan::LocalId::Bool(crate::plan::BoolLocalId(0))],
            )
            .0
            .kind(),
            crate::plan::BoolFunctionExprKind::LocalGet { .. },
        ));
        assert!(matches!(
            local_nil_function(
                0,
                "f",
                [crate::plan::LocalId::Nil(crate::plan::NilLocalId(0))],
            )
            .0
            .kind(),
            crate::plan::NilFunctionExprKind::LocalGet { .. },
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
            int_function_arg(0, int_function_ref(0, Vec::<ParamLocal>::new())).kind(),
            CallArgKind::IntFunction { .. },
        ));
        assert!(matches!(
            string_arg(0, string("a")).kind(),
            CallArgKind::String { .. },
        ));
        assert!(matches!(
            string_function_arg(0, string_function_ref(0, Vec::<ParamLocal>::new())).kind(),
            CallArgKind::StringFunction { .. },
        ));
        assert!(matches!(
            bool_arg(0, bool_(true)).kind(),
            CallArgKind::Bool { .. },
        ));
        assert!(matches!(
            bool_function_arg(0, bool_function_ref(0, Vec::<ParamLocal>::new())).kind(),
            CallArgKind::BoolFunction { .. },
        ));
        assert!(matches!(nil_arg(0, nil()).kind(), CallArgKind::Nil { .. },));
        assert!(matches!(
            nil_function_arg(0, nil_function_ref(0, Vec::<ParamLocal>::new())).kind(),
            CallArgKind::NilFunction { .. },
        ));
    }

    #[test]
    fn step_dsl() {
        assert!(matches!(
            let_int_function_step(
                0,
                "f",
                int_function_ref(0, [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))]),
            )
            .kind(),
            StepKind::LetIntFunction { .. },
        ));
        assert!(matches!(
            let_string_function_step(
                0,
                "f",
                string_function_ref(
                    0,
                    [crate::plan::LocalId::String(crate::plan::StringLocalId(0))],
                ),
            )
            .kind(),
            StepKind::LetStringFunction { .. },
        ));
        assert!(matches!(
            let_bool_function_step(
                0,
                "f",
                bool_function_ref(0, [crate::plan::LocalId::Bool(crate::plan::BoolLocalId(0))]),
            )
            .kind(),
            StepKind::LetBoolFunction { .. },
        ));
        assert!(matches!(
            let_nil_function_step(
                0,
                "f",
                nil_function_ref(0, [crate::plan::LocalId::Nil(crate::plan::NilLocalId(0))]),
            )
            .kind(),
            StepKind::LetNilFunction { .. },
        ));
        assert!(matches!(
            evaluate_step(int(1)).kind(),
            StepKind::Evaluate(_),
        ));
    }
}
