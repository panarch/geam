use super::call::{CallError, Target};
use super::catalog::{Catalog, Function};
use super::input::{self, InputError};
use super::type_::{TypeError, Types};
use crate::plan::execution::function::{
    ExecutionGraphProfile, FunctionReturnFamily, LibraryListFunctionId,
};
use crate::plan::execution::prepared::Export;
use crate::plan::execution::type_::{FunctionMetadata, TypeMetadata};
use crate::plan::execution::{
    LibraryFunctionEntries, LibraryFunctionEntry, LibraryInputConstructions,
};
use std::collections::HashSet;

pub(super) struct Entry<'data> {
    pub family: FunctionReturnFamily,
    pub function: Function<'data>,
    pub inputs: &'data LibraryInputConstructions,
    pub callables: &'data [crate::plan::execution::LibraryCallable],
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum MainError {
    Target(CallError),
    Arguments { count: usize },
    Captures { count: usize },
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum EntryError {
    Empty,
    Main,
    MainType(TypeError),
    DuplicateName {
        index: usize,
    },
    DuplicateSlot {
        index: usize,
    },
    Unclaimed {
        family: FunctionReturnFamily,
        count: usize,
    },
    Export {
        index: usize,
        error: ExportError,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ExportError {
    ReturnFamily,
    MissingSlot { slot: usize },
    Target(CallError),
    Type(TypeError),
    Captures { count: usize },
    ArgumentCount { declared: usize, actual: usize },
    ArgumentType { index: usize },
    NarrowedInput { index: usize },
    ReturnType,
    Input(InputError),
    Callable(super::callables::CallableError),
}

pub(super) fn main<Graph: ExecutionGraphProfile>(
    main: &crate::plan::execution::function::ProfiledRuntimeFunctionId<Graph>,
    catalog: &Catalog<'_>,
    types: &Types<'_>,
) -> Result<(), MainError>
where
    Graph::ExternalFunctionId: Target,
    Graph::ExternalListFunctionId: Target,
    Graph::RuntimeFunctionFunctionId: Target,
{
    let function = main.resolve(catalog, types).map_err(MainError::Target)?;
    if !function.parameters.is_empty() {
        return Err(MainError::Arguments {
            count: function.parameters.len(),
        });
    }
    if !function.captures.is_empty() {
        return Err(MainError::Captures {
            count: function.captures.len(),
        });
    }
    Ok(())
}

pub(super) fn all<'data, Graph: ExecutionGraphProfile>(
    main: &crate::plan::execution::function::ProfiledRuntimeFunctionId<Graph>,
    entries: &'data LibraryFunctionEntries<Graph>,
    exports: &[Export],
    catalog: &Catalog<'data>,
    types: &Types<'data>,
) -> Result<
    (
        Vec<&'data LibraryInputConstructions>,
        Vec<&'data [crate::plan::execution::LibraryCallable]>,
    ),
    EntryError,
>
where
    Graph::ExternalFunctionId: Target,
    Graph::ExternalListFunctionId: Target,
    Graph::RuntimeFunctionFunctionId: Target,
    Graph::InvocableFunctionFunctionId: Target,
{
    let Some(first) = exports.first() else {
        return Err(EntryError::Empty);
    };
    let mut names = HashSet::new();
    let mut slots = HashSet::new();
    let mut inputs = Vec::with_capacity(exports.len());
    let mut callables = Vec::with_capacity(exports.len());
    for (index, export) in exports.iter().enumerate() {
        if !names.insert(export.name.as_str()) {
            return Err(EntryError::DuplicateName { index });
        }
        let checked = resolve(entries, export, catalog, types)
            .map_err(|error| EntryError::Export { index, error })?;
        admit_export(index, export, &checked, types, &mut slots)?;
        inputs.push(checked.inputs);
        callables.push(checked.callables);
    }
    if !main_matches(main, entries, first, types).map_err(EntryError::MainType)? {
        return Err(EntryError::Main);
    }
    let LibraryFunctionEntries {
        ints,
        floats,
        strings,
        bit_arrays,
        utf_codepoints,
        customs,
        externals,
        bools,
        nils,
        tuples,
        lists,
        functions,
    } = entries;
    for (family, length) in [
        (FunctionReturnFamily::Int, ints.len()),
        (FunctionReturnFamily::Float, floats.len()),
        (FunctionReturnFamily::String, strings.len()),
        (FunctionReturnFamily::BitArray, bit_arrays.len()),
        (FunctionReturnFamily::UtfCodepoint, utf_codepoints.len()),
        (FunctionReturnFamily::Custom, customs.len()),
        (FunctionReturnFamily::External, externals.len()),
        (FunctionReturnFamily::Bool, bools.len()),
        (FunctionReturnFamily::Nil, nils.len()),
        (FunctionReturnFamily::Tuple, tuples.len()),
        (FunctionReturnFamily::List, lists.len()),
        (FunctionReturnFamily::Function, functions.len()),
    ] {
        let claimed = slots.iter().filter(|(kind, _)| *kind == family).count();
        if claimed != length {
            return Err(EntryError::Unclaimed {
                family,
                count: length - claimed,
            });
        }
    }
    Ok((inputs, callables))
}

// Target resolution depends on the graph profile. The resolved export's
// signature, input views and callable permissions share one admission contract.
fn admit_export(
    index: usize,
    export: &Export,
    checked: &Entry<'_>,
    types: &Types<'_>,
    slots: &mut HashSet<(FunctionReturnFamily, usize)>,
) -> Result<(), EntryError> {
    signature(&export.signature, &checked.function, types)
        .map_err(|error| EntryError::Export { index, error })?;
    input::admit(checked.inputs, types).map_err(|error| EntryError::Export {
        index,
        error: ExportError::Input(error),
    })?;
    if !slots.insert((checked.family, export.slot)) {
        return Err(EntryError::DuplicateSlot { index });
    }
    super::callables::admit(checked.callables, types).map_err(|error| EntryError::Export {
        index,
        error: ExportError::Callable(error),
    })
}

fn main_matches<Graph: ExecutionGraphProfile>(
    main: &crate::plan::execution::function::ProfiledRuntimeFunctionId<Graph>,
    entries: &LibraryFunctionEntries<Graph>,
    export: &Export,
    types: &Types<'_>,
) -> Result<bool, TypeError> {
    use crate::plan::execution::function::{
        ProfiledCoreRuntimeFunctionId as Core, ProfiledRuntimeFunctionId as Runtime,
    };
    Ok(match (export.signature.return_.as_ref(), main) {
        (TypeMetadata::Int, Runtime::Core(Core::Int(id))) => entries
            .ints
            .get(export.slot)
            .is_some_and(|entry| &entry.function == id),
        (TypeMetadata::Float, Runtime::Core(Core::Float(id))) => entries
            .floats
            .get(export.slot)
            .is_some_and(|entry| &entry.function == id),
        (TypeMetadata::String, Runtime::Core(Core::String(id))) => entries
            .strings
            .get(export.slot)
            .is_some_and(|entry| &entry.function == id),
        (TypeMetadata::BitArray, Runtime::Core(Core::BitArray(id))) => entries
            .bit_arrays
            .get(export.slot)
            .is_some_and(|entry| &entry.function == id),
        (TypeMetadata::UtfCodepoint, Runtime::Core(Core::UtfCodepoint(id))) => entries
            .utf_codepoints
            .get(export.slot)
            .is_some_and(|entry| &entry.function == id),
        (TypeMetadata::Custom(_), Runtime::Core(Core::Custom(id))) => entries
            .customs
            .get(export.slot)
            .is_some_and(|entry| &entry.function == id),
        (TypeMetadata::Bool, Runtime::Core(Core::Bool(id))) => entries
            .bools
            .get(export.slot)
            .is_some_and(|entry| &entry.function == id),
        (TypeMetadata::Nil, Runtime::Core(Core::Nil(id))) => entries
            .nils
            .get(export.slot)
            .is_some_and(|entry| &entry.function == id),
        (TypeMetadata::External(_), Runtime::External(id)) => entries
            .externals
            .get(export.slot)
            .is_some_and(|entry| &entry.function == id),
        (TypeMetadata::List(_), Runtime::Core(Core::List(id))) => entries
            .lists
            .get(export.slot)
            .is_some_and(|entry| &entry.function.profiled_runtime_id() == id),
        (TypeMetadata::Tuple(_), Runtime::Core(Core::Tuple { id, return_type })) => {
            entries
                .tuples
                .get(export.slot)
                .is_some_and(|entry| &entry.function == id)
                && types.matches(
                    &export.signature.return_,
                    &crate::plan::execution::type_::ValueType::Tuple(return_type.clone()),
                )?
        }
        (TypeMetadata::Function(_), Runtime::Core(Core::Function { id, return_type })) => {
            entries.functions.get(export.slot).is_some_and(|entry| {
                &Graph::function_value_target(&entry.function.function) == id
                    && &entry.function.type_ == return_type
            })
        }
        _ => false,
    })
}

pub(super) fn resolve<'data, Graph: ExecutionGraphProfile>(
    entries: &'data LibraryFunctionEntries<Graph>,
    export: &Export,
    catalog: &Catalog<'data>,
    types: &Types<'data>,
) -> Result<Entry<'data>, ExportError>
where
    Graph::ExternalFunctionId: Target,
    Graph::ExternalListFunctionId: Target,
    Graph::RuntimeFunctionFunctionId: Target,
    Graph::InvocableFunctionFunctionId: Target,
{
    let slot = export.slot;
    match export.signature.return_.as_ref() {
        TypeMetadata::Int => entry(
            &entries.ints,
            slot,
            FunctionReturnFamily::Int,
            catalog,
            types,
        ),
        TypeMetadata::Float => entry(
            &entries.floats,
            slot,
            FunctionReturnFamily::Float,
            catalog,
            types,
        ),
        TypeMetadata::String => entry(
            &entries.strings,
            slot,
            FunctionReturnFamily::String,
            catalog,
            types,
        ),
        TypeMetadata::BitArray => entry(
            &entries.bit_arrays,
            slot,
            FunctionReturnFamily::BitArray,
            catalog,
            types,
        ),
        TypeMetadata::UtfCodepoint => entry(
            &entries.utf_codepoints,
            slot,
            FunctionReturnFamily::UtfCodepoint,
            catalog,
            types,
        ),
        TypeMetadata::Custom(_) => entry(
            &entries.customs,
            slot,
            FunctionReturnFamily::Custom,
            catalog,
            types,
        ),
        TypeMetadata::External(_) => entry(
            &entries.externals,
            slot,
            FunctionReturnFamily::External,
            catalog,
            types,
        ),
        TypeMetadata::Bool => entry(
            &entries.bools,
            slot,
            FunctionReturnFamily::Bool,
            catalog,
            types,
        ),
        TypeMetadata::Nil => entry(
            &entries.nils,
            slot,
            FunctionReturnFamily::Nil,
            catalog,
            types,
        ),
        TypeMetadata::Tuple(_) => entry(
            &entries.tuples,
            slot,
            FunctionReturnFamily::Tuple,
            catalog,
            types,
        ),
        TypeMetadata::List(_) => entry(
            &entries.lists,
            slot,
            FunctionReturnFamily::List,
            catalog,
            types,
        ),
        TypeMetadata::Function(_) => entry(
            &entries.functions,
            slot,
            FunctionReturnFamily::Function,
            catalog,
            types,
        ),
        TypeMetadata::Parameter(_) => Err(ExportError::ReturnFamily),
    }
}

fn entry<'data, Id: Target>(
    entries: &'data [LibraryFunctionEntry<Id>],
    slot: usize,
    family: FunctionReturnFamily,
    catalog: &Catalog<'data>,
    types: &Types<'data>,
) -> Result<Entry<'data>, ExportError> {
    let entry = entries.get(slot).ok_or(ExportError::MissingSlot { slot })?;
    Ok(Entry {
        family,
        function: entry
            .function
            .resolve(catalog, types)
            .map_err(ExportError::Target)?,
        inputs: &entry.inputs,
        callables: &entry.callables,
    })
}

