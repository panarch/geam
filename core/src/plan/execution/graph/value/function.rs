use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::type_::{
    BitArrayListTypeId, BoolListTypeId, CustomFunctionType, CustomListTypeId, ExternalFunctionType,
    ExternalListTypeId, FloatListTypeId, FunctionFunctionType, FunctionListTypeId, FunctionType,
    GenericFunctionType, IntListTypeId, ListListTypeId, NilListTypeId, ParameterListListTypeId,
    ParameterListTypeId, StringListTypeId, TupleListTypeId, UtfCodepointListTypeId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntFunctionLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FloatFunctionLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringFunctionLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitArrayFunctionLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UtfCodepointFunctionLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GenericFunctionLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NeverFunctionLocalId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NeverFunctionLocal {
    pub id: NeverFunctionLocalId,
    pub type_: GenericFunctionType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenericFunctionLocal {
    pub id: GenericFunctionLocalId,
    pub type_: GenericFunctionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomFunctionLocalId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CustomFunctionLocal {
    pub id: CustomFunctionLocalId,
    pub type_: CustomFunctionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExternalFunctionLocalId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternalFunctionLocal {
    pub id: ExternalFunctionLocalId,
    pub type_: ExternalFunctionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoolFunctionLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NilFunctionLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TupleFunctionLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntListFunctionLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringListFunctionLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitArrayListFunctionLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UtfCodepointListFunctionLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParameterListFunctionLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParameterListListFunctionLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomListFunctionLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExternalListFunctionLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FloatListFunctionLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoolListFunctionLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NilListFunctionLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TupleListFunctionLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ListListFunctionLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionListFunctionLocalId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListFunctionLocal {
    Parameter {
        local: ParameterListFunctionLocalId,
        type_: FunctionType,
        list_type: ParameterListTypeId,
    },
    ParameterList {
        local: ParameterListListFunctionLocalId,
        type_: FunctionType,
        list_type: ParameterListListTypeId,
    },
    Int {
        local: IntListFunctionLocalId,
        type_: FunctionType,
        list_type: IntListTypeId,
    },
    String {
        local: StringListFunctionLocalId,
        type_: FunctionType,
        list_type: StringListTypeId,
    },
    BitArray {
        local: BitArrayListFunctionLocalId,
        type_: FunctionType,
        list_type: BitArrayListTypeId,
    },
    UtfCodepoint {
        local: UtfCodepointListFunctionLocalId,
        type_: FunctionType,
        list_type: UtfCodepointListTypeId,
    },
    Custom {
        local: CustomListFunctionLocalId,
        type_: FunctionType,
        list_type: CustomListTypeId,
    },
    External {
        local: ExternalListFunctionLocalId,
        type_: FunctionType,
        list_type: ExternalListTypeId,
    },
    Float {
        local: FloatListFunctionLocalId,
        type_: FunctionType,
        list_type: FloatListTypeId,
    },
    Bool {
        local: BoolListFunctionLocalId,
        type_: FunctionType,
        list_type: BoolListTypeId,
    },
    Nil {
        local: NilListFunctionLocalId,
        type_: FunctionType,
        list_type: NilListTypeId,
    },
    Tuple {
        local: TupleListFunctionLocalId,
        type_: FunctionType,
        list_type: TupleListTypeId,
    },
    List {
        local: ListListFunctionLocalId,
        type_: FunctionType,
        list_type: ListListTypeId,
    },
    Function {
        local: FunctionListFunctionLocalId,
        type_: FunctionType,
        list_type: FunctionListTypeId,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CoreFunctionFunctionLocalId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FunctionFunctionLocal {
    Core(CoreFunctionFunctionLocal),
    External(ExternalFunctionFunctionLocal),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CoreFunctionFunctionLocal {
    pub id: CoreFunctionFunctionLocalId,
    pub type_: FunctionFunctionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExternalFunctionFunctionLocalId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternalFunctionFunctionLocal {
    pub id: ExternalFunctionFunctionLocalId,
    pub type_: FunctionFunctionType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FunctionLocal {
    Generic(GenericFunctionLocal),
    Never(NeverFunctionLocal),
    Int(IntFunctionLocalId),
    Float(FloatFunctionLocalId),
    String(StringFunctionLocalId),
    BitArray(BitArrayFunctionLocalId),
    UtfCodepoint(UtfCodepointFunctionLocalId),
    Custom(CustomFunctionLocal),
    External(ExternalFunctionLocal),
    Bool(BoolFunctionLocalId),
    Nil(NilFunctionLocalId),
    Tuple(TupleFunctionLocalId),
    List(ListFunctionLocal),
    Function(FunctionFunctionLocal),
}

impl CustomFunctionLocal {
    pub(in crate::plan::execution) fn new(
        id: CustomFunctionLocalId,
        type_: CustomFunctionType,
    ) -> Self {
        Self { id, type_ }
    }

    pub(crate) fn id(&self) -> CustomFunctionLocalId {
        self.id
    }
}

impl ExternalFunctionLocal {
    pub(in crate::plan::execution) fn new(
        id: ExternalFunctionLocalId,
        type_: ExternalFunctionType,
    ) -> Self {
        Self { id, type_ }
    }

    pub(crate) fn id(&self) -> ExternalFunctionLocalId {
        self.id
    }
}

impl GenericFunctionLocal {
    pub(in crate::plan::execution) fn new(
        id: GenericFunctionLocalId,
        type_: GenericFunctionType,
    ) -> Self {
        Self { id, type_ }
    }

    pub(crate) fn id(&self) -> GenericFunctionLocalId {
        self.id
    }
}

impl NeverFunctionLocal {
    pub(in crate::plan::execution) fn new(
        id: NeverFunctionLocalId,
        type_: GenericFunctionType,
    ) -> Self {
        Self { id, type_ }
    }

    pub(crate) fn id(&self) -> NeverFunctionLocalId {
        self.id
    }
}

impl CoreFunctionFunctionLocal {
    pub(in crate::plan::execution) fn new(
        id: CoreFunctionFunctionLocalId,
        type_: FunctionFunctionType,
    ) -> Self {
        Self { id, type_ }
    }

    pub(crate) fn id(&self) -> CoreFunctionFunctionLocalId {
        self.id
    }
}

impl ExternalFunctionFunctionLocal {
    pub(in crate::plan::execution) fn new(
        id: ExternalFunctionFunctionLocalId,
        type_: FunctionFunctionType,
    ) -> Self {
        Self { id, type_ }
    }

    pub(crate) fn id(&self) -> ExternalFunctionFunctionLocalId {
        self.id
    }
}

impl ListFunctionLocal {
    #[cfg(test)]
    pub(crate) fn type_(&self) -> &FunctionType {
        match self {
            Self::Parameter { type_, .. }
            | Self::ParameterList { type_, .. }
            | Self::Int { type_, .. }
            | Self::String { type_, .. }
            | Self::BitArray { type_, .. }
            | Self::UtfCodepoint { type_, .. }
            | Self::Custom { type_, .. }
            | Self::External { type_, .. }
            | Self::Float { type_, .. }
            | Self::Bool { type_, .. }
            | Self::Nil { type_, .. }
            | Self::Tuple { type_, .. }
            | Self::List { type_, .. }
            | Self::Function { type_, .. } => type_,
        }
    }

    #[cfg(test)]
    pub(crate) fn list_type(&self) -> crate::plan::execution::type_::ListTypeId {
        match self {
            Self::Parameter { list_type, .. } => list_type.list_type(),
            Self::ParameterList { list_type, .. } => list_type.list_type(),
            Self::Int { list_type, .. } => list_type.list_type(),
            Self::String { list_type, .. } => list_type.list_type(),
            Self::BitArray { list_type, .. } => list_type.list_type(),
            Self::UtfCodepoint { list_type, .. } => list_type.list_type(),
            Self::Custom { list_type, .. } => list_type.list_type(),
            Self::External { list_type, .. } => list_type.list_type(),
            Self::Float { list_type, .. } => list_type.list_type(),
            Self::Bool { list_type, .. } => list_type.list_type(),
            Self::Nil { list_type, .. } => list_type.list_type(),
            Self::Tuple { list_type, .. } => list_type.list_type(),
            Self::List { list_type, .. } => list_type.list_type(),
            Self::Function { list_type, .. } => list_type.list_type(),
        }
    }
}

impl Emit for IntFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::IntFunctionLocalId", &[field_0]);
    }
}

impl Emit for FloatFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::FloatFunctionLocalId", &[field_0]);
    }
}

impl Emit for StringFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::StringFunctionLocalId", &[field_0]);
    }
}

impl Emit for BitArrayFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::BitArrayFunctionLocalId", &[field_0]);
    }
}

impl Emit for UtfCodepointFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::UtfCodepointFunctionLocalId", &[field_0]);
    }
}

