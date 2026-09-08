use super::ScopeBrand;
use crate::host::{HostExternalSchema, HostFutureStore};
use crate::runtime::shared::Shared;
use crate::runtime::{BorrowedValue, EmbeddingList, EvaluatedExternalValue, StoredRuntimeValue};
use std::marker::PhantomData;
use std::sync::Arc;

/// A Future type declaration with nominal identity selected by the host profile.
pub struct FutureType<Value, Schema: HostExternalSchema>(PhantomData<fn() -> (Value, Schema)>);

/// One source operation, owned by its original attached execution.
///
/// Work from another execution cannot be used as an input:
///
/// ```compile_fail
/// use geam_core::embedding::{BigInt, ExecutionScope, Function, Future, FutureType};
/// use geam_core::host::{HostWorkProfile, HostWorkSchema};
/// fn foreign<'a, 'b, P: HostWorkProfile>(
///     scope: &mut ExecutionScope<'a, '_, P>,
///     function: &Function<(FutureType<BigInt, HostWorkSchema<P>>,), FutureType<BigInt, HostWorkSchema<P>>>,
///     work: Future<'b, BigInt, HostWorkSchema<P>>,
/// ) {
///     let _ = scope.call(function, (work,));
/// }
/// ```
///
/// The same ownership holds inside retained lists:
///
/// ```compile_fail
/// use geam_core::embedding::{List, BigInt, ExecutionScope, Function, Future, FutureType, SharedList};
/// use geam_core::host::{HostWorkProfile, HostWorkSchema};
/// fn foreign<'a, 'b, P: HostWorkProfile>(
///     scope: &mut ExecutionScope<'a, '_, P>,
///     function: &Function<(List<FutureType<BigInt, HostWorkSchema<P>>>,), List<FutureType<BigInt, HostWorkSchema<P>>>>,
///     values: &SharedList<Future<'b, BigInt, HostWorkSchema<P>>>,
/// ) {
///     let _ = scope.call(function, (values,));
/// }
/// ```
#[allow(private_bounds)]
pub struct Future<'scope, Value: SharedValue, Schema: HostExternalSchema> {
    pub(in crate::embedding) value: EvaluatedExternalValue,
    pub(in crate::embedding) context: FutureContext<'scope, Value, Schema>,
}

/// A shared successful completion, read without requiring payload cloning.
///
/// Completing an outer operation does not extend nested work's lifetime:
///
/// ```compile_fail
/// use geam_core::embedding::{BigInt, Completed, Future, SharedList};
/// use geam_core::host::HostExternalSchema;
/// fn escape<'scope, S: HostExternalSchema>(value: Completed<SharedList<Future<'scope, BigInt, S>>>)
///     -> Completed<SharedList<Future<'static, BigInt, S>>>
/// {
///     value
/// }
/// ```
#[allow(private_bounds)]
pub struct Completed<Value: SharedValue> {
    value: Shared<StoredRuntimeValue>,
    context: Value::Context,
}

/// A lazy list read from a shared completion.
#[allow(private_bounds)]
pub struct SharedList<Value: SharedValue> {
    value: Shared<EmbeddingList>,
    context: ListContext<Value>,
}

/// The Rust value returned by a source type within an attached execution.
#[allow(private_bounds)]
pub trait SourceType: crate::embedding::value::EmbeddingValue {
    /// Preserves execution ownership recursively, including nested Futures.
    type Value<'scope>: SharedValue;
}

/// The borrowed view of a supported shared completion value.
pub trait ReadValue: sealed::Value {
    /// Reading borrows scalar payloads and retains lazy lists or work handles.
    type View<'value>;
}

mod sealed {
    pub trait Value {}
    impl<T: super::SharedValue> Value for T {}
}

pub(crate) trait SharedValue: ReadValue {
    type Context: Clone + Send;

    fn view<'value>(value: BorrowedValue<'value>, context: &Self::Context) -> Self::View<'value>;
}

pub(crate) trait ScopedOutput<Schema: HostExternalSchema>: SourceType {
    fn context<'scope>(
        brand: ScopeBrand<'scope>,
        store: &HostFutureStore,
        owner: &Arc<()>,
    ) -> <Self::Value<'scope> as SharedValue>::Context;
}

