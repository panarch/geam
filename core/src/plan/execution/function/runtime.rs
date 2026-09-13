use super::{
    BitArrayFunctionId, BoolFunctionId, CustomFunctionId, ExecutionGraphProfile, FloatFunctionId,
    FunctionLabelSource, HostedExecutionGraph, IntFunctionId, NeverFunctionId, NilFunctionId,
    StringFunctionId, TupleFunctionId, UtfCodepointFunctionId,
};
use crate::plan::execution::explain::FunctionLabel;
use crate::plan::execution::graph::ExternalFunctionCallTarget;
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;
use crate::plan::execution::type_::{CustomConstructorId, FunctionType, ValueShapeId, ValueType};
use std::convert::Infallible;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GenericCallableId {
    Function {
        template: usize,
        substitution: Table<ValueShapeId>,
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
            substitution: substitution.into(),
        }
    }

    pub(in crate::plan::execution) fn constructor(constructor: CustomConstructorId) -> Self {
        Self::Constructor(constructor)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfiledRuntimeFunctionId<Graph: ExecutionGraphProfile> {
    Core(ProfiledCoreRuntimeFunctionId<Graph>),
    External(Graph::ExternalFunctionId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfiledCoreRuntimeFunctionId<Graph: ExecutionGraphProfile> {
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
        return_type: Table<ValueType>,
    },
    List(super::ProfiledListFunctionId<Graph>),
    Function {
        id: Graph::RuntimeFunctionFunctionId,
        return_type: FunctionType,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeFunctionFunctionTarget {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl Emit for GenericCallableId {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Function {
                template,
                substitution,
            } => output.structure(
                "function::GenericCallableId::Function",
                &[("template", template), ("substitution", substitution)],
            ),
            Self::Constructor(field_0) => {
                output.call("function::GenericCallableId::Constructor", &[field_0])
            }
        }
    }
}

impl<Graph: ExecutionGraphProfile> Emit for ProfiledRuntimeFunctionId<Graph>
where
    ProfiledCoreRuntimeFunctionId<Graph>: Emit,
    Graph::ExternalFunctionId: Emit,
{
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Core(field_0) => {
                output.call("function::ProfiledRuntimeFunctionId::Core", &[field_0])
            }
            Self::External(field_0) => {
                output.call("function::ProfiledRuntimeFunctionId::External", &[field_0])
            }
        }
    }
}

impl<Graph: ExecutionGraphProfile> Emit for ProfiledCoreRuntimeFunctionId<Graph>
where
    super::ProfiledListFunctionId<Graph>: Emit,
    Graph::RuntimeFunctionFunctionId: Emit,
{
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Never(field_0) => {
                output.call("function::ProfiledCoreRuntimeFunctionId::Never", &[field_0])
            }
            Self::Int(field_0) => {
                output.call("function::ProfiledCoreRuntimeFunctionId::Int", &[field_0])
            }
            Self::Float(field_0) => {
                output.call("function::ProfiledCoreRuntimeFunctionId::Float", &[field_0])
            }
            Self::String(field_0) => output.call(
                "function::ProfiledCoreRuntimeFunctionId::String",
                &[field_0],
            ),
            Self::BitArray(field_0) => output.call(
                "function::ProfiledCoreRuntimeFunctionId::BitArray",
                &[field_0],
            ),
            Self::UtfCodepoint(field_0) => output.call(
                "function::ProfiledCoreRuntimeFunctionId::UtfCodepoint",
                &[field_0],
            ),
            Self::Custom(field_0) => output.call(
                "function::ProfiledCoreRuntimeFunctionId::Custom",
                &[field_0],
            ),
            Self::Bool(field_0) => {
                output.call("function::ProfiledCoreRuntimeFunctionId::Bool", &[field_0])
            }
            Self::Nil(field_0) => {
                output.call("function::ProfiledCoreRuntimeFunctionId::Nil", &[field_0])
            }
            Self::Tuple { id, return_type } => output.structure(
                "function::ProfiledCoreRuntimeFunctionId::Tuple",
                &[("id", id), ("return_type", return_type)],
            ),
            Self::List(field_0) => {
                output.call("function::ProfiledCoreRuntimeFunctionId::List", &[field_0])
            }
            Self::Function { id, return_type } => output.structure(
                "function::ProfiledCoreRuntimeFunctionId::Function",
                &[("id", id), ("return_type", return_type)],
            ),
        }
    }
}