impl Emit for GenericFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::GenericFunctionLocalId", &[field_0]);
    }
}

impl Emit for NeverFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::NeverFunctionLocalId", &[field_0]);
    }
}

impl Emit for NeverFunctionLocal {
    fn emit(&self, output: &mut Rust) {
        let Self { id, type_ } = self;
        output.structure("graph::NeverFunctionLocal", &[("id", id), ("type_", type_)]);
    }
}

impl Emit for GenericFunctionLocal {
    fn emit(&self, output: &mut Rust) {
        let Self { id, type_ } = self;
        output.structure(
            "graph::GenericFunctionLocal",
            &[("id", id), ("type_", type_)],
        );
    }
}

impl Emit for CustomFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::CustomFunctionLocalId", &[field_0]);
    }
}

impl Emit for CustomFunctionLocal {
    fn emit(&self, output: &mut Rust) {
        let Self { id, type_ } = self;
        output.structure(
            "graph::CustomFunctionLocal",
            &[("id", id), ("type_", type_)],
        );
    }
}

impl Emit for ExternalFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::ExternalFunctionLocalId", &[field_0]);
    }
}

impl Emit for ExternalFunctionLocal {
    fn emit(&self, output: &mut Rust) {
        let Self { id, type_ } = self;
        output.structure(
            "graph::ExternalFunctionLocal",
            &[("id", id), ("type_", type_)],
        );
    }
}

impl Emit for BoolFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::BoolFunctionLocalId", &[field_0]);
    }
}

impl Emit for NilFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::NilFunctionLocalId", &[field_0]);
    }
}

