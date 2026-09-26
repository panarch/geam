use super::construction::ConstructionIndex;
use super::{NativeConversions, RegistrationContract};
use crate::host::{HostNeverFunction, HostTypeDescriptor, HostValueFunction};
use crate::plan::execution::function::{
    ExecutionFunctionBody, FunctionBodyOwner, FunctionTableFamily, RuntimeFunctionId,
};
use crate::plan::execution::graph::{
    BitArrayLocalId, BoolLocalId, FloatLocalId, IntLocalId, NilLocalId, ParamLocal, ParamSlot,
    StringLocalId, UtfCodepointLocalId,
};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::{Node, Table};
use crate::plan::execution::type_::{
    CustomTypeId, ExternalTypeId, FunctionMetadata, FunctionType, ListTypeId, TypeMetadata,
    ValueShapeId,
};
use crate::plan::{self, HostCallSite, Text, ValueType};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

pub(crate) struct HostedFunction<Implementation> {
    metadata: Arc<HostedFunctionMetadata>,
    implementation: Implementation,
}

pub enum HostedFunctionTarget<Body: FunctionBodyOwner> {
    Value(HostFunctionId<Body>),
    Never(HostNeverFunctionId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HostCallableEntry {
    pub family: FunctionTableFamily,
    pub index: usize,
}

pub struct HostedFunctionMetadata {
    pub completion: HostFunctionCompletion,
    pub callable_entry: Option<HostCallableEntry>,
    pub package: Text,
    pub site: HostCallSite,
    pub signature: FunctionMetadata,
    pub type_arguments: Table<HostTypeArgument>,
    pub parameters: HostedFunctionParameters,
    pub constructions: HostConstructionTypes,
    pub type_: FunctionType,
    pub registration: Node<RegistrationContract>,
}

/// The declared completion and the executable result chosen at specialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostFunctionCompletion {
    /// A value-returning registration with inhabited result storage.
    Value,
    /// An explicitly non-returning registration.
    Never,
    /// A value-returning registration specialized to a failure-only target.
    Uninhabited,
}

impl HostFunctionCompletion {
    pub(in crate::plan::execution) fn declared_value(self) -> bool {
        match self {
            Self::Value | Self::Uninhabited => true,
            Self::Never => false,
        }
    }
}

pub struct HostTypeArgument {
    pub type_: TypeMetadata,
    pub shape: ValueShapeId,
}

#[derive(Clone)]
pub struct HostConstructionTypes {
    pub lists: ConstructionIndex<ListTypeId>,
    pub customs: ConstructionIndex<CustomTypeId>,
    pub externals: ConstructionIndex<ExternalTypeId>,
    pub natives: NativeConversions,
    pub callables: Table<HostCallableConstruction>,
}

#[derive(Clone)]
pub struct HostCallableConstruction {
    pub target: RuntimeFunctionId,
    pub type_: FunctionType,
    pub parameters: Table<ParamSlot>,
    pub captures: Table<ParamSlot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostCallParameter {
    Int(IntLocalId),
    Float(FloatLocalId),
    String(StringLocalId),
    BitArray(BitArrayLocalId),
    UtfCodepoint(UtfCodepointLocalId),
    Bool(BoolLocalId),
    Nil(NilLocalId),
    Value(ParamLocal),
    List(ParamLocal),
    Tuple(ParamLocal),
    Custom(ParamLocal),
    External(ParamLocal),
    Function { local: ParamLocal, arity: usize },
}

pub struct HostedFunctionParameters {
    pub call: Table<HostCallParameter>,
    pub captures: Table<ParamSlot>,
}

#[derive(Debug)]
pub struct HostFunctionId<Body: FunctionBodyOwner> {
    pub index: usize,
    pub return_: Body::Return,
    pub body: PhantomData<fn() -> Body>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HostNeverFunctionId(pub usize);

pub(crate) type HostedValueFunction<Profile> = HostedFunction<HostValueFunction<Profile>>;
pub(crate) type HostedNeverFunction<Profile> = HostedFunction<HostNeverFunction<Profile>>;

impl<Body> Clone for HostFunctionId<Body>
where
    Body: FunctionBodyOwner,
    Body::Return: Clone,
{
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            return_: self.return_.clone(),
            body: PhantomData,
        }
    }
}

impl<Body> Copy for HostFunctionId<Body>
where
    Body: FunctionBodyOwner,
    Body::Return: Copy,
{
}

impl<Body> PartialEq for HostFunctionId<Body>
where
    Body: FunctionBodyOwner,
    Body::Return: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.return_ == other.return_
    }
}

