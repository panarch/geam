use crate::host::{HostNeverFunction, HostValueFunction};
use crate::plan::execution::function::{ExecutionFunctionBody, FunctionBodyOwner};
use crate::plan::execution::graph::{
    BitArrayLocalId, BoolLocalId, FloatLocalId, IntLocalId, NilLocalId, ParamLocal, StringLocalId,
    UtfCodepointLocalId,
};
use crate::plan::execution::type_::FunctionType;
use ecow::EcoString;
use std::marker::PhantomData;

pub(crate) struct HostedFunction<Implementation> {
    metadata: HostedFunctionMetadata,
    implementation: Implementation,
}

pub(crate) enum HostedFunctionTarget<Body: FunctionBodyOwner> {
    Value(HostFunctionId<Body>),
    Never(HostNeverFunctionId),
}

pub(crate) struct HostedFunctionMetadata {
    package: EcoString,
    site: crate::plan::HostCallSite,
    signature: crate::plan::FunctionType,
    type_arguments: Box<[crate::plan::ValueType]>,
    parameters: Box<[ParamLocal]>,
    call_parameters: Box<[HostCallParameter]>,
    type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum HostCallParameter {
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
    Function(ParamLocal),
}

#[derive(Debug)]
pub(crate) struct HostFunctionId<Body: FunctionBodyOwner> {
    index: usize,
    return_: Body::Return,
    body: PhantomData<fn() -> Body>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostNeverFunctionId(usize);

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
        metadata: HostedFunctionMetadata,
        implementation: Implementation,
    ) -> Self {
        Self {
            metadata,
            implementation,
        }
    }

    pub(crate) fn package(&self) -> &EcoString {
        self.metadata.package()
    }

    pub(crate) fn module(&self) -> &EcoString {
        self.metadata.module()
    }

    pub(crate) fn name(&self) -> &EcoString {
        self.metadata.name()
    }

    pub(crate) fn parameters(&self) -> &[ParamLocal] {
        self.metadata.parameters()
    }

    pub(crate) fn type_arguments(&self) -> &[crate::plan::ValueType] {
        self.metadata.type_arguments()
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

    pub(crate) fn implementation(&self) -> &Implementation {
        &self.implementation
    }
}

impl HostedFunctionMetadata {
    pub(in crate::plan::execution) fn new(
        package: EcoString,
        site: crate::plan::HostCallSite,
        signature: crate::plan::FunctionType,
        type_arguments: Box<[crate::plan::ValueType]>,
        parameters: Box<[ParamLocal]>,
        call_parameters: Box<[HostCallParameter]>,
        type_: FunctionType,
    ) -> Self {
        Self {
            package,
            site,
            signature,
            type_arguments,
            parameters,
            call_parameters,
            type_,
        }
    }

    pub(crate) fn package(&self) -> &EcoString {
        &self.package
    }

    pub(crate) fn module(&self) -> &EcoString {
        self.site.module()
    }

    pub(crate) fn name(&self) -> &EcoString {
        self.site.function()
    }

    pub(crate) fn site(&self) -> &crate::plan::HostCallSite {
        &self.site
    }

    pub(crate) fn signature(&self) -> &crate::plan::FunctionType {
        &self.signature
    }

    fn type_arguments(&self) -> &[crate::plan::ValueType] {
        &self.type_arguments
    }

    fn parameters(&self) -> &[ParamLocal] {
        &self.parameters
    }

    fn call_parameters(&self) -> &[HostCallParameter] {
        &self.call_parameters
    }

    fn type_(&self) -> &FunctionType {
        &self.type_
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
            Self::Function(local) => local.clone(),
        }
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
            type_: vec![ValueType::Int],
        };
        let tuple = ParamLocal::Tuple {
            local: TupleLocalId(10),
            type_: vec![ValueType::Bool],
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
            (HostCallParameter::Function(function.clone()), function),
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