impl Emit for TupleFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::TupleFunctionLocalId", &[field_0]);
    }
}

impl Emit for IntListFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::IntListFunctionLocalId", &[field_0]);
    }
}

impl Emit for StringListFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::StringListFunctionLocalId", &[field_0]);
    }
}

impl Emit for BitArrayListFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::BitArrayListFunctionLocalId", &[field_0]);
    }
}

impl Emit for UtfCodepointListFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::UtfCodepointListFunctionLocalId", &[field_0]);
    }
}

impl Emit for ParameterListFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::ParameterListFunctionLocalId", &[field_0]);
    }
}

impl Emit for ParameterListListFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::ParameterListListFunctionLocalId", &[field_0]);
    }
}

impl Emit for CustomListFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::CustomListFunctionLocalId", &[field_0]);
    }
}

impl Emit for ExternalListFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::ExternalListFunctionLocalId", &[field_0]);
    }
}

impl Emit for FloatListFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::FloatListFunctionLocalId", &[field_0]);
    }
}

impl Emit for BoolListFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::BoolListFunctionLocalId", &[field_0]);
    }
}

impl Emit for NilListFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::NilListFunctionLocalId", &[field_0]);
    }
}

impl Emit for TupleListFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::TupleListFunctionLocalId", &[field_0]);
    }
}

impl Emit for ListListFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::ListListFunctionLocalId", &[field_0]);
    }
}

impl Emit for FunctionListFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::FunctionListFunctionLocalId", &[field_0]);
    }
}

impl Emit for ListFunctionLocal {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Parameter {
                local,
                type_,
                list_type,
            } => output.structure(
                "graph::ListFunctionLocal::Parameter",
                &[("local", local), ("type_", type_), ("list_type", list_type)],
            ),
            Self::ParameterList {
                local,
                type_,
                list_type,
            } => output.structure(
                "graph::ListFunctionLocal::ParameterList",
                &[("local", local), ("type_", type_), ("list_type", list_type)],
            ),
            Self::Int {
                local,
                type_,
                list_type,
            } => output.structure(
                "graph::ListFunctionLocal::Int",
                &[("local", local), ("type_", type_), ("list_type", list_type)],
            ),
            Self::String {
                local,
                type_,
                list_type,
            } => output.structure(
                "graph::ListFunctionLocal::String",
                &[("local", local), ("type_", type_), ("list_type", list_type)],
            ),
            Self::BitArray {
                local,
                type_,
                list_type,
            } => output.structure(
                "graph::ListFunctionLocal::BitArray",
                &[("local", local), ("type_", type_), ("list_type", list_type)],
            ),
            Self::UtfCodepoint {
                local,
                type_,
                list_type,
            } => output.structure(
                "graph::ListFunctionLocal::UtfCodepoint",
                &[("local", local), ("type_", type_), ("list_type", list_type)],
            ),
            Self::Custom {
                local,
                type_,
                list_type,
            } => output.structure(
                "graph::ListFunctionLocal::Custom",
                &[("local", local), ("type_", type_), ("list_type", list_type)],
            ),
            Self::External {
                local,
                type_,
                list_type,
            } => output.structure(
                "graph::ListFunctionLocal::External",
                &[("local", local), ("type_", type_), ("list_type", list_type)],
            ),
            Self::Float {
                local,
                type_,
                list_type,
            } => output.structure(
                "graph::ListFunctionLocal::Float",
                &[("local", local), ("type_", type_), ("list_type", list_type)],
            ),
            Self::Bool {
                local,
                type_,
                list_type,
            } => output.structure(
                "graph::ListFunctionLocal::Bool",
                &[("local", local), ("type_", type_), ("list_type", list_type)],
            ),
            Self::Nil {
                local,
                type_,
                list_type,
            } => output.structure(
                "graph::ListFunctionLocal::Nil",
                &[("local", local), ("type_", type_), ("list_type", list_type)],
            ),
            Self::Tuple {
                local,
                type_,
                list_type,
            } => output.structure(
                "graph::ListFunctionLocal::Tuple",
                &[("local", local), ("type_", type_), ("list_type", list_type)],
            ),
            Self::List {
                local,
                type_,
                list_type,
            } => output.structure(
                "graph::ListFunctionLocal::List",
                &[("local", local), ("type_", type_), ("list_type", list_type)],
            ),
            Self::Function {
                local,
                type_,
                list_type,
            } => output.structure(
                "graph::ListFunctionLocal::Function",
                &[("local", local), ("type_", type_), ("list_type", list_type)],
            ),
        }
    }
}

impl Emit for CoreFunctionFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::CoreFunctionFunctionLocalId", &[field_0]);
    }
}

impl Emit for FunctionFunctionLocal {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Core(field_0) => output.call("graph::FunctionFunctionLocal::Core", &[field_0]),
            Self::External(field_0) => {
                output.call("graph::FunctionFunctionLocal::External", &[field_0])
            }
        }
    }
}

impl Emit for CoreFunctionFunctionLocal {
    fn emit(&self, output: &mut Rust) {
        let Self { id, type_ } = self;
        output.structure(
            "graph::CoreFunctionFunctionLocal",
            &[("id", id), ("type_", type_)],
        );
    }
}

