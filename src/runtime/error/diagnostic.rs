use super::{ExecutionError, Panic, PanicDetails};
use crate::plan::{FunctionType, Value, ValueType};
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
            Self::ListIndexFamilyMismatch { .. } => {
                Some(Box::new("geam::list_index_family_mismatch"))
            }
            Self::ListIndexOutOfBounds { .. } => Some(Box::new("geam::list_index_out_of_bounds")),
        }
    }

    fn help<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        match self {
            Self::Panic(panic) => panic.help(),
            Self::FunctionReturnFamilyMismatch { .. }
            | Self::TupleIndexFamilyMismatch { .. }
            | Self::ListIndexFamilyMismatch { .. }
            | Self::ListIndexOutOfBounds { .. } => None,
        }
    }

    fn source_code(&self) -> Option<&dyn SourceCode> {
        match self {
            Self::Panic(panic) => panic.source_code(),
            Self::FunctionReturnFamilyMismatch { .. }
            | Self::TupleIndexFamilyMismatch { .. }
            | Self::ListIndexFamilyMismatch { .. }
            | Self::ListIndexOutOfBounds { .. } => None,
        }
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        match self {
            Self::Panic(panic) => panic.labels(),
            Self::FunctionReturnFamilyMismatch { .. }
            | Self::TupleIndexFamilyMismatch { .. }
            | Self::ListIndexFamilyMismatch { .. }
            | Self::ListIndexOutOfBounds { .. } => None,
        }
    }
}

fn render_value(value: &Value) -> String {
    match value {
        Value::Int(value) => format!("Int({value})"),
        Value::Float(value) => format!("Float({value:?})"),
        Value::String(value) => format!("String({value:?})"),
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
    }
}

#[cfg(test)]
mod tests {
    use super::{render_value, render_value_type};
    use crate::plan::{
        FunctionReturnFamily, FunctionType, FunctionValue, IntFunctionId, IntLocalId, ListValue,
        PanicSite, ParamLocal, RuntimeFunctionId, SourceContext, SourceSpan, Value, ValueType,
    };
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
                value: Value::List(crate::plan::ListValue::empty(ValueType::Int)),
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
                ExecutionError::function_return_family_mismatch(
                    FunctionReturnFamily::Int,
                    FunctionReturnFamily::String,
                ),
                "geam::function_return_family_mismatch",
            ),
            (
                ExecutionError::tuple_index_family_mismatch(ValueType::Int, ValueType::String),
                "geam::tuple_index_mismatch",
            ),
            (
                ExecutionError::list_index_out_of_bounds(ValueType::Int, 1, 1),
                "geam::list_index_out_of_bounds",
            ),
            (
                ExecutionError::ListIndexFamilyMismatch {
                    expected: ValueType::List(Box::new(ValueType::Int)),
                    actual: ValueType::List(Box::new(ValueType::String)),
                },
                "geam::list_index_family_mismatch",
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
            vec![ParamLocal::int(IntLocalId(0))],
        ));

        for (value, expected) in [
            (Value::Int(1.into()), "Int(1)"),
            (Value::Float(1.5), "Float(1.5)"),
            (Value::String("one".into()), "String(\"one\")"),
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
    }

    #[test]
    fn render_value_type_preserves_compound_shapes() {
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
    }
}