impl Emit for RuntimeFunctionFunctionTarget {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Core(field_0) => {
                output.call("function::RuntimeFunctionFunctionTarget::Core", &[field_0])
            }
            Self::External(field_0) => output.call(
                "function::RuntimeFunctionFunctionTarget::External",
                &[field_0],
            ),
        }
    }
}

impl Emit for FunctionReturnFamily {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Generic => output.path("function::FunctionReturnFamily::Generic"),
            Self::Never => output.path("function::FunctionReturnFamily::Never"),
            Self::Int => output.path("function::FunctionReturnFamily::Int"),
            Self::Float => output.path("function::FunctionReturnFamily::Float"),
            Self::String => output.path("function::FunctionReturnFamily::String"),
            Self::BitArray => output.path("function::FunctionReturnFamily::BitArray"),
            Self::UtfCodepoint => output.path("function::FunctionReturnFamily::UtfCodepoint"),
            Self::Custom => output.path("function::FunctionReturnFamily::Custom"),
            Self::External => output.path("function::FunctionReturnFamily::External"),
            Self::Bool => output.path("function::FunctionReturnFamily::Bool"),
            Self::Nil => output.path("function::FunctionReturnFamily::Nil"),
            Self::Tuple => output.path("function::FunctionReturnFamily::Tuple"),
            Self::List => output.path("function::FunctionReturnFamily::List"),
            Self::Function => output.path("function::FunctionReturnFamily::Function"),
        }
    }
}