impl Emit for ExternalFunctionFunctionLocalId {
    fn emit(&self, output: &mut Rust) {
        let Self(field_0) = self;
        output.call("graph::ExternalFunctionFunctionLocalId", &[field_0]);
    }
}

impl Emit for ExternalFunctionFunctionLocal {
    fn emit(&self, output: &mut Rust) {
        let Self { id, type_ } = self;
        output.structure(
            "graph::ExternalFunctionFunctionLocal",
            &[("id", id), ("type_", type_)],
        );
    }
}

impl Emit for FunctionLocal {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Generic(field_0) => output.call("graph::FunctionLocal::Generic", &[field_0]),
            Self::Never(field_0) => output.call("graph::FunctionLocal::Never", &[field_0]),
            Self::Int(field_0) => output.call("graph::FunctionLocal::Int", &[field_0]),
            Self::Float(field_0) => output.call("graph::FunctionLocal::Float", &[field_0]),
            Self::String(field_0) => output.call("graph::FunctionLocal::String", &[field_0]),
            Self::BitArray(field_0) => output.call("graph::FunctionLocal::BitArray", &[field_0]),
            Self::UtfCodepoint(field_0) => {
                output.call("graph::FunctionLocal::UtfCodepoint", &[field_0])
            }
            Self::Custom(field_0) => output.call("graph::FunctionLocal::Custom", &[field_0]),
            Self::External(field_0) => output.call("graph::FunctionLocal::External", &[field_0]),
            Self::Bool(field_0) => output.call("graph::FunctionLocal::Bool", &[field_0]),
            Self::Nil(field_0) => output.call("graph::FunctionLocal::Nil", &[field_0]),
            Self::Tuple(field_0) => output.call("graph::FunctionLocal::Tuple", &[field_0]),
            Self::List(field_0) => output.call("graph::FunctionLocal::List", &[field_0]),
            Self::Function(field_0) => output.call("graph::FunctionLocal::Function", &[field_0]),
        }
    }
}

