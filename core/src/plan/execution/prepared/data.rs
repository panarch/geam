//! Compiler-visible data accepted only through prepared loading.

pub use super::{Export, HostedEntryArtifact, HostedModuleArtifact, ModuleArtifact, ProgramTables};
pub use crate::plan::execution::storage::Storage;
pub use crate::plan::text::Text;
pub use num_bigint::Sign;

pub mod profile {
    pub type Plain = std::convert::Infallible;
    pub use crate::plan::execution::function::profile::HostedExecutionGraph;
    pub use crate::plan::execution::host::HostedExecutionProfile as Hosted;
}

pub mod host {
    pub use crate::plan::execution::host::construction::ConstructionIndex;
    pub use crate::plan::execution::host::function::{
        HostCallParameter, HostCallableConstruction, HostCallableEntry, HostConstructionTypes,
        HostFunctionId, HostNeverFunctionId, HostTypeArgument, HostedFunctionMetadata,
        HostedFunctionParameters, HostedFunctionTarget,
    };
    pub use crate::plan::execution::host::native::{
        NativeConstructor, NativeConversion, NativeConversionId, NativeConversionKind,
        NativeConversions,
    };
    pub use crate::plan::execution::host::registration::{
        CallableRegistration, ConstructorSchema, CustomSchema, ExternalSchema, FieldSchema,
        RegistrationContract, RegistrationParameter, RegistrationType, SchemaType,
    };
}

pub mod type_ {
    pub use crate::plan::CustomTypePublicity;
    pub use crate::plan::execution::type_::custom::{
        ConstructorDefinition, CustomConstructorDescriptor, CustomConstructorId, CustomDefinition,
        CustomFieldDescriptor, CustomTypeDescriptor, CustomTypeId, CustomTypeTable,
        FieldDefinition, FieldRefinement,
    };
    pub use crate::plan::execution::type_::external::{ExternalTypeId, ExternalTypeTable};
    pub use crate::plan::execution::type_::function::{
        CustomFunctionType, ExternalFunctionType, FunctionFunctionType, FunctionType,
        GenericFunctionType,
    };
    pub use crate::plan::execution::type_::list::{
        BitArrayListTypeId, BoolListTypeId, CustomListTypeId, ExternalListTypeId, FloatListTypeId,
        FunctionItemTypeId, FunctionListTypeId, IntListTypeId, ListListTypeId, ListStorageTypeId,
        ListTypeId, ListTypeTable, NilListTypeId, ParameterListListTypeId, ParameterListTypeId,
        StringListTypeId, TupleItemTypeId, TupleListTypeId, UtfCodepointListTypeId,
    };
    pub use crate::plan::execution::type_::metadata::{
        FunctionMetadata, NominalTypeMetadata, TypeMetadata,
    };
    pub use crate::plan::execution::type_::shape::{
        CustomConstructorRefinement, CustomValueShape, CustomValueShapeDescriptor,
        CustomValueShapeId, FunctionShape, ValueShapeDescriptor, ValueShapeId, ValueShapeTable,
    };
    pub use crate::plan::execution::type_::value::ValueType;
    pub const fn parameter_id(index: usize) -> crate::plan::TypeParameterId {
        crate::plan::TypeParameterId(index)
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn parameter_identity_preserves_the_generated_index() {
            const PARAMETER: crate::plan::TypeParameterId = super::parameter_id(7);
            assert_eq!(PARAMETER, crate::plan::TypeParameterId(7));
            assert_eq!(super::parameter_id(std::hint::black_box(7)), PARAMETER);
        }
    }
}

