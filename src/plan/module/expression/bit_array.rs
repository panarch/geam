use super::{
    BitArrayFunctionExpr, BitArrayListExpr, BoolExpr, CallArg, CustomFieldAccess, FloatExpr,
    IntExpr, PanicExpr, StringExpr, TupleExpr, UtfCodepointExpr,
};
use crate::plan::{
    BitArrayLocalId, ConstantBitArrayReference, FunctionInstantiation, PanicSite, Step,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
    Big,
    Little,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringEncoding {
    Utf8,
    Utf16(Endianness),
    Utf32(Endianness),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatBitSize {
    Sixteen,
    ThirtyTwo,
    SixtyFour,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BitArrayEvaluatedSize {
    value: IntExpr,
    unit: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BitArrayBitsSize {
    Fixed(usize),
    Evaluated(BitArrayEvaluatedSize),
}

impl BitArrayEvaluatedSize {
    pub(crate) fn new(value: IntExpr, unit: u8) -> Self {
        Self { value, unit }
    }

    pub(crate) fn value(&self) -> &IntExpr {
        &self.value
    }

    pub(crate) fn unit(&self) -> u8 {
        self.unit
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BitArraySegment {
    Int {
        value: IntExpr,
        bit_size: usize,
        endianness: Endianness,
    },
    EvaluatedInt {
        value: IntExpr,
        size: BitArrayEvaluatedSize,
        endianness: Endianness,
        site: PanicSite,
    },
    Float {
        value: FloatExpr,
        bit_size: FloatBitSize,
        endianness: Endianness,
    },
    EvaluatedFloat {
        value: FloatExpr,
        size: BitArrayEvaluatedSize,
        endianness: Endianness,
        site: PanicSite,
    },
    String {
        value: StringExpr,
        encoding: StringEncoding,
    },
    UtfCodepoint {
        value: UtfCodepointExpr,
        encoding: StringEncoding,
    },
    Bits(BitArrayExpr),
    SizedBits {
        value: BitArrayExpr,
        size: BitArrayBitsSize,
        site: PanicSite,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct BitArrayExpr {
    kind: BitArrayExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BitArrayExprKind {
    Value(Vec<BitArraySegment>),
    Constant(ConstantBitArrayReference),
    LocalGet {
        local: BitArrayLocalId,
        name: EcoString,
    },
    Call {
        function: FunctionInstantiation,
        args: Vec<CallArg>,
    },
    FunctionCall {
        function: Box<BitArrayFunctionExpr>,
        args: Vec<CallArg>,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    CustomField(CustomFieldAccess),
    ListIndex {
        list: Box<BitArrayListExpr>,
        index: usize,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<BitArrayExpr>,
        false_: Box<BitArrayExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, BitArrayExpr)>,
        fallback: Box<BitArrayExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, BitArrayExpr)>,
        fallback: Box<BitArrayExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, BitArrayExpr)>,
        fallback: Box<BitArrayExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<BitArrayExpr>,
    },
}

impl BitArrayExpr {
    pub(crate) fn value(segments: Vec<BitArraySegment>) -> Self {
        Self::new(BitArrayExprKind::Value(segments))
    }

    pub(in crate::plan::module) fn constant(reference: ConstantBitArrayReference) -> Self {
        Self::new(BitArrayExprKind::Constant(reference))
    }

    pub(crate) fn local_get(local: BitArrayLocalId, name: EcoString) -> Self {
        Self::new(BitArrayExprKind::LocalGet { local, name })
    }

    pub(crate) fn call(function: FunctionInstantiation, args: Vec<CallArg>) -> Self {
        Self::new(BitArrayExprKind::Call { function, args })
    }

    pub(crate) fn function_call(function: BitArrayFunctionExpr, args: Vec<CallArg>) -> Self {
        Self::new(BitArrayExprKind::FunctionCall {
            function: Box::new(function),
            args,
        })
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize) -> Self {
        Self::new(BitArrayExprKind::TupleIndex {
            tuple: Box::new(tuple),
            index,
        })
    }

    pub(crate) fn custom_field(access: CustomFieldAccess) -> Self {
        Self::new(BitArrayExprKind::CustomField(access))
    }

    pub(crate) fn list_index(list: BitArrayListExpr, index: usize) -> Self {
        Self::new(BitArrayExprKind::ListIndex {
            list: Box::new(list),
            index,
        })
    }

    pub(crate) fn panic(panic: PanicExpr) -> Self {
        Self::new(BitArrayExprKind::Panic(panic))
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: Self, false_: Self) -> Self {
        Self::new(BitArrayExprKind::BoolCase {
            subject: Box::new(subject),
            true_: Box::new(true_),
            false_: Box::new(false_),
        })
    }

    pub(crate) fn int_case(subject: IntExpr, clauses: Vec<(BigInt, Self)>, fallback: Self) -> Self {
        Self::new(BitArrayExprKind::IntCase {
            subject: Box::new(subject),
            clauses,
            fallback: Box::new(fallback),
        })
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, Self)>,
        fallback: Self,
    ) -> Self {
        Self::new(BitArrayExprKind::StringCase {
            subject: Box::new(subject),
            clauses,
            fallback: Box::new(fallback),
        })
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, Self)>,
        fallback: Self,
    ) -> Self {
        Self::new(BitArrayExprKind::FloatCase {
            subject: Box::new(subject),
            clauses,
            fallback: Box::new(fallback),
        })
    }

    pub(crate) fn block(steps: Vec<Step>, return_: Self) -> Self {
        Self::new(BitArrayExprKind::Block {
            steps,
            return_: Box::new(return_),
        })
    }

    pub(crate) fn kind(&self) -> &BitArrayExprKind {
        &self.kind
    }

    fn new(kind: BitArrayExprKind) -> Self {
        Self { kind }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayExpr, BitArrayExprKind, BitArraySegment,
        Endianness,
    };
    use crate::plan::{
        BitArrayFunctionReference, BitArrayLocalId, BoolExpr, Expr, FloatExpr, FloatLocalId,
        FunctionInstantiation, FunctionShape, IntExpr, IntLocalId, PanicSite, Step, StringExpr,
        TupleExpr, ValueShape, ValueType, monomorphic_function_instantiation,
    };
    use num_bigint::BigInt;

    #[test]
    fn bit_array_expr_kind_accessors() {
        let segments = vec![BitArraySegment::Int {
            value: IntExpr::value(1.into()),
            bit_size: 8,
            endianness: Endianness::Big,
        }];
        assert_eq!(
            BitArrayExpr::value(segments.clone()).kind(),
            &BitArrayExprKind::Value(segments),
        );
        assert_eq!(
            BitArrayExpr::local_get(BitArrayLocalId(0), "value".into()).kind(),
            &BitArrayExprKind::LocalGet {
                local: BitArrayLocalId(0),
                name: "value".into(),
            },
        );
        assert_eq!(
            BitArrayExpr::call(function_instantiation(), Vec::new()).kind(),
            &BitArrayExprKind::Call {
                function: function_instantiation(),
                args: Vec::new(),
            },
        );
        assert_eq!(
            BitArrayExpr::function_call(function_expr(), Vec::new()).kind(),
            &BitArrayExprKind::FunctionCall {
                function: Box::new(function_expr()),
                args: Vec::new(),
            },
        );
        assert_eq!(
            BitArrayExpr::tuple_index(tuple_expr(), 0).kind(),
            &BitArrayExprKind::TupleIndex {
                tuple: Box::new(tuple_expr()),
                index: 0,
            },
        );
        assert_eq!(
            BitArrayExpr::bool_case(
                BoolExpr::value(true),
                bit_array_value(1),
                bit_array_value(2),
            )
            .kind(),
            &BitArrayExprKind::BoolCase {
                subject: Box::new(BoolExpr::value(true)),
                true_: Box::new(bit_array_value(1)),
                false_: Box::new(bit_array_value(2)),
            },
        );
        assert_eq!(
            BitArrayExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), bit_array_value(1))],
                bit_array_value(2),
            )
            .kind(),
            &BitArrayExprKind::IntCase {
                subject: Box::new(IntExpr::value(1.into())),
                clauses: vec![(BigInt::from(1), bit_array_value(1))],
                fallback: Box::new(bit_array_value(2)),
            },
        );
        assert_eq!(
            BitArrayExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), bit_array_value(1))],
                bit_array_value(2),
            )
            .kind(),
            &BitArrayExprKind::StringCase {
                subject: Box::new(StringExpr::value("one".into())),
                clauses: vec![("one".into(), bit_array_value(1))],
                fallback: Box::new(bit_array_value(2)),
            },
        );
        assert_eq!(
            BitArrayExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, bit_array_value(1))],
                bit_array_value(2),
            )
            .kind(),
            &BitArrayExprKind::FloatCase {
                subject: Box::new(FloatExpr::value(1.0)),
                clauses: vec![(1.0, bit_array_value(1))],
                fallback: Box::new(bit_array_value(2)),
            },
        );
        assert_eq!(
            BitArrayExpr::block(
                vec![Step::evaluate(Expr::bit_array(bit_array_value(1)))],
                bit_array_value(2),
            )
            .kind(),
            &BitArrayExprKind::Block {
                steps: vec![Step::evaluate(Expr::bit_array(bit_array_value(1)))],
                return_: Box::new(bit_array_value(2)),
            },
        );
    }

    #[test]
    fn evaluated_segment_owners_preserve_value_size_and_failure_site() {
        let site = PanicSite::new(
            "main".into(),
            "main".into(),
            crate::plan::SourceSpan::new(4, 20),
        );
        let int_size =
            BitArrayEvaluatedSize::new(IntExpr::local_get(IntLocalId(1), "int_size".into()), 2);
        let float_size =
            BitArrayEvaluatedSize::new(IntExpr::local_get(IntLocalId(2), "float_size".into()), 4);
        let bits_size =
            BitArrayEvaluatedSize::new(IntExpr::local_get(IntLocalId(3), "bits_size".into()), 8);
        let segments = vec![
            BitArraySegment::EvaluatedInt {
                value: IntExpr::local_get(IntLocalId(0), "int_value".into()),
                size: int_size.clone(),
                endianness: Endianness::Little,
                site: site.clone(),
            },
            BitArraySegment::EvaluatedFloat {
                value: FloatExpr::local_get(FloatLocalId(0), "float_value".into()),
                size: float_size.clone(),
                endianness: Endianness::Big,
                site: site.clone(),
            },
            BitArraySegment::SizedBits {
                value: BitArrayExpr::local_get(BitArrayLocalId(0), "bits".into()),
                size: BitArrayBitsSize::Fixed(12),
                site: site.clone(),
            },
            BitArraySegment::SizedBits {
                value: BitArrayExpr::local_get(BitArrayLocalId(1), "dynamic_bits".into()),
                size: BitArrayBitsSize::Evaluated(bits_size.clone()),
                site: site.clone(),
            },
        ];

        assert_eq!(
            BitArrayExpr::value(segments.clone()).kind(),
            &BitArrayExprKind::Value(segments),
        );
        assert_eq!(
            int_size.value(),
            &IntExpr::local_get(IntLocalId(1), "int_size".into())
        );
        assert_eq!(int_size.unit(), 2);
        assert_eq!(float_size.unit(), 4);
        assert_eq!(bits_size.unit(), 8);
    }

    fn bit_array_value(value: u8) -> BitArrayExpr {
        BitArrayExpr::value(vec![BitArraySegment::Int {
            value: IntExpr::value(value.into()),
            bit_size: 8,
            endianness: Endianness::Big,
        }])
    }

    fn function_expr() -> crate::plan::BitArrayFunctionExpr {
        crate::plan::BitArrayFunctionExpr::reference(BitArrayFunctionReference::new(
            function_instantiation(),
            Vec::new(),
        ))
    }

    fn function_instantiation() -> FunctionInstantiation {
        monomorphic_function_instantiation(0, FunctionShape::new(Vec::new(), ValueShape::BitArray))
    }

    fn tuple_expr() -> TupleExpr {
        TupleExpr::value(
            vec![Expr::bit_array(bit_array_value(1))],
            vec![ValueType::BitArray],
        )
    }
}