impl<Body> Eq for HostFunctionId<Body>
where
    Body: FunctionBodyOwner,
    Body::Return: Eq,
{
}

impl<Body> Clone for HostedFunctionTarget<Body>
where
    Body: FunctionBodyOwner,
    HostFunctionId<Body>: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Value(target) => Self::Value(target.clone()),
            Self::Never(target) => Self::Never(*target),
        }
    }
}

impl<Body> Copy for HostedFunctionTarget<Body>
where
    Body: FunctionBodyOwner,
    HostFunctionId<Body>: Copy,
{
}

impl<Body> PartialEq for HostedFunctionTarget<Body>
where
    Body: FunctionBodyOwner,
    HostFunctionId<Body>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(left), Self::Value(right)) => left == right,
            (Self::Never(left), Self::Never(right)) => left == right,
            (Self::Value(_), Self::Never(_)) | (Self::Never(_), Self::Value(_)) => false,
        }
    }
}

impl<Body> Eq for HostedFunctionTarget<Body>
where
    Body: FunctionBodyOwner,
    HostFunctionId<Body>: Eq,
{
}

impl<Body: ExecutionFunctionBody> HostFunctionId<Body> {
    pub(in crate::plan::execution) fn new(index: usize, return_: Body::Return) -> Self {
        Self {
            index,
            return_,
            body: PhantomData,
        }
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }

    pub(crate) fn return_(&self) -> &Body::Return {
        &self.return_
    }
}

impl HostNeverFunctionId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl<Body: ExecutionFunctionBody> HostedFunctionTarget<Body> {
    pub(in crate::plan::execution) fn value(target: HostFunctionId<Body>) -> Self {
        Self::Value(target)
    }

    pub(in crate::plan::execution) fn never(target: HostNeverFunctionId) -> Self {
        Self::Never(target)
    }
}

impl<Implementation> HostedFunction<Implementation> {
    pub(in crate::plan::execution) fn new(
        metadata: impl Into<Arc<HostedFunctionMetadata>>,
        implementation: Implementation,
    ) -> Self {
        Self {
            metadata: metadata.into(),
            implementation,
        }
    }

    pub(crate) fn package(&self) -> &str {
        self.metadata.package()
    }

    pub(crate) fn module(&self) -> &str {
        self.metadata.module()
    }

    pub(crate) fn name(&self) -> &str {
        self.metadata.name()
    }

    pub(crate) fn call_parameters(&self) -> &[HostCallParameter] {
        self.metadata.call_parameters()
    }

    pub(crate) fn type_(&self) -> &FunctionType {
        self.metadata.type_()
    }

    pub(crate) fn capture_parameters(&self) -> &[ParamSlot] {
        &self.metadata.parameters.captures
    }

    pub(crate) fn metadata(&self) -> &HostedFunctionMetadata {
        &self.metadata
    }

    pub(crate) fn metadata_handle(&self) -> &Arc<HostedFunctionMetadata> {
        &self.metadata
    }

    pub(crate) fn implementation(&self) -> &Implementation {
        &self.implementation
    }

    pub(in crate::plan::execution) fn into_metadata(self) -> Arc<HostedFunctionMetadata> {
        self.metadata
    }
}

impl HostedFunctionMetadata {
    pub(in crate::plan::execution) fn borrowed(&'static self) -> Self {
        Self {
            completion: self.completion,
            callable_entry: self.callable_entry,
            package: Text::Static(self.package()),
            site: HostCallSite::from_static(self.module(), self.name(), self.site.span()),
            signature: FunctionMetadata {
                arguments: Table::Static(&self.signature.arguments),
                return_: Node::Static(&self.signature.return_),
            },
            type_arguments: Table::Static(&self.type_arguments),
            parameters: HostedFunctionParameters {
                call: Table::Static(&self.parameters.call),
                captures: Table::Static(&self.parameters.captures),
            },
            constructions: HostConstructionTypes {
                callables: Table::Static(&self.constructions.callables),
                lists: ConstructionIndex {
                    entries: Table::Static(&self.constructions.lists.entries),
                },
                customs: ConstructionIndex {
                    entries: Table::Static(&self.constructions.customs.entries),
                },
                externals: ConstructionIndex {
                    entries: Table::Static(&self.constructions.externals.entries),
                },
                natives: NativeConversions {
                    roots: Table::Static(&self.constructions.natives.roots),
                    nodes: Table::Static(&self.constructions.natives.nodes),
                },
            },
            type_: FunctionType {
                arguments: Table::Static(&self.type_.arguments),
                return_: Node::Static(&self.type_.return_),
            },
            registration: Node::Static(&self.registration),
        }
    }

