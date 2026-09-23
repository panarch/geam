use super::type_::{TypeError, Types};
use crate::plan::execution::function::parameters::FunctionContract;
use crate::plan::execution::function::{
    ExecutionProfile, FunctionCatalog, FunctionFunctionTables, FunctionTableFamily, FunctionTables,
    ListFunctionTables, ValueFunctionTables,
};
use crate::plan::execution::graph::ParamLocal;
use crate::plan::execution::type_::{ValueShapeDescriptor, ValueShapeId, ValueType};
use std::ops::Range;

pub(super) struct Catalog<'data> {
    raw: &'data FunctionCatalog,
    shape_types: &'data [ValueType],
    shapes: &'data [ValueShapeDescriptor],
}

pub(super) struct Function<'data> {
    pub family: FunctionTableFamily,
    pub index: usize,
    pub parameters: &'data [ParamLocal],
    pub parameter_shapes: &'data [ValueShapeId],
    pub return_: ValueShapeId,
    pub return_type: &'data ValueType,
    pub return_shape: &'data ValueShapeDescriptor,
    pub captures: &'data [crate::plan::execution::graph::ParamSlot],
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum CatalogError {
    FamilyRange {
        family: usize,
        range: Range<usize>,
        length: usize,
    },
    FamilyOrder {
        family: usize,
        expected: usize,
        found: usize,
    },
    FamilyCount {
        family: usize,
        expected: usize,
        found: usize,
    },
    UnclaimedFunctions {
        claimed: usize,
        length: usize,
    },
    ParameterRange {
        function: usize,
        range: Range<usize>,
        length: usize,
    },
    ParameterOrder {
        function: usize,
        expected: usize,
        found: usize,
    },
    UnclaimedParameters {
        claimed: usize,
        length: usize,
    },
    ParameterShapeCount {
        function: usize,
        expected: usize,
        found: usize,
    },
    MissingFunction {
        family: FunctionTableFamily,
        index: usize,
    },
    Type(TypeError),
}

impl<'data> Catalog<'data> {
    pub(super) fn admit<Profile: ExecutionProfile>(
        raw: &'data FunctionCatalog,
        tables: &FunctionTables<Profile>,
        types: &Types<'data>,
    ) -> Result<Self, CatalogError> {
        let counts = function_counts(tables);
        let mut end = 0;
        for (family, (range, expected)) in raw.families.iter().zip(counts).enumerate() {
            let functions =
                raw.functions
                    .get(range.clone())
                    .ok_or_else(|| CatalogError::FamilyRange {
                        family,
                        range: range.clone(),
                        length: raw.functions.len(),
                    })?;
            if functions.len() != expected {
                return Err(CatalogError::FamilyCount {
                    family,
                    expected,
                    found: functions.len(),
                });
            }
            if !functions.is_empty() {
                if range.start != end {
                    return Err(CatalogError::FamilyOrder {
                        family,
                        expected: end,
                        found: range.start,
                    });
                }
                end = range.end;
            }
        }
        if end != raw.functions.len() {
            return Err(CatalogError::UnclaimedFunctions {
                claimed: end,
                length: raw.functions.len(),
            });
        }
        let mut end = 0;
        for (function, contract) in raw.functions.iter().enumerate() {
            let FunctionContract {
                parameters,
                parameter_shapes,
                return_,
                captures,
            } = contract;
            let params = raw.parameters.get(parameters.clone()).ok_or_else(|| {
                CatalogError::ParameterRange {
                    function,
                    range: parameters.clone(),
                    length: raw.parameters.len(),
                }
            })?;
            if parameters.start != end {
                return Err(CatalogError::ParameterOrder {
                    function,
                    expected: end,
                    found: parameters.start,
                });
            }
            end = parameters.end;
            if params.len() != parameter_shapes.len() {
                return Err(CatalogError::ParameterShapeCount {
                    function,
                    expected: params.len(),
                    found: parameter_shapes.len(),
                });
            }
            for (parameter, shape) in params.iter().zip(parameter_shapes.iter()) {
                if &types.local_type(parameter).map_err(CatalogError::Type)?
                    != types.shape_type(*shape).map_err(CatalogError::Type)?
                {
                    return Err(CatalogError::Type(TypeError::LocalTypeMismatch));
                }
            }
            types.shape_type(*return_).map_err(CatalogError::Type)?;
            for capture in captures.iter() {
                types.slot(capture).map_err(CatalogError::Type)?;
            }
        }
        if end != raw.parameters.len() {
            return Err(CatalogError::UnclaimedParameters {
                claimed: end,
                length: raw.parameters.len(),
            });
        }
        Ok(Self {
            raw,
            shape_types: types.shape_types(),
            shapes: &types.shapes.shapes,
        })
    }

