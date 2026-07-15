use super::{ExecutionError, Panic, PanicDetails};
use crate::plan::{FunctionType, ValueType};
use crate::runtime::Value;
use miette::{Diagnostic, LabeledSpan, SourceCode};
use std::fmt;

impl Diagnostic for Panic {
    fn code<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        Some(Box::new(format!("geam::{}", self.kind().code())))
    }

    fn help<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        match self.details() {
            Some(PanicDetails::LetAssert { value, .. }) => {
                Some(Box::new(format!("failed value: {}", render_value(value))))
            }
            None => None,
        }
    }

    fn source_code(&self) -> Option<&dyn SourceCode> {
        self.source().map(|source| source as &dyn SourceCode)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        self.source()?;

        let mut labels = vec![LabeledSpan::new_primary_with_span(
            Some(self.primary_label()),
            self.site().span().to_miette(),
        )];
        if let Some(PanicDetails::LetAssert { pattern_span, .. }) = self.details() {
            labels.push(LabeledSpan::at(pattern_span.to_miette(), "pattern"));
        }

        Some(Box::new(labels.into_iter()))
    }
}

impl Diagnostic for ExecutionError {
    fn code<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        match self {
            Self::Panic(panic) => panic.code(),
            Self::FunctionReturnFamilyMismatch { .. } => {
                Some(Box::new("geam::function_return_family_mismatch"))
            }
            Self::TupleIndexFamilyMismatch { .. } => Some(Box::new("geam::tuple_index_mismatch")),
            Self::CustomFieldFamilyMismatch { .. } => {
                Some(Box::new("geam::custom_field_family_mismatch"))
            }
            Self::CustomFieldArityMismatch { .. } => {
                Some(Box::new("geam::custom_field_arity_mismatch"))
            }
            Self::CustomFieldDiscriminantMismatch { .. } => {
                Some(Box::new("geam::custom_field_discriminant_mismatch"))
            }
            Self::ListIndexOutOfBounds { .. } => Some(Box::new("geam::list_index_out_of_bounds")),
        }
    }

    fn help<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        match self {
            Self::Panic(panic) => panic.help(),
            Self::FunctionReturnFamilyMismatch { .. }
            | Self::TupleIndexFamilyMismatch { .. }
            | Self::CustomFieldFamilyMismatch { .. }
            | Self::CustomFieldArityMismatch { .. }
            | Self::CustomFieldDiscriminantMismatch { .. }
            | Self::ListIndexOutOfBounds { .. } => None,
        }
    }

    fn source_code(&self) -> Option<&dyn SourceCode> {
        match self {
            Self::Panic(panic) => panic.source_code(),
            Self::FunctionReturnFamilyMismatch { .. }
            | Self::TupleIndexFamilyMismatch { .. }
            | Self::CustomFieldFamilyMismatch { .. }
            | Self::CustomFieldArityMismatch { .. }
            | Self::CustomFieldDiscriminantMismatch { .. }
            | Self::ListIndexOutOfBounds { .. } => None,
        }
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        match self {
            Self::Panic(panic) => panic.labels(),
            Self::FunctionReturnFamilyMismatch { .. }
            | Self::TupleIndexFamilyMismatch { .. }
            | Self::CustomFieldFamilyMismatch { .. }
            | Self::CustomFieldArityMismatch { .. }
            | Self::CustomFieldDiscriminantMismatch { .. }
            | Self::ListIndexOutOfBounds { .. } => None,
        }
    }
}

