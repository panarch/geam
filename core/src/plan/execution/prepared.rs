mod admission;
pub mod data;
mod entry;
mod hosted;
pub(crate) mod rust;

pub use admission::PreparedError;
pub(crate) use admission::{AdmittedHostedModule, AdmittedModule};
pub use entry::{HostedEntryArtifact, PreparedHostedEntry};
pub use hosted::{HostedModuleArtifact, PreparedHostedModule};

use super::constant::ProfiledConstantTable;
use super::function::{
    ExecutionProfile, FunctionCatalog, FunctionTables, ProfiledRuntimeFunctionId,
};
use super::storage::Table;
use super::type_::{
    CustomTypeTable, ExternalTypeTable, FunctionMetadata, ListTypeTable, ValueShapeTable,
};
use super::{ExecutionModuleContext, ExecutionProgram, LibraryFunctionEntries};
use crate::plan::ModuleId;
use rust::{Emit, Rust};
use std::convert::Infallible;

const FORMAT_VERSION: u32 = 1;

/// A prepared plain program which can be emitted as compiler-visible Rust data.
pub struct PreparedModule {
    program: ExecutionProgram<Infallible>,
    entries: LibraryFunctionEntries<Infallible>,
    exports: Table<Export>,
}

pub struct ModuleArtifact<Profile: ExecutionProfile> {
    pub format: u32,
    pub program: ProgramTables<Profile>,
    pub entries: LibraryFunctionEntries<Profile::Graph>,
    pub exports: Table<Export>,
}

pub struct Export {
    pub name: crate::plan::Text,
    pub signature: FunctionMetadata,
    pub slot: usize,
}

impl PreparedModule {
    pub(crate) fn new(
        plan: crate::plan::LibraryModulePlan,
        first: crate::plan::LibraryEntry<Infallible>,
        remaining: Vec<crate::plan::LibraryEntry<Infallible>>,
        exports: Vec<Export>,
    ) -> Self {
        let (program, entries) = super::lowering::lower_library(plan, first, remaining);
        Self {
            program,
            entries,
            exports: exports.into(),
        }
    }

    /// Emits a Rust expression using `data` as an alias for the generated support API.
    ///
    /// This performs no application calls and requires no source files at load time.
    pub fn emit_rust(&self) -> String {
        Rust::expression(self)
    }
}

impl ModuleArtifact<Infallible> {
    pub(crate) fn admit(
        &'static self,
    ) -> Result<AdmittedModule<'static, Infallible>, PreparedError> {
        admission::plain(self)
    }
}

impl HostedModuleArtifact {
    pub(crate) fn admit<Profile: crate::HostProfile>(
        &'static self,
        providers: crate::HostProviderSet<Profile>,
    ) -> Result<AdmittedHostedModule<Profile>, PreparedError> {
        admission::hosted(self, providers)
    }
}

impl<Graph: super::function::ExecutionGraphProfile> LibraryFunctionEntries<Graph> {
    fn borrowed(&'static self) -> Self {
        Self {
            ints: Table::Static(&self.ints),
            floats: Table::Static(&self.floats),
            strings: Table::Static(&self.strings),
            bit_arrays: Table::Static(&self.bit_arrays),
            utf_codepoints: Table::Static(&self.utf_codepoints),
            customs: Table::Static(&self.customs),
            externals: Table::Static(&self.externals),
            bools: Table::Static(&self.bools),
            nils: Table::Static(&self.nils),
            tuples: Table::Static(&self.tuples),
            lists: Table::Static(&self.lists),
        }
    }
}

impl Emit for PreparedModule {
    fn emit(&self, output: &mut Rust) {
        let Self {
            program,
            entries,
            exports,
        } = self;
        ModuleEmission {
            program,
            entries,
            exports,
        }
        .emit(output);
    }
}

struct ModuleEmission<'data, Profile: ExecutionProfile> {
    program: &'data ExecutionProgram<Profile>,
    entries: &'data LibraryFunctionEntries<Profile::Graph>,
    exports: &'data Table<Export>,
}

