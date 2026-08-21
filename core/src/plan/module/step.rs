use super::expression::{
    BitArrayExpr, BitArrayFunctionExpr, BoolExpr, BoolFunctionExpr, CustomExpr, CustomFunctionExpr,
    CustomLocalExpr, Expr, ExternalExpr, ExternalFunctionExpr, FloatExpr, FloatFunctionExpr,
    FunctionFunctionExpr, GenericExpr, GenericFunctionExpr, IntExpr, IntFunctionExpr,
    ListFunctionExpr, ListLocalExpr, NilExpr, NilFunctionExpr, StringExpr, StringFunctionExpr,
    TupleExpr, TupleFunctionExpr, TypedFunctionExpr, UtfCodepointExpr, UtfCodepointFunctionExpr,
};
use super::function::{ParamLocal, ParamSlot};
use super::id::{
    BitArrayFunctionLocalId, BitArrayLocalId, BoolFunctionLocalId, BoolLocalId,
    CustomFunctionLocal, CustomFunctionLocalId, CustomLocal, CustomLocalId, ExternalFunctionLocal,
    ExternalFunctionLocalId, ExternalLocal, FloatFunctionLocalId, FloatLocalId,
    FunctionFunctionLocal, FunctionFunctionLocalId, GenericFunctionLocal, GenericLocal,
    IntFunctionLocalId, IntLocalId, ListFunctionLocal, ListLocal, NilFunctionLocalId, NilLocalId,
    StringFunctionLocalId, StringLocalId, TupleFunctionLocalId, TupleLocalId,
    UtfCodepointFunctionLocalId, UtfCodepointLocalId,
};
use crate::plan::{BitArrayPattern, CustomBindingPattern};
use crate::plan::{EchoSite, PanicSite, SourceSpan, ValueType};
use ecow::EcoString;

