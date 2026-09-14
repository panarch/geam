use super::construction::ConstructionIndex;
use crate::host::{HostNeverFunction, HostValueFunction};
use crate::plan::Text;
use crate::plan::execution::function::{ExecutionFunctionBody, FunctionBodyOwner};
use crate::plan::execution::graph::{
    BitArrayLocalId, BoolLocalId, FloatLocalId, IntLocalId, NilLocalId, ParamLocal, StringLocalId,
    UtfCodepointLocalId,
};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::{Node, Table};
use crate::plan::execution::type_::{FunctionMetadata, FunctionType, TypeMetadata};
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

pub struct HostedFunctionMetadata {
    pub package: Text,
    pub site: crate::plan::HostCallSite,
    pub signature: FunctionMetadata,
    pub type_arguments: Table<HostTypeArgument>,
    pub parameters: HostedFunctionParameters,
    pub constructions: HostConstructionTypes,
    pub type_: FunctionType,
    pub registration: Node<super::RegistrationContract>,
}

pub struct HostTypeArgument {
    pub type_: TypeMetadata,
    pub shape: crate::plan::execution::type_::ValueShapeId,
}

#[derive(Clone)]
pub struct HostConstructionTypes {
    pub lists: ConstructionIndex<crate::plan::execution::type_::ListTypeId>,
    pub customs: ConstructionIndex<crate::plan::execution::type_::CustomTypeId>,
    pub externals: ConstructionIndex<crate::plan::execution::type_::ExternalTypeId>,
    pub natives: super::NativeConversions,
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
            package: Text::Static(self.package()),
            site: crate::plan::HostCallSite::from_static(
                self.module(),
                self.name(),
                self.site.span(),
            ),
            signature: FunctionMetadata {
                arguments: Table::Static(&self.signature.arguments),
                return_: Node::Static(&self.signature.return_),
            },
            type_arguments: Table::Static(&self.type_arguments),
            parameters: HostedFunctionParameters {
                call: Table::Static(&self.parameters.call),
            },
            constructions: HostConstructionTypes {
                lists: ConstructionIndex {
                    entries: Table::Static(&self.constructions.lists.entries),
                },
                customs: ConstructionIndex {
                    entries: Table::Static(&self.constructions.customs.entries),
                },
                externals: ConstructionIndex {
                    entries: Table::Static(&self.constructions.externals.entries),
                },
                natives: super::NativeConversions {
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

    pub(crate) fn site(&self) -> &crate::plan::HostCallSite {
        &self.site
    }

    pub(crate) fn signature(&self) -> crate::plan::FunctionType {
        self.signature.materialize()
    }

    pub(crate) fn resolve_type(
        &self,
        descriptor: &crate::host::HostTypeDescriptor,
    ) -> Option<crate::plan::ValueType> {
        descriptor.resolve(&|index| {
            self.type_arguments
                .get(index)
                .map(|argument| argument.type_.materialize())
        })
    }

    pub(crate) fn resolve_construction(
        &self,
        descriptor: &crate::host::HostTypeDescriptor,
    ) -> crate::plan::ValueType {
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
        lists: HashMap<crate::plan::ValueType, crate::plan::execution::type_::ListTypeId>,
        customs: HashMap<crate::plan::ValueType, crate::plan::execution::type_::CustomTypeId>,
        externals: HashMap<crate::plan::ValueType, crate::plan::execution::type_::ExternalTypeId>,
    ) -> Self {
        Self {
            lists: ConstructionIndex::new(lists),
            customs: ConstructionIndex::new(customs),
            externals: ConstructionIndex::new(externals),
            natives: super::NativeConversions::default(),
        }
    }

    pub(crate) fn list(
        &self,
        type_: &crate::plan::ValueType,
    ) -> crate::plan::execution::type_::ListTypeId {
        self.lists.get(type_)
    }

    pub(in crate::plan::execution) fn with_natives(
        mut self,
        natives: super::NativeConversions,
    ) -> Self {
        self.natives = natives;
        self
    }

    pub(crate) fn natives(&self) -> &super::NativeConversions {
        &self.natives
    }

    pub(crate) fn custom(
        &self,
        type_: &crate::plan::ValueType,
    ) -> crate::plan::execution::type_::CustomTypeId {
        self.customs.get(type_)
    }

    pub(crate) fn external(
        &self,
        type_: &crate::plan::ValueType,
    ) -> crate::plan::execution::type_::ExternalTypeId {
        self.externals.get(type_)
    }
}

impl HostedFunctionParameters {
    pub(in crate::plan::execution) fn new(call: Box<[HostCallParameter]>) -> Self {
        Self { call: call.into() }
    }

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
        } = self;
        output.structure(
            "host::HostConstructionTypes",
            &[
                ("lists", lists),
                ("customs", customs),
                ("externals", externals),
                ("natives", natives),
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
        let Self { call } = self;
        output.structure("host::HostedFunctionParameters", &[("call", call)]);
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
    use super::{HostCallParameter, HostFunctionId, HostNeverFunctionId, HostedFunctionTarget};
    use crate::plan::execution::function::GenericFunctionFunctionBody;
    use crate::plan::execution::graph::{
        BitArrayLocalId, BoolLocalId, CustomLocal, CustomLocalId, FloatLocalId,
        GenericFunctionLocal, GenericFunctionLocalId, IntListLocalId, IntLocalId, ListLocal,
        NilLocalId, ParamLocal, StringLocalId, TupleLocalId, UtfCodepointLocalId,
    };
    use crate::plan::execution::type_::{
        CustomTypeId, CustomValueShape, CustomValueShapeId, FunctionShape, FunctionType,
        GenericFunctionType, IntListTypeId, ListTypeId, ValueShapeId, ValueType,
    };

    #[test]
    fn emitted_host_type_argument_keeps_nominal_and_specialized_shape_together() {
        assert_eq!(
            crate::plan::execution::prepared::rust::Rust::expression(&super::HostTypeArgument {
                type_: crate::plan::execution::type_::TypeMetadata::Int,
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
        use crate::plan::execution::prepared::rust::Rust;
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
        use crate::plan::execution::prepared::rust::Rust;

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