#[cfg(test)]
mod emission_tests {
    use super::{
        BitArrayFunctionLocalId, BitArrayListFunctionLocalId, BoolFunctionLocalId,
        BoolListFunctionLocalId, CoreFunctionFunctionLocal, CoreFunctionFunctionLocalId,
        CustomFunctionLocal, CustomFunctionLocalId, CustomListFunctionLocalId,
        ExternalFunctionFunctionLocal, ExternalFunctionFunctionLocalId, ExternalFunctionLocal,
        ExternalFunctionLocalId, ExternalListFunctionLocalId, FloatFunctionLocalId,
        FloatListFunctionLocalId, FunctionFunctionLocal, FunctionListFunctionLocalId,
        FunctionLocal, GenericFunctionLocal, GenericFunctionLocalId, IntFunctionLocalId,
        IntListFunctionLocalId, ListFunctionLocal, ListListFunctionLocalId, NeverFunctionLocal,
        NeverFunctionLocalId, NilFunctionLocalId, NilListFunctionLocalId,
        ParameterListFunctionLocalId, ParameterListListFunctionLocalId, StringFunctionLocalId,
        StringListFunctionLocalId, TupleFunctionLocalId, TupleListFunctionLocalId,
        UtfCodepointFunctionLocalId, UtfCodepointListFunctionLocalId,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::list::{FunctionItemTypeId, TupleItemTypeId};
    use crate::plan::execution::type_::{
        BitArrayListTypeId, BoolListTypeId, CustomFunctionType, CustomListTypeId, CustomTypeId,
        CustomValueShape, CustomValueShapeId, ExternalFunctionType, ExternalListTypeId,
        ExternalTypeId, FloatListTypeId, FunctionFunctionType, FunctionListTypeId, FunctionShape,
        FunctionType, GenericFunctionType, IntListTypeId, ListListTypeId, ListTypeId,
        NilListTypeId, ParameterListListTypeId, ParameterListTypeId, StringListTypeId,
        TupleListTypeId, UtfCodepointListTypeId, ValueShapeId, ValueType,
    };

    #[test]
    fn emits_list_function_locals_with_their_signature_and_item_metadata() {
        let cases = [
            (
                ListFunctionLocal::Int {
                    local: IntListFunctionLocalId(2),
                    type_: FunctionType::new(Vec::new(), ValueType::List(ListTypeId(3))),
                    list_type: IntListTypeId::new(ListTypeId(3)),
                },
                concat!(
                    "data::graph::ListFunctionLocal::Int {local: data::graph::IntListFunctionLocalId(2,),",
                    "type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(3,),)),},",
                    "list_type: data::type_::IntListTypeId {list_type: data::type_::ListTypeId(3,),},}"
                ),
            ),
            (
                ListFunctionLocal::String {
                    local: StringListFunctionLocalId(2),
                    type_: FunctionType::new(Vec::new(), ValueType::List(ListTypeId(3))),
                    list_type: StringListTypeId::new(ListTypeId(3)),
                },
                concat!(
                    "data::graph::ListFunctionLocal::String {local: data::graph::StringListFunctionLocalId(2,),",
                    "type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(3,),)),},",
                    "list_type: data::type_::StringListTypeId {list_type: data::type_::ListTypeId(3,),},}"
                ),
            ),
            (
                ListFunctionLocal::BitArray {
                    local: BitArrayListFunctionLocalId(2),
                    type_: FunctionType::new(Vec::new(), ValueType::List(ListTypeId(3))),
                    list_type: BitArrayListTypeId::new(ListTypeId(3)),
                },
                concat!(
                    "data::graph::ListFunctionLocal::BitArray {local: data::graph::BitArrayListFunctionLocalId(2,),",
                    "type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(3,),)),},",
                    "list_type: data::type_::BitArrayListTypeId {list_type: data::type_::ListTypeId(3,),},}"
                ),
            ),
            (
                ListFunctionLocal::UtfCodepoint {
                    local: UtfCodepointListFunctionLocalId(2),
                    type_: FunctionType::new(Vec::new(), ValueType::List(ListTypeId(3))),
                    list_type: UtfCodepointListTypeId::new(ListTypeId(3)),
                },
                concat!(
                    "data::graph::ListFunctionLocal::UtfCodepoint {local: data::graph::UtfCodepointListFunctionLocalId(2,),",
                    "type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(3,),)),},",
                    "list_type: data::type_::UtfCodepointListTypeId {list_type: data::type_::ListTypeId(3,),},}"
                ),
            ),
            (
                ListFunctionLocal::Float {
                    local: FloatListFunctionLocalId(2),
                    type_: FunctionType::new(Vec::new(), ValueType::List(ListTypeId(3))),
                    list_type: FloatListTypeId::new(ListTypeId(3)),
                },
                concat!(
                    "data::graph::ListFunctionLocal::Float {local: data::graph::FloatListFunctionLocalId(2,),",
                    "type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(3,),)),},",
                    "list_type: data::type_::FloatListTypeId {list_type: data::type_::ListTypeId(3,),},}"
                ),
            ),
            (
                ListFunctionLocal::Bool {
                    local: BoolListFunctionLocalId(2),
                    type_: FunctionType::new(Vec::new(), ValueType::List(ListTypeId(3))),
                    list_type: BoolListTypeId::new(ListTypeId(3)),
                },
                concat!(
                    "data::graph::ListFunctionLocal::Bool {local: data::graph::BoolListFunctionLocalId(2,),",
                    "type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(3,),)),},",
                    "list_type: data::type_::BoolListTypeId {list_type: data::type_::ListTypeId(3,),},}"
                ),
            ),
            (
                ListFunctionLocal::Nil {
                    local: NilListFunctionLocalId(2),
                    type_: FunctionType::new(Vec::new(), ValueType::List(ListTypeId(3))),
                    list_type: NilListTypeId::new(ListTypeId(3)),
                },
                concat!(
                    "data::graph::ListFunctionLocal::Nil {local: data::graph::NilListFunctionLocalId(2,),",
                    "type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(3,),)),},",
                    "list_type: data::type_::NilListTypeId {list_type: data::type_::ListTypeId(3,),},}"
                ),
            ),
            (
                ListFunctionLocal::Parameter {
                    local: ParameterListFunctionLocalId(2),
                    type_: FunctionType::new(Vec::new(), ValueType::List(ListTypeId(3))),
                    list_type: ParameterListTypeId::new(
                        ListTypeId(3),
                        crate::plan::TypeParameterId(4),
                    ),
                },
                concat!(
                    "data::graph::ListFunctionLocal::Parameter {local: data::graph::ParameterListFunctionLocalId(2,),",
                    "type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(3,),)),},",
                    "list_type: data::type_::ParameterListTypeId {list_type: data::type_::ListTypeId(3,),item: data::type_::parameter_id(4,),},}"
                ),
            ),
            (
                ListFunctionLocal::ParameterList {
                    local: ParameterListListFunctionLocalId(2),
                    type_: FunctionType::new(Vec::new(), ValueType::List(ListTypeId(3))),
                    list_type: ParameterListListTypeId::new(
                        ListTypeId(3),
                        ParameterListTypeId::new(ListTypeId(4), crate::plan::TypeParameterId(5)),
                    ),
                },
                concat!(
                    "data::graph::ListFunctionLocal::ParameterList {local: data::graph::ParameterListListFunctionLocalId(2,),",
                    "type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(3,),)),},",
                    "list_type: data::type_::ParameterListListTypeId {list_type: data::type_::ListTypeId(3,),item_type: data::type_::ParameterListTypeId {list_type: data::type_::ListTypeId(4,),item: data::type_::parameter_id(5,),},},}"
                ),
            ),
            (
                ListFunctionLocal::Custom {
                    local: CustomListFunctionLocalId(2),
                    type_: FunctionType::new(Vec::new(), ValueType::List(ListTypeId(3))),
                    list_type: CustomListTypeId::new(ListTypeId(3), CustomTypeId(4)),
                },
                concat!(
                    "data::graph::ListFunctionLocal::Custom {local: data::graph::CustomListFunctionLocalId(2,),",
                    "type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(3,),)),},",
                    "list_type: data::type_::CustomListTypeId {list_type: data::type_::ListTypeId(3,),item_type: data::type_::CustomTypeId(4,),},}"
                ),
            ),
            (
                ListFunctionLocal::External {
                    local: ExternalListFunctionLocalId(2),
                    type_: FunctionType::new(Vec::new(), ValueType::List(ListTypeId(3))),
                    list_type: ExternalListTypeId::new(ListTypeId(3), ExternalTypeId(4)),
                },
                concat!(
                    "data::graph::ListFunctionLocal::External {local: data::graph::ExternalListFunctionLocalId(2,),",
                    "type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(3,),)),},",
                    "list_type: data::type_::ExternalListTypeId {list_type: data::type_::ListTypeId(3,),item_type: data::type_::ExternalTypeId(4,),},}"
                ),
            ),
            (
                ListFunctionLocal::Tuple {
                    local: TupleListFunctionLocalId(2),
                    type_: FunctionType::new(Vec::new(), ValueType::List(ListTypeId(3))),
                    list_type: TupleListTypeId {
                        list_type: ListTypeId(3),
                        item_type: TupleItemTypeId(4),
                    },
                },
                concat!(
                    "data::graph::ListFunctionLocal::Tuple {local: data::graph::TupleListFunctionLocalId(2,),",
                    "type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(3,),)),},",
                    "list_type: data::type_::TupleListTypeId {list_type: data::type_::ListTypeId(3,),item_type: data::type_::TupleItemTypeId(4,),},}"
                ),
            ),
            (
                ListFunctionLocal::List {
                    local: ListListFunctionLocalId(2),
                    type_: FunctionType::new(Vec::new(), ValueType::List(ListTypeId(3))),
                    list_type: ListListTypeId::new(ListTypeId(3), ListTypeId(4)),
                },
                concat!(
                    "data::graph::ListFunctionLocal::List {local: data::graph::ListListFunctionLocalId(2,),",
                    "type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(3,),)),},",
                    "list_type: data::type_::ListListTypeId {list_type: data::type_::ListTypeId(3,),item_type: data::type_::ListTypeId(4,),},}"
                ),
            ),
            (
                ListFunctionLocal::Function {
                    local: FunctionListFunctionLocalId(2),
                    type_: FunctionType::new(Vec::new(), ValueType::List(ListTypeId(3))),
                    list_type: FunctionListTypeId {
                        list_type: ListTypeId(3),
                        item_type: FunctionItemTypeId(4),
                    },
                },
                concat!(
                    "data::graph::ListFunctionLocal::Function {local: data::graph::FunctionListFunctionLocalId(2,),",
                    "type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(3,),)),},",
                    "list_type: data::type_::FunctionListTypeId {list_type: data::type_::ListTypeId(3,),item_type: data::type_::FunctionItemTypeId(4,),},}"
                ),
            ),
        ];
        for (local, expected) in cases {
            assert_eq!(Rust::expression(&local), expected);
        }
    }

    #[test]
    fn emits_function_locals_with_nominal_and_callable_shape_metadata() {
        let symbolic = FunctionType::new(
            Vec::new(),
            ValueType::Parameter(crate::plan::TypeParameterId(0)),
        );
        let inner = FunctionType::new(Vec::new(), ValueType::Int);
        let cases = [
            (
                FunctionLocal::Int(IntFunctionLocalId(2)),
                "data::graph::FunctionLocal::Int(data::graph::IntFunctionLocalId(2,),)",
            ),
            (
                FunctionLocal::Float(FloatFunctionLocalId(2)),
                "data::graph::FunctionLocal::Float(data::graph::FloatFunctionLocalId(2,),)",
            ),
            (
                FunctionLocal::String(StringFunctionLocalId(2)),
                "data::graph::FunctionLocal::String(data::graph::StringFunctionLocalId(2,),)",
            ),
            (
                FunctionLocal::BitArray(BitArrayFunctionLocalId(2)),
                "data::graph::FunctionLocal::BitArray(data::graph::BitArrayFunctionLocalId(2,),)",
            ),
            (
                FunctionLocal::UtfCodepoint(UtfCodepointFunctionLocalId(2)),
                "data::graph::FunctionLocal::UtfCodepoint(data::graph::UtfCodepointFunctionLocalId(2,),)",
            ),
            (
                FunctionLocal::Bool(BoolFunctionLocalId(2)),
                "data::graph::FunctionLocal::Bool(data::graph::BoolFunctionLocalId(2,),)",
            ),
            (
                FunctionLocal::Nil(NilFunctionLocalId(2)),
                "data::graph::FunctionLocal::Nil(data::graph::NilFunctionLocalId(2,),)",
            ),
            (
                FunctionLocal::Tuple(TupleFunctionLocalId(2)),
                "data::graph::FunctionLocal::Tuple(data::graph::TupleFunctionLocalId(2,),)",
            ),
            (
                FunctionLocal::Generic(GenericFunctionLocal::new(
                    GenericFunctionLocalId(2),
                    GenericFunctionType::from_shapes(
                        symbolic.clone(),
                        FunctionShape::new(ValueShapeId(3), symbolic.clone()),
                    ),
                )),
                concat!(
                    "data::graph::FunctionLocal::Generic(data::graph::GenericFunctionLocal {id: data::graph::GenericFunctionLocalId(2,),",
                    "type_: data::type_::GenericFunctionType {type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0,),)),},shape: data::type_::FunctionShape {shape_id: data::type_::ValueShapeId(3,),type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0,),)),},},},},)"
                ),
            ),
            (
                FunctionLocal::Never(NeverFunctionLocal::new(
                    NeverFunctionLocalId(2),
                    GenericFunctionType::from_shapes(
                        symbolic.clone(),
                        FunctionShape::new(ValueShapeId(3), symbolic.clone()),
                    ),
                )),
                concat!(
                    "data::graph::FunctionLocal::Never(data::graph::NeverFunctionLocal {id: data::graph::NeverFunctionLocalId(2,),",
                    "type_: data::type_::GenericFunctionType {type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0,),)),},shape: data::type_::FunctionShape {shape_id: data::type_::ValueShapeId(3,),type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0,),)),},},},},)"
                ),
            ),
            (
                FunctionLocal::Custom(CustomFunctionLocal::new(
                    CustomFunctionLocalId(2),
                    CustomFunctionType::from_shapes(
                        FunctionType::new(Vec::new(), ValueType::Custom(CustomTypeId(3))),
                        Vec::new(),
                        CustomValueShape::new(CustomTypeId(3), CustomValueShapeId(4)),
                    ),
                )),
                concat!(
                    "data::graph::FunctionLocal::Custom(data::graph::CustomFunctionLocal {id: data::graph::CustomFunctionLocalId(2,),",
                    "type_: data::type_::CustomFunctionType {type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(3,),)),},arguments: data::Storage::Static(&[]),return_: data::type_::CustomValueShape {type_id: data::type_::CustomTypeId(3,),shape_id: data::type_::CustomValueShapeId(4,),},},},)"
                ),
            ),
            (
                FunctionLocal::External(ExternalFunctionLocal::new(
                    ExternalFunctionLocalId(2),
                    ExternalFunctionType::from_shapes(
                        FunctionType::new(Vec::new(), ValueType::External(ExternalTypeId(3))),
                        Vec::new(),
                        ExternalTypeId(3),
                    ),
                )),
                concat!(
                    "data::graph::FunctionLocal::External(data::graph::ExternalFunctionLocal {id: data::graph::ExternalFunctionLocalId(2,),",
                    "type_: data::type_::ExternalFunctionType {type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(3,),)),},arguments: data::Storage::Static(&[]),return_: data::type_::ExternalTypeId(3,),},},)"
                ),
            ),
            (
                FunctionLocal::List(ListFunctionLocal::Int {
                    local: IntListFunctionLocalId(2),
                    type_: FunctionType::new(Vec::new(), ValueType::List(ListTypeId(3))),
                    list_type: IntListTypeId::new(ListTypeId(3)),
                }),
                concat!(
                    "data::graph::FunctionLocal::List(data::graph::ListFunctionLocal::Int {local: data::graph::IntListFunctionLocalId(2,),",
                    "type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(3,),)),},",
                    "list_type: data::type_::IntListTypeId {list_type: data::type_::ListTypeId(3,),},},)"
                ),
            ),
            (
                FunctionLocal::Function(FunctionFunctionLocal::Core(
                    CoreFunctionFunctionLocal::new(
                        CoreFunctionFunctionLocalId(2),
                        FunctionFunctionType::from_shapes(
                            FunctionType::new(Vec::new(), ValueType::Function(inner.clone())),
                            Vec::new(),
                            FunctionShape::new(ValueShapeId(7), inner.clone()),
                        ),
                    ),
                )),
                concat!(
                    "data::graph::FunctionLocal::Function(data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {",
                    "id: data::graph::CoreFunctionFunctionLocalId(2,),type_: data::type_::FunctionFunctionType {type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::Int),},)),},arguments: data::Storage::Static(&[]),return_: data::type_::FunctionShape {shape_id: data::type_::ValueShapeId(7,),type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::Int),},},},},),)"
                ),
            ),
            (
                FunctionLocal::Function(FunctionFunctionLocal::External(
                    ExternalFunctionFunctionLocal::new(
                        ExternalFunctionFunctionLocalId(2),
                        FunctionFunctionType::from_shapes(
                            FunctionType::new(Vec::new(), ValueType::Function(inner.clone())),
                            Vec::new(),
                            FunctionShape::new(ValueShapeId(7), inner.clone()),
                        ),
                    ),
                )),
                concat!(
                    "data::graph::FunctionLocal::Function(data::graph::FunctionFunctionLocal::External(data::graph::ExternalFunctionFunctionLocal {",
                    "id: data::graph::ExternalFunctionFunctionLocalId(2,),type_: data::type_::FunctionFunctionType {type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::Int),},)),},arguments: data::Storage::Static(&[]),return_: data::type_::FunctionShape {shape_id: data::type_::ValueShapeId(7,),type_: data::type_::FunctionType {arguments: data::Storage::Static(&[]),return_: data::Storage::Static(&data::type_::ValueType::Int),},},},},),)"
                ),
            ),
        ];
        for (local, expected) in cases {
            assert_eq!(Rust::expression(&local), expected);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BitArrayListFunctionLocalId, BoolListFunctionLocalId, CustomListFunctionLocalId,
        ExternalListFunctionLocalId, FloatListFunctionLocalId, FunctionListFunctionLocalId,
        IntListFunctionLocalId, ListFunctionLocal, ListListFunctionLocalId, NilListFunctionLocalId,
        ParameterListFunctionLocalId, ParameterListListFunctionLocalId, StringListFunctionLocalId,
        TupleListFunctionLocalId, UtfCodepointListFunctionLocalId,
    };
    use crate::plan::execution::type_::{ExternalListTypeId, ExternalTypeId, ListTypeId};

    #[test]
    fn parameter_list_function_locals_preserve_types_and_storage() {
        let parameter_plan = execution_plan("pub fn main() -> List(value) { [] }");
        let parameter = parameter_plan.parameter_list_function_id(0).type_id();
        let nested_plan = execution_plan("pub fn main() -> List(List(value)) { [] }");
        let nested = nested_plan.parameter_list_list_function_id(0).type_id();
        let type_ = function_type();
        let locals = [
            ListFunctionLocal::Parameter {
                local: ParameterListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: parameter,
            },
            ListFunctionLocal::ParameterList {
                local: ParameterListListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: nested,
            },
        ];

        assert_eq!(
            locals.clone().map(|local| local.list_type()),
            [parameter.list_type(), nested.list_type()],
        );
        assert_eq!(
            locals.map(|local| local.type_().clone()),
            std::array::from_fn(|_| type_.clone()),
        );
    }

    #[test]
    fn concrete_list_function_locals_preserve_every_type_and_storage() {
        let plan = execution_plan(
            r#"
pub type Boxed { Boxed(Int) }

fn ints() -> List(Int) { [] }
fn strings() -> List(String) { [] }
fn bit_arrays() -> List(BitArray) { [] }
fn utf_codepoints() -> List(UtfCodepoint) { [] }
fn customs() -> List(Boxed) { [] }
fn floats() -> List(Float) { [] }
fn bools() -> List(Bool) { [] }
fn nils() -> List(Nil) { [] }
fn tuples() -> List(#(Int)) { [] }
fn lists() -> List(List(Int)) { [] }
fn functions() -> List(fn() -> Int) { [] }

pub fn main() {
  let _ = #(
    ints, strings, bit_arrays, utf_codepoints, customs, floats,
    bools, nils, tuples, lists, functions,
  )
  Nil
}
"#,
        );
        let type_ = function_type();
        let locals = [
            ListFunctionLocal::Int {
                local: IntListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.int_list_function_id(0).type_id(),
            },
            ListFunctionLocal::String {
                local: StringListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.string_list_function_id(0).type_id(),
            },
            ListFunctionLocal::BitArray {
                local: BitArrayListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.bit_array_list_function_id(0).type_id(),
            },
            ListFunctionLocal::UtfCodepoint {
                local: UtfCodepointListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.utf_codepoint_list_function_id(0).type_id(),
            },
            ListFunctionLocal::Custom {
                local: CustomListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.custom_list_function_id(0).type_id(),
            },
            ListFunctionLocal::Float {
                local: FloatListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.float_list_function_id(0).type_id(),
            },
            ListFunctionLocal::Bool {
                local: BoolListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.bool_list_function_id(0).type_id(),
            },
            ListFunctionLocal::Nil {
                local: NilListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.nil_list_function_id(0).type_id(),
            },
            ListFunctionLocal::Tuple {
                local: TupleListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.tuple_list_function_id(0).type_id(),
            },
            ListFunctionLocal::List {
                local: ListListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.list_list_function_id(0).type_id(),
            },
            ListFunctionLocal::Function {
                local: FunctionListFunctionLocalId(0),
                type_: type_.clone(),
                list_type: plan.function_list_function_id(0).type_id(),
            },
        ];

        assert_eq!(
            locals.clone().map(|local| local.list_type()),
            [
                plan.int_list_function_id(0).type_id().list_type(),
                plan.string_list_function_id(0).type_id().list_type(),
                plan.bit_array_list_function_id(0).type_id().list_type(),
                plan.utf_codepoint_list_function_id(0).type_id().list_type(),
                plan.custom_list_function_id(0).type_id().list_type(),
                plan.float_list_function_id(0).type_id().list_type(),
                plan.bool_list_function_id(0).type_id().list_type(),
                plan.nil_list_function_id(0).type_id().list_type(),
                plan.tuple_list_function_id(0).type_id().list_type(),
                plan.list_list_function_id(0).type_id().list_type(),
                plan.function_list_function_id(0).type_id().list_type(),
            ],
        );
        assert_eq!(
            locals.map(|local| local.type_().clone()),
            std::array::from_fn(|_| type_.clone()),
        );
    }

    #[test]
    fn external_list_function_locals_preserve_type_and_storage() {
        let type_ = function_type();
        let list_type = ExternalListTypeId::new(ListTypeId::new(17), ExternalTypeId::new(4));
        let local = ListFunctionLocal::External {
            local: ExternalListFunctionLocalId(2),
            type_: type_.clone(),
            list_type,
        };

        assert_eq!(local.type_(), &type_);
        assert_eq!(local.list_type(), ListTypeId::new(17));
    }

    fn function_type() -> crate::plan::execution::type_::FunctionType {
        crate::plan::execution::type_::FunctionType::new(
            Vec::new(),
            crate::plan::execution::type_::ValueType::Nil,
        )
    }

    fn execution_plan(source: &str) -> crate::ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        crate::ExecutionPlan::from_module_plan(module_plan)
    }
}