    pub(crate) fn package(&self) -> &str {
        &self.package
    }

    pub(crate) fn module(&self) -> &str {
        self.site.module()
    }

    pub(crate) fn name(&self) -> &str {
        self.site.function()
    }

    pub(crate) fn site(&self) -> &HostCallSite {
        &self.site
    }

    pub(crate) fn signature(&self) -> plan::FunctionType {
        self.signature.materialize()
    }

    pub(crate) fn resolve_type(&self, descriptor: &HostTypeDescriptor) -> Option<ValueType> {
        descriptor.resolve(&|index| {
            self.type_arguments
                .get(index)
                .map(|argument| argument.type_.materialize())
        })
    }

    pub(crate) fn resolve_construction(&self, descriptor: &HostTypeDescriptor) -> ValueType {
        descriptor.resolve_sealed(&|index| self.type_arguments[index].type_.materialize())
    }

    pub(crate) fn call_parameters(&self) -> &[HostCallParameter] {
        self.parameters.call()
    }

    pub(crate) fn constructions(&self) -> &HostConstructionTypes {
        &self.constructions
    }

    fn type_(&self) -> &FunctionType {
        &self.type_
    }
}

impl HostConstructionTypes {
    pub(in crate::plan::execution) fn new(
        lists: HashMap<ValueType, ListTypeId>,
        customs: HashMap<ValueType, CustomTypeId>,
        externals: HashMap<ValueType, ExternalTypeId>,
    ) -> Self {
        Self {
            lists: ConstructionIndex::new(lists),
            customs: ConstructionIndex::new(customs),
            externals: ConstructionIndex::new(externals),
            natives: NativeConversions::default(),
            callables: Vec::new().into(),
        }
    }

    pub(crate) fn list(&self, type_: &ValueType) -> ListTypeId {
        self.lists.get(type_)
    }

    pub(in crate::plan::execution) fn with_natives(mut self, natives: NativeConversions) -> Self {
        self.natives = natives;
        self
    }

    pub(crate) fn natives(&self) -> &NativeConversions {
        &self.natives
    }

    pub(crate) fn custom(&self, type_: &ValueType) -> CustomTypeId {
        self.customs.get(type_)
    }

    pub(crate) fn external(&self, type_: &ValueType) -> ExternalTypeId {
        self.externals.get(type_)
    }
}

impl HostedFunctionParameters {
    fn call(&self) -> &[HostCallParameter] {
        &self.call
    }
}

impl HostCallParameter {
    pub(crate) fn local(&self) -> ParamLocal {
        match self {
            Self::Int(local) => ParamLocal::Int(*local),
            Self::Float(local) => ParamLocal::Float(*local),
            Self::String(local) => ParamLocal::String(*local),
            Self::BitArray(local) => ParamLocal::BitArray(*local),
            Self::UtfCodepoint(local) => ParamLocal::UtfCodepoint(*local),
            Self::Bool(local) => ParamLocal::Bool(*local),
            Self::Nil(local) => ParamLocal::Nil(*local),
            Self::Value(local) => local.clone(),
            Self::List(local) => local.clone(),
            Self::Tuple(local) => local.clone(),
            Self::Custom(local) => local.clone(),
            Self::External(local) => local.clone(),
            Self::Function { local, .. } => local.clone(),
        }
    }
}

impl<Body: FunctionBodyOwner> Emit for HostedFunctionTarget<Body>
where
    HostFunctionId<Body>: Emit,
{
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Value(field_0) => output.call("host::HostedFunctionTarget::Value", &[field_0]),
            Self::Never(field_0) => output.call("host::HostedFunctionTarget::Never", &[field_0]),
        }
    }
}

impl Emit for HostedFunctionMetadata {
    fn emit(&self, output: &mut Rust) {
        let Self {
            completion,
            callable_entry,
            package,
            site,
            signature,
            type_arguments,
            parameters,
            constructions,
            type_,
            registration,
        } = self;
        output.structure(
            "host::HostedFunctionMetadata",
            &[
                ("completion", completion),
                ("callable_entry", callable_entry),
                ("package", package),
                ("site", site),
                ("signature", signature),
                ("type_arguments", type_arguments),
                ("parameters", parameters),
                ("constructions", constructions),
                ("type_", type_),
                ("registration", registration),
            ],
        );
    }
}