    pub(super) fn function(
        &self,
        family: FunctionTableFamily,
        index: usize,
    ) -> Result<Function<'data>, CatalogError> {
        let range = &self.raw.families[family as usize];
        let contract = self.raw.functions[range.clone()]
            .get(index)
            .ok_or(CatalogError::MissingFunction { family, index })?;
        Ok(self.view(family, index, contract))
    }

    pub(super) fn family(
        &self,
        family: FunctionTableFamily,
    ) -> impl ExactSizeIterator<Item = Function<'data>> {
        let range = &self.raw.families[family as usize];
        self.raw.functions[range.clone()]
            .iter()
            .enumerate()
            .map(move |(index, contract)| self.view(family, index, contract))
    }

    fn view(
        &self,
        family: FunctionTableFamily,
        index: usize,
        contract: &'data FunctionContract,
    ) -> Function<'data> {
        Function {
            family,
            index,
            parameters: &self.raw.parameters[contract.parameters.clone()],
            parameter_shapes: &contract.parameter_shapes,
            return_: contract.return_,
            return_type: &self.shape_types[contract.return_.index()],
            return_shape: &self.shapes[contract.return_.index()],
            captures: &contract.captures,
        }
    }
}

fn function_counts<Profile: ExecutionProfile>(
    tables: &FunctionTables<Profile>,
) -> [usize; FunctionTableFamily::COUNT] {
    let FunctionTables {
        value_returns,
        list_returns,
        function_returns,
    } = tables;
    let ValueFunctionTables {
        never_functions,
        int_functions,
        float_functions,
        string_functions,
        bit_array_functions,
        utf_codepoint_functions,
        custom_functions,
        external_functions,
        bool_functions,
        nil_functions,
        tuple_functions,
    } = value_returns;
    let ListFunctionTables {
        parameter_list_functions,
        int_list_functions,
        string_list_functions,
        bit_array_list_functions,
        utf_codepoint_list_functions,
        custom_list_functions,
        external_list_functions,
        float_list_functions,
        bool_list_functions,
        nil_list_functions,
        tuple_list_functions,
        parameter_list_list_functions,
        list_list_functions,
        function_list_functions,
    } = list_returns;
    let FunctionFunctionTables {
        int_function_functions,
        float_function_functions,
        string_function_functions,
        bit_array_function_functions,
        utf_codepoint_function_functions,
        custom_function_functions,
        external_function_functions,
        bool_function_functions,
        nil_function_functions,
        tuple_function_functions,
        generic_function_functions,
        never_function_functions,
        parameter_list_function_functions,
        parameter_list_list_function_functions,
        int_list_function_functions,
        string_list_function_functions,
        bit_array_list_function_functions,
        utf_codepoint_list_function_functions,
        custom_list_function_functions,
        external_list_function_functions,
        float_list_function_functions,
        bool_list_function_functions,
        nil_list_function_functions,
        tuple_list_function_functions,
        list_list_function_functions,
        function_list_function_functions,
        function_function_functions,
    } = function_returns;
    let mut counts = [0; FunctionTableFamily::COUNT];
    counts[FunctionTableFamily::Never as usize] = never_functions.len();
    counts[FunctionTableFamily::Int as usize] = int_functions.len();
    counts[FunctionTableFamily::Float as usize] = float_functions.len();
    counts[FunctionTableFamily::String as usize] = string_functions.len();
    counts[FunctionTableFamily::BitArray as usize] = bit_array_functions.len();
    counts[FunctionTableFamily::UtfCodepoint as usize] = utf_codepoint_functions.len();
    counts[FunctionTableFamily::Custom as usize] = custom_functions.len();
    counts[FunctionTableFamily::External as usize] = external_functions.len();
    counts[FunctionTableFamily::Bool as usize] = bool_functions.len();
    counts[FunctionTableFamily::Nil as usize] = nil_functions.len();
    counts[FunctionTableFamily::Tuple as usize] = tuple_functions.len();
    counts[FunctionTableFamily::ParameterList as usize] = parameter_list_functions.len();
    counts[FunctionTableFamily::IntList as usize] = int_list_functions.len();
    counts[FunctionTableFamily::StringList as usize] = string_list_functions.len();
    counts[FunctionTableFamily::BitArrayList as usize] = bit_array_list_functions.len();
    counts[FunctionTableFamily::UtfCodepointList as usize] = utf_codepoint_list_functions.len();
    counts[FunctionTableFamily::CustomList as usize] = custom_list_functions.len();
    counts[FunctionTableFamily::ExternalList as usize] = external_list_functions.len();
    counts[FunctionTableFamily::FloatList as usize] = float_list_functions.len();
    counts[FunctionTableFamily::BoolList as usize] = bool_list_functions.len();
    counts[FunctionTableFamily::NilList as usize] = nil_list_functions.len();
    counts[FunctionTableFamily::TupleList as usize] = tuple_list_functions.len();
    counts[FunctionTableFamily::ParameterListList as usize] = parameter_list_list_functions.len();
    counts[FunctionTableFamily::ListList as usize] = list_list_functions.len();
    counts[FunctionTableFamily::FunctionList as usize] = function_list_functions.len();
    counts[FunctionTableFamily::IntFunction as usize] = int_function_functions.len();
    counts[FunctionTableFamily::FloatFunction as usize] = float_function_functions.len();
    counts[FunctionTableFamily::StringFunction as usize] = string_function_functions.len();
    counts[FunctionTableFamily::BitArrayFunction as usize] = bit_array_function_functions.len();
    counts[FunctionTableFamily::UtfCodepointFunction as usize] =
        utf_codepoint_function_functions.len();
    counts[FunctionTableFamily::CustomFunction as usize] = custom_function_functions.len();
    counts[FunctionTableFamily::ExternalFunction as usize] = external_function_functions.len();
    counts[FunctionTableFamily::BoolFunction as usize] = bool_function_functions.len();
    counts[FunctionTableFamily::NilFunction as usize] = nil_function_functions.len();
    counts[FunctionTableFamily::TupleFunction as usize] = tuple_function_functions.len();
    counts[FunctionTableFamily::GenericFunction as usize] = generic_function_functions.len();
    counts[FunctionTableFamily::NeverFunction as usize] = never_function_functions.len();
    counts[FunctionTableFamily::ParameterListFunction as usize] =
        parameter_list_function_functions.len();
    counts[FunctionTableFamily::ParameterListListFunction as usize] =
        parameter_list_list_function_functions.len();
    counts[FunctionTableFamily::IntListFunction as usize] = int_list_function_functions.len();
    counts[FunctionTableFamily::StringListFunction as usize] = string_list_function_functions.len();
    counts[FunctionTableFamily::BitArrayListFunction as usize] =
        bit_array_list_function_functions.len();
    counts[FunctionTableFamily::UtfCodepointListFunction as usize] =
        utf_codepoint_list_function_functions.len();
    counts[FunctionTableFamily::CustomListFunction as usize] = custom_list_function_functions.len();
    counts[FunctionTableFamily::ExternalListFunction as usize] =
        external_list_function_functions.len();
    counts[FunctionTableFamily::FloatListFunction as usize] = float_list_function_functions.len();
    counts[FunctionTableFamily::BoolListFunction as usize] = bool_list_function_functions.len();
    counts[FunctionTableFamily::NilListFunction as usize] = nil_list_function_functions.len();
    counts[FunctionTableFamily::TupleListFunction as usize] = tuple_list_function_functions.len();
    counts[FunctionTableFamily::ListListFunction as usize] = list_list_function_functions.len();
    counts[FunctionTableFamily::FunctionListFunction as usize] =
        function_list_function_functions.len();
    counts[FunctionTableFamily::FunctionFunction as usize] = function_function_functions.len();
    counts
}