fn render_value(value: &Value) -> String {
    match value {
        Value::Int(value) => format!("Int({value})"),
        Value::Float(value) => format!("Float({value:?})"),
        Value::String(value) => format!("String({value:?})"),
        Value::BitArray(value) => format!(
            "BitArray(bytes={:?}, bit_len={})",
            value.bytes(),
            value.bit_len(),
        ),
        Value::UtfCodepoint(value) => format!("UtfCodepoint({value:?})"),
        Value::Custom(value) => format!(
            "{}::{}({})",
            render_custom_type(value.type_()),
            value.constructor_name(),
            value
                .fields()
                .iter()
                .map(|field| render_value(field.value()))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Value::Bool(value) => format!("Bool({value})"),
        Value::Nil => "Nil".into(),
        Value::Tuple(values) => format!(
            "Tuple([{}])",
            values
                .iter()
                .map(render_value)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Value::List(value) => format!(
            "List({})([{}])",
            render_value_type(&value.item_type()),
            value
                .to_values()
                .iter()
                .map(render_value)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Value::Function(function) => {
            let type_ = function.type_();
            format!("Function({})", render_function_type(&type_))
        }
    }
}

fn render_function_type(type_: &FunctionType) -> String {
    let arguments = type_
        .argument_types()
        .iter()
        .map(render_value_type)
        .collect::<Vec<_>>()
        .join(", ");

    format!("fn({arguments}) -> {}", render_value_type(type_.return_()))
}

fn render_value_type(type_: &ValueType) -> String {
    match type_ {
        ValueType::Int => "Int".into(),
        ValueType::Float => "Float".into(),
        ValueType::String => "String".into(),
        ValueType::BitArray => "BitArray".into(),
        ValueType::UtfCodepoint => "UtfCodepoint".into(),
        ValueType::Bool => "Bool".into(),
        ValueType::Nil => "Nil".into(),
        ValueType::Tuple(types) => format!(
            "#({})",
            types
                .iter()
                .map(render_value_type)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        ValueType::List(element) => format!("List({})", render_value_type(element)),
        ValueType::Function(type_) => render_function_type(type_),
        ValueType::Custom(type_) => render_custom_type(type_),
    }
}

fn render_custom_type(type_: &crate::plan::CustomType) -> String {
    let name = type_.type_name();
    let identity = format!("{}/{}/{}", name.package(), name.module(), name.name());
    if type_.arguments().is_empty() {
        identity
    } else {
        format!(
            "{}({})",
            identity,
            type_
                .arguments()
                .iter()
                .map(render_value_type)
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{render_value, render_value_type};
    use crate::plan::execution::{
        FunctionReturnFamily, IntFunctionId, IntLocalId, ParamLocal, RuntimeFunctionId,
    };
    use crate::plan::{
        CustomType, CustomTypeName, FunctionType, PanicSite, SourceContext, SourceSpan, ValueType,
    };
    use crate::runtime::{BitArrayValue, FunctionValue, ListValue, Value};
    use crate::runtime::{ExecutionError, Panic, PanicDetails, PanicKind, PanicMessage};
    use miette::Diagnostic;

    #[test]
    fn source_less_panic_diagnostic_has_no_source_labels_or_help() {
        let error =
            ExecutionError::source_panic(None, PanicKind::Panic, None, PanicSite::unknown());

        assert_eq!(
            error.code().map(|code| code.to_string()),
            Some("geam::panic".into()),
        );
        assert!(error.help().is_none());
        assert!(error.source_code().is_none());
        assert!(error.labels().is_none());
    }

    #[test]
    fn panic_diagnostic_has_source_labels_and_failed_value_help() {
        let source = SourceContext::new(
            "main.gleam",
            "pub fn main() {\n  let assert [x, ..] = []\n}",
        );
        let panic = Panic::new(
            PanicKind::LetAssert,
            PanicMessage::Default,
            PanicSite::new("main".into(), "main".into(), SourceSpan::new(18, 43)),
            Some(&source),
            Some(PanicDetails::LetAssert {
                value: Value::List(crate::runtime::ListValue::empty(ValueType::Int)),
                pattern_span: SourceSpan::new(29, 36),
            }),
        );
        let labels = panic
            .labels()
            .expect("source-backed panic should have labels")
            .collect::<Vec<_>>();

        assert_eq!(
            panic.code().map(|code| code.to_string()),
            Some("geam::let_assert".into())
        );
        assert!(panic.source_code().is_some());
        assert_eq!(labels.len(), 2);
        assert_eq!(labels[0].label(), Some("let assert in main.main"));
        assert_eq!(labels[0].offset(), 18);
        assert_eq!(labels[0].len(), 25);
        assert_eq!(labels[1].label(), Some("pattern"));
        assert_eq!(labels[1].offset(), 29);
        assert_eq!(
            panic.help().map(|help| help.to_string()),
            Some("failed value: List(Int)([])".into()),
        );
    }

    #[test]
    fn source_backed_panic_without_details_has_one_primary_label() {
        let source = SourceContext::new("main.gleam", "pub fn main() {\n  assert False\n}");
        let panic = Panic::new(
            PanicKind::Assert,
            PanicMessage::Default,
            PanicSite::new("main".into(), "main".into(), SourceSpan::new(18, 30)),
            Some(&source),
            None,
        );
        let labels = panic
            .labels()
            .expect("source-backed panic should have labels")
            .collect::<Vec<_>>();

        assert_eq!(labels.len(), 1);
        assert_eq!(labels[0].label(), Some("assert in main.main"));
        assert_eq!(labels[0].offset(), 18);
        assert_eq!(labels[0].len(), 12);
        assert!(panic.help().is_none());
    }

    #[test]
    fn invariant_diagnostics_have_codes_without_source_labels_or_help() {
        for (error, expected_code) in [
            (
                ExecutionError::FunctionReturnFamilyMismatch {
                    expected: FunctionReturnFamily::Int,
                    actual: FunctionReturnFamily::String,
                },
                "geam::function_return_family_mismatch",
            ),
            (
                ExecutionError::TupleIndexFamilyMismatch {
                    expected: ValueType::Int,
                    actual: ValueType::String,
                },
                "geam::tuple_index_mismatch",
            ),
            (
                ExecutionError::CustomFieldFamilyMismatch {
                    custom_type: CustomType::new(
                        CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
                        Vec::new(),
                    ),
                    constructor: "Boxed".into(),
                    field_index: 0,
                    expected: ValueType::Int,
                    actual: ValueType::String,
                },
                "geam::custom_field_family_mismatch",
            ),
            (
                ExecutionError::CustomFieldArityMismatch {
                    custom_type: CustomType::new(
                        CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
                        Vec::new(),
                    ),
                    constructor: "Boxed".into(),
                    expected: 1,
                    actual: 0,
                },
                "geam::custom_field_arity_mismatch",
            ),
            (
                ExecutionError::CustomFieldDiscriminantMismatch {
                    expected_type: CustomType::new(
                        CustomTypeName::new("geam".into(), "main".into(), "Shape".into()),
                        Vec::new(),
                    ),
                    expected_constructors: vec!["Circle".into()],
                    actual_type: CustomType::new(
                        CustomTypeName::new("geam".into(), "main".into(), "Shape".into()),
                        Vec::new(),
                    ),
                    actual_constructor: "Square".into(),
                },
                "geam::custom_field_discriminant_mismatch",
            ),
            (
                ExecutionError::ListIndexOutOfBounds {
                    item_type: ValueType::Int,
                    index: 1,
                    length: 1,
                },
                "geam::list_index_out_of_bounds",
            ),
        ] {
            assert_eq!(
                error.code().map(|code| code.to_string()),
                Some(expected_code.into()),
            );
            assert!(error.help().is_none());
            assert!(error.source_code().is_none());
            assert!(error.labels().is_none());
        }
    }

    #[test]
    fn render_value_preserves_every_runtime_value_family() {
        let function = Value::Function(FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![ParamLocal::Int(IntLocalId(0))],
            crate::plan::FunctionType::new(
                vec![crate::plan::ValueType::Int],
                crate::plan::ValueType::Int,
            ),
        ));

        for (value, expected) in [
            (Value::Int(1.into()), "Int(1)"),
            (Value::Float(1.5), "Float(1.5)"),
            (Value::String("one".into()), "String(\"one\")"),
            (
                Value::BitArray(BitArrayValue::from_bytes(vec![0xa5])),
                "BitArray(bytes=[165], bit_len=8)",
            ),
            (
                Value::UtfCodepoint('\u{10ffff}'),
                "UtfCodepoint('\\u{10ffff}')",
            ),
            (Value::Bool(true), "Bool(true)"),
            (Value::Nil, "Nil"),
            (
                Value::Tuple(vec![Value::Int(1.into()), Value::String("one".into())]),
                "Tuple([Int(1), String(\"one\")])",
            ),
            (
                Value::List(ListValue::int(vec![1.into(), 2.into()])),
                "List(Int)([Int(1), Int(2)])",
            ),
            (function, "Function(fn(Int) -> Int)"),
        ] {
            assert_eq!(render_value(&value), expected);
        }

        let custom =
            crate::runtime::run_src("pub type Boxed { Boxed(Int) } pub fn main() { Boxed(1) }");
        assert_eq!(render_value(&custom), "geam/main/Boxed::Boxed(Int(1))",);
    }

    #[test]
    fn render_value_type_preserves_compound_shapes() {
        assert_eq!(render_value_type(&ValueType::BitArray), "BitArray");
        assert_eq!(render_value_type(&ValueType::UtfCodepoint), "UtfCodepoint");
        assert_eq!(
            render_value_type(&ValueType::Tuple(vec![ValueType::Int, ValueType::String])),
            "#(Int, String)",
        );
        assert_eq!(
            render_value_type(&ValueType::List(Box::new(ValueType::Bool))),
            "List(Bool)",
        );
        assert_eq!(
            render_value_type(&ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Float],
                ValueType::Nil,
            )))),
            "fn(Float) -> Nil",
        );
        assert_eq!(
            render_value_type(&ValueType::Custom(CustomType::new(
                CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
                vec![ValueType::Int],
            ))),
            "geam/main/Boxed(Int)",
        );
    }
}