#[cfg(test)]
mod emission_tests {
    use super::{
        FunctionReturnFamily, ProfiledCoreRuntimeFunctionId, ProfiledRuntimeFunctionId,
        RuntimeFunctionFunctionTarget,
    };
    use crate::plan::execution::function::{
        BitArrayFunctionId, BoolFunctionId, CustomFunctionId, ExternalFunctionId, FloatFunctionId,
        HostedExecutionGraph, IntFunctionFunctionId, IntFunctionId, IntListFunctionId,
        ListFunctionId, NeverFunctionId, NilFunctionId, ProfiledFunctionFunctionId,
        ProfiledListFunctionId, StringFunctionId, TupleFunctionId, UtfCodepointFunctionId,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::{
        CustomTypeId, CustomValueShape, CustomValueShapeId, ExternalTypeId, FunctionType,
        IntListTypeId, ListTypeId, ValueType,
    };

    #[test]
    fn emits_runtime_entry_ids_without_erasing_return_contracts() {
        let cases: [(ProfiledCoreRuntimeFunctionId<HostedExecutionGraph>, &str); 12] = [
            (
                ProfiledCoreRuntimeFunctionId::Never(NeverFunctionId(2)),
                "data::function::ProfiledCoreRuntimeFunctionId::Never(data::function::NeverFunctionId(2,),)",
            ),
            (
                ProfiledCoreRuntimeFunctionId::Int(IntFunctionId(2)),
                "data::function::ProfiledCoreRuntimeFunctionId::Int(data::function::IntFunctionId(2,),)",
            ),
            (
                ProfiledCoreRuntimeFunctionId::Float(FloatFunctionId(2)),
                "data::function::ProfiledCoreRuntimeFunctionId::Float(data::function::FloatFunctionId(2,),)",
            ),
            (
                ProfiledCoreRuntimeFunctionId::String(StringFunctionId(2)),
                "data::function::ProfiledCoreRuntimeFunctionId::String(data::function::StringFunctionId(2,),)",
            ),
            (
                ProfiledCoreRuntimeFunctionId::BitArray(BitArrayFunctionId(2)),
                "data::function::ProfiledCoreRuntimeFunctionId::BitArray(data::function::BitArrayFunctionId(2,),)",
            ),
            (
                ProfiledCoreRuntimeFunctionId::UtfCodepoint(UtfCodepointFunctionId(2)),
                "data::function::ProfiledCoreRuntimeFunctionId::UtfCodepoint(data::function::UtfCodepointFunctionId(2,),)",
            ),
            (
                ProfiledCoreRuntimeFunctionId::Bool(BoolFunctionId(2)),
                "data::function::ProfiledCoreRuntimeFunctionId::Bool(data::function::BoolFunctionId(2,),)",
            ),
            (
                ProfiledCoreRuntimeFunctionId::Nil(NilFunctionId(2)),
                "data::function::ProfiledCoreRuntimeFunctionId::Nil(data::function::NilFunctionId(2,),)",
            ),
            (
                ProfiledCoreRuntimeFunctionId::Custom(CustomFunctionId::new(
                    2,
                    CustomValueShape::new(CustomTypeId(3), CustomValueShapeId(4)),
                )),
                "data::function::ProfiledCoreRuntimeFunctionId::Custom(data::function::CustomFunctionId {index: 2,return_shape: data::type_::CustomValueShape {type_id: data::type_::CustomTypeId(3,),shape_id: data::type_::CustomValueShapeId(4,),},},)",
            ),
            (
                ProfiledCoreRuntimeFunctionId::Tuple {
                    id: TupleFunctionId(2),
                    return_type: vec![ValueType::Int].into(),
                },
                "data::function::ProfiledCoreRuntimeFunctionId::Tuple {id: data::function::TupleFunctionId(2,),return_type: data::Storage::Static(&[data::type_::ValueType::Int,]),}",
            ),
            (
                ProfiledCoreRuntimeFunctionId::List(ProfiledListFunctionId::Core(
                    ListFunctionId::Int(IntListFunctionId::new(
                        2,
                        IntListTypeId::new(ListTypeId(3)),
                    )),
                )),
                "data::function::ProfiledCoreRuntimeFunctionId::List(data::function::ProfiledListFunctionId::Core(data::function::ListFunctionId::Int(data::function::IntListFunctionId {index: 2,type_id: data::type_::IntListTypeId {list_type: data::type_::ListTypeId(3,),},},),),)",
            ),
            (
                ProfiledCoreRuntimeFunctionId::Function {
                    id: RuntimeFunctionFunctionTarget::Core(ProfiledFunctionFunctionId::Int(
                        IntFunctionFunctionId(2),
                    )),
                    return_type: FunctionType::new(Vec::new(), ValueType::Int),
                },
                "data::function::ProfiledCoreRuntimeFunctionId::Function {id: data::function::RuntimeFunctionFunctionTarget::Core(data::function::ProfiledFunctionFunctionId::Int(data::function::IntFunctionFunctionId(2,),),),return_type: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::Int),},}",
            ),
        ];
        for (id, expected) in cases {
            assert_eq!(Rust::expression(&id), expected);
        }
        let entries: [(ProfiledRuntimeFunctionId<HostedExecutionGraph>, &str); 2] = [
            (
                ProfiledRuntimeFunctionId::Core(ProfiledCoreRuntimeFunctionId::Int(IntFunctionId(
                    2,
                ))),
                "data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Int(data::function::IntFunctionId(2,),),)",
            ),
            (
                ProfiledRuntimeFunctionId::External(ExternalFunctionId::new(2, ExternalTypeId(3))),
                "data::function::ProfiledRuntimeFunctionId::External(data::function::ExternalFunctionId {index: 2,return_type: data::type_::ExternalTypeId(3,),},)",
            ),
        ];
        for (id, expected) in entries {
            assert_eq!(Rust::expression(&id), expected);
        }
    }

    #[test]
    fn emits_every_function_return_family() {
        let cases = [
            (
                FunctionReturnFamily::Generic,
                "data::function::FunctionReturnFamily::Generic",
            ),
            (
                FunctionReturnFamily::Never,
                "data::function::FunctionReturnFamily::Never",
            ),
            (
                FunctionReturnFamily::Int,
                "data::function::FunctionReturnFamily::Int",
            ),
            (
                FunctionReturnFamily::Float,
                "data::function::FunctionReturnFamily::Float",
            ),
            (
                FunctionReturnFamily::String,
                "data::function::FunctionReturnFamily::String",
            ),
            (
                FunctionReturnFamily::BitArray,
                "data::function::FunctionReturnFamily::BitArray",
            ),
            (
                FunctionReturnFamily::UtfCodepoint,
                "data::function::FunctionReturnFamily::UtfCodepoint",
            ),
            (
                FunctionReturnFamily::Custom,
                "data::function::FunctionReturnFamily::Custom",
            ),
            (
                FunctionReturnFamily::External,
                "data::function::FunctionReturnFamily::External",
            ),
            (
                FunctionReturnFamily::Bool,
                "data::function::FunctionReturnFamily::Bool",
            ),
            (
                FunctionReturnFamily::Nil,
                "data::function::FunctionReturnFamily::Nil",
            ),
            (
                FunctionReturnFamily::Tuple,
                "data::function::FunctionReturnFamily::Tuple",
            ),
            (
                FunctionReturnFamily::List,
                "data::function::FunctionReturnFamily::List",
            ),
            (
                FunctionReturnFamily::Function,
                "data::function::FunctionReturnFamily::Function",
            ),
        ];
        for (family, expected) in cases {
            assert_eq!(Rust::expression(&family), expected);
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
