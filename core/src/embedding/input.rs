use super::value::{Arguments, EmbeddingValue};
use crate::plan::execution::type_::CustomConstructorId;
use crate::plan::execution::{LibraryInputConstructions, LibraryListConstructions};
use crate::runtime::{
    EmbeddingCustomInput, EmbeddingInputStorage, EmbeddingInputValue, EmbeddingTupleInput,
    RetainedValues,
};
use std::sync::Arc;

/// A generated function's static input inference constraint.
///
/// This relation cannot supply a codec or establish value compatibility. Calls
/// independently require the private adapter for their validated signature:
///
/// ```compile_fail
/// use geam_core::embedding::{EcoString, Function, InputShape, Module};
/// struct Forged;
/// impl InputShape<(bool,)> for Forged {}
/// fn invalid(module: &Module, function: Function<(EcoString,), ()>) {
///     let function = function.with_input_shape::<Forged>();
///     let _ = module.call(&function, (true,), &mut Vec::new());
/// }
/// ```
#[doc(hidden)]
pub trait InputShape<Input> {}

impl<Arguments, Input> InputShape<Input> for Arguments where Arguments: ArgumentsInput<Input> {}

pub(super) trait ArgumentsInput<Input>: Arguments {
    fn owners_match(input: &Input, owner: &Arc<()>) -> bool;

    fn into_inputs(input: Input, constructions: &LibraryInputConstructions) -> RetainedValues;
}

pub(super) trait InputValue<Input>: EmbeddingValue {
    fn owners_match(input: &Input, owner: &Arc<()>) -> bool;

    fn into_runtime(
        input: Input,
        constructions: &mut InputConstructions<'_>,
        storage: &EmbeddingInputStorage,
    ) -> Self::Runtime;
}

pub(super) trait FreshInput<Input>: InputValue<Input> {}

#[derive(Clone, Copy)]
pub(super) struct InputConstructions<'a> {
    variants: &'a [[CustomConstructorId; 2]],
    lists: &'a LibraryListConstructions,
    next_variant: usize,
    next_lists: [usize; 10],
}

pub(super) enum ListFamily {
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Custom,
    Bool,
    Nil,
    Tuple,
    List,
}

impl InputConstructions<'_> {
    fn new(constructions: &LibraryInputConstructions) -> InputConstructions<'_> {
        InputConstructions {
            variants: constructions.variants(),
            lists: constructions.lists(),
            next_variant: 0,
            next_lists: [0; 10],
        }
    }

    fn take_variant(&mut self) -> [CustomConstructorId; 2] {
        let constructors = self.variants[self.next_variant];
        self.next_variant += 1;
        constructors
    }

    pub(super) fn take_list<Value: EmbeddingValue>(
        &mut self,
    ) -> <Value::Runtime as EmbeddingInputValue>::ListType {
        let next = &mut self.next_lists[Value::LIST_FAMILY as usize];
        let type_ = Value::list_id(self.lists, *next);
        *next += 1;
        type_
    }

    pub(super) fn skip<Value: EmbeddingValue>(&mut self) {
        self.next_variant += Value::VARIANT_COUNT;
        self.next_lists = add_list_counts(self.next_lists, Value::LIST_COUNTS);
    }
}

pub(super) const fn add_list_counts(left: [usize; 10], right: [usize; 10]) -> [usize; 10] {
    let mut counts = left;
    let mut index = 0;
    while index < counts.len() {
        counts[index] += right[index];
        index += 1;
    }
    counts
}

macro_rules! scalar_input {
    ($type:ty) => {
        impl InputValue<$type> for $type {
            fn owners_match(_input: &Self, _owner: &Arc<()>) -> bool {
                true
            }

            fn into_runtime(
                input: Self,
                _constructions: &mut InputConstructions<'_>,
                _storage: &EmbeddingInputStorage,
            ) -> Self::Runtime {
                input
            }
        }

        impl FreshInput<$type> for $type {}
    };
}

scalar_input!(super::BigInt);
scalar_input!(f64);
scalar_input!(super::EcoString);
scalar_input!(super::BitArrayValue);
scalar_input!(char);
scalar_input!(bool);
scalar_input!(());