impl Emit for HostFunctionCompletion {
    fn emit(&self, output: &mut Rust) {
        output.path(match self {
            Self::Value => "host::HostFunctionCompletion::Value",
            Self::Never => "host::HostFunctionCompletion::Never",
            Self::Uninhabited => "host::HostFunctionCompletion::Uninhabited",
        });
    }
}

impl Emit for HostTypeArgument {
    fn emit(&self, output: &mut Rust) {
        let Self { type_, shape } = self;
        output.structure(
            "host::HostTypeArgument",
            &[("type_", type_), ("shape", shape)],
        );
    }
}

impl Emit for HostConstructionTypes {
    fn emit(&self, output: &mut Rust) {
        let Self {
            lists,
            customs,
            externals,
            natives,
            callables,
        } = self;
        output.structure(
            "host::HostConstructionTypes",
            &[
                ("lists", lists),
                ("customs", customs),
                ("externals", externals),
                ("natives", natives),
                ("callables", callables),
            ],
        );
    }
}

impl Emit for HostCallableEntry {
    fn emit(&self, output: &mut Rust) {
        let Self { family, index } = self;
        output.structure(
            "host::HostCallableEntry",
            &[("family", family), ("index", index)],
        );
    }
}

impl Emit for HostCallableConstruction {
    fn emit(&self, output: &mut Rust) {
        let Self {
            target,
            type_,
            parameters,
            captures,
        } = self;
        output.structure(
            "host::HostCallableConstruction",
            &[
                ("target", target),
                ("type_", type_),
                ("parameters", parameters),
                ("captures", captures),
            ],
        );
    }
}

impl Emit for HostCallParameter {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Int(field_0) => output.call("host::HostCallParameter::Int", &[field_0]),
            Self::Float(field_0) => output.call("host::HostCallParameter::Float", &[field_0]),
            Self::String(field_0) => output.call("host::HostCallParameter::String", &[field_0]),
            Self::BitArray(field_0) => output.call("host::HostCallParameter::BitArray", &[field_0]),
            Self::UtfCodepoint(field_0) => {
                output.call("host::HostCallParameter::UtfCodepoint", &[field_0])
            }
            Self::Bool(field_0) => output.call("host::HostCallParameter::Bool", &[field_0]),
            Self::Nil(field_0) => output.call("host::HostCallParameter::Nil", &[field_0]),
            Self::Value(field_0) => output.call("host::HostCallParameter::Value", &[field_0]),
            Self::List(field_0) => output.call("host::HostCallParameter::List", &[field_0]),
            Self::Tuple(field_0) => output.call("host::HostCallParameter::Tuple", &[field_0]),
            Self::Custom(field_0) => output.call("host::HostCallParameter::Custom", &[field_0]),
            Self::External(field_0) => output.call("host::HostCallParameter::External", &[field_0]),
            Self::Function { local, arity } => output.structure(
                "host::HostCallParameter::Function",
                &[("local", local), ("arity", arity)],
            ),
        }
    }
}

impl Emit for HostedFunctionParameters {
    fn emit(&self, output: &mut Rust) {
        let Self { call, captures } = self;
        output.structure(
            "host::HostedFunctionParameters",
            &[("call", call), ("captures", captures)],
        );
    }
}

impl<Body: FunctionBodyOwner> Emit for HostFunctionId<Body>
where
    Body::Return: Emit,
{
    fn emit(&self, output: &mut Rust) {
        let Self {
            index,
            return_,
            body,
        } = self;
        output.structure(
            "host::HostFunctionId",
            &[("index", index), ("return_", return_), ("body", body)],
        );
    }
}

