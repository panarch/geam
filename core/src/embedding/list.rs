use super::Module;
use super::input::{FreshInput, InputConstructions, InputValue, ListFamily};
use super::value::{EmbeddingValue, ReturnValue};
use crate::plan::execution::{
    LibraryFunctionEntries, LibraryInputConstructions, LibraryListConstructions,
};
use crate::plan::{LibraryValueType, StandardVariant};
use crate::runtime::{
    EmbeddingInputStorage, EmbeddingInputValue, EmbeddingList, EmbeddingListInput, EmbeddingOutput,
    RetainedValues,
};
use crate::{EchoSink, ExecutionError, HostProfile, HostedExecution};
use std::marker::PhantomData;
use std::sync::Arc;

/// A read-only Gleam List retaining its source allocation.
///
/// Reading remains valid after its Module and caller state have been dropped.
/// Passing it back as an argument requires the same live Module. A consumed
/// Rust Vec creates a new list; borrowing this value reuses its existing list.
///
/// Retained lists are not transferable between threads:
///
/// ```compile_fail
/// use geam_core::embedding::List;
/// fn require_send<T: Send>() {}
/// require_send::<List<bool>>();
/// ```
///
/// ```compile_fail
/// use geam_core::embedding::List;
/// fn require_sync<T: Sync>() {}
/// require_sync::<List<bool>>();
/// ```
///
/// A fresh outer Vec cannot contain retained child Lists. Materialize each
/// child explicitly to construct fresh nested Vecs instead:
///
/// ```compile_fail
/// use geam_core::embedding::{Function, List, Module};
/// fn mixed(module: &Module, function: &Function<(List<List<bool>>,), ()>, children: Vec<List<bool>>) {
///     let _ = module.call(function, (children,), &mut Vec::new());
/// }
/// ```
///
/// ```compile_fail
/// use geam_core::embedding::{Function, List, Module};
/// fn mixed(module: &Module, function: &Function<(List<List<bool>>,), ()>, child: &List<bool>) {
///     let _ = module.call(function, (vec![child],), &mut Vec::new());
/// }
/// ```
pub struct List<T> {
    value: EmbeddingList,
    owner: Arc<()>,
    marker: PhantomData<T>,
}

/// An iterator that decodes retained List items only as they are requested.
pub struct Iter<'a, T> {
    list: &'a List<T>,
    indices: std::ops::Range<usize>,
}

#[allow(private_bounds)]
impl<T: EmbeddingValue> List<T> {
    /// Returns the number of items without decoding them.
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Checks whether the list is empty without decoding items.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Decodes one item, returning None for an out-of-range index.
    pub fn get(&self, index: usize) -> Option<T> {
        self.value
            .item(index)
            .map(|mut output| T::take(&mut output, &self.owner))
    }

    /// Iterates over owned Rust items without materializing the whole list.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            list: self,
            indices: 0..self.len(),
        }
    }

    /// Explicitly decodes every item into a new Rust Vec.
    pub fn to_vec(&self) -> Vec<T> {
        self.iter().collect()
    }
}

impl<T: EmbeddingValue> Iterator for Iter<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.indices.next().and_then(|index| self.list.get(index))
    }
}

impl<T: EmbeddingValue> EmbeddingValue for List<T> {
    type Runtime = EmbeddingListInput;

    const VARIANT_COUNT: usize = T::VARIANT_COUNT;
    const LIST_COUNTS: [usize; 10] = {
        let mut counts = T::LIST_COUNTS;
        counts[T::LIST_FAMILY as usize] += 1;
        counts
    };
    const LIST_FAMILY: ListFamily = ListFamily::List;

    fn library_type() -> LibraryValueType {
        LibraryValueType::List(Box::new(T::library_type()))
    }

    fn collect_variants(variants: &mut Vec<StandardVariant>) {
        T::collect_variants(variants);
    }

    fn collect_lists(lists: &mut Vec<LibraryValueType>) {
        lists.push(T::library_type());
        T::collect_lists(lists);
    }

    fn list_id(
        lists: &LibraryListConstructions,
        index: usize,
    ) -> <Self::Runtime as EmbeddingInputValue>::ListType {
        lists.lists[index]
    }

    fn take(output: &mut EmbeddingOutput, owner: &Arc<()>) -> Self {
        Self {
            value: output.take_list(),
            owner: Arc::clone(owner),
            marker: PhantomData,
        }
    }
}