macro_rules! tuple_input {
    ($($type:ident => $input:ident => $value:ident),+) => {
        impl<$($type, $input),+> InputValue<($($input,)+)> for ($($type,)+)
        where
            $($type: InputValue<$input>,)+
        {
            fn owners_match(input: &($($input,)+), owner: &Arc<()>) -> bool {
                let ($($value,)+) = input;
                true $(&& $type::owners_match($value, owner))+
            }

            fn into_runtime(
                input: ($($input,)+),
                constructions: &mut InputConstructions<'_>,
                storage: &EmbeddingInputStorage,
            ) -> Self::Runtime {
                let ($($value,)+) = input;
                EmbeddingTupleInput::new([
                    $($type::into_runtime($value, constructions, storage).into_input()),+
                ])
            }
        }

        impl<$($type, $input),+> FreshInput<($($input,)+)> for ($($type,)+)
        where
            $($type: FreshInput<$input>,)+
        {}

        impl<$($type, $input),+> ArgumentsInput<($($input,)+)> for ($($type,)+)
        where
            $($type: InputValue<$input>,)+
        {
            fn owners_match(input: &($($input,)+), owner: &Arc<()>) -> bool {
                let ($($value,)+) = input;
                true $(&& $type::owners_match($value, owner))+
            }

            fn into_inputs(input: ($($input,)+), constructions: &LibraryInputConstructions) -> RetainedValues {
                let ($($value,)+) = input;
                let mut constructions = InputConstructions::new(constructions);
                let storage = EmbeddingInputStorage::default();
                let mut inputs = RetainedValues::empty();
                $($type::into_runtime($value, &mut constructions, &storage).into_input().retain(&mut inputs);)+
                inputs
            }
        }
    };
}

tuple_input!(A => IA => a);
tuple_input!(A => IA => a, B => IB => b);
tuple_input!(A => IA => a, B => IB => b, C => IC => c);
tuple_input!(A => IA => a, B => IB => b, C => IC => c, D => ID => d);
tuple_input!(A => IA => a, B => IB => b, C => IC => c, D => ID => d, E => IE => e);
tuple_input!(A => IA => a, B => IB => b, C => IC => c, D => ID => d, E => IE => e, F => IF => f);
tuple_input!(A => IA => a, B => IB => b, C => IC => c, D => ID => d, E => IE => e, F => IF => f, G => IG => g);

impl ArgumentsInput<()> for () {
    fn owners_match(_input: &(), _owner: &Arc<()>) -> bool {
        true
    }

    fn into_inputs(_input: (), _constructions: &LibraryInputConstructions) -> RetainedValues {
        RetainedValues::empty()
    }
}

impl<Success, Failure, SuccessInput, FailureInput> InputValue<Result<SuccessInput, FailureInput>>
    for Result<Success, Failure>
where
    Success: InputValue<SuccessInput>,
    Failure: InputValue<FailureInput>,
{
    fn owners_match(input: &Result<SuccessInput, FailureInput>, owner: &Arc<()>) -> bool {
        match input {
            Ok(value) => Success::owners_match(value, owner),
            Err(value) => Failure::owners_match(value, owner),
        }
    }

    fn into_runtime(
        input: Result<SuccessInput, FailureInput>,
        constructions: &mut InputConstructions<'_>,
        storage: &EmbeddingInputStorage,
    ) -> Self::Runtime {
        let constructors = constructions.take_variant();
        match input {
            Ok(value) => {
                let value = Success::into_runtime(value, constructions, storage).into_input();
                constructions.skip::<Failure>();
                EmbeddingCustomInput::new(constructors[0], [value])
            }
            Err(value) => {
                constructions.skip::<Success>();
                let value = Failure::into_runtime(value, constructions, storage).into_input();
                EmbeddingCustomInput::new(constructors[1], [value])
            }
        }
    }
}

impl<Success, Failure, SuccessInput, FailureInput> FreshInput<Result<SuccessInput, FailureInput>>
    for Result<Success, Failure>
where
    Success: FreshInput<SuccessInput>,
    Failure: FreshInput<FailureInput>,
{
}

impl<Value, Input> InputValue<Option<Input>> for Option<Value>
where
    Value: InputValue<Input>,
{
    fn owners_match(input: &Option<Input>, owner: &Arc<()>) -> bool {
        match input {
            Some(value) => Value::owners_match(value, owner),
            None => true,
        }
    }

    fn into_runtime(
        input: Option<Input>,
        constructions: &mut InputConstructions<'_>,
        storage: &EmbeddingInputStorage,
    ) -> Self::Runtime {
        let constructors = constructions.take_variant();
        match input {
            Some(value) => EmbeddingCustomInput::new(
                constructors[0],
                [Value::into_runtime(value, constructions, storage).into_input()],
            ),
            None => {
                constructions.skip::<Value>();
                EmbeddingCustomInput::new(constructors[1], [])
            }
        }
    }
}

impl<Value, Input> FreshInput<Option<Input>> for Option<Value> where Value: FreshInput<Input> {}

#[cfg(test)]
mod tests {
    use crate::embedding::{
        BigInt, EcoString, FunctionDeclaration, InputShape, List, ModuleBuilder,
    };
    use crate::{ModuleSource, PackageSource, compile_typed_module, compile_typed_package_program};