impl<Profile: ExecutionProfile> Emit for ModuleEmission<'_, Profile>
where
    FunctionTables<Profile>: Emit,
    ProfiledConstantTable<Profile::Graph>: Emit,
    ProfiledRuntimeFunctionId<Profile::Graph>: Emit,
    LibraryFunctionEntries<Profile::Graph>: Emit,
{
    fn emit(&self, output: &mut Rust) {
        output.structure(
            "ModuleArtifact",
            &[
                ("format", &FORMAT_VERSION),
                ("program", &ProgramEmission::new(self.program)),
                ("entries", self.entries),
                ("exports", self.exports),
            ],
        );
    }
}

impl Export {
    pub(crate) fn new(
        name: ecow::EcoString,
        signature: crate::plan::FunctionType,
        slot: usize,
    ) -> Self {
        Self {
            name: name.into(),
            signature: FunctionMetadata::from_public(&signature),
            slot,
        }
    }
}

impl Emit for Export {
    fn emit(&self, output: &mut Rust) {
        let Self {
            name,
            signature,
            slot,
        } = self;
        output.structure(
            "Export",
            &[("name", name), ("signature", signature), ("slot", slot)],
        );
    }
}

pub struct ProgramTables<Profile: ExecutionProfile> {
    pub root: ModuleId,
    pub modules: Table<ExecutionModuleContext>,
    pub main: ProfiledRuntimeFunctionId<Profile::Graph>,
    pub functions: FunctionTables<Profile>,
    pub constants: ProfiledConstantTable<Profile::Graph>,
    pub function_parameters: FunctionCatalog,
    pub list_types: ListTypeTable,
    pub custom_types: CustomTypeTable,
    pub external_types: ExternalTypeTable,
    pub value_shapes: ValueShapeTable,
}

pub(crate) struct ProgramEmission<'program, Profile: ExecutionProfile> {
    program: &'program ExecutionProgram<Profile>,
}

impl<Profile: ExecutionProfile> ProgramTables<Profile> {
    fn execution(&'static self) -> ExecutionProgram<Profile> {
        use super::storage::Node;
        use std::sync::Arc;

        ExecutionProgram {
            common: Arc::new(super::ExecutionProgramCommon {
                root: self.root,
                modules: Table::Static(&self.modules),
                main: self.main.clone(),
                constants: Node::Static(&self.constants),
                function_parameters: Arc::new(FunctionCatalog {
                    families: self.function_parameters.families.clone(),
                    functions: Table::Static(&self.function_parameters.functions),
                    parameters: Table::Static(&self.function_parameters.parameters),
                }),
                list_types: Arc::new(ListTypeTable {
                    types: Table::Static(&self.list_types.types),
                    tuple_items: Table::Static(&self.list_types.tuple_items),
                    function_items: Table::Static(&self.list_types.function_items),
                }),
                custom_types: Arc::new(CustomTypeTable {
                    types: Table::Static(&self.custom_types.types),
                    definitions: Table::Static(&self.custom_types.definitions),
                }),
                external_types: Arc::new(ExternalTypeTable {
                    types: Table::Static(&self.external_types.types),
                }),
                value_shapes: Node::Static(&self.value_shapes),
            }),
            functions: Node::Static(&self.functions),
        }
    }
}

impl<'program, Profile: ExecutionProfile> ProgramEmission<'program, Profile> {
    pub(super) fn new(program: &'program ExecutionProgram<Profile>) -> Self {
        Self { program }
    }
}