pub mod function {
    pub use crate::plan::execution::function::ExecutableFunction;
    pub use crate::plan::execution::function::body::{FunctionExit, ProfiledFunctionBody};
    pub use crate::plan::execution::function::entry::FunctionEntry;
    pub use crate::plan::execution::function::function_return::body::{
        ProfiledCustomFunctionFunctionBody, ProfiledExternalFunctionFunctionBody,
        ProfiledFunctionFunctionFunctionBody, TypedFunctionBody,
    };
    pub use crate::plan::execution::function::function_return::id::{
        BitArrayFunctionFunctionId, BitArrayListFunctionFunctionId, BoolFunctionFunctionId,
        BoolListFunctionFunctionId, CustomFunctionFunctionId, CustomListFunctionFunctionId,
        ExternalFunctionFunctionId, ExternalListFunctionFunctionId, FloatFunctionFunctionId,
        FloatListFunctionFunctionId, FunctionFunctionFunctionId, FunctionListFunctionFunctionId,
        GenericFunctionFunctionId, IntFunctionFunctionId, IntListFunctionFunctionId,
        ListListFunctionFunctionId, NeverFunctionFunctionId, NilFunctionFunctionId,
        NilListFunctionFunctionId, ParameterListFunctionFunctionId,
        ParameterListListFunctionFunctionId, ProfiledFunctionFunctionId,
        ProfiledListFunctionFunctionId, StringFunctionFunctionId, StringListFunctionFunctionId,
        TupleFunctionFunctionId, TupleListFunctionFunctionId, UtfCodepointFunctionFunctionId,
        UtfCodepointListFunctionFunctionId,
    };
    pub use crate::plan::execution::function::function_return::table::FunctionFunctionTables;
    pub use crate::plan::execution::function::list_return::id::{
        BitArrayListFunctionId, BoolListFunctionId, CustomListFunctionId, ExternalListFunctionId,
        FloatListFunctionId, FunctionListFunctionId, IntListFunctionId, LibraryListFunctionId,
        ListFunctionId, ListListFunctionId, NilListFunctionId, ParameterListFunctionId,
        ParameterListListFunctionId, ProfiledListFunctionId, StringListFunctionId,
        TupleListFunctionId, UtfCodepointListFunctionId,
    };
    pub use crate::plan::execution::function::list_return::table::ListFunctionTables;
    pub use crate::plan::execution::function::parameters::FunctionTableFamily;
    pub use crate::plan::execution::function::parameters::{FunctionCatalog, FunctionContract};
    pub use crate::plan::execution::function::runtime::{
        FunctionReturnFamily, GenericCallableId, ProfiledCoreRuntimeFunctionId,
        ProfiledRuntimeFunctionId, RuntimeFunctionFunctionTarget,
    };
    pub use crate::plan::execution::function::table::FunctionTables;
    pub use crate::plan::execution::function::value_return::body::{
        ProfiledCustomFunctionBody, ProfiledExternalFunctionBody,
    };
    pub use crate::plan::execution::function::value_return::entry::ValueFunctionEntry;
    pub use crate::plan::execution::function::value_return::id::{
        BitArrayFunctionId, BoolFunctionId, CustomFunctionId, ExternalFunctionId, FloatFunctionId,
        IntFunctionId, NeverFunctionId, NilFunctionId, StringFunctionId, TupleFunctionId,
        UtfCodepointFunctionId,
    };
    pub use crate::plan::execution::function::value_return::table::ValueFunctionTables;
}

pub mod constant {
    pub use crate::plan::execution::constant::id::ConstantId;
    pub use crate::plan::execution::constant::program::ProfiledConstantProgram;
    pub use crate::plan::execution::constant::table::ProfiledConstantTable;
}

