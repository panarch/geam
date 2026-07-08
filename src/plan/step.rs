use super::expression::{
    BoolExpr, BoolFunctionExpr, Expr, FloatExpr, FloatFunctionExpr, FunctionFunctionExpr, IntExpr,
    IntFunctionExpr, ListExpr, ListFunctionExpr, NilExpr, NilFunctionExpr, StringExpr,
    StringFunctionExpr, TupleExpr, TupleFunctionExpr,
};
use super::function::ParamLocal;
use super::id::{
    BoolFunctionLocalId, BoolLocalId, FloatFunctionLocalId, FloatLocalId, FunctionFunctionLocalId,
    IntFunctionLocalId, IntLocalId, ListFunctionLocal, ListLocal, NilFunctionLocalId, NilLocalId,
    StringFunctionLocalId, StringLocalId, TupleFunctionLocalId, TupleLocalId,
};
use super::source::{PanicSite, SourceSpan};
use super::value::ValueType;
use ecow::EcoString;

#[derive(Debug, Clone, PartialEq)]
pub struct Step {
    kind: StepKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AssertBinding {
    local: ParamLocal,
    name: EcoString,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AssertPattern {
    Bind(AssertBinding),
    Discard,
    Tuple(Vec<AssertPattern>),
    List(ListAssertPattern),
    Alias {
        pattern: Box<AssertPattern>,
        binding: AssertBinding,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ListAssertPattern {
    element_type: ValueType,
    elements: Vec<AssertPattern>,
    tail: Option<ListAssertTail>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ListAssertTailBinding {
    local: ListLocal,
    name: EcoString,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ListAssertTail {
    Ignore,
    Bind(ListAssertTailBinding),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum StepKind {
    LetInt {
        local: IntLocalId,
        name: EcoString,
        value: IntExpr,
    },
    LetFloat {
        local: FloatLocalId,
        name: EcoString,
        value: FloatExpr,
    },
    LetString {
        local: StringLocalId,
        name: EcoString,
        value: StringExpr,
    },
    LetBool {
        local: BoolLocalId,
        name: EcoString,
        value: BoolExpr,
    },
    LetNil {
        local: NilLocalId,
        name: EcoString,
        value: NilExpr,
    },
    LetTuple {
        local: TupleLocalId,
        name: EcoString,
        value: TupleExpr,
    },
    LetList {
        local: ListLocal,
        name: EcoString,
        value: ListExpr,
    },
    LetIntFunction {
        local: IntFunctionLocalId,
        name: EcoString,
        value: IntFunctionExpr,
    },
    LetFloatFunction {
        local: FloatFunctionLocalId,
        name: EcoString,
        value: FloatFunctionExpr,
    },
    LetStringFunction {
        local: StringFunctionLocalId,
        name: EcoString,
        value: StringFunctionExpr,
    },
    LetBoolFunction {
        local: BoolFunctionLocalId,
        name: EcoString,
        value: BoolFunctionExpr,
    },
    LetNilFunction {
        local: NilFunctionLocalId,
        name: EcoString,
        value: NilFunctionExpr,
    },
    LetTupleFunction {
        local: TupleFunctionLocalId,
        name: EcoString,
        value: TupleFunctionExpr,
    },
    LetListFunction {
        local: ListFunctionLocal,
        name: EcoString,
        value: ListFunctionExpr,
    },
    LetFunctionFunction {
        local: FunctionFunctionLocalId,
        name: EcoString,
        value: FunctionFunctionExpr,
    },
    AssertList {
        local: ListLocal,
        pattern: AssertPattern,
        message: Option<StringExpr>,
        site: PanicSite,
        pattern_span: SourceSpan,
    },
    AssertBool {
        condition: BoolExpr,
        message: Option<StringExpr>,
        site: PanicSite,
    },
    Evaluate(Expr),
}

impl AssertBinding {
    pub(crate) fn new(local: ParamLocal, name: EcoString) -> Self {
        Self { local, name }
    }

    pub(crate) fn local(&self) -> &ParamLocal {
        &self.local
    }
}

impl AssertPattern {
    pub(crate) fn list(pattern: ListAssertPattern) -> Self {
        Self::List(pattern)
    }

    pub(crate) fn alias(pattern: AssertPattern, binding: AssertBinding) -> Self {
        Self::Alias {
            pattern: Box::new(pattern),
            binding,
        }
    }
}

impl ListAssertPattern {
    pub(crate) fn new(
        element_type: ValueType,
        elements: Vec<AssertPattern>,
        tail: Option<ListAssertTail>,
    ) -> Self {
        Self {
            element_type,
            elements,
            tail,
        }
    }

    pub(crate) fn elements(&self) -> &[AssertPattern] {
        &self.elements
    }

    pub(crate) fn tail(&self) -> Option<&ListAssertTail> {
        self.tail.as_ref()
    }
}

impl ListAssertTail {
    pub(crate) fn bind(local: ListLocal, name: EcoString) -> Self {
        Self::Bind(ListAssertTailBinding { local, name })
    }
}

impl ListAssertTailBinding {
    pub(crate) fn local(&self) -> &ListLocal {
        &self.local
    }
}

impl Step {
    pub(crate) fn let_int(local: IntLocalId, name: EcoString, value: IntExpr) -> Self {
        Self {
            kind: StepKind::LetInt { local, name, value },
        }
    }

    pub(crate) fn let_float(local: FloatLocalId, name: EcoString, value: FloatExpr) -> Self {
        Self {
            kind: StepKind::LetFloat { local, name, value },
        }
    }

    pub(crate) fn let_string(local: StringLocalId, name: EcoString, value: StringExpr) -> Self {
        Self {
            kind: StepKind::LetString { local, name, value },
        }
    }

    pub(crate) fn let_bool(local: BoolLocalId, name: EcoString, value: BoolExpr) -> Self {
        Self {
            kind: StepKind::LetBool { local, name, value },
        }
    }

    pub(crate) fn let_nil(local: NilLocalId, name: EcoString, value: NilExpr) -> Self {
        Self {
            kind: StepKind::LetNil { local, name, value },
        }
    }

    pub(crate) fn let_tuple(local: TupleLocalId, name: EcoString, value: TupleExpr) -> Self {
        Self {
            kind: StepKind::LetTuple { local, name, value },
        }
    }

    pub(crate) fn let_list(local: ListLocal, name: EcoString, value: ListExpr) -> Self {
        Self {
            kind: StepKind::LetList { local, name, value },
        }
    }

    pub(crate) fn let_int_function(
        local: IntFunctionLocalId,
        name: EcoString,
        value: IntFunctionExpr,
    ) -> Self {
        Self {
            kind: StepKind::LetIntFunction { local, name, value },
        }
    }

    pub(crate) fn let_float_function(
        local: FloatFunctionLocalId,
        name: EcoString,
        value: FloatFunctionExpr,
    ) -> Self {
        Self {
            kind: StepKind::LetFloatFunction { local, name, value },
        }
    }

    pub(crate) fn let_string_function(
        local: StringFunctionLocalId,
        name: EcoString,
        value: StringFunctionExpr,
    ) -> Self {
        Self {
            kind: StepKind::LetStringFunction { local, name, value },
        }
    }

    pub(crate) fn let_bool_function(
        local: BoolFunctionLocalId,
        name: EcoString,
        value: BoolFunctionExpr,
    ) -> Self {
        Self {
            kind: StepKind::LetBoolFunction { local, name, value },
        }
    }

    pub(crate) fn let_nil_function(
        local: NilFunctionLocalId,
        name: EcoString,
        value: NilFunctionExpr,
    ) -> Self {
        Self {
            kind: StepKind::LetNilFunction { local, name, value },
        }
    }

    pub(crate) fn let_tuple_function(
        local: TupleFunctionLocalId,
        name: EcoString,
        value: TupleFunctionExpr,
    ) -> Self {
        Self {
            kind: StepKind::LetTupleFunction { local, name, value },
        }
    }

    pub(crate) fn let_list_function(
        local: ListFunctionLocal,
        name: EcoString,
        value: ListFunctionExpr,
    ) -> Self {
        Self {
            kind: StepKind::LetListFunction { local, name, value },
        }
    }

    pub(crate) fn let_function_function(
        local: FunctionFunctionLocalId,
        name: EcoString,
        value: FunctionFunctionExpr,
    ) -> Self {
        Self {
            kind: StepKind::LetFunctionFunction { local, name, value },
        }
    }

    pub(crate) fn evaluate(value: Expr) -> Self {
        Self {
            kind: StepKind::Evaluate(value),
        }
    }

    pub(crate) fn assert_bool_at(
        condition: BoolExpr,
        message: Option<StringExpr>,
        site: PanicSite,
    ) -> Self {
        Self {
            kind: StepKind::AssertBool {
                condition,
                message,
                site,
            },
        }
    }

    pub(crate) fn assert_list_at(
        local: ListLocal,
        pattern: AssertPattern,
        message: Option<StringExpr>,
        site: PanicSite,
        pattern_span: SourceSpan,
    ) -> Self {
        Self {
            kind: StepKind::AssertList {
                local,
                pattern,
                message,
                site,
                pattern_span,
            },
        }
    }

    pub(crate) fn kind(&self) -> &StepKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{Step, StepKind};
    use crate::plan::{
        AssertPattern, BoolExpr, Expr, IntExpr, IntFunctionId, IntFunctionLocalId,
        IntFunctionValue, IntListLocalId, IntLocalId, ListAssertPattern, ListAssertTail, ListLocal,
        ParamLocal, StringExpr, ValueType,
    };
    use num_bigint::BigInt;

    #[test]
    fn step_kind_accessors() {
        assert_eq!(
            Step::let_int(IntLocalId(0), "x".into(), IntExpr::value(BigInt::from(1))).kind(),
            &StepKind::LetInt {
                local: IntLocalId(0),
                name: "x".into(),
                value: IntExpr::value(BigInt::from(1)),
            },
        );
        assert_eq!(
            Step::let_int_function(IntFunctionLocalId(0), "f".into(), function_expr()).kind(),
            &StepKind::LetIntFunction {
                local: IntFunctionLocalId(0),
                name: "f".into(),
                value: function_expr(),
            },
        );
        assert_eq!(
            Step::evaluate(Expr::int(IntExpr::value(BigInt::from(1)))).kind(),
            &StepKind::Evaluate(Expr::int(IntExpr::value(BigInt::from(1)))),
        );
        assert_eq!(
            Step::assert_bool_at(
                BoolExpr::value(false),
                Some(StringExpr::value("nope".into())),
                crate::plan::PanicSite::unknown(),
            )
            .kind(),
            &StepKind::AssertBool {
                condition: BoolExpr::value(false),
                message: Some(StringExpr::value("nope".into())),
                site: crate::plan::PanicSite::unknown(),
            },
        );
        assert_eq!(
            Step::assert_list_at(
                ListLocal::int(IntListLocalId(0)),
                AssertPattern::list(ListAssertPattern::new(
                    ValueType::Int,
                    vec![AssertPattern::Discard],
                    Some(ListAssertTail::bind(
                        ListLocal::int(IntListLocalId(1)),
                        "tail".into()
                    )),
                )),
                None,
                crate::plan::PanicSite::unknown(),
                crate::plan::SourceSpan::new(0, 0),
            )
            .kind(),
            &StepKind::AssertList {
                local: ListLocal::int(IntListLocalId(0)),
                pattern: AssertPattern::list(ListAssertPattern::new(
                    ValueType::Int,
                    vec![AssertPattern::Discard],
                    Some(ListAssertTail::bind(
                        ListLocal::int(IntListLocalId(1)),
                        "tail".into()
                    )),
                )),
                message: None,
                site: crate::plan::PanicSite::unknown(),
                pattern_span: crate::plan::SourceSpan::new(0, 0),
            },
        );
    }

    fn function_expr() -> crate::plan::IntFunctionExpr {
        crate::plan::IntFunctionExpr::value(IntFunctionValue::new(
            IntFunctionId(0),
            vec![ParamLocal::int(IntLocalId(0))],
        ))
    }
}