impl<Value: crate::embedding::value::EmbeddingValue, Schema: HostExternalSchema>
    crate::embedding::value::EmbeddingValue for FutureType<Value, Schema>
{
    const VARIANT_COUNT: usize = 0;
    const LIST_COUNTS: [usize; 11] = [0; 11];
    const LIST_FAMILY: crate::embedding::input::ListFamily =
        crate::embedding::input::ListFamily::External;

    fn library_type() -> crate::plan::LibraryValueType {
        crate::plan::LibraryValueType::External(crate::plan::ExternalType::new(
            crate::plan::ExternalTypeName::new(
                Schema::PACKAGE.into(),
                Schema::MODULE.into(),
                Schema::NAME.into(),
            ),
            vec![Value::value_type()],
        ))
    }

    fn collect_variants(variants: &mut Vec<crate::plan::StandardVariant>) {
        Value::collect_variants(variants);
    }
    fn collect_input_variants(_: &mut Vec<crate::plan::StandardVariant>) {}
    fn collect_lists(_: &mut Vec<crate::plan::LibraryValueType>) {}
}

pub(crate) struct FutureContext<'scope, Value: SharedValue, Schema: HostExternalSchema> {
    brand: ScopeBrand<'scope>,
    store: HostFutureStore,
    output: Value::Context,
    schema: PhantomData<fn() -> Schema>,
}

pub(crate) struct ListContext<Value: SharedValue> {
    owner: Arc<()>,
    item: Value::Context,
}

impl<Value: SharedValue> Clone for ListContext<Value> {
    fn clone(&self) -> Self {
        Self {
            owner: self.owner.clone(),
            item: self.item.clone(),
        }
    }
}

#[allow(private_bounds)]
impl<'scope, Value: SharedValue, Schema: HostExternalSchema> Future<'scope, Value, Schema> {
    pub(in crate::embedding) fn new(
        value: EvaluatedExternalValue,
        context: FutureContext<'scope, Value, Schema>,
    ) -> Self {
        Self { value, context }
    }

    pub(in crate::embedding) fn work(&self) -> crate::runtime::work::execution::SourceWork {
        self.context.store.work(self.value.lease())
    }

    pub(in crate::embedding) fn output_context(&self) -> Value::Context {
        self.context.output.clone()
    }
}

impl<Value: SharedValue, Schema: HostExternalSchema> Clone for Future<'_, Value, Schema> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            context: self.context.clone(),
        }
    }
}

#[allow(private_bounds)]
impl<Value: SharedValue> Completed<Value> {
    pub(in crate::embedding) fn new(
        value: Shared<StoredRuntimeValue>,
        context: Value::Context,
    ) -> Self {
        Self { value, context }
    }

    /// Borrows this operation's same cached result on every read.
    pub fn read<Output>(
        &self,
        read: impl for<'value> FnOnce(Value::View<'value>) -> Output,
    ) -> Output {
        self.value.read(|value| {
            read(Value::view(
                BorrowedValue::from_stored(value),
                &self.context,
            ))
        })
    }
}

impl<Value: SharedValue> Clone for Completed<Value> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            context: self.context.clone(),
        }
    }
}

#[allow(private_bounds)]
impl<Value: SharedValue> SharedList<Value> {
    pub(in crate::embedding) fn new(value: EmbeddingList, context: ListContext<Value>) -> Self {
        Self {
            value: Shared::new(value),
            context,
        }
    }
    /// Returns the source list length without decoding its items.
    pub fn len(&self) -> usize {
        self.value.read(EmbeddingList::len)
    }

    /// Returns whether the source list is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Reads one item through the list's retained runtime storage.
    pub fn read_item<Output>(
        &self,
        index: usize,
        read: impl for<'value> FnOnce(Value::View<'value>) -> Output,
    ) -> Option<Output> {
        self.value.read(|list| {
            list.read_item(index, |value| read(Value::view(value, &self.context.item)))
        })
    }

    pub(in crate::embedding) fn owners_match(&self, owner: &Arc<()>) -> bool {
        Arc::ptr_eq(&self.context.owner, owner)
    }

    pub(in crate::embedding) fn input(&self) -> crate::runtime::EmbeddingListInput {
        self.value.read(EmbeddingList::input)
    }
}