pub mod graph {
    pub use crate::plan::execution::graph::ProfiledBlockGraph;
    pub use crate::plan::execution::graph::bit_array::{Endianness, FloatBitSize, StringEncoding};
    pub use crate::plan::execution::graph::block::instruction::bit_array::{
        BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayInstruction, BitArraySegment,
    };
    pub use crate::plan::execution::graph::block::instruction::bool::BoolInstruction;
    pub use crate::plan::execution::graph::block::instruction::custom::CustomInstruction;
    pub use crate::plan::execution::graph::block::instruction::external::ExternalInstruction;
    pub use crate::plan::execution::graph::block::instruction::float::FloatInstruction;
    pub use crate::plan::execution::graph::block::instruction::function::{
        ExternalFunctionCallTarget, ExternalFunctionInstruction, ExternalFunctionInstructionKind,
        ExternalFunctionTarget, FunctionCapture, FunctionInstruction, FunctionInstructionKind,
        FunctionTarget,
    };
    pub use crate::plan::execution::graph::block::instruction::int::IntInstruction;
    pub use crate::plan::execution::graph::block::instruction::list::{
        ExternalListInstruction, ListInstruction, ParameterListInstruction, TypedListInstruction,
    };
    pub use crate::plan::execution::graph::block::instruction::nil::NilInstruction;
    pub use crate::plan::execution::graph::block::instruction::string::StringInstruction;
    pub use crate::plan::execution::graph::block::instruction::tuple::TupleInstruction;
    pub use crate::plan::execution::graph::block::instruction::utf_codepoint::UtfCodepointInstruction;
    pub use crate::plan::execution::graph::block::instruction::{
        ProfiledInstruction, ProfiledInstructionKind,
    };
    pub use crate::plan::execution::graph::block::terminator::Terminator;
    pub use crate::plan::execution::graph::block::terminator::branch::BoolBranch;
    pub use crate::plan::execution::graph::block::terminator::echo::Echo;
    pub use crate::plan::execution::graph::block::terminator::edge::{
        Edge, MatchEdge, MatchEdgeArgument,
    };
    pub use crate::plan::execution::graph::block::terminator::jump::Jump;
    pub use crate::plan::execution::graph::block::terminator::let_assert::LetAssertPanic;
    pub use crate::plan::execution::graph::block::terminator::match_::Match;
    pub use crate::plan::execution::graph::block::terminator::never::{NeverCall, NeverCallTarget};
    pub use crate::plan::execution::graph::block::terminator::pattern::bit_array::{
        BitArrayBindingPattern, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
        BitArrayPatternSizeExpr, BitArrayPatternValue, BitArrayStringPattern, Signedness,
    };
    pub use crate::plan::execution::graph::block::terminator::pattern::list::{
        MatchPatternList, MatchPatternListTail,
    };
    pub use crate::plan::execution::graph::block::terminator::pattern::{
        MatchIntBindingId, MatchPattern, MatchPatternBinding,
    };
    pub use crate::plan::execution::graph::block::terminator::source_stop::{
        SourceStop, SourceStopKind,
    };
    pub use crate::plan::execution::graph::block::terminator::switch::float::FloatSwitch;
    pub use crate::plan::execution::graph::block::terminator::switch::int::IntSwitch;
    pub use crate::plan::execution::graph::block::terminator::switch::string::StringSwitch;
    pub use crate::plan::execution::graph::block::{BlockHeader, BlockId};
    pub use crate::plan::execution::graph::exit::BlockGraphExitId;
    pub use crate::plan::execution::graph::integer::IntegerLiteral;
    pub use crate::plan::execution::graph::transfer::{FamilyTransfer, StorageFamily, Transfer};
    pub use crate::plan::execution::graph::value::function::{
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
    pub use crate::plan::execution::graph::value::list::{
        BitArrayListLocalId, BoolListLocalId, CustomListLocalId, ExternalListLocalId,
        FloatListLocalId, FunctionListLocalId, IntListLocalId, ListListLocalId, ListLocal,
        NilListLocalId, ParameterListListLocalId, ParameterListLocalId, StoredListLocal,
        StringListLocalId, TupleListLocalId, UtfCodepointListLocalId,
    };
    pub use crate::plan::execution::graph::value::local::{
        BitArrayLocalId, BoolLocalId, CustomLocal, CustomLocalId, ExternalLocal, ExternalLocalId,
        FloatLocalId, IntLocalId, NilLocalId, StringLocalId, TupleLocalId, UtfCodepointLocalId,
    };
    pub use crate::plan::execution::graph::value::param::{ParamLocal, ParamSlot};
}

pub mod program {
    pub use crate::plan::execution::{
        ExecutionModuleContext, LibraryCallable, LibraryCallableEntry, LibraryFunctionEntries,
        LibraryFunctionEntry, LibraryInputConstructions, LibraryListConstructions,
        LibraryNativeConstruction,
    };
}

pub mod source {
    pub use crate::plan::source::FunctionCallTarget;
    pub use crate::plan::{EchoSite, HostCallSite, PanicSite, SourceContext, SourceSpan};
    pub const fn module_id(index: usize) -> crate::plan::ModuleId {
        crate::plan::ModuleId::new(index)
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn module_identity_preserves_the_generated_index() {
            const MODULE: crate::plan::ModuleId = super::module_id(7);
            assert_eq!(MODULE, crate::plan::ModuleId::new(7));
            assert_eq!(super::module_id(std::hint::black_box(7)), MODULE);
        }
    }
}