impl Emit for HostNeverFunctionId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("host::HostNeverFunctionId", &[field_0]);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        HostCallParameter, HostFunctionCompletion, HostFunctionId, HostNeverFunctionId,
        HostTypeArgument, HostedFunctionTarget,
    };
    use crate::plan::execution::function::GenericFunctionFunctionBody;
    use crate::plan::execution::graph::{
        BitArrayLocalId, BoolLocalId, CustomLocal, CustomLocalId, FloatLocalId,
        GenericFunctionLocal, GenericFunctionLocalId, IntListLocalId, IntLocalId, ListLocal,
        NilLocalId, ParamLocal, ParamSlot, StringLocalId, TupleLocalId, UtfCodepointLocalId,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::{
        CustomTypeId, CustomValueShape, CustomValueShapeId, FunctionShape, FunctionType,
        GenericFunctionType, IntListTypeId, ListTypeId, TypeMetadata, ValueShapeId, ValueType,
    };

    #[test]
    fn emitted_native_targets_keep_argument_and_capture_slots_separate() {
        use super::{HostCallableConstruction, HostCallableEntry};
        use crate::plan::execution::function::{
            CoreRuntimeFunctionId, FunctionTableFamily, IntFunctionId, RuntimeFunctionId,
        };

        let entry = HostCallableEntry {
            family: FunctionTableFamily::Int,
            index: 4,
        };
        assert_eq!(
            Rust::expression(&entry),
            r#"
data::host::HostCallableEntry {
    family: data::function::FunctionTableFamily::Int,
    index: 4,
}"#
            .trim_start_matches('\n')
        );
        let construction = HostCallableConstruction {
            target: RuntimeFunctionId::Core(CoreRuntimeFunctionId::Int(IntFunctionId(4))),
            type_: FunctionType::new(vec![ValueType::Int], ValueType::Int),
            parameters: vec![ParamSlot::new(
                ParamLocal::Int(IntLocalId(0)),
                ValueShapeId(0),
            )]
            .into(),
            captures: vec![ParamSlot::new(
                ParamLocal::Bool(BoolLocalId(0)),
                ValueShapeId(1),
            )]
            .into(),
        };
        assert_eq!(Rust::expression(&construction), r#"
data::host::HostCallableConstruction {
    target: data::function::ProfiledRuntimeFunctionId::Core(data::function::ProfiledCoreRuntimeFunctionId::Int(data::function::IntFunctionId(4))),
    type_: data::type_::FunctionType {
        arguments: data::Storage::Static(&[
            data::type_::ValueType::Int,
        ]),
        return_: data::Storage::Static(&data::type_::ValueType::Int),
    },
    parameters: data::Storage::Static(&[
        data::graph::ParamSlot {
            local: data::graph::ParamLocal::Int(data::graph::IntLocalId(0)),
            shape: data::type_::ValueShapeId(0),
        },
    ]),
    captures: data::Storage::Static(&[
        data::graph::ParamSlot {
            local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
            shape: data::type_::ValueShapeId(1),
        },
    ]),
}"#.trim_start_matches('\n'));
    }

    #[test]
    fn completion_emission_preserves_declared_and_specialized_returns() {
        for (completion, declared_value, expected) in [
            (
                HostFunctionCompletion::Value,
                true,
                "data::host::HostFunctionCompletion::Value",
            ),
            (
                HostFunctionCompletion::Never,
                false,
                "data::host::HostFunctionCompletion::Never",
            ),
            (
                HostFunctionCompletion::Uninhabited,
                true,
                "data::host::HostFunctionCompletion::Uninhabited",
            ),
        ] {
            assert_eq!(completion.declared_value(), declared_value);
            assert_eq!(Rust::expression(&completion), expected);
        }
    }

    #[test]
    fn emitted_native_metadata_preserves_the_body_entry_in_borrowed_artifacts() {
        use super::{
            HostCallableEntry, HostConstructionTypes, HostedFunctionMetadata,
            HostedFunctionParameters,
        };
        use crate::plan::execution::function::FunctionTableFamily;
        use crate::plan::execution::host::NativeConversions;
        use crate::plan::execution::host::construction::ConstructionIndex;
        use crate::plan::execution::host::registration::{RegistrationContract, RegistrationType};
        use crate::plan::execution::storage::{Node, Table};
        use crate::plan::execution::type_::FunctionMetadata;
        use crate::plan::{HostCallSite, SourceSpan, Text};

        static METADATA: HostedFunctionMetadata = HostedFunctionMetadata {
            completion: HostFunctionCompletion::Value,
            callable_entry: Some(HostCallableEntry {
                family: FunctionTableFamily::Int,
                index: 4,
            }),
            package: Text::Static("example"),
            site: HostCallSite::from_static("callbacks", "answer", SourceSpan::new(0, 0)),
            signature: FunctionMetadata {
                arguments: Table::Static(&[]),
                return_: Node::Static(&TypeMetadata::Int),
            },
            type_arguments: Table::Static(&[]),
            parameters: HostedFunctionParameters {
                call: Table::Static(&[]),
                captures: Table::Static(&[ParamSlot {
                    local: ParamLocal::Bool(BoolLocalId(0)),
                    shape: ValueShapeId(1),
                }]),
            },
            constructions: HostConstructionTypes {
                lists: ConstructionIndex {
                    entries: Table::Static(&[]),
                },
                customs: ConstructionIndex {
                    entries: Table::Static(&[]),
                },
                externals: ConstructionIndex {
                    entries: Table::Static(&[]),
                },
                natives: NativeConversions {
                    roots: Table::Static(&[]),
                    nodes: Table::Static(&[]),
                },
                callables: Table::Static(&[]),
            },
            type_: FunctionType {
                arguments: Table::Static(&[]),
                return_: Node::Static(&ValueType::Int),
            },
            registration: Node::Static(&RegistrationContract {
                parameter_count: 0,
                parameters: Table::Static(&[]),
                captures: Table::Static(&[RegistrationType::Bool]),
                callable: true,
                callable_constructions: Table::Static(&[]),
                return_: RegistrationType::Int,
                layout: Table::Static(&[]),
                custom_schemas: Table::Static(&[]),
                external_schemas: Table::Static(&[]),
                constructions: Table::Static(&[]),
                construction_customs: Table::Static(&[]),
                construction_externals: Table::Static(&[]),
                native_rules: None,
            }),
        };
        let expected = r#"
data::host::HostedFunctionMetadata {
    completion: data::host::HostFunctionCompletion::Value,
    callable_entry: Some(data::host::HostCallableEntry {
        family: data::function::FunctionTableFamily::Int,
        index: 4,
    }),
    package: data::Text::Static("example"),
    site: data::source::HostCallSite::from_static("callbacks", "answer", data::source::SourceSpan::new(0, 0)),
    signature: data::type_::FunctionMetadata {
        arguments: data::Storage::Static(&[]),
        return_: data::Storage::Static(&data::type_::TypeMetadata::Int),
    },
    type_arguments: data::Storage::Static(&[]),
    parameters: data::host::HostedFunctionParameters {
        call: data::Storage::Static(&[]),
        captures: data::Storage::Static(&[
            data::graph::ParamSlot {
                local: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
                shape: data::type_::ValueShapeId(1),
            },
        ]),
    },
    constructions: data::host::HostConstructionTypes {
        lists: data::host::ConstructionIndex {
            entries: data::Storage::Static(&[]),
        },
        customs: data::host::ConstructionIndex {
            entries: data::Storage::Static(&[]),
        },
        externals: data::host::ConstructionIndex {
            entries: data::Storage::Static(&[]),
        },
        natives: data::host::NativeConversions {
            roots: data::Storage::Static(&[]),
            nodes: data::Storage::Static(&[]),
        },
        callables: data::Storage::Static(&[]),
    },
    type_: data::type_::FunctionType {
        arguments: data::Storage::Static(&[]),
        return_: data::Storage::Static(&data::type_::ValueType::Int),
    },
    registration: data::Storage::Static(&data::host::RegistrationContract {
        parameter_count: 0,
        parameters: data::Storage::Static(&[]),
        captures: data::Storage::Static(&[
            data::host::RegistrationType::Bool,
        ]),
        callable: true,
        callable_constructions: data::Storage::Static(&[]),
        return_: data::host::RegistrationType::Int,
        layout: data::Storage::Static(&[]),
        custom_schemas: data::Storage::Static(&[]),
        external_schemas: data::Storage::Static(&[]),
        constructions: data::Storage::Static(&[]),
        construction_customs: data::Storage::Static(&[]),
        construction_externals: data::Storage::Static(&[]),
        native_rules: None,
    }),
}"#.trim_start_matches('\n');
        assert_eq!(Rust::expression(&METADATA), expected);
        assert_eq!(Rust::expression(&METADATA.borrowed()), expected);
    }

    #[test]
    fn emitted_host_type_argument_keeps_nominal_and_specialized_shape_together() {
        assert_eq!(
            Rust::expression(&HostTypeArgument {
                type_: TypeMetadata::Int,
                shape: ValueShapeId(7),
            }),
            r#"
data::host::HostTypeArgument {
    type_: data::type_::TypeMetadata::Int,
    shape: data::type_::ValueShapeId(7),
}"#
            .trim_start_matches('\n')
        );
    }

    #[test]
    fn emitted_host_parameters_preserve_every_abi_family() {
        use crate::plan::execution::graph::{ExternalLocal, ExternalLocalId, IntFunctionLocalId};
        use crate::plan::execution::type_::ExternalTypeId;

        let cases = [
            (
                HostCallParameter::Int(IntLocalId(1)),
                "data::host::HostCallParameter::Int(data::graph::IntLocalId(1))",
            ),
            (
                HostCallParameter::Float(FloatLocalId(2)),
                "data::host::HostCallParameter::Float(data::graph::FloatLocalId(2))",
            ),
            (
                HostCallParameter::String(StringLocalId(3)),
                "data::host::HostCallParameter::String(data::graph::StringLocalId(3))",
            ),
            (
                HostCallParameter::BitArray(BitArrayLocalId(4)),
                "data::host::HostCallParameter::BitArray(data::graph::BitArrayLocalId(4))",
            ),
            (
                HostCallParameter::UtfCodepoint(UtfCodepointLocalId(5)),
                "data::host::HostCallParameter::UtfCodepoint(data::graph::UtfCodepointLocalId(5))",
            ),
            (
                HostCallParameter::Bool(BoolLocalId(6)),
                "data::host::HostCallParameter::Bool(data::graph::BoolLocalId(6))",
            ),
            (
                HostCallParameter::Nil(NilLocalId(7)),
                "data::host::HostCallParameter::Nil(data::graph::NilLocalId(7))",
            ),
            (
                HostCallParameter::Value(ParamLocal::Int(IntLocalId(8))),
                "data::host::HostCallParameter::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(8)))",
            ),
            (
                HostCallParameter::List(ParamLocal::List(ListLocal::Int {
                    local: IntListLocalId(9),
                    type_id: IntListTypeId::new(ListTypeId(2)),
                })),
                r#"
data::host::HostCallParameter::List(data::graph::ParamLocal::List(data::graph::ListLocal::Int {
    local: data::graph::IntListLocalId(9),
    type_id: data::type_::IntListTypeId {
        list_type: data::type_::ListTypeId(2),
    },
}))"#.trim_start_matches('\n'),
            ),
            (
                HostCallParameter::Tuple(ParamLocal::Tuple {
                    local: TupleLocalId(10),
                    type_: vec![ValueType::Int].into(),
                }),
                r#"
data::host::HostCallParameter::Tuple(data::graph::ParamLocal::Tuple {
    local: data::graph::TupleLocalId(10),
    type_: data::Storage::Static(&[
        data::type_::ValueType::Int,
    ]),
})"#.trim_start_matches('\n'),
            ),
            (
                HostCallParameter::Custom(ParamLocal::Custom(CustomLocal {
                    id: CustomLocalId(11),
                    shape: CustomValueShape {
                        type_id: CustomTypeId(3),
                        shape_id: CustomValueShapeId(4),
                    },
                })),
                r#"
data::host::HostCallParameter::Custom(data::graph::ParamLocal::Custom(data::graph::CustomLocal {
    id: data::graph::CustomLocalId(11),
    shape: data::type_::CustomValueShape {
        type_id: data::type_::CustomTypeId(3),
        shape_id: data::type_::CustomValueShapeId(4),
    },
}))"#.trim_start_matches('\n'),
            ),
            (
                HostCallParameter::External(ParamLocal::External(ExternalLocal {
                    id: ExternalLocalId(12),
                    type_id: ExternalTypeId(5),
                })),
                r#"
data::host::HostCallParameter::External(data::graph::ParamLocal::External(data::graph::ExternalLocal {
    id: data::graph::ExternalLocalId(12),
    type_id: data::type_::ExternalTypeId(5),
}))"#.trim_start_matches('\n'),
            ),
            (
                HostCallParameter::Function {
                    local: ParamLocal::IntFunction {
                        local: IntFunctionLocalId(13),
                        type_: FunctionType::new(vec![ValueType::Int], ValueType::Int),
                    },
                    arity: 1,
                },
                r#"
data::host::HostCallParameter::Function {
    local: data::graph::ParamLocal::IntFunction {
        local: data::graph::IntFunctionLocalId(13),
        type_: data::type_::FunctionType {
            arguments: data::Storage::Static(&[
                data::type_::ValueType::Int,
            ]),
            return_: data::Storage::Static(&data::type_::ValueType::Int),
        },
    },
    arity: 1,
}"#.trim_start_matches('\n'),
            ),
        ];
        for (parameter, expected) in cases {
            assert_eq!(Rust::expression(&parameter), expected);
        }
    }

    #[test]
    fn emitted_host_targets_distinguish_value_and_never_slots() {
        use crate::plan::execution::function::IntFunctionBody;

        let value =
            HostedFunctionTarget::<IntFunctionBody>::value(HostFunctionId::new(3, IntLocalId(2)));
        let never = HostedFunctionTarget::<IntFunctionBody>::never(HostNeverFunctionId(4));
        assert_eq!(
            Rust::expression(&value),
            r#"
data::host::HostedFunctionTarget::Value(data::host::HostFunctionId {
    index: 3,
    return_: data::graph::IntLocalId(2),
    body: ::core::marker::PhantomData,
})"#
            .trim_start_matches('\n')
        );
        assert_eq!(
            Rust::expression(&never),
            "data::host::HostedFunctionTarget::Never(data::host::HostNeverFunctionId(4))"
        );
    }

    #[test]
    fn host_function_ids_clone_and_compare_the_exact_return_local() {
        let first =
            HostFunctionId::<GenericFunctionFunctionBody>::new(3, generic_function_local(5));
        let same = Clone::clone(&first);
        let other_return =
            HostFunctionId::<GenericFunctionFunctionBody>::new(3, generic_function_local(6));
        let other_index =
            HostFunctionId::<GenericFunctionFunctionBody>::new(4, generic_function_local(5));

        assert!(first == same);
        assert!(first != other_return);
        assert!(first != other_index);
    }

    #[test]
    fn hosted_function_targets_preserve_value_and_never_identity() {
        let value = HostedFunctionTarget::<GenericFunctionFunctionBody>::value(
            HostFunctionId::new(2, generic_function_local(7)),
        );
        let same_value = Clone::clone(&value);
        let never =
            HostedFunctionTarget::<GenericFunctionFunctionBody>::never(HostNeverFunctionId::new(2));
        let same_never = Clone::clone(&never);

        assert!(value == same_value);
        assert!(never == same_never);
        assert!(value != never);
        assert!(never != value);
    }

    #[test]
    fn host_call_parameters_expose_their_exact_typed_local() {
        let custom = ParamLocal::Custom(CustomLocal::new(
            CustomLocalId(8),
            CustomValueShape::new(CustomTypeId::new(0), CustomValueShapeId::new(1)),
        ));
        let list = ParamLocal::List(ListLocal::Int {
            local: IntListLocalId(9),
            type_id: IntListTypeId::new(ListTypeId::new(2)),
        });
        let value_tuple = ParamLocal::Tuple {
            local: TupleLocalId(7),
            type_: vec![ValueType::Int].into(),
        };
        let tuple = ParamLocal::Tuple {
            local: TupleLocalId(10),
            type_: vec![ValueType::Bool].into(),
        };
        let function = ParamLocal::GenericFunction(generic_function_local(11));
        let cases = [
            (
                HostCallParameter::Int(IntLocalId(0)),
                ParamLocal::Int(IntLocalId(0)),
            ),
            (
                HostCallParameter::Float(FloatLocalId(1)),
                ParamLocal::Float(FloatLocalId(1)),
            ),
            (
                HostCallParameter::String(StringLocalId(2)),
                ParamLocal::String(StringLocalId(2)),
            ),
            (
                HostCallParameter::BitArray(BitArrayLocalId(3)),
                ParamLocal::BitArray(BitArrayLocalId(3)),
            ),
            (
                HostCallParameter::UtfCodepoint(UtfCodepointLocalId(4)),
                ParamLocal::UtfCodepoint(UtfCodepointLocalId(4)),
            ),
            (
                HostCallParameter::Bool(BoolLocalId(5)),
                ParamLocal::Bool(BoolLocalId(5)),
            ),
            (
                HostCallParameter::Nil(NilLocalId(6)),
                ParamLocal::Nil(NilLocalId(6)),
            ),
            (HostCallParameter::Value(value_tuple.clone()), value_tuple),
            (HostCallParameter::List(list.clone()), list),
            (HostCallParameter::Tuple(tuple.clone()), tuple),
            (HostCallParameter::Custom(custom.clone()), custom),
            (
                HostCallParameter::Function {
                    local: function.clone(),
                    arity: 3,
                },
                function,
            ),
        ];

        for (parameter, expected) in cases {
            assert_eq!(parameter.local(), expected);
        }
    }

    fn generic_function_local(index: usize) -> GenericFunctionLocal {
        let type_ = FunctionType::new(Vec::new(), ValueType::Int);
        let shape = FunctionShape::new(ValueShapeId::new(0), type_.clone());
        GenericFunctionLocal::new(
            GenericFunctionLocalId(index),
            GenericFunctionType::from_shapes(type_, shape),
        )
    }
}