fn signature(
    signature: &FunctionMetadata,
    function: &Function<'_>,
    types: &Types<'_>,
) -> Result<(), ExportError> {
    if !function.captures.is_empty() {
        return Err(ExportError::Captures {
            count: function.captures.len(),
        });
    }
    if signature.arguments.len() != function.parameter_shapes.len() {
        return Err(ExportError::ArgumentCount {
            declared: signature.arguments.len(),
            actual: function.parameter_shapes.len(),
        });
    }
    for (index, (metadata, shape)) in signature
        .arguments
        .iter()
        .zip(function.parameter_shapes)
        .enumerate()
    {
        let type_ = &types.shape_types()[shape.index()];
        if !types.matches(metadata, type_).map_err(ExportError::Type)? {
            return Err(ExportError::ArgumentType { index });
        }
        if !types.is_nominal_shape(*shape) {
            return Err(ExportError::NarrowedInput { index });
        }
    }
    if !types
        .matches(&signature.return_, function.return_type)
        .map_err(ExportError::Type)?
    {
        return Err(ExportError::ReturnType);
    }
    Ok(())
}

impl<Graph: ExecutionGraphProfile> Target for LibraryListFunctionId<Graph>
where
    Graph::ExternalListFunctionId: Target,
{
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        self.profiled_runtime_id().resolve(catalog, types)
    }
}