impl<T, Input> InputValue<Vec<Input>> for List<T>
where
    T: FreshInput<Input>,
{
    fn owners_match(_input: &Vec<Input>, _owner: &Arc<()>) -> bool {
        true
    }

    fn into_runtime(
        input: Vec<Input>,
        constructions: &mut InputConstructions<'_>,
        storage: &EmbeddingInputStorage,
    ) -> Self::Runtime {
        let type_ = constructions.take_list::<T>();
        let item_constructions = *constructions;
        constructions.skip::<T>();
        let values = input.into_iter().map(|value| {
            let mut constructions = item_constructions;
            T::into_runtime(value, &mut constructions, storage)
        });
        T::Runtime::into_list(type_, values, storage)
    }
}

impl<T, Input> FreshInput<Vec<Input>> for List<T> where T: FreshInput<Input> {}

impl<T: EmbeddingValue> InputValue<&List<T>> for List<T> {
    fn owners_match(input: &&List<T>, owner: &Arc<()>) -> bool {
        Arc::ptr_eq(&input.owner, owner)
    }

    fn into_runtime(
        input: &List<T>,
        constructions: &mut InputConstructions<'_>,
        _storage: &EmbeddingInputStorage,
    ) -> Self::Runtime {
        constructions.skip::<Self>();
        input.value.input()
    }
}

impl<T: EmbeddingValue> ReturnValue for List<T> {
    fn input_constructions(
        entries: &LibraryFunctionEntries,
        slot: usize,
    ) -> &LibraryInputConstructions {
        entries.lists[slot].inputs()
    }

    fn call(
        module: &Module,
        slot: usize,
        inputs: RetainedValues,
        echo: &mut dyn EchoSink,
    ) -> Result<Self, ExecutionError> {
        let entry = &module.entries.lists[slot];
        crate::runtime::run_embedded_list(&module.execution, entry.function(), inputs, echo)
            .map(|mut output| Self::take(&mut output, &module.owner))
    }

    fn call_hosted<Profile: HostProfile>(
        execution: &HostedExecution<Profile>,
        entries: &LibraryFunctionEntries,
        slot: usize,
        inputs: RetainedValues,
        state: &mut Profile::RunState,
        echo: &mut dyn EchoSink,
        owner: &Arc<()>,
    ) -> Result<Self, ExecutionError> {
        let entry = &entries.lists[slot];
        crate::runtime::run_hosted_embedded_list(execution, entry.function(), inputs, state, echo)
            .map(|mut output| Self::take(&mut output, owner))
    }
}

#[cfg(test)]
mod tests {
    use super::List;
    use crate::compile_typed_module;
    use crate::embedding::value::{Arguments, ReturnValue};
    use crate::embedding::{
        BigInt, BitArrayValue, CallError, EcoString, Function, FunctionDeclaration, ModuleBindings,
        ModuleBuilder,
    };

    fn library(source: &str) -> ModuleBuilder {
        let typed = compile_typed_module("library", "library.gleam", source)
            .expect("list library should compile");
        ModuleBuilder::new(typed).expect("list library should plan")
    }

    fn bind<Args: Arguments, Return: ReturnValue>(
        bindings: &mut ModuleBindings,
        name: &str,
    ) -> Function<Args, Return> {
        bindings
            .function(FunctionDeclaration::new(name))
            .expect("list function should bind")
    }

