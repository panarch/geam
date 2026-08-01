use super::{
    BitArrayFunctionId, BoolFunctionId, CustomFunctionId, ExecutionGraphProfile, FloatFunctionId,
    FunctionLabelSource, HostedExecutionGraph, IntFunctionId, NeverFunctionId, NilFunctionId,
    StringFunctionId, TupleFunctionId, UtfCodepointFunctionId,
};
use crate::plan::execution::explain::FunctionLabel;
use crate::plan::execution::graph::ExternalFunctionCallTarget;
use crate::plan::execution::type_::{CustomConstructorId, FunctionType, ValueShapeId, ValueType};
use std::convert::Infallible;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum GenericCallableId {
    Function {
        template: usize,
        substitution: Box<[ValueShapeId]>,
    },
    Constructor(CustomConstructorId),
}

impl GenericCallableId {
    pub(in crate::plan::execution) fn function(
        template: usize,
        substitution: Vec<ValueShapeId>,
    ) -> Self {
        Self::Function {
            template,
            substitution: substitution.into_boxed_slice(),
        }
    }

    pub(in crate::plan::execution) fn constructor(constructor: CustomConstructorId) -> Self {
        Self::Constructor(constructor)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ProfiledRuntimeFunctionId<Graph: ExecutionGraphProfile> {
    Core(ProfiledCoreRuntimeFunctionId<Graph>),
    External(Graph::ExternalFunctionId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ProfiledCoreRuntimeFunctionId<Graph: ExecutionGraphProfile> {
    Never(NeverFunctionId),
    Int(IntFunctionId),
    Float(FloatFunctionId),
    String(StringFunctionId),
    BitArray(BitArrayFunctionId),
    UtfCodepoint(UtfCodepointFunctionId),
    Custom(CustomFunctionId),
    Bool(BoolFunctionId),
    Nil(NilFunctionId),
    Tuple {
        id: TupleFunctionId,
        return_type: Vec<ValueType>,
    },
    List(super::ProfiledListFunctionId<Graph>),
    Function {
        id: Graph::RuntimeFunctionFunctionId,
        return_type: FunctionType,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RuntimeFunctionFunctionTarget {
    Core(super::ProfiledFunctionFunctionId<Infallible>),
    External(ExternalFunctionCallTarget),
}

pub(crate) type RuntimeFunctionId = ProfiledRuntimeFunctionId<HostedExecutionGraph>;
pub(crate) type CoreRuntimeFunctionId = ProfiledCoreRuntimeFunctionId<HostedExecutionGraph>;

#[cfg(test)]
impl ProfiledRuntimeFunctionId<Infallible> {
    pub(crate) fn runtime_id(&self) -> RuntimeFunctionId {
        match self {
            Self::Core(id) => {
                RuntimeFunctionId::Core(super::profile::plain_core_runtime_function_id(id))
            }
            Self::External(id) => match *id {},
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionReturnFamily {
    Generic,
    Never,
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Custom,
    External,
    Bool,
    Nil,
    Tuple,
    List,
    Function,
}

impl std::fmt::Display for FunctionReturnFamily {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Generic => "Generic",
            Self::Never => "Never",
            Self::Int => "Int",
            Self::Float => "Float",
            Self::String => "String",
            Self::BitArray => "BitArray",
            Self::UtfCodepoint => "UtfCodepoint",
            Self::Custom => "Custom",
            Self::External => "External",
            Self::Bool => "Bool",
            Self::Nil => "Nil",
            Self::Tuple => "Tuple",
            Self::List => "List",
            Self::Function => "Function",
        })
    }
}

impl<Graph: ExecutionGraphProfile> FunctionLabelSource for ProfiledRuntimeFunctionId<Graph>
where
    Graph::ExternalFunctionId: FunctionLabelSource,
    Graph::ExternalListFunctionId: FunctionLabelSource,
    Graph::ExternalFunctionFunctionId: FunctionLabelSource,
    Graph::ExternalListFunctionFunctionId: FunctionLabelSource,
    Graph::RuntimeFunctionFunctionId: FunctionLabelSource,
{
    fn function_label(&self) -> FunctionLabel {
        match self {
            Self::Core(id) => id.function_label(),
            Self::External(id) => id.function_label(),
        }
    }
}

impl<Graph: ExecutionGraphProfile> FunctionLabelSource for ProfiledCoreRuntimeFunctionId<Graph>
where
    Graph::ExternalListFunctionId: FunctionLabelSource,
    Graph::ExternalListFunctionFunctionId: FunctionLabelSource,
    Graph::RuntimeFunctionFunctionId: FunctionLabelSource,
{
    fn function_label(&self) -> FunctionLabel {
        match self {
            Self::Never(id) => FunctionLabel::new("never", id.0),
            Self::Int(id) => FunctionLabel::new("int", id.0),
            Self::Float(id) => FunctionLabel::new("float", id.0),
            Self::String(id) => FunctionLabel::new("string", id.0),
            Self::BitArray(id) => FunctionLabel::new("bit_array", id.0),
            Self::UtfCodepoint(id) => FunctionLabel::new("utf_codepoint", id.0),
            Self::Custom(id) => FunctionLabel::new("custom", id.index()),
            Self::Bool(id) => FunctionLabel::new("bool", id.0),
            Self::Nil(id) => FunctionLabel::new("nil", id.0),
            Self::Tuple { id, .. } => FunctionLabel::new("tuple", id.0),
            Self::List(id) => id.function_label(),
            Self::Function { id, .. } => id.function_label(),
        }
    }
}

impl FunctionLabelSource for RuntimeFunctionFunctionTarget {
    fn function_label(&self) -> FunctionLabel {
        self.runtime_id().function_label()
    }
}

impl RuntimeFunctionFunctionTarget {
    pub(crate) fn runtime_id(&self) -> super::FunctionFunctionId {
        match self {
            Self::Core(function) => Infallible::function_function(function),
            Self::External(function) => function.runtime_id(),
        }
    }
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::explain;
    use crate::plan::execution::function::{
        CoreRuntimeFunctionId, ExternalFunctionFunctionId, ExternalFunctionId, FunctionLabelSource,
        RuntimeFunctionFunctionTarget, RuntimeFunctionId,
    };
    use crate::plan::execution::graph::ExternalFunctionCallTarget;
    use crate::plan::execution::type_::{
        ExternalFunctionType, ExternalTypeId, FunctionType, ValueType,
    };

    #[test]
    fn labels_runtime_function_families() {
        let cases = [
            ("pub fn main() -> value { main() }", "never#0"),
            ("pub fn main() { 1 }", "int#0"),
            ("pub fn main() { 1.0 }", "float#0"),
            ("pub fn main() { \"one\" }", "string#0"),
            ("pub fn main() { <<1>> }", "bit_array#0"),
            (
                "pub fn main() -> UtfCodepoint { let assert <<value:utf8_codepoint>> = <<65>> value }",
                "utf_codepoint#0",
            ),
            (
                "pub type Boxed { Boxed(Int) } pub fn main() { Boxed(1) }",
                "custom#0",
            ),
            ("pub fn main() { True }", "bool#0"),
            ("pub fn main() { Nil }", "nil#0"),
            ("pub fn main() { #(1) }", "tuple#0"),
            ("pub fn main() -> List(Int) { [] }", "list.int#0"),
            (
                "pub fn main() -> fn() -> Int { fn() { 1 } }",
                "function.int#0",
            ),
        ];

        for (source, expected) in cases {
            assert_explanation(source, expected);
        }

        explain::assert_written("external#13", |output| {
            RuntimeFunctionId::External(ExternalFunctionId::new(13, ExternalTypeId::new(0)))
                .function_label()
                .write(output);
        });
        explain::assert_written("function.external#14", |output| {
            let external_type = ExternalTypeId::new(0);
            RuntimeFunctionFunctionTarget::External(ExternalFunctionCallTarget::Function(
                ExternalFunctionFunctionId::new(
                    14,
                    ExternalFunctionType::from_shapes(
                        FunctionType::new(Vec::new(), ValueType::External(external_type)),
                        Vec::new(),
                        external_type,
                    ),
                ),
            ))
            .function_label()
            .write(output);
        });
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            plan.main_runtime().function_label().write(output);
        });
    }

    #[test]
    fn labels_core_runtime_function_ids() {
        explain::assert_written("int#13", |output| {
            CoreRuntimeFunctionId::Int(crate::plan::execution::function::IntFunctionId(13))
                .function_label()
                .write(output);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::FunctionReturnFamily;

    #[test]
    fn display_names_every_family() {
        assert_eq!(
            [
                FunctionReturnFamily::Generic,
                FunctionReturnFamily::Never,
                FunctionReturnFamily::Int,
                FunctionReturnFamily::Float,
                FunctionReturnFamily::String,
                FunctionReturnFamily::BitArray,
                FunctionReturnFamily::UtfCodepoint,
                FunctionReturnFamily::Custom,
                FunctionReturnFamily::External,
                FunctionReturnFamily::Bool,
                FunctionReturnFamily::Nil,
                FunctionReturnFamily::Tuple,
                FunctionReturnFamily::List,
                FunctionReturnFamily::Function,
            ]
            .map(|family| family.to_string()),
            [
                "Generic",
                "Never",
                "Int",
                "Float",
                "String",
                "BitArray",
                "UtfCodepoint",
                "Custom",
                "External",
                "Bool",
                "Nil",
                "Tuple",
                "List",
                "Function",
            ],
        );
    }
}