impl<Value: SharedValue> Clone for SharedList<Value> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            context: self.context.clone(),
        }
    }
}

macro_rules! scalar {
    ($type:ty, $view:ty, $method:ident) => {
        impl ReadValue for $type {
            type View<'value> = $view;
        }
        impl SharedValue for $type {
            type Context = ();
            fn view<'value>(value: BorrowedValue<'value>, _: &()) -> Self::View<'value> {
                value.$method()
            }
        }
        impl SourceType for $type {
            type Value<'scope> = Self;
        }
        impl<Schema: HostExternalSchema> ScopedOutput<Schema> for $type {
            fn context<'scope>(_: ScopeBrand<'scope>, _: &HostFutureStore, _: &Arc<()>) {}
        }
    };
}

scalar!(super::super::BigInt, &'value super::super::BigInt, int);
scalar!(f64, f64, float);
scalar!(
    super::super::EcoString,
    &'value super::super::EcoString,
    string
);
scalar!(
    crate::BitArrayValue,
    &'value crate::BitArrayValue,
    bit_array
);
scalar!(char, char, utf_codepoint);
scalar!(bool, bool, bool);

impl ReadValue for () {
    type View<'value> = ();
}

impl SharedValue for () {
    type Context = ();
    fn view(_: BorrowedValue<'_>, _: &()) {}
}

impl SourceType for () {
    type Value<'scope> = Self;
}

impl<Schema: HostExternalSchema> ScopedOutput<Schema> for () {
    fn context<'scope>(_: ScopeBrand<'scope>, _: &HostFutureStore, _: &Arc<()>) {}
}

macro_rules! tuple {
    ($($index:tt: $type:ident),+) => {
        impl<$($type: SharedValue),+> ReadValue for ($($type,)+) {
            type View<'value> = ($($type::View<'value>,)+);
        }
        impl<$($type: SharedValue),+> SharedValue for ($($type,)+) {
            type Context = ($($type::Context,)+);
            fn view<'value>(value: BorrowedValue<'value>, context: &Self::Context) -> Self::View<'value> {
                ($($type::view(value.tuple_item($index), &context.$index),)+)
            }
        }
        impl<$($type: SourceType),+> SourceType for ($($type,)+) {
            type Value<'scope> = ($($type::Value<'scope>,)+);
        }
        impl<Schema: HostExternalSchema, $($type: ScopedOutput<Schema>),+> ScopedOutput<Schema> for ($($type,)+) {
            fn context<'scope>(brand: ScopeBrand<'scope>, store: &HostFutureStore, owner: &Arc<()>) -> <Self::Value<'scope> as SharedValue>::Context {
                ($($type::context(brand, store, owner),)+)
            }
        }
    };
}

tuple!(0:A);
tuple!(0:A, 1:B);
tuple!(0:A, 1:B, 2:C);
tuple!(0:A, 1:B, 2:C, 3:D);
tuple!(0:A, 1:B, 2:C, 3:D, 4:E);
tuple!(0:A, 1:B, 2:C, 3:D, 4:E, 5:F);
tuple!(0:A, 1:B, 2:C, 3:D, 4:E, 5:F, 6:G);

impl<Success: SharedValue, Failure: SharedValue> ReadValue for Result<Success, Failure> {
    type View<'value> = Result<Success::View<'value>, Failure::View<'value>>;
}

impl<Success: SharedValue, Failure: SharedValue> SharedValue for Result<Success, Failure> {
    type Context = (Success::Context, Failure::Context);
    fn view<'value>(value: BorrowedValue<'value>, context: &Self::Context) -> Self::View<'value> {
        if value.variant() == 0 {
            Ok(Success::view(value.custom_field(0), &context.0))
        } else {
            Err(Failure::view(value.custom_field(0), &context.1))
        }
    }
}