    #[test]
    fn moves_vectors_of_every_scalar_family_into_typed_lists() {
        let builder = library(
            r#"
pub fn ints(values: List(Int)) { values }
pub fn floats(values: List(Float)) { values }
pub fn strings(values: List(String)) { values }
pub fn bits(values: List(BitArray)) { values }
pub fn codepoints(values: List(UtfCodepoint)) { values }
pub fn bools(values: List(Bool)) { values }
pub fn nils(values: List(Nil)) { values }
"#,
        );
        let (mut bindings, ints) = builder
            .function(FunctionDeclaration::<(List<BigInt>,), List<BigInt>>::new(
                "ints",
            ))
            .expect("first List entry should bind");
        let floats = bind::<(List<f64>,), List<f64>>(&mut bindings, "floats");
        let strings = bind::<(List<EcoString>,), List<EcoString>>(&mut bindings, "strings");
        let bits = bind::<(List<BitArrayValue>,), List<BitArrayValue>>(&mut bindings, "bits");
        let codepoints = bind::<(List<char>,), List<char>>(&mut bindings, "codepoints");
        let bools = bind::<(List<bool>,), List<bool>>(&mut bindings, "bools");
        let nils = bind::<(List<()>,), List<()>>(&mut bindings, "nils");
        let module = bindings.seal();
        let mut echo = Vec::new();

        let values = module
            .call(&ints, (vec![BigInt::from(7), 12.into()],), &mut echo)
            .expect("ints");
        assert_eq!(values.to_vec(), [BigInt::from(7), 12.into()]);
        assert_eq!(
            module
                .call(&floats, (vec![1.5, 2.5],), &mut echo)
                .expect("floats")
                .to_vec(),
            [1.5, 2.5]
        );
        assert_eq!(
            module
                .call(
                    &strings,
                    (vec![EcoString::from("first"), "second".into()],),
                    &mut echo
                )
                .expect("strings")
                .to_vec(),
            ["first", "second"]
        );
        let bit_values = vec![
            BitArrayValue::from_bytes(vec![0, 255]),
            BitArrayValue::from_bytes(vec![2]),
        ];
        assert_eq!(
            module
                .call(&bits, (bit_values.clone(),), &mut echo)
                .expect("bits")
                .to_vec(),
            bit_values
        );
        assert_eq!(
            module
                .call(&codepoints, (vec!['A', 'Z'],), &mut echo)
                .expect("codepoints")
                .to_vec(),
            ['A', 'Z']
        );
        assert_eq!(
            module
                .call(&bools, (vec![true, false],), &mut echo)
                .expect("bools")
                .to_vec(),
            [true, false]
        );
        assert_eq!(
            module
                .call(&nils, (vec![(), ()],), &mut echo)
                .expect("nils")
                .to_vec(),
            [(), ()]
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn reads_lazily_and_passes_a_retained_list_back_without_decoding() {
        let (bindings, keep) = library("pub fn keep(values: List(String)) { values }")
            .function(FunctionDeclaration::<(List<EcoString>,), List<EcoString>>::new("keep"))
            .expect("list function should bind");
        let module = bindings.seal();
        let mut echo = Vec::new();
        let values = module
            .call(
                &keep,
                (vec![EcoString::from("one"), "two".into(), "three".into()],),
                &mut echo,
            )
            .expect("fresh list");
        assert_eq!(values.value.item_reads(), 0);
        assert_eq!(values.len(), 3);
        assert!(!values.is_empty());
        assert_eq!(values.value.item_reads(), 0);

        let retained = module
            .call(&keep, (&values,), &mut echo)
            .expect("same-owner retained list");
        assert_eq!(values.value.item_reads(), 0);
        assert_eq!(retained.value.item_reads(), 0);
        assert_eq!(values.get(1), Some("two".into()));
        assert_eq!(values.value.item_reads(), 1);
        assert_eq!(values.get(3), None);
        assert_eq!(values.value.item_reads(), 2);
        let mut iter = values.iter();
        assert_eq!(values.value.item_reads(), 2);
        assert_eq!(iter.next(), Some("one".into()));
        assert_eq!(values.value.item_reads(), 3);
        assert_eq!(iter.next(), Some("two".into()));
        assert_eq!(iter.next(), Some("three".into()));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(values.value.item_reads(), 5);
        assert_eq!(retained.to_vec(), ["one", "two", "three"]);
        assert_eq!(retained.value.item_reads(), 3);

        let empty = module
            .call(&keep, (Vec::<EcoString>::new(),), &mut echo)
            .expect("empty list");
        assert!(empty.is_empty());
        assert_eq!(empty.iter().next(), None);
        assert!(empty.to_vec().is_empty());
        assert_eq!(empty.value.item_reads(), 0);
        drop(echo);
        drop(module);
        assert_eq!(values.to_vec(), ["one", "two", "three"]);
        assert_eq!(retained.get(2), Some("three".into()));
    }

    #[test]
    fn retains_nested_lists_and_recursively_constructs_tuple_and_variant_items() {
        let (mut bindings, nested) = library(
            r#"
pub fn nested(values: List(List(String))) { values }
pub fn rows(values: List(#(String, Int))) { values }
pub fn checked(values: List(Result(#(String, Int), String))) { values }
pub fn grouped(values: List(#(Int, List(Result(Int, String))))) { values }
"#,
        )
        .function(FunctionDeclaration::<
            (List<List<EcoString>>,),
            List<List<EcoString>>,
        >::new("nested"))
        .expect("nested lists should bind");
        let rows =
            bind::<(List<(EcoString, BigInt)>,), List<(EcoString, BigInt)>>(&mut bindings, "rows");
        type CheckedRow = Result<(EcoString, BigInt), EcoString>;
        let checked = bind::<(List<CheckedRow>,), List<CheckedRow>>(&mut bindings, "checked");
        type Group = (BigInt, List<Result<BigInt, EcoString>>);
        let grouped = bind::<(List<Group>,), List<Group>>(&mut bindings, "grouped");
        let module = bindings.seal();
        let mut echo = Vec::new();

        let values = module
            .call(
                &nested,
                (vec![
                    vec![EcoString::from("first")],
                    vec![],
                    vec!["last".into()],
                ],),
                &mut echo,
            )
            .expect("nested vectors");
        assert_eq!(values.value.item_reads(), 0);
        let first = values.get(0).expect("first child");
        let empty = values.get(1).expect("empty child");
        assert_eq!(first.value.item_reads(), 0);
        assert!(empty.is_empty());
        let retained = module
            .call(&nested, (&values,), &mut echo)
            .expect("nested pass through");
        assert_eq!(values.value.item_reads(), 2);
        assert_eq!(retained.get(2).expect("last child").to_vec(), ["last"]);
        drop(retained);

        let row_values = vec![
            (EcoString::from("A"), BigInt::from(2)),
            ("B".into(), 3.into()),
        ];
        assert_eq!(
            module
                .call(&rows, (row_values.clone(),), &mut echo)
                .expect("tuple items")
                .to_vec(),
            row_values
        );
        let checks: Vec<CheckedRow> = vec![
            Ok(("A".into(), 2.into())),
            Err("invalid".into()),
            Ok(("C".into(), 5.into())),
        ];
        let results = module
            .call(&checked, (checks.clone(),), &mut echo)
            .expect("Result items");
        assert_eq!(results.to_vec(), checks);
        assert_eq!(
            module
                .call(&checked, (&results,), &mut echo)
                .expect("Result pass through")
                .to_vec(),
            checks
        );

        let groups = module
            .call(
                &grouped,
                (vec![
                    (
                        BigInt::from(1),
                        vec![Ok(BigInt::from(7)), Err(EcoString::from("bad"))],
                    ),
                    (2.into(), vec![]),
                ],),
                &mut echo,
            )
            .expect("recursive grouped items");
        let group = groups.get(0).expect("first group");
        assert_eq!(group.0, BigInt::from(1));
        assert_eq!(group.1.to_vec(), [Ok(BigInt::from(7)), Err("bad".into())]);
        drop(groups);
        drop(values);
        drop(module);
        drop(echo);
        assert_eq!(first.to_vec(), ["first"]);
        assert!(empty.is_empty());
        assert_eq!(group.1.get(0), Some(Ok(BigInt::from(7))));
    }

    #[test]
    fn rejects_foreign_retained_inputs_before_execution_and_allows_explicit_copy() {
        let source = r#"
pub fn values(input: List(Int)) { input }
pub fn inspect(input: List(Int)) {
  echo "entered"
  case input { [] -> 0 [first, ..] -> first }
}
pub fn optional(input: Result(List(Int), String), fallback: List(Int)) {
  echo "optional"
  case input { Ok(values) -> inspect(values) Error(_) -> inspect(fallback) }
}
"#;
        let (bindings, own_values) = library(source)
            .function(FunctionDeclaration::<(List<BigInt>,), List<BigInt>>::new(
                "values",
            ))
            .expect("first owner");
        let owner = bindings.seal();
        let values = owner
            .call(&own_values, (vec![BigInt::from(8)],), &mut Vec::new())
            .expect("owned List");

        let (mut bindings, target_values) = library(source)
            .function(FunctionDeclaration::<(List<BigInt>,), List<BigInt>>::new(
                "values",
            ))
            .expect("second owner");
        let inspect = bind::<(List<BigInt>,), BigInt>(&mut bindings, "inspect");
        let optional = bind::<(Result<List<BigInt>, EcoString>, List<BigInt>), BigInt>(
            &mut bindings,
            "optional",
        );
        let target = bindings.seal();
        let mut echo = Vec::new();
        assert_eq!(
            target.call(&inspect, (&values,), &mut echo),
            Err(CallError::ForeignValue)
        );
        assert!(echo.is_empty());
        let accepted = target
            .call(&target_values, (values.to_vec(),), &mut echo)
            .expect("explicit cross-owner copy");
        let branch: Result<&List<BigInt>, EcoString> = Ok(&values);
        assert_eq!(
            target.call(&optional, (branch, &accepted), &mut echo),
            Err(CallError::ForeignValue)
        );
        let branch: Result<Vec<BigInt>, EcoString> = Err("use fallback".into());
        assert_eq!(
            target.call(&optional, (branch, &values), &mut echo),
            Err(CallError::ForeignValue)
        );
        assert!(echo.is_empty());
        assert_eq!(
            target.call(&inspect, (&accepted,), &mut echo),
            Ok(BigInt::from(8))
        );
        assert_eq!(echo.len(), 1);
        assert_eq!(values.value.item_reads(), 1);
        assert_eq!(accepted.value.item_reads(), 0);
    }
}