#[derive(Debug, Clone, PartialEq)]
pub struct Step {
    kind: StepKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AssertBinding {
    slot: ParamSlot,
    name: EcoString,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StringAssertBinding {
    local: StringLocalId,
    name: EcoString,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum AssertPattern {
    Bind(AssertBinding),
    Discard,
    Int(num_bigint::BigInt),
    Float(f64),
    String(EcoString),
    Bool(bool),
    Nil,
    Tuple(Vec<AssertPattern>),
    List(ListAssertPattern),
    BitArray(BitArrayPattern),
    Custom(crate::plan::CustomPattern),
    StringPrefix {
        prefix: EcoString,
        left: Option<StringAssertBinding>,
        right: Option<StringAssertBinding>,
    },
    Alias {
        pattern: Box<AssertPattern>,
        binding: AssertBinding,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AssertSubject {
    Int(IntLocalId),
    Float(FloatLocalId),
    String(StringLocalId),
    BitArray(BitArrayLocalId),
    Custom(CustomLocal),
    Bool(BoolLocalId),
    Nil(NilLocalId),
    Tuple(TupleLocalId),
    List(ListLocal),
}

#[derive(Debug, Clone, PartialEq)]
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
    LetGeneric {
        local: GenericLocal,
        name: EcoString,
        value: GenericExpr,
    },
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
    LetBitArray {
        local: BitArrayLocalId,
        name: EcoString,
        value: BitArrayExpr,
    },
    LetUtfCodepoint {
        local: UtfCodepointLocalId,
        name: EcoString,
        value: UtfCodepointExpr,
    },
    LetCustom {
        binding: CustomLocalExpr,
        name: EcoString,
    },
    LetExternal {
        local: ExternalLocal,
        name: EcoString,
        value: ExternalExpr,
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
        name: EcoString,
        value: ListLocalExpr,
    },
    LetIntFunction {
        local: IntFunctionLocalId,
        name: EcoString,
        value: TypedFunctionExpr<IntFunctionExpr>,
    },
    LetFloatFunction {
        local: FloatFunctionLocalId,
        name: EcoString,
        value: TypedFunctionExpr<FloatFunctionExpr>,
    },
    LetStringFunction {
        local: StringFunctionLocalId,
        name: EcoString,
        value: TypedFunctionExpr<StringFunctionExpr>,
    },
    LetBitArrayFunction {
        local: BitArrayFunctionLocalId,
        name: EcoString,
        value: TypedFunctionExpr<BitArrayFunctionExpr>,
    },
    LetUtfCodepointFunction {
        local: UtfCodepointFunctionLocalId,
        name: EcoString,
        value: TypedFunctionExpr<UtfCodepointFunctionExpr>,
    },
    LetCustomFunction {
        local: CustomFunctionLocal,
        name: EcoString,
        value: TypedFunctionExpr<CustomFunctionExpr>,
    },
    LetExternalFunction {
        local: ExternalFunctionLocal,
        name: EcoString,
        value: TypedFunctionExpr<ExternalFunctionExpr>,
    },
    LetBoolFunction {
        local: BoolFunctionLocalId,
        name: EcoString,
        value: TypedFunctionExpr<BoolFunctionExpr>,
    },
    LetNilFunction {
        local: NilFunctionLocalId,
        name: EcoString,
        value: TypedFunctionExpr<NilFunctionExpr>,
    },
    LetTupleFunction {
        local: TupleFunctionLocalId,
        name: EcoString,
        value: TypedFunctionExpr<TupleFunctionExpr>,
    },
    LetListFunction {
        local: ListFunctionLocal,
        name: EcoString,
        value: TypedFunctionExpr<ListFunctionExpr>,
    },
    LetFunctionFunction {
        local: FunctionFunctionLocal,
        name: EcoString,
        value: TypedFunctionExpr<FunctionFunctionExpr>,
    },
    LetGenericFunction {
        local: GenericFunctionLocal,
        name: EcoString,
        value: TypedFunctionExpr<GenericFunctionExpr>,
    },
    Echo(Echo),
    AssertPattern {
        subject: AssertSubject,
        pattern: AssertPattern,
        message: Option<StringExpr>,
        site: PanicSite,
        pattern_span: SourceSpan,
    },
    BindCustomFields {
        local: CustomLocal,
        pattern: CustomBindingPattern,
    },
    AssertBool {
        condition: BoolExpr,
        message: Option<StringExpr>,
        site: PanicSite,
    },
    Evaluate(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Echo {
    subject: EchoSubject,
    message: Option<StringExpr>,
    site: EchoSite,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum EchoSubject {
    Generic {
        local: GenericLocal,
        value: GenericExpr,
    },
    Int {
        local: IntLocalId,
        value: IntExpr,
    },
    Float {
        local: FloatLocalId,
        value: FloatExpr,
    },
    String {
        local: StringLocalId,
        value: StringExpr,
    },
    BitArray {
        local: BitArrayLocalId,
        value: BitArrayExpr,
    },
    UtfCodepoint {
        local: UtfCodepointLocalId,
        value: UtfCodepointExpr,
    },
    Custom(CustomLocalExpr),
    External {
        local: ExternalLocal,
        value: ExternalExpr,
    },
    Bool {
        local: BoolLocalId,
        value: BoolExpr,
    },
    Nil {
        local: NilLocalId,
        value: NilExpr,
    },
    Tuple {
        local: TupleLocalId,
        value: TupleExpr,
    },
    List(ListLocalExpr),
    IntFunction {
        local: IntFunctionLocalId,
        value: TypedFunctionExpr<IntFunctionExpr>,
    },
    FloatFunction {
        local: FloatFunctionLocalId,
        value: TypedFunctionExpr<FloatFunctionExpr>,
    },
    StringFunction {
        local: StringFunctionLocalId,
        value: TypedFunctionExpr<StringFunctionExpr>,
    },
    BitArrayFunction {
        local: BitArrayFunctionLocalId,
        value: TypedFunctionExpr<BitArrayFunctionExpr>,
    },
    UtfCodepointFunction {
        local: UtfCodepointFunctionLocalId,
        value: TypedFunctionExpr<UtfCodepointFunctionExpr>,
    },
    CustomFunction {
        local: CustomFunctionLocal,
        value: TypedFunctionExpr<CustomFunctionExpr>,
    },
    ExternalFunction {
        local: ExternalFunctionLocal,
        value: TypedFunctionExpr<ExternalFunctionExpr>,
    },
    BoolFunction {
        local: BoolFunctionLocalId,
        value: TypedFunctionExpr<BoolFunctionExpr>,
    },
    NilFunction {
        local: NilFunctionLocalId,
        value: TypedFunctionExpr<NilFunctionExpr>,
    },
    TupleFunction {
        local: TupleFunctionLocalId,
        value: TypedFunctionExpr<TupleFunctionExpr>,
    },
    ListFunction {
        local: ListFunctionLocal,
        value: TypedFunctionExpr<ListFunctionExpr>,
    },
    FunctionFunction {
        local: FunctionFunctionLocal,
        value: TypedFunctionExpr<FunctionFunctionExpr>,
    },
    GenericFunction {
        local: GenericFunctionLocal,
        value: TypedFunctionExpr<GenericFunctionExpr>,
    },
}

impl AssertBinding {
    pub(crate) fn new(local: ParamLocal, name: EcoString, shape: crate::plan::ValueShape) -> Self {
        Self {
            slot: ParamSlot::new(local, shape),
            name,
        }
    }

    pub(crate) fn local(&self) -> &ParamLocal {
        self.slot.local()
    }

    pub(crate) fn slot(&self) -> &ParamSlot {
        &self.slot
    }
}

impl StringAssertBinding {
    pub(crate) fn new(local: StringLocalId, name: EcoString) -> Self {
        Self { local, name }
    }

    pub(crate) fn local(&self) -> StringLocalId {
        self.local
    }
}

impl AssertPattern {
    pub(crate) fn list(pattern: ListAssertPattern) -> Self {
        Self::List(pattern)
    }

    pub(crate) fn bit_array(pattern: BitArrayPattern) -> Self {
        Self::BitArray(pattern)
    }

    pub(crate) fn custom(pattern: crate::plan::CustomPattern) -> Self {
        Self::Custom(pattern)
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

    pub(crate) fn element_type(&self) -> &ValueType {
        &self.element_type
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
    pub(crate) fn let_generic(local: GenericLocal, name: EcoString, value: GenericExpr) -> Self {
        Self {
            kind: StepKind::LetGeneric { local, name, value },
        }
    }

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

    pub(crate) fn let_bit_array(
        local: BitArrayLocalId,
        name: EcoString,
        value: BitArrayExpr,
    ) -> Self {
        Self {
            kind: StepKind::LetBitArray { local, name, value },
        }
    }

    pub(crate) fn let_utf_codepoint(
        local: UtfCodepointLocalId,
        name: EcoString,
        value: UtfCodepointExpr,
    ) -> Self {
        Self {
            kind: StepKind::LetUtfCodepoint { local, name, value },
        }
    }

    pub(crate) fn let_custom(local: CustomLocalId, name: EcoString, value: CustomExpr) -> Self {
        Self {
            kind: StepKind::LetCustom {
                binding: CustomLocalExpr::from_value(local, value),
                name,
            },
        }
    }

    pub(crate) fn let_external(local: ExternalLocal, name: EcoString, value: ExternalExpr) -> Self {
        Self {
            kind: StepKind::LetExternal { local, name, value },
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

    pub(crate) fn let_list_expr(name: EcoString, value: ListLocalExpr) -> Self {
        Self {
            kind: StepKind::LetList { name, value },
        }
    }

    pub(crate) fn let_int_function_expr(
        local: IntFunctionLocalId,
        name: EcoString,
        value: TypedFunctionExpr<IntFunctionExpr>,
    ) -> Self {
        Self {
            kind: StepKind::LetIntFunction { local, name, value },
        }
    }

    pub(crate) fn let_float_function_expr(
        local: FloatFunctionLocalId,
        name: EcoString,
        value: TypedFunctionExpr<FloatFunctionExpr>,
    ) -> Self {
        Self {
            kind: StepKind::LetFloatFunction { local, name, value },
        }
    }

    pub(crate) fn let_string_function_expr(
        local: StringFunctionLocalId,
        name: EcoString,
        value: TypedFunctionExpr<StringFunctionExpr>,
    ) -> Self {
        Self {
            kind: StepKind::LetStringFunction { local, name, value },
        }
    }

    pub(crate) fn let_bit_array_function_expr(
        local: BitArrayFunctionLocalId,
        name: EcoString,
        value: TypedFunctionExpr<BitArrayFunctionExpr>,
    ) -> Self {
        Self {
            kind: StepKind::LetBitArrayFunction { local, name, value },
        }
    }

    pub(crate) fn let_utf_codepoint_function_expr(
        local: UtfCodepointFunctionLocalId,
        name: EcoString,
        value: TypedFunctionExpr<UtfCodepointFunctionExpr>,
    ) -> Self {
        Self {
            kind: StepKind::LetUtfCodepointFunction { local, name, value },
        }
    }

    pub(crate) fn let_custom_function_expr(
        local: CustomFunctionLocalId,
        name: EcoString,
        value: TypedFunctionExpr<CustomFunctionExpr>,
    ) -> Self {
        let local =
            CustomFunctionLocal::new(local, value.expression().custom_function_type().clone());
        Self {
            kind: StepKind::LetCustomFunction { local, name, value },
        }
    }

    pub(crate) fn let_external_function_expr(
        local: ExternalFunctionLocalId,
        name: EcoString,
        value: TypedFunctionExpr<ExternalFunctionExpr>,
    ) -> Self {
        let local =
            ExternalFunctionLocal::new(local, value.expression().external_function_type().clone());
        Self {
            kind: StepKind::LetExternalFunction { local, name, value },
        }
    }

    pub(crate) fn let_bool_function_expr(
        local: BoolFunctionLocalId,
        name: EcoString,
        value: TypedFunctionExpr<BoolFunctionExpr>,
    ) -> Self {
        Self {
            kind: StepKind::LetBoolFunction { local, name, value },
        }
    }

    pub(crate) fn let_nil_function_expr(
        local: NilFunctionLocalId,
        name: EcoString,
        value: TypedFunctionExpr<NilFunctionExpr>,
    ) -> Self {
        Self {
            kind: StepKind::LetNilFunction { local, name, value },
        }
    }

    pub(crate) fn let_tuple_function_expr(
        local: TupleFunctionLocalId,
        name: EcoString,
        value: TypedFunctionExpr<TupleFunctionExpr>,
    ) -> Self {
        Self {
            kind: StepKind::LetTupleFunction { local, name, value },
        }
    }

    pub(crate) fn let_list_function_expr(
        local: ListFunctionLocal,
        name: EcoString,
        value: TypedFunctionExpr<ListFunctionExpr>,
    ) -> Self {
        Self {
            kind: StepKind::LetListFunction { local, name, value },
        }
    }

    pub(crate) fn let_function_function_expr(
        local: FunctionFunctionLocalId,
        name: EcoString,
        value: TypedFunctionExpr<FunctionFunctionExpr>,
    ) -> Self {
        let local =
            FunctionFunctionLocal::new(local, value.expression().function_function_type().clone());
        Self {
            kind: StepKind::LetFunctionFunction { local, name, value },
        }
    }

    pub(crate) fn let_generic_function_expr(
        local: GenericFunctionLocal,
        name: EcoString,
        value: TypedFunctionExpr<GenericFunctionExpr>,
    ) -> Self {
        Self {
            kind: StepKind::LetGenericFunction { local, name, value },
        }
    }

    pub(crate) fn echo(subject: EchoSubject, message: Option<StringExpr>, site: EchoSite) -> Self {
        Self {
            kind: StepKind::Echo(Echo {
                subject,
                message,
                site,
            }),
        }
    }

    #[cfg(test)]
    pub(crate) fn let_int_function(
        local: IntFunctionLocalId,
        name: EcoString,
        value: IntFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::let_int_function_expr(local, name, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn let_float_function(
        local: FloatFunctionLocalId,
        name: EcoString,
        value: FloatFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::let_float_function_expr(local, name, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn let_string_function(
        local: StringFunctionLocalId,
        name: EcoString,
        value: StringFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::let_string_function_expr(local, name, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn let_bit_array_function(
        local: BitArrayFunctionLocalId,
        name: EcoString,
        value: BitArrayFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::let_bit_array_function_expr(local, name, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn let_utf_codepoint_function(
        local: UtfCodepointFunctionLocalId,
        name: EcoString,
        value: UtfCodepointFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::let_utf_codepoint_function_expr(local, name, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn let_custom_function(
        local: CustomFunctionLocalId,
        name: EcoString,
        value: CustomFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::new(
            value.custom_function_type().argument_shapes().to_vec(),
            crate::plan::ValueShape::Custom(value.custom_function_type().return_().clone()),
        );
        Self::let_custom_function_expr(local, name, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn let_bool_function(
        local: BoolFunctionLocalId,
        name: EcoString,
        value: BoolFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::let_bool_function_expr(local, name, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn let_nil_function(
        local: NilFunctionLocalId,
        name: EcoString,
        value: NilFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::let_nil_function_expr(local, name, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn let_tuple_function(
        local: TupleFunctionLocalId,
        name: EcoString,
        value: TupleFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::let_tuple_function_expr(local, name, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn let_list_function(
        local: ListFunctionLocal,
        name: EcoString,
        value: ListFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_().clone());
        Self::let_list_function_expr(local, name, TypedFunctionExpr::new(shape, value))
    }

    #[cfg(test)]
    pub(crate) fn let_function_function(
        local: FunctionFunctionLocalId,
        name: EcoString,
        value: FunctionFunctionExpr,
    ) -> Self {
        let shape = crate::plan::FunctionShape::from_function_type(value.type_());
        Self::let_function_function_expr(local, name, TypedFunctionExpr::new(shape, value))
    }

    pub(crate) fn assert_pattern_at(
        subject: AssertSubject,
        pattern: AssertPattern,
        message: Option<StringExpr>,
        site: PanicSite,
        pattern_span: SourceSpan,
    ) -> Self {
        Self {
            kind: StepKind::AssertPattern {
                subject,
                pattern,
                message,
                site,
                pattern_span,
            },
        }
    }

    pub(crate) fn bind_custom_fields(local: CustomLocalId, pattern: CustomBindingPattern) -> Self {
        let local = CustomLocal::from_shape(local, pattern.source_shape().clone());
        Self {
            kind: StepKind::BindCustomFields { local, pattern },
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

    pub(crate) fn evaluate(value: Expr) -> Self {
        Self {
            kind: StepKind::Evaluate(value),
        }
    }

    pub(crate) fn kind(&self) -> &StepKind {
        &self.kind
    }
}

impl Echo {
    pub(crate) fn subject(&self) -> &EchoSubject {
        &self.subject
    }

    pub(crate) fn message(&self) -> Option<&StringExpr> {
        self.message.as_ref()
    }

    pub(crate) fn site(&self) -> &EchoSite {
        &self.site
    }
}

#[cfg(test)]
mod tests {
    use super::{Step, StepKind};
    use crate::plan::module::TypedFunctionExpr;
    use crate::plan::{
        AssertPattern, AssertSubject, BoolExpr, CustomFunctionExpr, CustomFunctionLocal,
        CustomFunctionLocalId, CustomFunctionType, CustomType, CustomTypeName, Expr,
        FunctionFunctionExpr, FunctionFunctionLocal, FunctionFunctionLocalId, FunctionFunctionType,
        FunctionType, IntExpr, IntFunctionLocalId, IntFunctionReference, IntListLocalId,
        IntLocalId, ListAssertPattern, ListAssertTail, ListLocal, PanicExpr, PanicSite, StringExpr,
        ValueShape, ValueType,
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
                value: TypedFunctionExpr::new(
                    crate::plan::FunctionShape::from_function_type(function_expr().type_().clone()),
                    function_expr(),
                ),
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
            Step::assert_pattern_at(
                AssertSubject::List(ListLocal::int(IntListLocalId(0))),
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
            &StepKind::AssertPattern {
                subject: AssertSubject::List(ListLocal::int(IntListLocalId(0))),
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

    #[test]
    fn callable_let_steps_derive_the_local_type_from_the_value() {
        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let custom_function_type =
            CustomFunctionType::new(vec![ValueType::Int], custom_type.clone());
        let custom_value = CustomFunctionExpr::panic(
            PanicExpr::panic_at(None, PanicSite::unknown()),
            custom_function_type.clone(),
        );
        assert_eq!(
            Step::let_custom_function(
                CustomFunctionLocalId(3),
                "custom".into(),
                custom_value.clone(),
            )
            .kind(),
            &StepKind::LetCustomFunction {
                local: CustomFunctionLocal::new(CustomFunctionLocalId(3), custom_function_type,),
                name: "custom".into(),
                value: TypedFunctionExpr::new(
                    crate::plan::FunctionShape::new(
                        custom_value
                            .custom_function_type()
                            .argument_shapes()
                            .to_vec(),
                        ValueShape::Custom(custom_value.custom_function_type().return_().clone(),),
                    ),
                    custom_value,
                ),
            },
        );

        let function_function_type = FunctionFunctionType::new(
            vec![ValueType::String],
            FunctionType::new(vec![ValueType::Bool], ValueType::Int),
        );
        let function_value = FunctionFunctionExpr::panic(
            PanicExpr::panic_at(None, PanicSite::unknown()),
            function_function_type.clone(),
        );
        assert_eq!(
            Step::let_function_function(
                FunctionFunctionLocalId(4),
                "function".into(),
                function_value.clone(),
            )
            .kind(),
            &StepKind::LetFunctionFunction {
                local: FunctionFunctionLocal::new(
                    FunctionFunctionLocalId(4),
                    function_function_type,
                ),
                name: "function".into(),
                value: TypedFunctionExpr::new(
                    crate::plan::FunctionShape::from_function_type(function_value.type_()),
                    function_value,
                ),
            },
        );
    }

    fn function_expr() -> crate::plan::IntFunctionExpr {
        crate::plan::IntFunctionExpr::reference(IntFunctionReference::new(
            crate::plan::monomorphic_function_instantiation(
                0,
                crate::plan::FunctionShape::new(
                    vec![crate::plan::ValueShape::Int],
                    crate::plan::ValueShape::Int,
                ),
            ),
        ))
    }
}