impl<Success: SourceType, Failure: SourceType> SourceType for Result<Success, Failure> {
    type Value<'scope> = Result<Success::Value<'scope>, Failure::Value<'scope>>;
}

impl<Schema: HostExternalSchema, Success: ScopedOutput<Schema>, Failure: ScopedOutput<Schema>>
    ScopedOutput<Schema> for Result<Success, Failure>
{
    fn context<'scope>(
        brand: ScopeBrand<'scope>,
        store: &HostFutureStore,
        owner: &Arc<()>,
    ) -> <Self::Value<'scope> as SharedValue>::Context {
        (
            Success::context(brand, store, owner),
            Failure::context(brand, store, owner),
        )
    }
}

impl<Value: SharedValue> ReadValue for Option<Value> {
    type View<'value> = Option<Value::View<'value>>;
}

impl<Value: SharedValue> SharedValue for Option<Value> {
    type Context = Value::Context;
    fn view<'value>(value: BorrowedValue<'value>, context: &Self::Context) -> Self::View<'value> {
        if value.variant() == 0 {
            Some(Value::view(value.custom_field(0), context))
        } else {
            None
        }
    }
}

impl<Value: SourceType> SourceType for Option<Value> {
    type Value<'scope> = Option<Value::Value<'scope>>;
}

impl<Schema: HostExternalSchema, Value: ScopedOutput<Schema>> ScopedOutput<Schema>
    for Option<Value>
{
    fn context<'scope>(
        brand: ScopeBrand<'scope>,
        store: &HostFutureStore,
        owner: &Arc<()>,
    ) -> <Self::Value<'scope> as SharedValue>::Context {
        Value::context(brand, store, owner)
    }
}

impl<Value: SharedValue> ReadValue for SharedList<Value> {
    type View<'value> = Self;
}

impl<Value: SharedValue> SharedValue for SharedList<Value> {
    type Context = ListContext<Value>;
    fn view(value: BorrowedValue<'_>, context: &Self::Context) -> Self {
        Self {
            value: Shared::new(EmbeddingList::from_borrowed(value)),
            context: context.clone(),
        }
    }
}

impl<Value: SourceType> SourceType for super::super::List<Value> {
    type Value<'scope> = SharedList<Value::Value<'scope>>;
}

impl<Schema: HostExternalSchema, Value: ScopedOutput<Schema>> ScopedOutput<Schema>
    for super::super::List<Value>
{
    fn context<'scope>(
        brand: ScopeBrand<'scope>,
        store: &HostFutureStore,
        owner: &Arc<()>,
    ) -> <Self::Value<'scope> as SharedValue>::Context {
        ListContext {
            owner: owner.clone(),
            item: Value::context(brand, store, owner),
        }
    }
}

impl<Value: SharedValue, Schema: HostExternalSchema> Clone for FutureContext<'_, Value, Schema> {
    fn clone(&self) -> Self {
        Self {
            brand: self.brand,
            store: self.store.clone_handle(),
            output: self.output.clone(),
            schema: PhantomData,
        }
    }
}

impl<Value: SharedValue, Schema: HostExternalSchema> ReadValue for Future<'_, Value, Schema> {
    type View<'value> = Self;
}

impl<'scope, Value: SharedValue, Schema: HostExternalSchema> SharedValue
    for Future<'scope, Value, Schema>
{
    type Context = FutureContext<'scope, Value, Schema>;
    fn view(value: BorrowedValue<'_>, context: &Self::Context) -> Self {
        Self {
            value: value.external().clone(),
            context: context.clone(),
        }
    }
}

impl<Value: SourceType, Schema: HostExternalSchema> SourceType for FutureType<Value, Schema> {
    type Value<'scope> = Future<'scope, Value::Value<'scope>, Schema>;
}