impl<Graph: ExecutionGraphProfile> Target for crate::plan::execution::LibraryCallableEntry<Graph>
where
    Graph::InvocableFunctionFunctionId: Target,
{
    fn resolve<'data>(
        &self,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        types.function_type(&self.type_).map_err(CallError::Type)?;
        let function = self.function.resolve(catalog, types)?;
        match function.return_type {
            crate::plan::execution::type_::ValueType::Function(type_) if *type_ == self.type_ => {
                Ok(function)
            }
            _ => Err(CallError::ReturnType),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Catalog, EntryError, Export, ExportError, FunctionMetadata, FunctionReturnFamily,
        TypeError, TypeMetadata, Types, all, resolve,
    };
    use crate::embedding::{BigInt, FunctionDeclaration, List, ModuleBuilder};
    use crate::plan::execution::prepared::PreparedModule;
    use crate::plan::{FunctionType, ValueType};

    #[test]
    fn prepared_callable_roots_validate_main_identity_and_nested_view_metadata() {
        use super::super::tests::owned_mut;
        use crate::plan::execution::function::{
            ProfiledCoreRuntimeFunctionId as Core, ProfiledRuntimeFunctionId as Runtime,
        };
        use crate::plan::execution::type_::{CustomTypeId, ValueType as RuntimeType};
        use crate::plan::{LibraryCallableSignature, LibraryEntry, LibraryValueType};
        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [crate::PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [crate::ModuleSource::new(
                    "main",
                    "main.gleam",
                    "pub fn main() { fn(value: Int) { value + 1 } }",
                )],
            )],
            crate::HostProviderSet::<crate::StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        let plan = crate::planner::plan_host_library_program(typed).unwrap();
        let main = plan
            .functions()
            .iter()
            .find(|function| function.name() == "main")
            .unwrap()
            .signature()
            .id();
        let callback = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let declaration = LibraryCallableSignature {
            type_: callback.clone(),
            input_variants: vec![],
            input_lists: vec![],
            callables: vec![],
        };
        let (program, _, mut entries, _) = crate::plan::execution::lowering::lower_hosted_library(
            plan,
            LibraryEntry::new(
                main,
                LibraryValueType::Function(callback.clone()),
                vec![],
                vec![],
            )
            .with_callables(vec![declaration]),
            vec![],
        )
        .unwrap();
        let common = &program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let catalog =
            Catalog::admit(&common.function_parameters, &program.functions, &types).unwrap();
        let export = Export::new(
            "main".into(),
            FunctionType::new(vec![], ValueType::Function(Box::new(callback))),
            0,
        );
        assert_eq!(
            all(
                &common.main,
                &entries,
                std::slice::from_ref(&export),
                &catalog,
                &types
            )
            .map(|_| ()),
            Ok(())
        );
        let wrong_main = Runtime::Core(Core::Function {
            id: <crate::plan::execution::function::HostedExecutionGraph as crate::plan::execution::function::ExecutionGraphProfile>::function_value_target(
                &entries.functions[0].function.function,
            ),
            return_type: crate::plan::execution::type_::FunctionType::new(
                vec![RuntimeType::Bool],
                RuntimeType::Int,
            ),
        });
        assert_eq!(
            super::main_matches(&wrong_main, &entries, &export, &types),
            Ok(false)
        );
        let callables = &mut owned_mut(&mut entries.functions)[0].callables;
        assert_eq!(callables.len(), 1);
        owned_mut(callables)[0].type_.arguments =
            vec![RuntimeType::Custom(CustomTypeId(999))].into();
        assert_eq!(
            all(&common.main, &entries, &[export], &catalog, &types).err(),
            Some(EntryError::Export {
                index: 0,
                error: ExportError::Callable(super::super::callables::CallableError::Type(
                    TypeError::MissingCustom { index: 999 }
                )),
            })
        );
    }

    #[test]
    fn callable_entry_links_require_the_exact_returned_signature_and_a_valid_target() {
        use super::super::call::{CallError, Target};
        use super::super::catalog::CatalogError;
        use crate::plan::execution::LibraryCallableEntry;
        use crate::plan::execution::function::{
            FunctionTableFamily, IntFunctionFunctionId, ProfiledFunctionFunctionId,
            RuntimeFunctionFunctionTarget,
        };
        use crate::plan::execution::type_::{CustomTypeId, FunctionType, ValueType};

        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [crate::PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [crate::ModuleSource::new(
                    "main",
                    "src/main.gleam",
                    "pub fn main() { fn(value: Int) { value + 1 } }",
                )],
            )],
            crate::HostProviderSet::<crate::StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        let (program, _) = crate::plan::execution::lowering::lower_hosted(
            crate::plan_host_program(typed).unwrap(),
        )
        .unwrap();
        let common = &program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let catalog =
            Catalog::admit(&common.function_parameters, &program.functions, &types).unwrap();
        let entry: LibraryCallableEntry = LibraryCallableEntry {
            function: RuntimeFunctionFunctionTarget::Core(ProfiledFunctionFunctionId::Int(
                IntFunctionFunctionId(0),
            )),
            type_: FunctionType::new(vec![ValueType::Int], ValueType::Int),
        };
        let function = entry.resolve(&catalog, &types).unwrap();
        assert_eq!(function.family, FunctionTableFamily::IntFunction);
        assert_eq!(function.parameters.len(), 0);
        assert_eq!(
            function.return_type,
            &ValueType::Function(entry.type_.clone())
        );
        let mut wrong_type = entry.clone();
        wrong_type.type_ =
            FunctionType::new(vec![ValueType::Int], ValueType::Custom(CustomTypeId(999)));
        assert_eq!(
            wrong_type.resolve(&catalog, &types).err(),
            Some(CallError::Type(TypeError::MissingCustom { index: 999 }))
        );
        let mut missing = entry.clone();
        missing.function = RuntimeFunctionFunctionTarget::Core(ProfiledFunctionFunctionId::Int(
            IntFunctionFunctionId(999),
        ));
        assert_eq!(
            missing.resolve(&catalog, &types).err(),
            Some(CallError::Catalog(CatalogError::MissingFunction {
                family: FunctionTableFamily::IntFunction,
                index: 999
            }))
        );
        let mut different = entry;
        different.type_ = FunctionType::new(vec![ValueType::Bool], ValueType::Int);
        assert_eq!(
            different.resolve(&catalog, &types).err(),
            Some(CallError::ReturnType)
        );
    }

    #[test]
    fn standalone_main_uses_the_actual_callable_target_without_library_export_restrictions() {
        use super::MainError;
        use crate::plan::execution::function::FunctionTableFamily;
        use crate::plan::execution::function::{
            IntFunctionId, ProfiledCoreRuntimeFunctionId as Core,
            ProfiledRuntimeFunctionId as Runtime,
        };
        use crate::plan::execution::prepared::admission::call::CallError;
        use crate::plan::execution::prepared::admission::catalog::CatalogError;

        for expression in [
            "42",
            "1.5",
            "\"text\"",
            "True",
            "Nil",
            "<<65>>",
            "{ let assert <<point:utf8_codepoint>> = <<65>> point }",
            "#(42, True)",
            "[42]",
            "Some(42)",
            "fn(value: Int) { value + 1 }",
            "fn(value) { value }",
            "panic",
        ] {
            let source =
                format!("pub type Option(a) {{ Some(a) None }} pub fn main() {{ {expression} }}");
            let typed = crate::compile_typed_module("main", "src/main.gleam", &source).unwrap();
            let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
            let common = &plan.program.common;
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes,
            )
            .unwrap();
            let catalog =
                Catalog::admit(&common.function_parameters, &plan.program.functions, &types)
                    .unwrap();
            assert_eq!(super::main(&common.main, &catalog, &types), Ok(()));
            assert_eq!(
                super::main(
                    &Runtime::<std::convert::Infallible>::Core(Core::Int(IntFunctionId(999))),
                    &catalog,
                    &types
                ),
                Err(MainError::Target(CallError::Catalog(
                    CatalogError::MissingFunction {
                        family: FunctionTableFamily::Int,
                        index: 999,
                    }
                )))
            );
        }
    }

    #[test]
    fn standalone_main_rejects_callable_arguments_and_retained_captures() {
        use super::MainError;
        use crate::plan::execution::function::{
            IntFunctionId, ProfiledCoreRuntimeFunctionId as Core,
            ProfiledRuntimeFunctionId as Runtime,
        };

        for (source, expected) in [
            (
                "pub fn main() { fn(value: Int) { value + 1 } }",
                MainError::Arguments { count: 1 },
            ),
            (
                "pub fn main() { let value = 42 fn() { value } }",
                MainError::Captures { count: 1 },
            ),
        ] {
            let typed = crate::compile_typed_module("main", "src/main.gleam", source).unwrap();
            let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
            let common = &plan.program.common;
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes,
            )
            .unwrap();
            let catalog =
                Catalog::admit(&common.function_parameters, &plan.program.functions, &types)
                    .unwrap();
            assert_eq!(super::main(&common.main, &catalog, &types), Ok(()));
            assert_eq!(
                super::main(
                    &Runtime::<std::convert::Infallible>::Core(Core::Int(IntFunctionId(0))),
                    &catalog,
                    &types
                ),
                Err(expected),
            );
        }
    }

    #[test]
    fn every_plain_export_family_can_be_the_prepared_main_entry() {
        use crate::embedding::{BitArrayValue, StringValue};

        type Prepare = fn(ModuleBuilder) -> PreparedModule;
        let cases: [(&str, Prepare, FunctionReturnFamily); 10] = [
            (
                "pub fn main() { 42 }",
                |builder| {
                    builder
                        .function(FunctionDeclaration::<(), BigInt>::new("main"))
                        .unwrap()
                        .0
                        .prepare()
                },
                FunctionReturnFamily::Int,
            ),
            (
                "pub fn main() { 1.5 }",
                |builder| {
                    builder
                        .function(FunctionDeclaration::<(), f64>::new("main"))
                        .unwrap()
                        .0
                        .prepare()
                },
                FunctionReturnFamily::Float,
            ),
            (
                "pub fn main() { \"hello\" }",
                |builder| {
                    builder
                        .function(FunctionDeclaration::<(), StringValue>::new("main"))
                        .unwrap()
                        .0
                        .prepare()
                },
                FunctionReturnFamily::String,
            ),
            (
                "pub fn main() { <<42>> }",
                |builder| {
                    builder
                        .function(FunctionDeclaration::<(), BitArrayValue>::new("main"))
                        .unwrap()
                        .0
                        .prepare()
                },
                FunctionReturnFamily::BitArray,
            ),
            (
                "pub fn main() { let assert <<point:utf8_codepoint>> = <<120>> point }",
                |builder| {
                    builder
                        .function(FunctionDeclaration::<(), char>::new("main"))
                        .unwrap()
                        .0
                        .prepare()
                },
                FunctionReturnFamily::UtfCodepoint,
            ),
            (
                "pub fn main() -> Result(Int, String) { Ok(42) }",
                |builder| {
                    builder
                        .function(FunctionDeclaration::<(), Result<BigInt, StringValue>>::new(
                            "main",
                        ))
                        .unwrap()
                        .0
                        .prepare()
                },
                FunctionReturnFamily::Custom,
            ),
            (
                "pub fn main() { True }",
                |builder| {
                    builder
                        .function(FunctionDeclaration::<(), bool>::new("main"))
                        .unwrap()
                        .0
                        .prepare()
                },
                FunctionReturnFamily::Bool,
            ),
            (
                "pub fn main() { Nil }",
                |builder| {
                    builder
                        .function(FunctionDeclaration::<(), ()>::new("main"))
                        .unwrap()
                        .0
                        .prepare()
                },
                FunctionReturnFamily::Nil,
            ),
            (
                "pub fn main() { #(42, True) }",
                |builder| {
                    builder
                        .function(FunctionDeclaration::<(), (BigInt, bool)>::new("main"))
                        .unwrap()
                        .0
                        .prepare()
                },
                FunctionReturnFamily::Tuple,
            ),
            (
                "pub fn main() { [42] }",
                |builder| {
                    builder
                        .function(FunctionDeclaration::<(), List<BigInt>>::new("main"))
                        .unwrap()
                        .0
                        .prepare()
                },
                FunctionReturnFamily::List,
            ),
        ];
        for (source, prepare, expected_family) in cases {
            let typed =
                crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
            let prepared = prepare(ModuleBuilder::new(typed).unwrap());
            let common = &prepared.program.common;
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes,
            )
            .unwrap();
            let catalog = Catalog::admit(
                &common.function_parameters,
                &prepared.program.functions,
                &types,
            )
            .unwrap();
            let (inputs, callables) = all(
                &common.main,
                &prepared.entries,
                &prepared.exports,
                &catalog,
                &types,
            )
            .unwrap();
            assert_eq!(inputs.len(), 1);
            assert_eq!(callables.len(), 1);
            assert!(callables[0].is_empty());
            let resolved =
                resolve(&prepared.entries, &prepared.exports[0], &catalog, &types).unwrap();
            assert_eq!(resolved.family, expected_family);
            assert!(std::ptr::eq(inputs[0], resolved.inputs));
            assert_eq!(resolved.function.parameters.len(), 0);
            assert_eq!(
                super::main_matches(
                    &common.main,
                    &prepared.entries,
                    &prepared.exports[0],
                    &types
                ),
                Ok(true)
            );
        }
    }

    #[test]
    fn hosted_scalar_and_container_entries_keep_their_native_requirements() {
        use crate::plan::execution::function::{
            BitArrayFunctionId, BoolFunctionId, CustomFunctionId, FloatFunctionId, IntFunctionId,
            IntListFunctionId, LibraryListFunctionId, NilFunctionId, StringFunctionId,
            TupleFunctionId, UtfCodepointFunctionId,
        };
        use crate::plan::execution::prepared::admission::tests::lowered_native;
        use crate::plan::execution::storage::{Node, Table};
        use crate::plan::execution::{
            LibraryFunctionEntries, LibraryFunctionEntry, LibraryInputConstructions,
            LibraryListConstructions,
        };

        type Select = fn(&mut LibraryFunctionEntries, LibraryInputConstructions);
        let cases: [(&str, FunctionReturnFamily, TypeMetadata, Select); 10] = [
            (
                "42",
                FunctionReturnFamily::Int,
                TypeMetadata::Int,
                |entries, inputs| {
                    entries.ints = vec![LibraryFunctionEntry {
                        callables: Vec::new().into(),
                        function: IntFunctionId(0),
                        inputs,
                    }]
                    .into();
                },
            ),
            (
                "1.5",
                FunctionReturnFamily::Float,
                TypeMetadata::Float,
                |entries, inputs| {
                    entries.floats = vec![LibraryFunctionEntry {
                        callables: Vec::new().into(),
                        function: FloatFunctionId(0),
                        inputs,
                    }]
                    .into();
                },
            ),
            (
                "\"hello\"",
                FunctionReturnFamily::String,
                TypeMetadata::String,
                |entries, inputs| {
                    entries.strings = vec![LibraryFunctionEntry {
                        callables: Vec::new().into(),
                        function: StringFunctionId(0),
                        inputs,
                    }]
                    .into();
                },
            ),
            (
                "<<42>>",
                FunctionReturnFamily::BitArray,
                TypeMetadata::BitArray,
                |entries, inputs| {
                    entries.bit_arrays = vec![LibraryFunctionEntry {
                        callables: Vec::new().into(),
                        function: BitArrayFunctionId(0),
                        inputs,
                    }]
                    .into();
                },
            ),
            (
                "{ let assert <<point:utf8_codepoint>> = <<120>> point }",
                FunctionReturnFamily::UtfCodepoint,
                TypeMetadata::UtfCodepoint,
                |entries, inputs| {
                    entries.utf_codepoints = vec![LibraryFunctionEntry {
                        callables: Vec::new().into(),
                        function: UtfCodepointFunctionId(0),
                        inputs,
                    }]
                    .into();
                },
            ),
            (
                "Box(42)",
                FunctionReturnFamily::Custom,
                TypeMetadata::Custom(crate::plan::execution::type_::NominalTypeMetadata {
                    package: "app".into(),
                    module: "main".into(),
                    name: "Box".into(),
                    arguments: Table::Static(&[]),
                }),
                |entries, inputs| {
                    entries.customs = vec![LibraryFunctionEntry {
                        callables: Vec::new().into(),
                        function: CustomFunctionId {
                            index: 0,
                            return_shape: crate::plan::execution::type_::CustomValueShape {
                                type_id: crate::plan::execution::type_::CustomTypeId(0),
                                shape_id: crate::plan::execution::type_::CustomValueShapeId(0),
                            },
                        },
                        inputs,
                    }]
                    .into();
                },
            ),
            (
                "key() == key()",
                FunctionReturnFamily::Bool,
                TypeMetadata::Bool,
                |entries, inputs| {
                    entries.bools = vec![LibraryFunctionEntry {
                        callables: Vec::new().into(),
                        function: BoolFunctionId(0),
                        inputs,
                    }]
                    .into();
                },
            ),
            (
                "Nil",
                FunctionReturnFamily::Nil,
                TypeMetadata::Nil,
                |entries, inputs| {
                    entries.nils = vec![LibraryFunctionEntry {
                        callables: Vec::new().into(),
                        function: NilFunctionId(0),
                        inputs,
                    }]
                    .into();
                },
            ),
            (
                "#(key() == key(), 42)",
                FunctionReturnFamily::Tuple,
                TypeMetadata::Tuple(vec![TypeMetadata::Bool, TypeMetadata::Int].into()),
                |entries, inputs| {
                    entries.tuples = vec![LibraryFunctionEntry {
                        callables: Vec::new().into(),
                        function: TupleFunctionId(0),
                        inputs,
                    }]
                    .into();
                },
            ),
            (
                "[42]",
                FunctionReturnFamily::List,
                TypeMetadata::List(Node::Static(&TypeMetadata::Int)),
                |entries, inputs| {
                    entries.lists = vec![LibraryFunctionEntry {
                        callables: Vec::new().into(),
                        function: LibraryListFunctionId::Int(IntListFunctionId::new(
                            0,
                            crate::plan::execution::type_::IntListTypeId {
                                list_type: crate::plan::execution::type_::ListTypeId(0),
                            },
                        )),
                        inputs,
                    }]
                    .into();
                },
            ),
        ];
        for (expression, expected_family, signature, select) in cases {
            let source = format!(
                "pub type Key\npub type Box {{ Box(Int) }}\n@external(erlang, \"native\", \"key\") fn key() -> Key\npub fn main() {{ echo key() {expression} }}"
            );
            let (program, _, _) = lowered_native(&source);
            let common = &program.common;
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes,
            )
            .unwrap();
            let catalog =
                Catalog::admit(&common.function_parameters, &program.functions, &types).unwrap();
            let inputs = LibraryInputConstructions {
                variants: Table::Static(&[]),
                lists: LibraryListConstructions {
                    functions: Vec::new().into(),
                    ints: Table::Static(&[]),
                    floats: Table::Static(&[]),
                    strings: Table::Static(&[]),
                    bit_arrays: Table::Static(&[]),
                    utf_codepoints: Table::Static(&[]),
                    customs: Table::Static(&[]),
                    externals: Table::Static(&[]),
                    bools: Table::Static(&[]),
                    nils: Table::Static(&[]),
                    tuples: Table::Static(&[]),
                    lists: Table::Static(&[]),
                },
            };
            let mut entries: LibraryFunctionEntries = LibraryFunctionEntries {
                functions: Vec::new().into(),
                ints: Table::Static(&[]),
                floats: Table::Static(&[]),
                strings: Table::Static(&[]),
                bit_arrays: Table::Static(&[]),
                utf_codepoints: Table::Static(&[]),
                customs: Table::Static(&[]),
                externals: Table::Static(&[]),
                bools: Table::Static(&[]),
                nils: Table::Static(&[]),
                tuples: Table::Static(&[]),
                lists: Table::Static(&[]),
            };
            select(&mut entries, inputs);
            let export = Export {
                name: "main".into(),
                slot: 0,
                signature: FunctionMetadata {
                    arguments: Table::Static(&[]),
                    return_: Node::Owned(Box::new(signature)),
                },
            };
            let (inputs, callables) = all(
                &common.main,
                &entries,
                std::slice::from_ref(&export),
                &catalog,
                &types,
            )
            .unwrap();
            assert_eq!(inputs.len(), 1);
            assert_eq!(callables.len(), 1);
            assert!(callables[0].is_empty());
            let entry = resolve(&entries, &export, &catalog, &types).unwrap();
            assert_eq!(entry.family, expected_family);
            assert!(std::ptr::eq(inputs[0], entry.inputs));
            let mut invalid_entries = entries.clone();
            invalid_entries.ints = vec![LibraryFunctionEntry {
                callables: Vec::new().into(),
                function: IntFunctionId(999),
                inputs: entry.inputs.clone(),
            }]
            .into();
            let invalid_int = Export {
                name: "missing".into(),
                slot: 0,
                signature: FunctionMetadata {
                    arguments: Table::Static(&[]),
                    return_: Node::Static(&TypeMetadata::Int),
                },
            };
            assert_eq!(
                resolve(&invalid_entries, &invalid_int, &catalog, &types).err(),
                Some(ExportError::Target(crate::plan::execution::prepared::admission::call::CallError::Catalog(
                    crate::plan::execution::prepared::admission::catalog::CatalogError::MissingFunction {
                        family: crate::plan::execution::function::FunctionTableFamily::Int, index: 999,
                    },
                )))
            );
            if expected_family == FunctionReturnFamily::Tuple {
                let main = crate::plan::execution::function::ProfiledRuntimeFunctionId::Core(
                    crate::plan::execution::function::ProfiledCoreRuntimeFunctionId::Tuple {
                        id: entries.tuples[0].function,
                        return_type: vec![
                            crate::plan::execution::type_::ValueType::List(
                                crate::plan::execution::type_::ListTypeId(999),
                            ),
                            crate::plan::execution::type_::ValueType::Bool,
                        ]
                        .into(),
                    },
                );
                assert_eq!(
                    super::main_matches(&main, &entries, &export, &types),
                    Err(TypeError::MissingList { index: 999 })
                );
            }
            assert_eq!(
                super::main_matches(
                    &crate::plan::execution::function::ProfiledRuntimeFunctionId::Core(
                        crate::plan::execution::function::ProfiledCoreRuntimeFunctionId::Never(
                            crate::plan::execution::function::NeverFunctionId(0)
                        )
                    ),
                    &entries,
                    &export,
                    &types,
                ),
                Ok(false)
            );
            for (signature, expected) in [
                (
                    TypeMetadata::Parameter(crate::plan::TypeParameterId(0)),
                    ExportError::ReturnFamily,
                ),
                (
                    TypeMetadata::Function(FunctionMetadata {
                        arguments: Table::Static(&[]),
                        return_: Node::Static(&TypeMetadata::Int),
                    }),
                    ExportError::MissingSlot { slot: 0 },
                ),
            ] {
                let invalid = Export {
                    name: "main".into(),
                    slot: 0,
                    signature: FunctionMetadata {
                        arguments: Table::Static(&[]),
                        return_: Node::Owned(Box::new(signature)),
                    },
                };
                assert_eq!(
                    resolve(&entries, &invalid, &catalog, &types).err(),
                    Some(expected)
                );
            }
        }
    }

    #[test]
    fn rejects_missing_function_slots_and_unbound_parameter_exports() {
        use crate::plan::execution::storage::{Node, Table};
        let prepared = prepare();
        let common = &prepared.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let catalog = Catalog::admit(
            &common.function_parameters,
            &prepared.program.functions,
            &types,
        )
        .unwrap();
        for (return_, expected) in [
            (
                TypeMetadata::Parameter(crate::plan::TypeParameterId(0)),
                ExportError::ReturnFamily,
            ),
            (
                TypeMetadata::Function(FunctionMetadata {
                    arguments: Table::Static(&[]),
                    return_: Node::Static(&TypeMetadata::Int),
                }),
                ExportError::MissingSlot { slot: 0 },
            ),
        ] {
            let export = Export {
                name: "unsupported".into(),
                slot: 0,
                signature: FunctionMetadata {
                    arguments: Table::Static(&[]),
                    return_: Node::Owned(Box::new(return_)),
                },
            };
            assert_eq!(
                resolve(&prepared.entries, &export, &catalog, &types).err(),
                Some(expected)
            );
            assert_eq!(
                super::main_matches(&common.main, &prepared.entries, &export, &types),
                Ok(false)
            );
        }
    }

    #[test]
    fn native_external_entries_preserve_the_exact_target_and_empty_input_contract() {
        use crate::plan::execution::function::{
            ExternalFunctionId, FunctionTableFamily, ProfiledRuntimeFunctionId,
        };
        use crate::plan::execution::prepared::admission::{
            call::CallError, catalog::CatalogError, tests::lowered_native,
        };
        use crate::plan::execution::storage::{Node, Table};
        use crate::plan::execution::type_::ExternalTypeId;
        use crate::plan::execution::{
            LibraryFunctionEntries, LibraryFunctionEntry, LibraryInputConstructions,
            LibraryListConstructions,
        };

        let source = r#"
pub type Key
@external(erlang, "native", "key")
fn key() -> Key
pub fn main() { key() }
"#;
        let (program, _, _) = lowered_native(source);
        let common = &program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let catalog =
            Catalog::admit(&common.function_parameters, &program.functions, &types).unwrap();
        let target = ExternalFunctionId {
            index: 0,
            return_type: ExternalTypeId(0),
        };
        let mut entries: LibraryFunctionEntries = LibraryFunctionEntries {
            functions: Vec::new().into(),
            ints: Table::Static(&[]),
            floats: Table::Static(&[]),
            strings: Table::Static(&[]),
            bit_arrays: Table::Static(&[]),
            utf_codepoints: Table::Static(&[]),
            customs: Table::Static(&[]),
            externals: vec![LibraryFunctionEntry {
                callables: Vec::new().into(),
                function: target,
                inputs: LibraryInputConstructions {
                    variants: Table::Static(&[]),
                    lists: LibraryListConstructions {
                        functions: Vec::new().into(),
                        ints: Table::Static(&[]),
                        floats: Table::Static(&[]),
                        strings: Table::Static(&[]),
                        bit_arrays: Table::Static(&[]),
                        utf_codepoints: Table::Static(&[]),
                        customs: Table::Static(&[]),
                        externals: Table::Static(&[]),
                        bools: Table::Static(&[]),
                        nils: Table::Static(&[]),
                        tuples: Table::Static(&[]),
                        lists: Table::Static(&[]),
                    },
                },
            }]
            .into(),
            bools: Table::Static(&[]),
            nils: Table::Static(&[]),
            tuples: Table::Static(&[]),
            lists: Table::Static(&[]),
        };
        let export = Export {
            name: "main".into(),
            slot: 0,
            signature: FunctionMetadata {
                arguments: Table::Static(&[]),
                return_: Node::Owned(Box::new(TypeMetadata::External(
                    common.external_types.types[0].clone(),
                ))),
            },
        };
        let resolved = resolve(&entries, &export, &catalog, &types).unwrap();
        assert_eq!(resolved.family, FunctionReturnFamily::External);
        assert_eq!(
            resolved.function.return_type,
            &crate::plan::execution::type_::ValueType::External(ExternalTypeId(0))
        );
        assert!(std::ptr::eq(resolved.inputs, &entries.externals[0].inputs));
        assert_eq!(
            super::main_matches(
                &ProfiledRuntimeFunctionId::External(target),
                &entries,
                &export,
                &types
            ),
            Ok(true)
        );
        assert_eq!(
            all(
                &ProfiledRuntimeFunctionId::External(target),
                &entries,
                std::slice::from_ref(&export),
                &catalog,
                &types
            )
            .unwrap()
            .0
            .len(),
            1
        );
        let mut invalid = entries.externals.to_vec();
        invalid[0].function.index = 999;
        entries.externals = invalid.into();
        assert_eq!(
            resolve(&entries, &export, &catalog, &types).err(),
            Some(ExportError::Target(CallError::Catalog(
                CatalogError::MissingFunction {
                    family: FunctionTableFamily::External,
                    index: 999
                }
            ))),
        );
    }

    fn prepare() -> PreparedModule {
        let source = r#"
pub fn double(value: Int) { value * 2 }
pub fn pair(value: Int) { #(value, True) }
pub fn values(value: Int) { [value] }
pub fn result(value: Result(Int, String)) { value }
"#;
        let module = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let (mut bindings, _) = ModuleBuilder::new(module)
            .unwrap()
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("double"))
            .unwrap();
        bindings
            .function(FunctionDeclaration::<(BigInt,), (BigInt, bool)>::new(
                "pair",
            ))
            .unwrap();
        bindings
            .function(FunctionDeclaration::<(BigInt,), List<BigInt>>::new(
                "values",
            ))
            .unwrap();
        bindings
            .function(FunctionDeclaration::<
                (Result<BigInt, crate::StringValue>,),
                Result<BigInt, crate::StringValue>,
            >::new("result"))
            .unwrap();
        bindings.prepare()
    }

    #[test]
    fn admits_selected_entries_and_constructor_inputs_without_executing_them() {
        let prepared = prepare();
        let common = &prepared.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let catalog = Catalog::admit(
            &common.function_parameters,
            &prepared.program.functions,
            &types,
        )
        .unwrap();
        assert_eq!(
            all(
                &common.main,
                &prepared.entries,
                &prepared.exports,
                &catalog,
                &types
            )
            .map(|_| ()),
            Ok(())
        );
        let entry = resolve(&prepared.entries, &prepared.exports[0], &catalog, &types).unwrap();
        assert_eq!(entry.function.parameters.len(), 1);
        assert!(std::ptr::eq(entry.inputs, &prepared.entries.ints[0].inputs));
        assert_eq!(prepared.entries.customs[0].inputs.variants.len(), 1);
    }

    #[test]
    fn rejects_missing_duplicate_unclaimed_and_wrongly_typed_exports() {
        let prepared = prepare();
        let common = &prepared.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let catalog = Catalog::admit(
            &common.function_parameters,
            &prepared.program.functions,
            &types,
        )
        .unwrap();
        let check = |exports: &[Export]| {
            all(&common.main, &prepared.entries, exports, &catalog, &types).map(|_| ())
        };
        let int = || FunctionType::new(vec![ValueType::Int], ValueType::Int);
        assert_eq!(check(&[]), Err(EntryError::Empty));
        assert_eq!(
            check(&[Export::new("missing".into(), int(), 1)]),
            Err(EntryError::Export {
                index: 0,
                error: ExportError::MissingSlot { slot: 1 },
            })
        );
        assert_eq!(
            check(&[
                Export::new("double".into(), int(), 0),
                Export::new("double".into(), int(), 0)
            ]),
            Err(EntryError::DuplicateName { index: 1 })
        );
        assert_eq!(
            check(&[
                Export::new("double".into(), int(), 0),
                Export::new("alias".into(), int(), 0)
            ]),
            Err(EntryError::DuplicateSlot { index: 1 })
        );
        assert_eq!(
            check(&[Export::new("double".into(), int(), 0)]),
            Err(EntryError::Unclaimed {
                family: FunctionReturnFamily::Custom,
                count: 1
            })
        );
        assert_eq!(
            check(&[Export::new(
                "double".into(),
                FunctionType::new(Vec::new(), ValueType::Int),
                0
            )]),
            Err(EntryError::Export {
                index: 0,
                error: ExportError::ArgumentCount {
                    declared: 0,
                    actual: 1
                },
            })
        );
        assert_eq!(
            check(&[Export::new(
                "double".into(),
                FunctionType::new(vec![ValueType::String], ValueType::Int),
                0
            )]),
            Err(EntryError::Export {
                index: 0,
                error: ExportError::ArgumentType { index: 0 },
            })
        );
        assert_eq!(
            check(&[Export::new(
                "pair".into(),
                FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Tuple(vec![ValueType::Int, ValueType::String])
                ),
                0
            )]),
            Err(EntryError::Export {
                index: 0,
                error: ExportError::ReturnType,
            })
        );
    }

    #[test]
    fn rejects_a_different_main_target_and_cyclic_export_metadata() {
        use crate::plan::execution::function::{
            IntFunctionId, ProfiledCoreRuntimeFunctionId as Core,
            ProfiledRuntimeFunctionId as Runtime,
        };
        use crate::plan::execution::storage::{Node, Table};
        let prepared = prepare();
        let common = &prepared.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let catalog = Catalog::admit(
            &common.function_parameters,
            &prepared.program.functions,
            &types,
        )
        .unwrap();
        assert_eq!(
            all(
                &Runtime::Core(Core::Int(IntFunctionId(usize::MAX))),
                &prepared.entries,
                &prepared.exports,
                &catalog,
                &types
            )
            .map(|_| ()),
            Err(EntryError::Main)
        );
        let tuple = &prepared.exports[1];
        assert_eq!(tuple.name.as_str(), "pair");
        let main = Runtime::Core(Core::Tuple {
            id: prepared.entries.tuples[0].function,
            return_type: vec![
                crate::plan::execution::type_::ValueType::External(
                    crate::plan::execution::type_::ExternalTypeId(99),
                ),
                crate::plan::execution::type_::ValueType::Bool,
            ]
            .into(),
        });
        assert_eq!(
            all(
                &main,
                &prepared.entries,
                std::slice::from_ref(tuple),
                &catalog,
                &types
            )
            .map(|_| ()),
            Err(EntryError::MainType(TypeError::MissingExternal {
                index: 99
            }))
        );
        static CYCLE: TypeMetadata = TypeMetadata::List(Node::Static(&CYCLE));
        static ARGUMENTS: [TypeMetadata; 1] = [TypeMetadata::List(Node::Static(&CYCLE))];
        let export = Export {
            name: "double".into(),
            slot: 0,
            signature: FunctionMetadata {
                arguments: Table::Static(&ARGUMENTS),
                return_: Node::Static(&TypeMetadata::Int),
            },
        };
        assert_eq!(
            all(&common.main, &prepared.entries, &[export], &catalog, &types).map(|_| ()),
            Err(EntryError::Export {
                index: 0,
                error: ExportError::Type(TypeError::RecursiveMetadata),
            })
        );
    }

    #[test]
    fn entry_inputs_are_checked_before_a_prepared_module_can_be_loaded() {
        use super::InputError;
        use crate::plan::execution::type_::{IntListTypeId, ListTypeId};
        let mut prepared = prepare();
        super::super::tests::owned_mut(&mut prepared.entries.ints)[0]
            .inputs
            .lists
            .ints = vec![IntListTypeId {
            list_type: ListTypeId(99),
        }]
        .into();
        let common = &prepared.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let catalog = Catalog::admit(
            &common.function_parameters,
            &prepared.program.functions,
            &types,
        )
        .unwrap();
        assert_eq!(
            all(
                &common.main,
                &prepared.entries,
                &prepared.exports,
                &catalog,
                &types
            )
            .map(|_| ()),
            Err(EntryError::Export {
                index: 0,
                error: ExportError::Input(InputError::Type(TypeError::MissingList { index: 99 })),
            })
        );
    }

    #[test]
    fn a_captured_closure_cannot_be_relabelled_as_a_public_entry() {
        use crate::plan::execution::function::{FunctionTableFamily, IntFunctionId};
        let source = "fn capture(value: Int) { fn() { value } } pub fn main() { let get = capture(42) get() }";
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let prepared = ModuleBuilder::new(typed)
            .unwrap()
            .function(FunctionDeclaration::<(), BigInt>::new("main"))
            .unwrap()
            .0
            .prepare();
        let common = &prepared.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let catalog = Catalog::admit(
            &common.function_parameters,
            &prepared.program.functions,
            &types,
        )
        .unwrap();
        let captures = catalog
            .family(FunctionTableFamily::Int)
            .enumerate()
            .filter_map(|(index, function)| {
                (!function.captures.is_empty()).then_some((index, function.captures.len()))
            })
            .collect::<Vec<_>>();
        assert_eq!(captures.len(), 1);
        assert_eq!(captures[0].1, 1);
        let mut entries = prepared.entries.clone();
        super::super::tests::owned_mut(&mut entries.ints)[0].function =
            IntFunctionId(captures[0].0);
        assert_eq!(
            all(&common.main, &entries, &prepared.exports, &catalog, &types).map(|_| ()),
            Err(EntryError::Export {
                index: 0,
                error: ExportError::Captures { count: 1 },
            })
        );
    }

    #[test]
    fn export_signatures_cannot_narrow_inputs_or_hide_recursive_return_metadata() {
        use crate::plan::execution::function::FunctionTableFamily;
        use crate::plan::execution::storage::Node;
        use crate::plan::execution::type_::{
            CustomConstructorRefinement, ValueShapeDescriptor, ValueShapeId,
        };
        let source = r#"
pub type Choice { Present(Int) Missing }
fn value(choice: Choice) { case choice { Present(value) -> value Missing -> 0 } }
pub fn main() { value(Present(42)) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let catalog =
            Catalog::admit(&common.function_parameters, &plan.program.functions, &types).unwrap();
        let function = catalog
            .family(FunctionTableFamily::Int)
            .find(|function| function.parameters.len() == 1)
            .unwrap();
        let mut signature = FunctionMetadata {
            arguments: vec![TypeMetadata::Custom(
                common.custom_types.types[0].type_.clone(),
            )]
            .into(),
            return_: Node::Static(&TypeMetadata::Int),
        };
        assert_eq!(super::signature(&signature, &function, &types), Ok(()));
        let exact = common
            .value_shapes
            .shapes
            .iter()
            .enumerate()
            .find_map(|(index, shape)| match shape {
                ValueShapeDescriptor::Custom(id) => (common.value_shapes.custom_shapes[id.0]
                    .constructor
                    == CustomConstructorRefinement::Exact(0))
                .then_some(ValueShapeId(index)),
                _ => None,
            })
            .unwrap();
        let narrow = super::Function {
            parameter_shapes: &[exact],
            ..function
        };
        assert_eq!(
            super::signature(&signature, &narrow, &types),
            Err(ExportError::NarrowedInput { index: 0 })
        );
        static CYCLE: TypeMetadata = TypeMetadata::List(Node::Static(&CYCLE));
        signature.return_ = Node::Static(&CYCLE);
        assert_eq!(
            super::signature(&signature, &function, &types),
            Err(ExportError::Type(TypeError::RecursiveMetadata))
        );
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()),
            Ok(crate::Value::Int(42.into()))
        );
    }
}
