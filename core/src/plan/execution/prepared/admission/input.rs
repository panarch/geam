use super::type_::{TypeError, Types};
use crate::plan::execution::function::FunctionReturnFamily;
use crate::plan::execution::type_::{
    CustomTypeDescriptor, CustomTypeId, ListStorageTypeId, TypeMetadata, ValueType,
};
use crate::plan::execution::{LibraryInputConstructions, LibraryListConstructions};
use crate::plan::{LibraryValueType, LibraryVariant, StandardVariant};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum InputError {
    Type(TypeError),
    VariantPair {
        index: usize,
    },
    StandardDefinition {
        type_index: usize,
    },
    VariantCount {
        expected: usize,
        actual: usize,
    },
    VariantType {
        index: usize,
    },
    ListCount {
        family: FunctionReturnFamily,
        expected: usize,
        actual: usize,
    },
    ListType {
        index: usize,
    },
}

pub(super) fn admit(
    inputs: &LibraryInputConstructions,
    types: &Types<'_>,
) -> Result<(), InputError> {
    for (index, [first, second]) in inputs.variants.iter().enumerate() {
        types.constructor(*first).map_err(InputError::Type)?;
        types.constructor(*second).map_err(InputError::Type)?;
        if first.type_id != second.type_id || first.index != 0 || second.index != 1 {
            return Err(InputError::VariantPair { index });
        }
        let descriptor = &types.customs.types[first.type_id.index()];
        let kind = if named(descriptor, StandardVariant::Result) {
            StandardVariant::Result
        } else if named(descriptor, StandardVariant::Option) {
            StandardVariant::Option
        } else {
            return Err(InputError::VariantPair { index });
        };
        standard_definition(first.type_id, descriptor, kind, types)?;
    }
    let LibraryListConstructions {
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
    } = &inputs.lists;
    for id in ints.iter() {
        types
            .list_storage(ListStorageTypeId::Int(*id))
            .map_err(InputError::Type)?;
    }
    for id in floats.iter() {
        types
            .list_storage(ListStorageTypeId::Float(*id))
            .map_err(InputError::Type)?;
    }
    for id in strings.iter() {
        types
            .list_storage(ListStorageTypeId::String(*id))
            .map_err(InputError::Type)?;
    }
    for id in bit_arrays.iter() {
        types
            .list_storage(ListStorageTypeId::BitArray(*id))
            .map_err(InputError::Type)?;
    }
    for id in utf_codepoints.iter() {
        types
            .list_storage(ListStorageTypeId::UtfCodepoint(*id))
            .map_err(InputError::Type)?;
    }
    for id in customs.iter() {
        types
            .list_storage(ListStorageTypeId::Custom(*id))
            .map_err(InputError::Type)?;
    }
    for id in externals.iter() {
        types
            .list_storage(ListStorageTypeId::External(*id))
            .map_err(InputError::Type)?;
    }
    for id in bools.iter() {
        types
            .list_storage(ListStorageTypeId::Bool(*id))
            .map_err(InputError::Type)?;
    }
    for id in nils.iter() {
        types
            .list_storage(ListStorageTypeId::Nil(*id))
            .map_err(InputError::Type)?;
    }
    for id in tuples.iter() {
        types
            .list_storage(ListStorageTypeId::Tuple(*id))
            .map_err(InputError::Type)?;
    }
    for id in lists.iter() {
        types
            .list_storage(ListStorageTypeId::List(*id))
            .map_err(InputError::Type)?;
    }
    Ok(())
}