#[cfg(test)]
mod tests {
    use super::{
        Catalog, CatalogError, FunctionCatalog, FunctionContract, FunctionTableFamily, ParamLocal,
        TypeError, Types, ValueShapeId, function_counts,
    };
    use crate::plan::execution::graph::IntLocalId;
    use crate::plan::execution::storage::Table;
    use crate::plan::execution::type_::ValueType;

    #[test]
    fn admits_real_function_families_and_retained_capture_contracts() {
        for source in [
            "pub fn main() { 42 }",
            "fn integer() { 1 } fn text() { \"text\" } pub fn main() { #(integer(), text()) }",
            "fn make(x: Int) { fn(y: Int) { x + y } } pub fn main() { make(20)(22) }",
            "fn values() { [1] } pub fn main() { values() }",
            "pub fn main() { panic as \"stopped\" }",
        ] {
            let module =
                crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
            let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(module).unwrap());
            let common = &plan.program.common;
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes,
            )
            .unwrap();
            assert_eq!(
                Catalog::admit(&common.function_parameters, &plan.program.functions, &types)
                    .map(|_| ()),
                Ok(()),
                "{source}"
            );
        }
    }

    #[test]
    fn checked_catalog_borrows_parameters_return_and_captures() {
        let plan = lower("pub fn main() { 21 * 2 }");
        let program = &plan.program;
        let types = execution_types(program);
        let catalog = Catalog::admit(
            &program.common.function_parameters,
            &program.functions,
            &types,
        )
        .unwrap();
        let function = catalog.function(FunctionTableFamily::Int, 0).unwrap();
        assert!(function.parameters.is_empty());
        assert_eq!(function.return_type, &ValueType::Int);
        assert!(std::ptr::eq(
            function.return_type,
            &program.common.value_shapes.shape_types[function.return_.index()]
        ));
        assert!(std::ptr::eq(
            function.return_shape,
            &program.common.value_shapes.shapes[function.return_.index()]
        ));
        assert!(function.captures.is_empty());
        assert_eq!(
            function.return_,
            program.common.function_parameters.functions[0].return_
        );
        assert!(std::ptr::eq(
            function.parameter_shapes,
            program.common.function_parameters.functions[0]
                .parameter_shapes
                .as_ref()
        ));
        let mut family = catalog.family(FunctionTableFamily::Int);
        assert_eq!(family.len(), 1);
        let first = family.next().unwrap();
        assert!(std::ptr::eq(first.parameters, function.parameters));
        assert!(std::ptr::eq(
            first.parameter_shapes,
            function.parameter_shapes
        ));
        assert!(std::ptr::eq(first.captures, function.captures));
        assert_eq!(first.return_, function.return_);
        assert!(std::ptr::eq(first.return_type, function.return_type));
        assert!(std::ptr::eq(first.return_shape, function.return_shape));
        assert_eq!(family.len(), 0);
        assert_eq!(catalog.family(FunctionTableFamily::Float).len(), 0);
        assert_eq!(
            catalog.function(FunctionTableFamily::Float, 0).map(|_| ()),
            Err(CatalogError::MissingFunction {
                family: FunctionTableFamily::Float,
                index: 0
            })
        );
    }

    #[test]
    fn each_actual_table_has_one_catalog_family() {
        let plan = lower("pub fn main() { 21 * 2 }");
        let mut expected = [0; FunctionTableFamily::COUNT];
        expected[FunctionTableFamily::Int as usize] = 1;
        assert_eq!(function_counts(&plan.program.functions), expected);
    }

    #[test]
    fn rejects_bad_family_ranges_counts_gaps_and_unclaimed_records() {
        let family = FunctionTableFamily::Int as usize;
        type CatalogMutation = fn(&mut FunctionCatalog);
        let cases: [(CatalogMutation, CatalogError); 4] = [
            (
                |catalog| catalog.families[FunctionTableFamily::Int as usize] = 0..2,
                CatalogError::FamilyRange {
                    family,
                    range: 0..2,
                    length: 1,
                },
            ),
            (
                |catalog| catalog.families[FunctionTableFamily::Int as usize] = 0..0,
                CatalogError::FamilyCount {
                    family,
                    expected: 1,
                    found: 0,
                },
            ),
            (
                |catalog| {
                    catalog.functions =
                        vec![catalog.functions[0].clone(), catalog.functions[0].clone()].into();
                    catalog.families[FunctionTableFamily::Int as usize] = 1..2;
                },
                CatalogError::FamilyOrder {
                    family,
                    expected: 0,
                    found: 1,
                },
            ),
            (
                |catalog| {
                    catalog.functions =
                        vec![catalog.functions[0].clone(), catalog.functions[0].clone()].into()
                },
                CatalogError::UnclaimedFunctions {
                    claimed: 1,
                    length: 2,
                },
            ),
        ];
        let plan = lower("pub fn main() { 21 * 2 }");
        let program = &plan.program;
        let types = execution_types(program);
        for (mutate, expected) in cases {
            let mut raw = copy_catalog(&program.common.function_parameters);
            mutate(&mut raw);
            assert_eq!(
                Catalog::admit(&raw, &program.functions, &types).map(|_| ()),
                Err(expected)
            );
        }
    }

    #[test]
    fn rejects_bad_parameter_ranges_gaps_and_unclaimed_parameters() {
        let cases = [
            (
                0..1,
                vec![],
                CatalogError::ParameterRange {
                    function: 0,
                    range: 0..1,
                    length: 0,
                },
            ),
            (
                1..1,
                vec![ParamLocal::Int(IntLocalId(0))],
                CatalogError::ParameterOrder {
                    function: 0,
                    expected: 0,
                    found: 1,
                },
            ),
            (
                0..0,
                vec![ParamLocal::Int(IntLocalId(0))],
                CatalogError::UnclaimedParameters {
                    claimed: 0,
                    length: 1,
                },
            ),
        ];
        let plan = lower("pub fn main() { 21 * 2 }");
        let program = &plan.program;
        let types = execution_types(program);
        for (parameters, values, expected) in cases {
            let mut raw = copy_catalog(&program.common.function_parameters);
            raw.functions = vec![FunctionContract {
                parameters,
                ..raw.functions[0].clone()
            }]
            .into();
            raw.parameters = values.into();
            assert_eq!(
                Catalog::admit(&raw, &program.functions, &types).map(|_| ()),
                Err(expected)
            );
        }
    }

    #[test]
    fn rejects_unknown_return_and_capture_shapes_before_reading_signatures() {
        let plan = lower("pub fn main() { 21 * 2 }");
        let program = &plan.program;
        let types = execution_types(program);
        for contract in [
            FunctionContract {
                parameters: 0..0,
                parameter_shapes: Table::Static(&[]),
                return_: ValueShapeId(99),
                captures: vec![].into(),
            },
            FunctionContract {
                parameters: 0..0,
                parameter_shapes: Table::Static(&[]),
                return_: ValueShapeId(0),
                captures: vec![crate::plan::execution::graph::ParamSlot::new(
                    ParamLocal::Int(IntLocalId(0)),
                    ValueShapeId(99),
                )]
                .into(),
            },
        ] {
            let mut raw = copy_catalog(&program.common.function_parameters);
            raw.functions = vec![contract].into();
            assert_eq!(
                Catalog::admit(&raw, &program.functions, &types).map(|_| ()),
                Err(CatalogError::Type(TypeError::MissingShape { index: 99 }))
            );
        }
    }

    #[test]
    fn rejects_parameter_shape_counts_missing_shapes_and_wrong_storage_types() {
        use crate::plan::execution::graph::{BoolLocalId, IntFunctionLocalId};
        use crate::plan::execution::type_::FunctionType;

        let plan = lower("pub fn main() { 42 }");
        let program = &plan.program;
        let types = execution_types(program);
        assert_eq!(types.shape_type(ValueShapeId(0)), Ok(&ValueType::Int));
        let cases = [
            (
                Vec::new(),
                vec![ValueShapeId(0)],
                CatalogError::ParameterShapeCount {
                    function: 0,
                    expected: 0,
                    found: 1,
                },
            ),
            (
                vec![ParamLocal::Int(IntLocalId(0))],
                vec![ValueShapeId(99)],
                CatalogError::Type(TypeError::MissingShape { index: 99 }),
            ),
            (
                vec![ParamLocal::Bool(BoolLocalId(0))],
                vec![ValueShapeId(0)],
                CatalogError::Type(TypeError::LocalTypeMismatch),
            ),
            (
                vec![ParamLocal::IntFunction {
                    local: IntFunctionLocalId(0),
                    type_: FunctionType::new(Vec::new(), ValueType::Float),
                }],
                vec![ValueShapeId(0)],
                CatalogError::Type(TypeError::LocalTypeMismatch),
            ),
        ];
        for (parameters, shapes, expected) in cases {
            let mut raw = copy_catalog(&program.common.function_parameters);
            raw.functions = vec![FunctionContract {
                parameters: 0..parameters.len(),
                parameter_shapes: shapes.into(),
                ..raw.functions[0].clone()
            }]
            .into();
            raw.parameters = parameters.into();
            assert_eq!(
                Catalog::admit(&raw, &program.functions, &types).err(),
                Some(expected)
            );
        }
    }

    fn execution_types(
        program: &crate::plan::execution::ExecutionProgram<std::convert::Infallible>,
    ) -> Types<'_> {
        Types::admit(
            &program.common.list_types,
            &program.common.custom_types,
            &program.common.external_types,
            &program.common.value_shapes,
        )
        .unwrap()
    }

    fn lower(source: &str) -> crate::ExecutionPlan {
        let module = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        crate::ExecutionPlan::from_module_plan(crate::plan_module(module).unwrap())
    }

    fn copy_catalog(catalog: &FunctionCatalog) -> FunctionCatalog {
        FunctionCatalog {
            families: catalog.families.clone(),
            parameters: catalog.parameters.clone(),
            functions: catalog.functions.clone(),
        }
    }
}