    #[test]
    fn generated_shapes_preserve_scalar_inference_and_the_bound_function_owner() {
        struct KeepInput;
        impl<Values> InputShape<(Values, EcoString)> for KeepInput {}

        let typed = compile_typed_module(
            "library",
            "library.gleam",
            "pub fn keep(values: List(String), label: String) { #(values, label) }",
        )
        .expect("input shape source");
        let (bindings, function) = ModuleBuilder::new(typed)
            .expect("input shape plan")
            .function(FunctionDeclaration::<
                (List<EcoString>, EcoString),
                (List<EcoString>, EcoString),
            >::new("keep"))
            .expect("input shape declaration");
        let function = function.with_input_shape::<KeepInput>();
        let cloned = function.clone();
        assert_eq!(cloned.name(), "keep");
        let module = bindings.seal();
        let mut echo = Vec::new();
        let (values, label) = module
            .call(&function, (vec!["value".into()], "fresh".into()), &mut echo)
            .expect("fresh shape input");
        assert_eq!(values.get(0), Some("value".into()));
        assert_eq!(label, "fresh");
        let (retained, label) = module
            .call(&cloned, (&values, "retained".into()), &mut echo)
            .expect("retained shape input");
        assert_eq!(retained.get(0), Some("value".into()));
        assert_eq!(label, "retained");
        assert!(echo.is_empty());
    }

    #[test]
    fn composes_lists_inside_tuple_result_and_standard_option_boundaries() {
        let program = compile_typed_package_program(
            "application",
            "library",
            [
                PackageSource::new(
                    "gleam_stdlib",
                    Vec::<EcoString>::new(),
                    [ModuleSource::new(
                        "gleam/option",
                        "gleam_stdlib/src/gleam/option.gleam",
                        "pub type Option(value) { Some(value) None }",
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["gleam_stdlib"],
                    [ModuleSource::new(
                        "library",
                        "src/library.gleam",
                        r#"
import gleam/option.{type Option}

pub fn options(values: List(Option(Int))) { values }
pub fn pair(values: #(List(Int), List(String))) { values }
pub fn result(values: Result(List(Int), List(String))) { values }
pub fn optional(values: Option(List(List(String)))) { values }
"#,
                    )],
                ),
            ],
        )
        .expect("recursive Option package should compile");
        let (mut bindings, options) = ModuleBuilder::from_program(program)
            .expect("Option library")
            .function(FunctionDeclaration::<
                (List<Option<BigInt>>,),
                List<Option<BigInt>>,
            >::new("options"))
            .expect("Option items");
        let pair = bindings
            .function(FunctionDeclaration::<
                ((List<BigInt>, List<EcoString>),),
                (List<BigInt>, List<EcoString>),
            >::new("pair"))
            .expect("List pair declaration");
        let result = bindings
            .function(FunctionDeclaration::<
                (Result<List<BigInt>, List<EcoString>>,),
                Result<List<BigInt>, List<EcoString>>,
            >::new("result"))
            .expect("Result List declaration");
        let optional = bindings
            .function(FunctionDeclaration::<
                (Option<List<List<EcoString>>>,),
                Option<List<List<EcoString>>>,
            >::new("optional"))
            .expect("Option List declaration");
        let module = bindings.seal();
        let mut echo = Vec::new();

        let option_values = vec![Some(BigInt::from(1)), None, Some(3.into())];
        assert_eq!(
            module
                .call(&options, (option_values.clone(),), &mut echo)
                .expect("Option list")
                .to_vec(),
            option_values
        );
        let pair_values = module
            .call(
                &pair,
                ((vec![BigInt::from(4)], vec![EcoString::from("label")]),),
                &mut echo,
            )
            .expect("List pair");
        assert_eq!(pair_values.0.to_vec(), [BigInt::from(4)]);
        assert_eq!(pair_values.1.to_vec(), ["label"]);
        let retained = module
            .call(&pair, ((&pair_values.0, &pair_values.1),), &mut echo)
            .expect("retained pair");
        assert_eq!(retained.0.to_vec(), [BigInt::from(4)]);
        assert_eq!(retained.1.to_vec(), ["label"]);

        for input in [
            Ok(vec![BigInt::from(6)]),
            Err(vec![EcoString::from("error")]),
        ] {
            let output = module
                .call(&result, (input.clone(),), &mut echo)
                .expect("Result List");
            assert_eq!(
                output
                    .map(|value| value.to_vec())
                    .map_err(|value| value.to_vec()),
                input,
            );
        }

        let some = module
            .call(
                &optional,
                (Some(vec![vec![EcoString::from("nested")]]),),
                &mut echo,
            )
            .expect("Some nested List")
            .expect("Some");
        let retained = module
            .call(&optional, (Some(&some),), &mut echo)
            .expect("retained Some")
            .expect("Some");
        assert_eq!(retained.get(0).expect("child").to_vec(), ["nested"]);
        let absent: Option<Vec<Vec<EcoString>>> = None;
        assert!(
            module
                .call(&optional, (absent,), &mut echo)
                .expect("None List")
                .is_none()
        );
        assert!(echo.is_empty());
    }
}