pub(super) fn mapping(
    inputs: &LibraryInputConstructions,
    variants: &[LibraryVariant],
    items: &[LibraryValueType],
    standard: &[StandardVariant],
    types: &Types<'_>,
) -> Result<(), InputError> {
    if inputs.variants.len() != variants.len() {
        return Err(InputError::VariantCount {
            expected: variants.len(),
            actual: inputs.variants.len(),
        });
    }
    // Entry admission has already checked every stored construction link.
    for (index, (actual, expected)) in inputs.variants.iter().zip(variants).enumerate() {
        let metadata = TypeMetadata::from_public(&crate::plan::ValueType::Custom(
            expected.kind.custom_type(expected.arguments.clone()),
        ));
        if !types.metadata_matches_value(&metadata, &ValueType::Custom(actual[0].type_id)) {
            return Err(InputError::VariantType { index });
        }
    }
    let lists = &inputs.lists;
    let mut counts = [0; 11];
    for (index, item) in items.iter().enumerate() {
        let (family, stored) = match item {
            LibraryValueType::Int => (
                0,
                lists
                    .ints
                    .get(counts[0])
                    .copied()
                    .map(ListStorageTypeId::Int),
            ),
            LibraryValueType::Float => (
                1,
                lists
                    .floats
                    .get(counts[1])
                    .copied()
                    .map(ListStorageTypeId::Float),
            ),
            LibraryValueType::String => (
                2,
                lists
                    .strings
                    .get(counts[2])
                    .copied()
                    .map(ListStorageTypeId::String),
            ),
            LibraryValueType::BitArray => (
                3,
                lists
                    .bit_arrays
                    .get(counts[3])
                    .copied()
                    .map(ListStorageTypeId::BitArray),
            ),
            LibraryValueType::UtfCodepoint => (
                4,
                lists
                    .utf_codepoints
                    .get(counts[4])
                    .copied()
                    .map(ListStorageTypeId::UtfCodepoint),
            ),
            LibraryValueType::Custom(_) => (
                5,
                lists
                    .customs
                    .get(counts[5])
                    .copied()
                    .map(ListStorageTypeId::Custom),
            ),
            LibraryValueType::External(_) => (
                6,
                lists
                    .externals
                    .get(counts[6])
                    .copied()
                    .map(ListStorageTypeId::External),
            ),
            LibraryValueType::Bool => (
                7,
                lists
                    .bools
                    .get(counts[7])
                    .copied()
                    .map(ListStorageTypeId::Bool),
            ),
            LibraryValueType::Nil => (
                8,
                lists
                    .nils
                    .get(counts[8])
                    .copied()
                    .map(ListStorageTypeId::Nil),
            ),
            LibraryValueType::Tuple(_) => (
                9,
                lists
                    .tuples
                    .get(counts[9])
                    .copied()
                    .map(ListStorageTypeId::Tuple),
            ),
            LibraryValueType::List(_) => (
                10,
                lists
                    .lists
                    .get(counts[10])
                    .copied()
                    .map(ListStorageTypeId::List),
            ),
        };
        counts[family] += 1;
        let Some(stored) = stored else {
            return Err(InputError::ListType { index });
        };
        let id = stored.list_type();
        let metadata =
            TypeMetadata::from_public(&crate::plan::ValueType::List(Box::new(item.value_type())));
        if !types.metadata_matches_value(&metadata, &ValueType::List(id)) {
            return Err(InputError::ListType { index });
        }
    }
    for ((family, actual), expected) in [
        (FunctionReturnFamily::Int, lists.ints.len()),
        (FunctionReturnFamily::Float, lists.floats.len()),
        (FunctionReturnFamily::String, lists.strings.len()),
        (FunctionReturnFamily::BitArray, lists.bit_arrays.len()),
        (
            FunctionReturnFamily::UtfCodepoint,
            lists.utf_codepoints.len(),
        ),
        (FunctionReturnFamily::Custom, lists.customs.len()),
        (FunctionReturnFamily::External, lists.externals.len()),
        (FunctionReturnFamily::Bool, lists.bools.len()),
        (FunctionReturnFamily::Nil, lists.nils.len()),
        (FunctionReturnFamily::Tuple, lists.tuples.len()),
        (FunctionReturnFamily::List, lists.lists.len()),
    ]
    .into_iter()
    .zip(counts)
    {
        if actual != expected {
            return Err(InputError::ListCount {
                family,
                expected,
                actual,
            });
        }
    }
    for (index, descriptor) in types.customs.types.iter().enumerate() {
        for kind in standard {
            if named(descriptor, *kind) {
                standard_definition(CustomTypeId(index), descriptor, *kind, types)?;
            }
        }
    }
    Ok(())
}

fn named(descriptor: &CustomTypeDescriptor, kind: StandardVariant) -> bool {
    let name = kind.type_name();
    descriptor.type_.package.as_str() == name.package().as_str()
        && descriptor.type_.module.as_str() == name.module().as_str()
        && descriptor.type_.name.as_str() == name.name().as_str()
}