impl<Value: ScopedOutput<Schema>, Schema: HostExternalSchema> ScopedOutput<Schema>
    for FutureType<Value, Schema>
{
    fn context<'scope>(
        brand: ScopeBrand<'scope>,
        store: &HostFutureStore,
        owner: &Arc<()>,
    ) -> <Self::Value<'scope> as SharedValue>::Context {
        FutureContext {
            brand,
            store: store.clone_handle(),
            output: Value::context(brand, store, owner),
            schema: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::embedding::{FunctionDeclaration, HostedModuleBuilder, List, with_execution_scope};
    use crate::frontend::compile_typed_host_program;
    use crate::host::{HostComponentProfile, HostFutureStore, HostProfile, HostProviderSet};
    use crate::work_fixture::WorkComponent;
    use crate::work_fixture::WorkType;
    use crate::{EchoOutput, EchoSink, ModuleSource, PackageSource};
    use ecow::EcoString;
    use futures_util::FutureExt;

    struct Profile;
    impl HostProfile for Profile {
        type RunState = ();
        type ExternalStores = HostFutureStore;
    }
    impl crate::host::HostWorkProfile for Profile {
        type Work = crate::work_fixture::WorkComponent;
    }
    impl HostComponentProfile<WorkComponent> for Profile {
        fn component_stores(stores: &HostFutureStore) -> &HostFutureStore {
            stores
        }
        fn component_state(state: &mut ()) -> &mut () {
            state
        }
    }
    #[derive(Default)]
    struct Echo(Vec<String>);
    impl EchoSink for Echo {
        fn emit(&mut self, value: EchoOutput) {
            self.0.push(value.to_string());
        }
    }

    #[test]
    fn retained_lists_share_source_storage_and_borrow_items_after_the_owner_drops() {
        let program = compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "library",
                    "src/library.gleam",
                    "pub fn keep(values: List(String)) { values }",
                )],
            )],
            HostProviderSet::<Profile>::from_providers([]).expect("empty providers"),
        )
        .expect("typed source");
        let (bindings, keep) = HostedModuleBuilder::new(program)
            .expect("plan")
            .function(FunctionDeclaration::<(List<EcoString>,), List<EcoString>>::new("keep"))
            .expect("list binding");
        let mut module = bindings.seal().expect("list seal");
        let mut state = ();
        let mut echo = Echo::default();
        let (list, retained) = with_execution_scope(async |guard| {
            let mut scope = module.attach(guard, &mut state, &mut echo);
            let list = scope
                .call(&keep, (vec![EcoString::from("first"), "second".into()],))
                .expect("fresh list");
            assert_eq!(list.len(), 2);
            assert!(!list.is_empty());
            let retained = scope.call(&keep, (&list,)).expect("retained input");
            list.value.read(|original| {
                retained.value.read(|returned| {
                    assert!(original.same_allocation(returned));
                    assert_eq!(original.item_reads(), 0);
                    assert_eq!(returned.item_reads(), 0);
                })
            });
            (list, retained)
        })
        .now_or_never()
        .expect("direct list calls");
        drop(module);
        let alias = retained.clone();
        list.read_item(1, |first| {
            retained
                .read_item(1, |second| assert!(std::ptr::eq(first, second)))
                .expect("same item");
        })
        .expect("second item");
        assert_eq!(list.read_item(2, |_| ()), None);
        drop(list);
        drop(retained);
        let text = std::thread::spawn(move || {
            assert_eq!(alias.len(), 2);
            alias
                .read_item(1, Clone::clone)
                .expect("escaped item on another worker")
        })
        .join()
        .expect("worker");
        assert_eq!(text, "second");
    }

    #[test]
    fn optional_results_and_nested_lists_have_the_same_direct_and_shared_views() {
        use num_bigint::BigInt;
        type Choice = Option<Result<BigInt, ()>>;
        type Choices = List<Choice>;
        let program = compile_typed_host_program(
            "application",
            "library",
            [
                PackageSource::new(
                    "gleam_stdlib",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "gleam/option",
                        "src/gleam/option.gleam",
                        "pub type Option(a) { Some(a) None }",
                    )],
                ),
                PackageSource::new(
                    "work_fixture",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "fixture/work",
                        "src/fixture/work.gleam",
                        WorkComponent::SOURCE,
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["gleam_stdlib", "work_fixture"],
                    [ModuleSource::new(
                        "library",
                        "src/library.gleam",
                        r#"
import gleam/option.{type Option}
import fixture/work as future
pub fn choice(value: Option(Result(Int, Nil))) { value }
pub fn choices(values: List(Option(Result(Int, Nil)))) { values }
pub fn ready(value: Option(Result(Int, Nil))) { future.ready(value) }
pub fn ready_list(values: List(Option(Result(Int, Nil)))) { future.ready(values) }
"#,
                    )],
                ),
            ],
            HostProviderSet::from_providers(
                WorkComponent::providers::<Profile>().expect("Future component"),
            )
            .expect("providers"),
        )
        .expect("ordinary source");
        let (mut bindings, choice) = HostedModuleBuilder::new(program)
            .expect("plan")
            .function(FunctionDeclaration::<(Choice,), Choice>::new("choice"))
            .expect("optional result");
        let choices = bindings
            .function(FunctionDeclaration::<(Choices,), Choices>::new("choices"))
            .expect("optional result list");
        let ready = bindings
            .function(FunctionDeclaration::<(Choice,), WorkType<Choice>>::new(
                "ready",
            ))
            .expect("shared optional result");
        let ready_list = bindings
            .function(FunctionDeclaration::<(Choices,), WorkType<Choices>>::new(
                "ready_list",
            ))
            .expect("shared list");
        let mut module = bindings.seal().expect("specializations");
        let mut state = ();
        let mut echo = Echo::default();
        let returned = with_execution_scope(async |guard| {
            let mut scope = module.attach(guard, &mut state, &mut echo);
            let inputs = vec![Some(Ok(BigInt::from(42))), Some(Err(())), None];
            for input in &inputs {
                assert_eq!(
                    scope
                        .call(&choice, (input.clone(),))
                        .expect("direct choice"),
                    *input
                );
                let work = scope
                    .call(&ready, (input.clone(),))
                    .expect("work construction");
                let completed = scope.observe(&work).await.expect("shared choice");
                assert_eq!(
                    completed.read(|value| value.map(|result| result.cloned())),
                    *input
                );
            }
            let list = scope
                .call(&choices, (inputs.clone(),))
                .expect("direct list");
            for (index, input) in inputs.iter().enumerate() {
                assert_eq!(
                    list.read_item(index, |value| value.map(|result| result.cloned())),
                    Some(input.clone())
                );
            }
            let work = scope.call(&ready_list, (&list,)).expect("same-owner list");
            scope.observe(&work).await.expect("shared list").clone()
        })
        .now_or_never()
        .expect("source-only work completes");
        drop(module);
        assert_eq!(
            returned.read(|list| list.read_item(0, |value| value.map(|result| result.cloned()))),
            Some(Some(Ok(BigInt::from(42))))
        );
    }

    #[test]
    fn shared_scalar_lists_and_nested_work_keep_their_typed_borrowed_views() {
        use crate::BitArrayValue;
        use crate::host::HostProvider;
        use num_bigint::BigInt;
        type Values = (
            f64,
            BitArrayValue,
            char,
            bool,
            List<f64>,
            List<BitArrayValue>,
            (List<char>, List<bool>, WorkType<BigInt>),
        );
        let program = compile_typed_host_program(
            "application", "library",
            [
                PackageSource::new("work_fixture", Vec::<String>::new(), [
                    ModuleSource::new("fixture/work", "src/fixture/work.gleam", WorkComponent::SOURCE),
                ]),
                PackageSource::new("application", ["work_fixture"], [ModuleSource::new(
                    "library", "src/library.gleam",
                    "import fixture/work as future\npub fn ready_int(value: Int) { echo \"ready\" future.ready(value) }\npub fn ready_values(value: #(Float, BitArray, UtfCodepoint, Bool, List(Float), List(BitArray), #(List(UtfCodepoint), List(Bool), future.Work(Int)))) { echo \"ready\" future.ready(value) }",
                )]),
            ],
            HostProviderSet::from_providers(WorkComponent::providers::<Profile>().expect("Future module"))
                .expect("providers"),
        ).expect("ordinary concrete source");
        let (mut bindings, ready_int) = HostedModuleBuilder::new(program)
            .expect("plan")
            .function(FunctionDeclaration::<(BigInt,), WorkType<BigInt>>::new(
                "ready_int",
            ))
            .expect("scalar completion");
        let ready_values = bindings
            .function(FunctionDeclaration::<(Values,), WorkType<Values>>::new(
                "ready_values",
            ))
            .expect("recursive completion");
        let mut module = bindings.seal().expect("specializations");
        let mut state = ();
        assert!(std::ptr::eq(
            <WorkComponent as HostProvider<Profile>>::project(&mut state),
            &state,
        ));
        let mut echo = Echo::default();
        with_execution_scope(async |guard| {
            let mut scope = module.attach(guard, &mut state, &mut echo);
            let number = scope
                .call(&ready_int, (BigInt::from(42),))
                .expect("inner work");
            let bits = BitArrayValue::try_from_parts(vec![0b1010_0000], 3).expect("three bits");
            let work = scope
                .call(
                    &ready_values,
                    ((
                        3.5,
                        bits.clone(),
                        'x',
                        true,
                        vec![3.5],
                        vec![bits.clone()],
                        (vec!['x'], vec![true], &number),
                    ),),
                )
                .expect("recursive work");
            let completed = scope.observe(&work).await.expect("shared completion");
            let nested = completed.read(
                |(
                    float,
                    bit_array,
                    codepoint,
                    boolean,
                    floats,
                    bit_arrays,
                    (codepoints, bools, nested),
                )| {
                    assert_eq!(float, 3.5);
                    assert_eq!(bit_array, &bits);
                    assert_eq!(codepoint, 'x');
                    assert!(boolean);
                    assert_eq!(floats.read_item(0, std::convert::identity), Some(3.5));
                    assert_eq!(floats.read_item(1, std::convert::identity), None);
                    let same_bits = |value: &BitArrayValue| value == &bits;
                    assert_eq!(bit_arrays.read_item(0, same_bits), Some(true));
                    assert_eq!(bit_arrays.read_item(1, same_bits), None);
                    assert_eq!(codepoints.read_item(0, std::convert::identity), Some('x'));
                    assert_eq!(codepoints.read_item(1, std::convert::identity), None);
                    assert_eq!(bools.read_item(0, std::convert::identity), Some(true));
                    assert_eq!(bools.read_item(1, std::convert::identity), None);
                    nested
                },
            );
            scope
                .observe(&nested)
                .await
                .expect("same nested operation")
                .read(|value| assert_eq!(value, &BigInt::from(42)));
        })
        .now_or_never()
        .expect("ready compositions");
        assert_eq!(
            echo.0,
            [
                "src/library.gleam:2\n\"ready\"",
                "src/library.gleam:3\n\"ready\""
            ]
        );
    }

    #[test]
    fn every_transferable_list_entry_preserves_its_family_and_accepts_same_owner_inputs() {
        use crate::BitArrayValue;
        use num_bigint::BigInt;

        let program = compile_typed_host_program(
            "application",
            "library",
            [
                PackageSource::new(
                    "work_fixture",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "fixture/work",
                        "src/fixture/work.gleam",
                        WorkComponent::SOURCE,
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["work_fixture"],
                    [ModuleSource::new(
                        "library",
                        "src/library.gleam",
                        r#"
import fixture/work as future
pub fn ready(value: Int) { future.ready(value) }
pub fn ints(values: List(Int)) { values }
pub fn floats(values: List(Float)) { values }
pub fn strings(values: List(String)) { values }
pub fn bits(values: List(BitArray)) { values }
pub fn codepoints(values: List(UtfCodepoint)) { values }
pub fn bools(values: List(Bool)) { values }
pub fn nils(values: List(Nil)) { values }
pub fn tuples(values: List(#(Int, String))) { values }
pub fn choices(values: List(Result(Int, String))) { values }
pub fn nested(values: List(List(Int))) { values }
pub fn work(values: List(future.Work(Int))) { values }
"#,
                    )],
                ),
            ],
            HostProviderSet::from_providers(
                WorkComponent::providers::<Profile>().expect("Future module"),
            )
            .expect("providers"),
        )
        .expect("ordinary source");
        let (mut bindings, ready) = HostedModuleBuilder::new(program)
            .expect("plan")
            .function(FunctionDeclaration::<(BigInt,), WorkType<BigInt>>::new(
                "ready",
            ))
            .expect("work constructor");
        macro_rules! bind_list {
            ($name:ident, $item:ty) => {
                let $name = bindings
                    .function(FunctionDeclaration::<(List<$item>,), List<$item>>::new(
                        stringify!($name),
                    ))
                    .expect("list entry");
            };
        }
        bind_list!(ints, BigInt);
        bind_list!(floats, f64);
        bind_list!(strings, EcoString);
        bind_list!(bits, BitArrayValue);
        bind_list!(codepoints, char);
        bind_list!(bools, bool);
        bind_list!(nils, ());
        bind_list!(tuples, (BigInt, EcoString));
        bind_list!(choices, Result<BigInt, EcoString>);
        bind_list!(nested, List<BigInt>);
        bind_list!(work, WorkType<BigInt>);
        let mut module = bindings.seal().expect("all list families seal together");
        let mut state = ();
        let mut echo = Echo::default();
        with_execution_scope(async |guard| {
            let mut scope = module.attach(guard, &mut state, &mut echo);
            let numbers = scope
                .call(&ints, (vec![BigInt::from(42)],))
                .expect("integer list");
            assert_eq!(numbers.read_item(0, Clone::clone), Some(BigInt::from(42)));
            assert_eq!(
                scope
                    .call(&floats, (vec![3.5],))
                    .expect("float list")
                    .read_item(0, std::convert::identity),
                Some(3.5)
            );
            assert_eq!(
                scope
                    .call(&strings, (vec![EcoString::from("hello")],))
                    .expect("string list")
                    .read_item(0, Clone::clone),
                Some(EcoString::from("hello"))
            );
            let bytes = BitArrayValue::try_from_parts(vec![0xa0], 3).expect("three bits");
            assert_eq!(
                scope
                    .call(&bits, (vec![bytes.clone()],))
                    .expect("bit array list")
                    .read_item(0, Clone::clone),
                Some(bytes)
            );
            assert_eq!(
                scope
                    .call(&codepoints, (vec!['x'],))
                    .expect("codepoint list")
                    .read_item(0, std::convert::identity),
                Some('x')
            );
            assert_eq!(
                scope
                    .call(&bools, (vec![true],))
                    .expect("bool list")
                    .read_item(0, std::convert::identity),
                Some(true)
            );
            assert_eq!(
                scope
                    .call(&nils, (vec![()],))
                    .expect("nil list")
                    .read_item(0, std::convert::identity),
                Some(())
            );
            assert_eq!(
                scope
                    .call(
                        &tuples,
                        (vec![(BigInt::from(42), EcoString::from("hello"))],)
                    )
                    .expect("tuple list")
                    .read_item(0, |(number, text)| (number.clone(), text.clone())),
                Some((BigInt::from(42), EcoString::from("hello")))
            );
            assert_eq!(
                scope
                    .call(&choices, (vec![Ok(BigInt::from(42))],))
                    .expect("custom list")
                    .read_item(0, |value| value.cloned().map_err(Clone::clone)),
                Some(Ok(BigInt::from(42)))
            );
            let nested_numbers = scope
                .call(&nested, (vec![vec![BigInt::from(42)]],))
                .expect("nested list");
            assert_eq!(
                scope
                    .call(&nested, (&nested_numbers,))
                    .expect("retained nested list")
                    .read_item(0, |list| list.read_item(0, Clone::clone)),
                Some(Some(BigInt::from(42)))
            );
            let operation = scope.call(&ready, (BigInt::from(42),)).expect("work");
            let operations = scope
                .call(&work, (vec![&operation],))
                .expect("external list");
            let repeated = scope
                .call(&work, (&operations,))
                .expect("retained external list");
            let operation = repeated
                .read_item(0, std::convert::identity)
                .expect("same work");
            scope
                .observe(&operation)
                .await
                .expect("completion")
                .read(|value| assert_eq!(value, &BigInt::from(42)));
        })
        .now_or_never()
        .expect("ready work is caller-driven");
        assert!(echo.0.is_empty());
    }
}