impl<Profile: ExecutionProfile> Emit for ProgramEmission<'_, Profile>
where
    FunctionTables<Profile>: Emit,
    ProfiledConstantTable<Profile::Graph>: Emit,
    ProfiledRuntimeFunctionId<Profile::Graph>: Emit,
{
    fn emit(&self, output: &mut Rust) {
        let ExecutionProgram { common, functions } = self.program;
        let super::ExecutionProgramCommon {
            root,
            modules,
            main,
            constants,
            function_parameters,
            list_types,
            custom_types,
            external_types,
            value_shapes,
        } = common.as_ref();
        output.structure(
            "ProgramTables",
            &[
                ("root", root),
                ("modules", modules),
                ("main", main),
                ("functions", functions.as_ref()),
                ("constants", constants.as_ref()),
                ("function_parameters", function_parameters.as_ref()),
                ("list_types", list_types.as_ref()),
                ("custom_types", custom_types.as_ref()),
                ("external_types", external_types.as_ref()),
                ("value_shapes", value_shapes.as_ref()),
            ],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{ExecutionProgram, ProgramEmission, ProgramTables, Rust};
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::plan::execution::storage::Storage;

    #[test]
    fn borrowed_runtime_views_share_tables_but_keep_independent_owners() {
        let module =
            crate::compile_typed_module("example", "src/example.gleam", "pub fn main() { 21 * 2 }")
                .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(module).unwrap());
        let ExecutionProgram { common, functions } = plan.program;
        let common = std::sync::Arc::try_unwrap(common).ok().unwrap();
        let functions = owned_table(functions);
        let constants = owned_table(common.constants);
        let value_shapes = owned_table(common.value_shapes);
        let tables = Box::leak(Box::new(ProgramTables {
            root: common.root,
            modules: common.modules,
            main: common.main,
            functions: *functions,
            constants: *constants,
            function_parameters: std::sync::Arc::try_unwrap(common.function_parameters)
                .ok()
                .unwrap(),
            list_types: std::sync::Arc::try_unwrap(common.list_types).ok().unwrap(),
            custom_types: std::sync::Arc::try_unwrap(common.custom_types)
                .ok()
                .unwrap(),
            external_types: std::sync::Arc::try_unwrap(common.external_types)
                .ok()
                .unwrap(),
            value_shapes: *value_shapes,
        }));
        let first = crate::ExecutionPlan {
            program: tables.execution(),
        };
        let second = crate::ExecutionPlan {
            program: tables.execution(),
        };
        assert_eq!(
            crate::run_main(&first, &mut Vec::new()).unwrap(),
            crate::Value::Int(42.into())
        );
        assert_eq!(
            crate::run_main(&second, &mut Vec::new()).unwrap(),
            crate::Value::Int(42.into())
        );
        assert!(std::ptr::eq(
            first.program.functions.as_ref(),
            &tables.functions
        ));
        assert!(std::ptr::eq(
            first.program.common.constants.as_ref(),
            &tables.constants
        ));
        assert!(std::ptr::eq(
            first.program.common.value_shapes.as_ref(),
            &tables.value_shapes
        ));
        assert_eq!(
            first.program.common.modules.as_ptr(),
            tables.modules.as_ptr()
        );
        assert_eq!(
            first.program.common.function_parameters.parameters.as_ptr(),
            tables.function_parameters.parameters.as_ptr()
        );
        assert!(!first.value_metadata().shares_owner(second.value_metadata()));
    }

    fn owned_table<T>(storage: Storage<T>) -> Box<T> {
        match storage {
            Storage::Owned(table) => table,
            Storage::Static(_) => panic!("dynamic lowering owns its table"),
        }
    }

    #[test]
    #[should_panic(expected = "dynamic lowering owns its table")]
    fn owned_table_rejects_a_static_fixture() {
        assert_eq!(*owned_table(Storage::Owned(Box::new(42))), 42);
        owned_table(Storage::Static(&42));
    }

    #[test]
    fn emission_order_is_independent_of_specialization_hash_maps() {
        let source = r#"
pub type Box(a) { Box(a) }
const number = 42
fn number_box() { Box(number) }
fn text_box() { Box("text") }
pub fn main() { #(number_box(), text_box(), fn() { Box(True) }) }
"#;
        let emit = || {
            let typed =
                crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
            let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
            Rust::expression(&ProgramEmission::new(&plan.program))
        };
        let expected = emit();
        for _ in 0..20 {
            assert_eq!(emit(), expected);
        }
    }

    #[test]
    fn emits_a_complete_real_execution_plan_as_rust_data() {
        let module =
            crate::compile_typed_module("example", "src/example.gleam", "pub fn main() { 21 * 2 }")
                .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(module).unwrap());
        let source = Rust::expression(&ProgramEmission::new(&plan.program));
        assert!(
            source.starts_with("data::ProgramTables {\n    root: data::source::module_id(0),\n")
        );
        assert!(
            source.contains(
                r#"
graph::IntInstruction::Mult {
                                        left: data::graph::IntLocalId(0),
                                        right: data::graph::IntLocalId(1),
                                    }"#
                .trim_start_matches('\n')
            )
        );
        assert!(
            source.contains(
                r#"
data::Storage::Static(&[
                                            21,
                                        ])"#
                .trim_start_matches('\n')
            )
        );
        assert!(
            source.contains(
                r#"
data::Storage::Static(&[
                                            2,
                                        ])"#
                .trim_start_matches('\n')
            )
        );
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()).unwrap(),
            crate::Value::Int(42.into())
        );
    }
}