fn standard_definition(
    id: CustomTypeId,
    descriptor: &CustomTypeDescriptor,
    kind: StandardVariant,
    types: &Types<'_>,
) -> Result<(), InputError> {
    let expected_arguments = match kind {
        StandardVariant::Result => 2,
        StandardVariant::Option => 1,
    };
    if descriptor.constructor_count != 2 || descriptor.type_.arguments.len() != expected_arguments {
        return Err(InputError::StandardDefinition {
            type_index: id.index(),
        });
    }
    let original = types.definition_of(id);
    if original.publicity != crate::plan::CustomTypePublicity::Public
        || original.opaque
        || original.parameters != expected_arguments
        || original.constructors.len() != 2
    {
        return Err(InputError::StandardDefinition {
            type_index: id.index(),
        });
    }
    for (index, constructor) in original.constructors.iter().enumerate() {
        let (name, argument) = match (kind, index) {
            (StandardVariant::Result, 0) => ("Ok", Some(0)),
            (StandardVariant::Result, _) => ("Error", Some(1)),
            (StandardVariant::Option, 0) => ("Some", Some(0)),
            (StandardVariant::Option, _) => ("None", None),
        };
        if constructor.name.as_str() != name
            || constructor.fields.len() != usize::from(argument.is_some())
            || argument.is_some_and(|argument| {
                constructor.fields[0].label.is_some()
                    || constructor.fields[0].type_
                        != crate::plan::execution::type_::TypeMetadata::Parameter(
                            crate::plan::TypeParameterId(argument),
                        )
            })
        {
            return Err(InputError::StandardDefinition {
                type_index: id.index(),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        CustomTypeId, FunctionReturnFamily, InputError, LibraryValueType, LibraryVariant,
        StandardVariant, TypeError, Types, admit, mapping, named, standard_definition,
    };
    use crate::embedding::{BigInt, FunctionDeclaration, List, ModuleBuilder};
    use crate::plan::execution::prepared::PreparedModule;

    #[test]
    fn validates_each_list_input_family_and_its_declared_mapping_count() {
        use crate::plan::execution::storage::Table;
        use crate::plan::execution::type_::custom::{ConstructorDefinition, CustomDefinition};
        use crate::plan::execution::type_::{
            BitArrayListTypeId, BoolListTypeId, CustomListTypeId, CustomTypeDescriptor,
            CustomTypeTable, ExternalListTypeId, ExternalTypeId, ExternalTypeTable,
            FloatListTypeId, IntListTypeId, ListListTypeId, ListStorageTypeId, ListTypeId,
            ListTypeTable, NilListTypeId, NominalTypeMetadata, StringListTypeId, TupleListTypeId,
            UtfCodepointListTypeId, ValueShapeTable, ValueType,
        };
        use crate::plan::execution::{LibraryInputConstructions, LibraryListConstructions};
        use crate::plan::{
            CustomType, CustomTypeName, CustomTypePublicity, ExternalType, ExternalTypeName,
            ValueType as Public,
        };

        let custom = CustomType::new(
            CustomTypeName::new("app".into(), "items".into(), "Choice".into()),
            Vec::new(),
        );
        let external = ExternalType::new(
            ExternalTypeName::new("app".into(), "items".into(), "Token".into()),
            Vec::new(),
        );
        let customs = CustomTypeTable {
            definitions: vec![CustomDefinition {
                package: "app".into(),
                module: "items".into(),
                name: "Choice".into(),
                publicity: CustomTypePublicity::Public,
                opaque: false,
                parameters: 0,
                constructors: vec![ConstructorDefinition {
                    name: "Choice".into(),
                    fields: Table::Static(&[]),
                }]
                .into(),
            }]
            .into(),
            types: vec![CustomTypeDescriptor {
                type_: NominalTypeMetadata::from_custom(&custom),
                constructor_count: 1,
                constructors: Table::Static(&[]),
            }]
            .into(),
        };
        let externals = ExternalTypeTable::new(vec![external.clone()]);
        let shapes = ValueShapeTable {
            shapes: Table::Static(&[]),
            shape_types: Table::Static(&[]),
            custom_shapes: Table::Static(&[]),
        };
        let int = IntListTypeId::new(ListTypeId(0));
        let float = FloatListTypeId::new(ListTypeId(1));
        let string = StringListTypeId::new(ListTypeId(2));
        let bits = BitArrayListTypeId::new(ListTypeId(3));
        let codepoint = UtfCodepointListTypeId::new(ListTypeId(4));
        let choice = CustomListTypeId::new(ListTypeId(5), CustomTypeId(0));
        let token = ExternalListTypeId::new(ListTypeId(6), ExternalTypeId(0));
        let bool_ = BoolListTypeId::new(ListTypeId(7));
        let nil = NilListTypeId::new(ListTypeId(8));
        let tuple = TupleListTypeId::new(ListTypeId(9), 0);
        let nested = ListListTypeId::new(ListTypeId(10), ListTypeId(0));
        let lists = ListTypeTable {
            types: vec![
                ListStorageTypeId::Int(int),
                ListStorageTypeId::Float(float),
                ListStorageTypeId::String(string),
                ListStorageTypeId::BitArray(bits),
                ListStorageTypeId::UtfCodepoint(codepoint),
                ListStorageTypeId::Custom(choice),
                ListStorageTypeId::External(token),
                ListStorageTypeId::Bool(bool_),
                ListStorageTypeId::Nil(nil),
                ListStorageTypeId::Tuple(tuple),
                ListStorageTypeId::List(nested),
            ]
            .into(),
            tuple_items: vec![vec![ValueType::Int, ValueType::Bool].into()].into(),
            function_items: Table::Static(&[]),
        };
        let types = Types::admit(&lists, &customs, &externals, &shapes).unwrap();
        let inputs = LibraryInputConstructions {
            variants: Table::Static(&[]),
            lists: LibraryListConstructions {
                ints: vec![int].into(),
                floats: vec![float].into(),
                strings: vec![string].into(),
                bit_arrays: vec![bits].into(),
                utf_codepoints: vec![codepoint].into(),
                customs: vec![choice].into(),
                externals: vec![token].into(),
                bools: vec![bool_].into(),
                nils: vec![nil].into(),
                tuples: vec![tuple].into(),
                lists: vec![nested].into(),
            },
        };
        let items = [
            LibraryValueType::Int,
            LibraryValueType::Float,
            LibraryValueType::String,
            LibraryValueType::BitArray,
            LibraryValueType::UtfCodepoint,
            LibraryValueType::Custom(custom),
            LibraryValueType::External(external),
            LibraryValueType::Bool,
            LibraryValueType::Nil,
            LibraryValueType::Tuple(vec![Public::Int, Public::Bool]),
            LibraryValueType::List(Box::new(LibraryValueType::Int)),
        ];
        assert_eq!(admit(&inputs, &types), Ok(()));
        assert_eq!(mapping(&inputs, &[], &items, &[], &types), Ok(()));

        let families = [
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
        ];
        for (index, family) in families.into_iter().enumerate() {
            let mut omitted = items.to_vec();
            omitted.remove(index);
            assert_eq!(
                mapping(&inputs, &[], &omitted, &[], &types),
                Err(InputError::ListCount {
                    family,
                    expected: 0,
                    actual: 1
                })
            );
        }
        let mutations: [fn(&mut LibraryListConstructions); 11] = [
            |lists| lists.ints = vec![IntListTypeId::new(ListTypeId(99))].into(),
            |lists| lists.floats = vec![FloatListTypeId::new(ListTypeId(99))].into(),
            |lists| lists.strings = vec![StringListTypeId::new(ListTypeId(99))].into(),
            |lists| lists.bit_arrays = vec![BitArrayListTypeId::new(ListTypeId(99))].into(),
            |lists| lists.utf_codepoints = vec![UtfCodepointListTypeId::new(ListTypeId(99))].into(),
            |lists| {
                lists.customs = vec![CustomListTypeId::new(ListTypeId(99), CustomTypeId(0))].into()
            },
            |lists| {
                lists.externals =
                    vec![ExternalListTypeId::new(ListTypeId(99), ExternalTypeId(0))].into()
            },
            |lists| lists.bools = vec![BoolListTypeId::new(ListTypeId(99))].into(),
            |lists| lists.nils = vec![NilListTypeId::new(ListTypeId(99))].into(),
            |lists| lists.tuples = vec![TupleListTypeId::new(ListTypeId(99), 0)].into(),
            |lists| lists.lists = vec![ListListTypeId::new(ListTypeId(99), ListTypeId(0))].into(),
        ];
        for mutate in mutations {
            let mut malformed = inputs.clone();
            mutate(&mut malformed.lists);
            assert_eq!(
                admit(&malformed, &types),
                Err(InputError::Type(TypeError::MissingList { index: 99 }))
            );
        }
    }

    #[test]
    fn standard_option_mapping_requires_the_original_public_nonopaque_definition() {
        use crate::{ModuleSource, PackageSource, compile_typed_package_program, plan_program};
        for (declaration, return_type, expected) in [
            ("pub type Option(a) { Some(a) None }", "Option(Int)", true),
            (
                "pub opaque type Option(a) { Some(a) None }",
                "Option(Int)",
                false,
            ),
            (
                "@internal pub type Option(a) { Some(a) None }",
                "Option(Int)",
                false,
            ),
            ("pub type Option(a) { None Some(a) }", "Option(Int)", false),
            (
                "pub type Option(a) { Some(value: a) None }",
                "Option(Int)",
                false,
            ),
            (
                "pub type Option(a) { Some(a) None Unknown }",
                "Option(Int)",
                false,
            ),
            (
                "pub type Option(a, b) { Some(a) None }",
                "Option(Int, String)",
                false,
            ),
        ] {
            let option = format!(
                "{declaration}\npub fn make(flag: Bool) -> {return_type} {{ case flag {{ True -> Some(42) False -> None }} }}"
            );
            let program = compile_typed_package_program(
                "app",
                "main",
                [
                    PackageSource::new(
                        "app",
                        ["gleam_stdlib"],
                        [ModuleSource::new(
                            "main",
                            "src/main.gleam",
                            "import gleam/option\npub fn main() { option.make(True) }",
                        )],
                    ),
                    PackageSource::new(
                        "gleam_stdlib",
                        Vec::<&str>::new(),
                        [ModuleSource::new(
                            "gleam/option",
                            "src/gleam/option.gleam",
                            option,
                        )],
                    ),
                ],
            )
            .unwrap();
            let plan = crate::ExecutionPlan::from_module_plan(plan_program(program).unwrap());
            let common = &plan.program.common;
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes,
            )
            .unwrap();
            let (index, descriptor) = common
                .custom_types
                .types
                .iter()
                .enumerate()
                .find(|(_, type_)| named(type_, StandardVariant::Option))
                .unwrap();
            let result = standard_definition(
                CustomTypeId(index),
                descriptor,
                StandardVariant::Option,
                &types,
            );
            assert_eq!(
                result,
                if expected {
                    Ok(())
                } else {
                    Err(InputError::StandardDefinition { type_index: index })
                },
                "{declaration}"
            );
            let inputs = crate::plan::execution::LibraryInputConstructions {
                variants: vec![[
                    crate::plan::execution::type_::CustomConstructorId {
                        type_id: CustomTypeId(index),
                        index: 0,
                    },
                    crate::plan::execution::type_::CustomConstructorId {
                        type_id: CustomTypeId(index),
                        index: 1,
                    },
                ]]
                .into(),
                lists: crate::plan::execution::LibraryListConstructions {
                    ints: vec![].into(),
                    floats: vec![].into(),
                    strings: vec![].into(),
                    bit_arrays: vec![].into(),
                    utf_codepoints: vec![].into(),
                    customs: vec![].into(),
                    externals: vec![].into(),
                    bools: vec![].into(),
                    nils: vec![].into(),
                    tuples: vec![].into(),
                    lists: vec![].into(),
                },
            };
            assert_eq!(admit(&inputs, &types), result, "{declaration}");
            let output_only = crate::plan::execution::LibraryInputConstructions {
                variants: vec![].into(),
                lists: inputs.lists,
            };
            assert_eq!(
                mapping(&output_only, &[], &[], &[StandardVariant::Option], &types),
                result
            );
            assert_eq!(
                mapping(&output_only, &[], &[], &[StandardVariant::Result], &types),
                Ok(())
            );
        }
    }

    #[test]
    fn ordinary_two_constructor_types_are_not_standard_variant_inputs() {
        use crate::plan::execution::{LibraryInputConstructions, LibraryListConstructions};
        let module = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            "pub type Choice { First Second } pub fn main() { #(First, Second) }",
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(module).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let inputs = LibraryInputConstructions {
            variants: vec![[0, 1].map(|index| {
                crate::plan::execution::type_::CustomConstructorId {
                    type_id: CustomTypeId(0),
                    index,
                }
            })]
            .into(),
            lists: LibraryListConstructions {
                ints: vec![].into(),
                floats: vec![].into(),
                strings: vec![].into(),
                bit_arrays: vec![].into(),
                utf_codepoints: vec![].into(),
                customs: vec![].into(),
                externals: vec![].into(),
                bools: vec![].into(),
                nils: vec![].into(),
                tuples: vec![].into(),
                lists: vec![].into(),
            },
        };
        assert_eq!(
            admit(&inputs, &types),
            Err(InputError::VariantPair { index: 0 })
        );
    }

    fn prepare() -> PreparedModule {
        let module = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            r#"
pub fn inspect(a: Result(List(Int), String), b: List(#(Int, Bool)), c: List(List(Int))) {
  let _ = #(a, b, c)
  42
}
"#,
        )
        .unwrap();
        let (bindings, _) = ModuleBuilder::new(module)
            .unwrap()
            .function(FunctionDeclaration::<
                (
                    Result<List<BigInt>, crate::StringValue>,
                    List<(BigInt, bool)>,
                    List<List<BigInt>>,
                ),
                BigInt,
            >::new("inspect"))
            .unwrap();
        bindings.prepare()
    }

    #[test]
    fn matches_rust_input_traversal_and_rejects_missing_or_different_constructions() {
        use crate::plan::ValueType as Public;
        let prepared = prepare();
        let common = &prepared.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let inputs = &prepared.entries.ints[0].inputs;
        let variants = [LibraryVariant::new(
            StandardVariant::Result,
            vec![Public::List(Box::new(Public::Int)), Public::String],
        )];
        let lists = [
            LibraryValueType::Int,
            LibraryValueType::Tuple(vec![Public::Int, Public::Bool]),
            LibraryValueType::List(Box::new(LibraryValueType::Int)),
            LibraryValueType::Int,
        ];
        let standard = [StandardVariant::Result];
        assert_eq!(admit(inputs, &types), Ok(()));
        assert_eq!(
            mapping(inputs, &variants, &lists, &standard, &types),
            Ok(())
        );
        assert_eq!(
            mapping(inputs, &[], &lists, &standard, &types),
            Err(InputError::VariantCount {
                expected: 0,
                actual: 1
            })
        );
        assert_eq!(
            mapping(
                inputs,
                &[LibraryVariant::new(
                    StandardVariant::Result,
                    vec![Public::Int, Public::String]
                )],
                &lists,
                &standard,
                &types
            ),
            Err(InputError::VariantType { index: 0 })
        );
        assert_eq!(
            mapping(inputs, &variants, &[], &standard, &types),
            Err(InputError::ListCount {
                family: FunctionReturnFamily::Int,
                expected: 0,
                actual: 2
            })
        );
        assert_eq!(
            mapping(
                inputs,
                &variants,
                &[LibraryValueType::String],
                &standard,
                &types
            ),
            Err(InputError::ListType { index: 0 })
        );
        assert_eq!(
            mapping(
                inputs,
                &variants,
                &[LibraryValueType::Tuple(vec![Public::String])],
                &standard,
                &types
            ),
            Err(InputError::ListType { index: 0 })
        );
        let mut reversed = inputs.clone();
        let mut pair = inputs.variants[0];
        pair.reverse();
        reversed.variants = vec![pair].into();
        assert_eq!(
            admit(&reversed, &types),
            Err(InputError::VariantPair { index: 0 })
        );
        let mut missing = inputs.clone();
        let mut pair = inputs.variants[0];
        pair[1].index = 2;
        missing.variants = vec![pair].into();
        assert_eq!(
            admit(&missing, &types),
            Err(InputError::Type(TypeError::MissingConstructor {
                type_index: pair[1].type_id.index(),
                index: 2
            }))
        );
        pair[0].index = 3;
        missing.variants = vec![pair].into();
        assert_eq!(
            admit(&missing, &types),
            Err(InputError::Type(TypeError::MissingConstructor {
                type_index: pair[0].type_id.index(),
                index: 3
            }))
        );
    }

    #[test]
    fn checks_standard_layouts_even_when_only_one_constructor_is_emitted() {
        let prepared = prepare();
        let common = &prepared.program.common;
        let id = prepared.entries.ints[0].inputs.variants[0][0].type_id;
        let mut customs = common.custom_types.as_ref().clone();
        let mut descriptors = customs.types.into_vec();
        let descriptor = &mut descriptors[id.index()];
        descriptor.constructors = vec![descriptor.constructors[0].clone()].into();
        customs.types = descriptors.into();
        let types = Types::admit(
            &common.list_types,
            &customs,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let descriptor = types.custom_type(id).unwrap();
        assert_eq!(
            standard_definition(id, descriptor, StandardVariant::Result, &types),
            Ok(())
        );
    }
}
