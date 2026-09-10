#[geam_macros::provider(
    package = "async_provider",
    state = BigInt,
    modules = [declarations, native],
    crate_path = geam_core
)]
pub struct Component;

#[geam_macros::module(
    path = "async_provider/declarations",
    crate_path = geam_core
)]
mod declarations {
    use ecow::EcoString;

    #[geam_macros::external(name = "Token")]
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Token(pub(super) EcoString);
}

#[geam_macros::module(
    path = "async_provider/native",
    crate_path = geam_core
)]
mod native {
    use super::declarations;
    use ecow::EcoString;
    use geam_core::BitArrayValue;
    use geam_core::provider::{
        BigInt, Call, Callback, ExternalPayload, Future, HostFailure, HostResult, Stored, Value,
    };
    use geam_core::provider::advanced::{
        Equality, Hashing, Index0, Inspection, Retained, RetainedExternalPayload,
    };
    use std::cell::Cell;
    use std::future::poll_fn;
    use std::task::Poll;

    type Number = BigInt;

    #[geam_macros::external(name = "Counter", manual)]
    pub(super) struct Counter {
        value: Cell<usize>,
    }

    impl ExternalPayload for Counter {
        fn source_equal(&self, other: &Self) -> bool {
            self.value.get() == other.value.get()
        }

        fn source_hash(&self) -> u64 {
            self.value.get() as u64
        }

        fn inspect(&self) -> EcoString {
            format!("Counter({})", self.value.get()).into()
        }
    }

    #[geam_macros::custom(input = CounterEnvelopeInput)]
    enum CounterEnvelope {
        Wrapped(Counter),
    }

    #[geam_macros::custom(input = BatchInput)]
    enum Batch {
        Numbers(Vec<BigInt>),
        Pairs(Vec<(EcoString, BigInt)>),
        Counters(Vec<Counter>),
    }

    #[geam_macros::function]
    fn is_numbers(value: BatchInput) -> bool {
        matches!(value, BatchInput::Numbers(_))
    }

    #[geam_macros::function]
    async fn is_numbers_after(value: BatchInput) -> bool {
        matches!(value, self::BatchInput::Numbers(_))
    }

    #[geam_macros::external(name = "Box", parameters = [Item], input = BoxInput)]
    struct BoxValue<Item> {
        #[geam_macros::stored]
        value: Stored<Item>,
    }

    pub struct ManualPayload {
        value: Retained<ManualPayload, Index0>,
    }

    impl RetainedExternalPayload for ManualPayload {
        fn source_equal(&self, context: &Equality<'_>, other: &Self) -> bool {
            self.value.source_equal(context, &other.value)
        }
        fn source_hash(&self, context: &Hashing<'_>) -> u64 {
            self.value.source_hash(context)
        }
        fn inspect(&self, context: &Inspection<'_>) -> EcoString {
            format!("Manual({})", self.value.inspect(context)).into()
        }
    }

    #[geam_macros::external(
        name = "Manual", parameters = [Item], input = ManualInput,
        payload = ManualPayload, manual,
    )]
    pub struct ManualValue<Item>;

    #[geam_macros::function]
    fn manual_new<Item>(#[geam_macros::call] call: &mut Call<BigInt>, value: Value<Item>) -> ManualValue<Item> {
        ManualValue::from_payload(ManualPayload { value: call.store(value).into_retained() })
    }

    #[geam_macros::function]
    fn manual_keep<Item>(value: ManualInput<Item>) -> ManualValue<Item> {
        value.into_value()
    }

    #[geam_macros::function(resumable)]
    async fn manual_callback<Item>(
        #[geam_macros::call] call: &mut Call<BigInt>,
        value: ManualInput<Item>,
        callback: Callback<fn(ManualValue<Item>) -> bool>,
    ) -> HostResult<bool> {
        call.invoke(&callback, (value.into_value(),)).await
    }

    #[geam_macros::function]
    async fn manual_callback_after<Item>(
        #[geam_macros::call] call: &mut Call<BigInt>,
        value: ManualInput<Item>,
        callback: Callback<fn(ManualValue<Item>) -> bool>,
    ) -> HostResult<bool> {
        let mut first = true;
        poll_fn(|context| {
            if std::mem::take(&mut first) {
                context.waker().wake_by_ref();
                Poll::Pending
            } else { Poll::Ready(()) }
        }).await;
        call.invoke(&callback, (value.into_value(),)).await
    }

    #[geam_macros::function]
    fn manual_read<Item>(#[geam_macros::call] call: &mut Call<BigInt>, value: ManualInput<Item>) -> Value<Item> {
        call.restore(value.stored_item(|payload| &payload.value))
    }

    #[geam_macros::function]
    async fn manual_keep_after<Item>(value: ManualInput<Item>) -> ManualValue<Item> {
        let mut first = true;
        poll_fn(|context| {
            if std::mem::take(&mut first) {
                context.waker().wake_by_ref();
                Poll::Pending
            } else { Poll::Ready(()) }
        }).await;
        value.into_value()
    }

    #[geam_macros::function]
    async fn manual_read_after<Item>(#[geam_macros::call] call: &mut Call<BigInt>, value: ManualInput<Item>) -> Value<Item> {
        let value = value.stored_item(|payload| &payload.value);
        let mut first = true;
        poll_fn(|context| {
            if std::mem::take(&mut first) {
                context.waker().wake_by_ref();
                Poll::Pending
            } else { Poll::Ready(()) }
        }).await;
        call.restore(value)
    }

    #[geam_macros::function]
    fn manual_same_allocation<Item>(left: ManualInput<Item>, right: ManualInput<Item>) -> bool {
        std::ptr::eq(left.payload(), right.payload())
    }

    #[geam_macros::function]
    async fn manual_same_allocation_after<Item>(left: ManualInput<Item>, right: ManualInput<Item>) -> bool {
        let left_address = left.with_payload(|payload| std::ptr::from_ref(payload).addr());
        let mut first = true;
        poll_fn(|context| {
            if std::mem::take(&mut first) {
                context.waker().wake_by_ref();
                Poll::Pending
            } else { Poll::Ready(()) }
        }).await;
        let right_address = right.with_payload(|payload| std::ptr::from_ref(payload).addr());
        left_address == right_address
    }

    #[geam_macros::function]
    async fn manual_new_after<Item>(#[geam_macros::call] call: &mut Call<BigInt>, value: Value<Item>) -> ManualValue<Item> {
        let mut first = true;
        poll_fn(|context| {
            if std::mem::take(&mut first) {
                context.waker().wake_by_ref();
                Poll::Pending
            } else { Poll::Ready(()) }
        }).await;
        ManualValue::from_payload(ManualPayload { value: call.store(value).into_retained() })
    }

    #[geam_macros::function]
    fn double(value: BigInt) -> BigInt {
        value * 2
    }

    #[geam_macros::function]
    async fn arity_zero() -> () {}

    #[geam_macros::function]
    async fn arity_one(value: (bool,)) -> (bool,) {
        value
    }

    #[geam_macros::function]
    async fn arity_two(a: f64, b: f64) -> f64 {
        a + b
    }

    #[geam_macros::function]
    async fn arity_three(a: EcoString, b: EcoString, c: EcoString) -> EcoString {
        format!("{a}{b}{c}").into()
    }

    #[geam_macros::function]
    async fn arity_four(a: BitArrayValue, b: bool, c: bool, d: ()) -> (BitArrayValue, bool, ()) {
        (a, b && c, d)
    }

    #[geam_macros::function]
    async fn arity_five(a: char, b: char, c: char, d: char, e: char) -> char {
        assert_eq!((a, b, c, d), ('a', 'b', 'c', 'd'));
        e
    }

    #[geam_macros::function]
    async fn arity_six(a: BigInt, b: BigInt, c: BigInt, d: BigInt, e: BigInt, f: BigInt) -> BigInt {
        a + b + c + d + e + f
    }

    #[geam_macros::function]
    async fn arity_seven(a: BigInt, b: f64, c: EcoString, d: BitArrayValue, e: char, f: bool, g: ())
        -> (BigInt, f64, EcoString, BitArrayValue, char, bool, ()) {
        (a, b, c, d, e, f, g)
    }

    #[geam_macros::function]
    fn number_identity(value: Number) -> Number {
        value
    }

    #[geam_macros::function]
    fn sum_numbers(values: geam_core::List<Number>) -> Number {
        let first = values.get(0).expect("the test List contains three numbers");
        let second = values.get(1).expect("the test List contains three numbers");
        let third = values.get(2).expect("the test List contains three numbers");
        first + second + third
    }

    #[geam_macros::function]
    async fn sum_numbers_async(values: geam_core::List<Number>) -> Number {
        std::future::ready(()).await;
        let first = values.get(0).expect("the test List contains three numbers");
        let second = values.get(1).expect("the test List contains three numbers");
        let third = values.get(2).expect("the test List contains three numbers");
        first + second + third
    }

    #[geam_macros::function]
    async fn qualified_bool_list(
        values: geam_core::List<std::primitive::bool>,
    ) -> std::primitive::bool {
        std::future::ready(()).await;
        values
            .get(0)
            .expect("the test List contains one qualified Bool")
    }

    #[geam_macros::function]
    fn float_identity(value: f64) -> f64 {
        value
    }

    #[geam_macros::function]
    fn string_identity(value: EcoString) -> EcoString {
        value
    }

    #[geam_macros::function]
    fn bit_array_identity(value: BitArrayValue) -> BitArrayValue {
        value
    }

    #[geam_macros::function]
    fn codepoint_identity(value: char) -> char {
        value
    }

    #[geam_macros::function]
    fn bool_identity(value: bool) -> bool {
        value
    }

    #[geam_macros::function]
    fn nil_identity(value: ()) -> () {
        value
    }

    #[geam_macros::function]
    fn tuple_identity(value: (EcoString, BigInt)) -> (EcoString, BigInt) {
        value
    }

    #[geam_macros::function]
    fn pass_function<Item>(callback: Value<fn(Item) -> Item>) -> Value<fn(Item) -> Item> {
        callback
    }

    #[geam_macros::function]
    fn pass_int_function(callback: Value<fn(BigInt) -> BigInt>) -> Value<fn(BigInt) -> BigInt> {
        callback
    }

    #[geam_macros::function]
    fn fail_direct() -> HostResult<BigInt> {
        Err(HostFailure::new("immediate provider failed").into())
    }

    #[geam_macros::function]
    fn fail_nil() -> HostResult<()> {
        Err(HostFailure::new("immediate Nil provider failed").into())
    }

    #[geam_macros::function]
    fn return_nil() -> HostResult<()> {
        Ok(())
    }

    #[geam_macros::function]
    fn new_token(value: EcoString) -> declarations::Token {
        declarations::Token(value)
    }

    #[geam_macros::function]
    fn first_token(values: geam_core::List<declarations::Token>) -> EcoString {
        values
            .get(0)
            .map_or_else(|| "missing".into(), |token| token.0.clone())
    }

    #[geam_macros::function]
    async fn first_token_async(values: geam_core::List<declarations::Token>) -> EcoString {
        let value = values.get(0).map_or_else(
            || "missing".into(),
            |token| token.with(|token| token.0.clone()),
        );
        std::future::ready(()).await;
        value
    }

    #[geam_macros::function]
    async fn add_one(value: BigInt) -> BigInt {
        let mut pending = true;
        poll_fn(move |context| {
            if pending {
                pending = false;
                context.waker().wake_by_ref();
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await;
        value + 1
    }

    #[geam_macros::function]
    async fn add_to_state(#[geam_macros::call] call: &mut Call<BigInt>, value: BigInt) -> HostResult<BigInt> {
        call.with_state(move |state| {
            *state += value;
            state.clone()
        })
        .await
    }

    #[geam_macros::function]
    fn add_to_state_direct(
        #[geam_macros::call] call: &mut Call<BigInt>,
        value: BigInt,
    ) -> (BigInt, BigInt) {
        let before = call.state().clone();
        *call.state_mut() += value;
        (before, call.state().clone())
    }

    #[geam_macros::function(resumable)]
    async fn invoke_immediate(
        #[geam_macros::call] call: &mut Call<BigInt>,
        callback: Callback<fn(BigInt) -> BigInt>,
        value: BigInt,
    ) -> HostResult<BigInt> {
        call.invoke(&callback, (value,)).await
    }

    #[geam_macros::function]
    async fn invoke_twice(
        #[geam_macros::call] call: &mut Call<BigInt>,
        callback: Callback<fn(BigInt) -> BigInt>,
        value: BigInt,
    ) -> HostResult<BigInt> {
        let first = call.invoke(&callback, (value,)).await?;
        let mut pending = true;
        poll_fn(move |context| {
            if pending {
                pending = false;
                context.waker().wake_by_ref();
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await;
        call.invoke(&callback, (first,)).await
    }

    #[geam_macros::function]
    async fn invoke_future_twice(
        #[geam_macros::call] call: &mut Call<BigInt>,
        callback: Callback<fn(BigInt) -> Future<BigInt>>,
        value: BigInt,
    ) -> HostResult<BigInt> {
        let retained_callback = callback.clone();
        let first = call.invoke(&retained_callback, (value,)).await?;
        let first_alias = first.clone();
        let result = call.observe(&first).await?;
        assert_eq!(call.observe(&first_alias).await?, result);
        let second = call.invoke(&callback, (result,)).await?;
        call.observe(&second).await
    }

    #[geam_macros::function]
    async fn observe_future(
        #[geam_macros::call] call: &mut Call<BigInt>,
        value: Future<geam_core::List<BigInt>>,
    ) -> HostResult<BigInt> {
        let values = call.observe(&value).await?;
        Ok(values.get(0).expect("completed native List"))
    }

    #[geam_macros::function]
    fn retain_future(value: Future<BigInt>) -> bool {
        let _alias = value.clone();
        true
    }

    #[geam_macros::function]
    async fn observe_nested_future(
        #[geam_macros::call] call: &mut Call<BigInt>,
        value: Future<Future<BigInt>>,
    ) -> HostResult<BigInt> {
        let inner = call.observe(&value).await?;
        call.observe(&inner).await
    }

    #[geam_macros::function]
    async fn cancel_queued_callback(
        #[geam_macros::call] call: &mut Call<BigInt>,
        callback: Callback<fn(BigInt) -> BigInt>,
        value: BigInt,
    ) -> BigInt {
        {
            let mut callback_call = std::pin::pin!(call.invoke(&callback, (value.clone(),)));
            poll_fn(|context| {
                let _ = callback_call.as_mut().poll(context);
                Poll::Ready(())
            })
            .await;
        }

        let mut pending = true;
        poll_fn(move |context| {
            if pending {
                pending = false;
                context.waker().wake_by_ref();
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await;
        value
    }

    #[geam_macros::function]
    async fn invoke_generic<Item>(
        #[geam_macros::call] call: &mut Call<BigInt>,
        callback: Callback<fn(Value<Item>) -> Value<Item>>,
        value: Value<Item>,
    ) -> HostResult<Value<Item>> {
        call.invoke(&callback, (value,)).await
    }

    #[geam_macros::function]
    async fn invoke_generic_map<Input, Output>(
        #[geam_macros::call] call: &mut Call<BigInt>,
        callback: Callback<fn(Value<Input>) -> Value<Output>>,
        value: Value<Input>,
    ) -> HostResult<Value<Output>> {
        call.invoke(&callback, (value,)).await
    }

    #[geam_macros::function]
    async fn invoke_generic_produce<Output>(
        #[geam_macros::call] call: &mut Call<BigInt>,
        callback: Callback<fn() -> Value<Output>>,
    ) -> HostResult<Value<Output>> {
        call.invoke(&callback, ()).await
    }

    #[geam_macros::function]
    async fn invoke_structured_callback(
        #[geam_macros::call] call: &mut Call<BigInt>,
        callback: Callback<
            fn(
                (BigInt, EcoString),
                Result<BigInt, EcoString>,
                Option<BigInt>,
                Vec<BigInt>,
            ) -> (
                (BigInt, EcoString),
                Result<BigInt, EcoString>,
                Option<BigInt>,
                geam_core::List<BigInt>,
            ),
        >,
        value: BigInt,
    ) -> HostResult<(BigInt, BigInt, BigInt, BigInt)> {
        let returned = call
            .invoke(
                &callback,
                (
                    (value.clone(), "input".into()),
                    Ok(value.clone() + 1),
                    Some(value.clone() + 2),
                    vec![value.clone() + 3, value + 4],
                ),
            )
            .await?;
        let ((number, label), result, option, values) = returned;
        assert_eq!(label, "input!");
        Ok((
            number,
            result.expect("callback result must remain successful"),
            option.expect("callback option must remain present"),
            values
                .get(0)
                .expect("callback List must retain its first item"),
        ))
    }

    #[geam_macros::function]
    async fn invoke_scalar_callback(
        #[geam_macros::call] call: &mut Call<BigInt>,
        callback: Callback<fn() -> (f64, BitArrayValue, char, bool, ())>,
    ) -> HostResult<(f64, BitArrayValue, char, bool, ())> {
        call.invoke(&callback, ()).await
    }

    #[geam_macros::function]
    async fn invoke_function_callback(
        #[geam_macros::call] call: &mut Call<BigInt>,
        callback: Callback<fn() -> Value<fn(BigInt) -> BigInt>>,
    ) -> HostResult<Value<fn(BigInt) -> BigInt>> {
        call.invoke(&callback, ()).await
    }

    #[geam_macros::function]
    async fn invoke_function_argument(
        #[geam_macros::call] call: &mut Call<BigInt>,
        callback: Callback<fn(Value<fn(BigInt) -> BigInt>, BigInt) -> BigInt>,
        function: Value<fn(BigInt) -> BigInt>,
        value: BigInt,
    ) -> HostResult<BigInt> {
        call.invoke(&callback, (function, value)).await
    }

    #[geam_macros::function]
    async fn invoke_external_callback(
        #[geam_macros::call] call: &mut Call<BigInt>,
        callback: Callback<fn(Counter, CounterEnvelope) -> BigInt>,
        value: BigInt,
    ) -> HostResult<BigInt> {
        let first = Counter {
            value: Cell::new(usize::try_from(value.clone()).expect("test counter value")),
        };
        let second = CounterEnvelope::Wrapped(Counter {
            value: Cell::new(usize::try_from(value + 1).expect("test counter value")),
        });
        call.invoke(&callback, (first, second)).await
    }

    #[geam_macros::function]
    async fn invoke_box_callback<Item>(
        #[geam_macros::call] call: &mut Call<BigInt>,
        callback: Callback<fn(BoxValue<Item>) -> BoxInput<Item>>,
        boxed: BoxInput<Item>,
    ) -> HostResult<Value<Item>> {
        let value = call.restore(boxed.value());
        let outbound = BoxValue {
            value: call.store(value),
        };
        let returned = call.invoke(&callback, (outbound,)).await?;
        Ok(call.restore(returned.value()))
    }

    #[geam_macros::function]
    async fn fail_async() -> HostResult<BigInt> {
        std::future::ready(()).await;
        Err(HostFailure::new("async provider failed").into())
    }

    #[geam_macros::function]
    fn list_identity(values: geam_core::List<BigInt>) -> geam_core::List<BigInt> {
        assert_eq!(values.len(), 2);
        assert_eq!(values.get(1), Some(BigInt::from(2)));
        values
    }

    #[geam_macros::function]
    async fn list_identity_async(values: geam_core::List<BigInt>) -> geam_core::List<BigInt> {
        std::future::ready(()).await;
        assert_eq!(values.get(0), Some(BigInt::from(1)));
        values
    }

    #[geam_macros::function]
    fn scalar_list_matches(values: geam_core::List<(f64, BitArrayValue, char, bool, ())>) -> bool {
        let Some((float, bits, codepoint, flag, ())) = values.get(0) else {
            return false;
        };
        float == 1.5 && bits == BitArrayValue::from_bytes(vec![1]) && codepoint == 'A' && flag
    }

    #[geam_macros::function]
    fn nested_list_matches(values: geam_core::List<(EcoString, (BigInt, bool))>) -> bool {
        let Some((label, (number, flag))) = values.get(0) else {
            return false;
        };
        label == "nested" && number == BigInt::from(7) && flag
    }

    #[geam_macros::function]
    fn first_envelope(values: geam_core::List<CounterEnvelopeInput>) -> BigInt {
        let CounterEnvelopeInput::Wrapped(counter) = values
            .get(0)
            .expect("the test List contains a counter envelope");
        counter.value.get().into()
    }

    #[geam_macros::function]
    fn make_numbers(value: BigInt) -> Batch {
        Batch::Numbers(vec![value.clone(), value + 1])
    }

    #[geam_macros::function]
    fn make_counter_batch(value: BigInt) -> Batch {
        Batch::Counters(vec![Counter {
            value: Cell::new(usize::try_from(value).expect("test counter value")),
        }])
    }

    #[geam_macros::function]
    async fn make_pairs(label: EcoString, value: BigInt) -> Batch {
        std::future::ready(()).await;
        Batch::Pairs(vec![(label, value)])
    }

    #[geam_macros::function]
    fn summarize_batch(value: BatchInput) -> (EcoString, BigInt) {
        match value {
            BatchInput::Numbers(values) => {
                assert_eq!(values.len(), 2);
                let first = values.get(0).expect("the test batch contains two numbers");
                let second = values.get(1).expect("the test batch contains two numbers");
                ("numbers".into(), first + second)
            }
            BatchInput::Pairs(values) => {
                assert_eq!(values.len(), 1);
                values
                    .get(0)
                    .expect("the test batch contains one labeled number")
            }
            BatchInput::Counters(values) => (
                "counters".into(),
                values
                    .get(0)
                    .expect("the test batch contains one counter")
                    .value
                    .get()
                    .into(),
            ),
        }
    }

    #[geam_macros::function]
    fn summarize_first_batch(values: geam_core::List<BatchInput>) -> (EcoString, BigInt) {
        summarize_batch(values.get(0).expect("the test List contains a batch value"))
    }

    #[geam_macros::function]
    async fn summarize_first_batch_async(
        values: geam_core::List<BatchInput>,
    ) -> (EcoString, BigInt) {
        let BatchInput::Counters(counters) = values
            .get(0)
            .expect("the test List contains a counter batch")
        else {
            panic!("the test List must contain the counter variant");
        };
        let value = counters
            .get(0)
            .expect("the test batch contains one counter")
            .with(|counter| counter.value.get());
        std::future::ready(()).await;
        ("counters".into(), value.into())
    }

    #[geam_macros::function]
    async fn summarize_batch_async(value: BatchInput) -> (EcoString, BigInt) {
        std::future::ready(()).await;
        match value {
            BatchInput::Numbers(values) => {
                assert_eq!(values.len(), 2);
                let first = values.get(0).expect("the test batch contains two numbers");
                let second = values.get(1).expect("the test batch contains two numbers");
                ("numbers".into(), first + second)
            }
            BatchInput::Pairs(values) => {
                assert_eq!(values.len(), 1);
                values
                    .get(0)
                    .expect("the test batch contains one labeled number")
            }
            BatchInput::Counters(values) => (
                "counters".into(),
                values
                    .get(0)
                    .expect("the test batch contains one counter")
                    .with(|counter| counter.value.get())
                    .into(),
            ),
        }
    }

    #[geam_macros::function]
    fn identity<Item>(value: Value<Item>) -> Value<Item> {
        value
    }

    #[geam_macros::function]
    async fn identity_async<Item>(value: Value<Item>) -> Value<Item> {
        std::future::ready(()).await;
        value
    }

    #[geam_macros::function]
    fn box_value<Item>(
        #[geam_macros::call] call: &mut Call<BigInt>,
        value: Value<Item>,
    ) -> BoxValue<Item> {
        BoxValue {
            value: call.store(value),
        }
    }

    #[geam_macros::function]
    fn unbox<Item>(
        #[geam_macros::call] call: &mut Call<BigInt>,
        boxed: BoxInput<Item>,
    ) -> Value<Item> {
        call.restore(boxed.value())
    }

    #[geam_macros::function]
    async fn unbox_async<Item>(
        #[geam_macros::call] call: &mut Call<BigInt>,
        boxed: BoxInput<Item>,
    ) -> Value<Item> {
        std::future::ready(()).await;
        call.restore(boxed.value())
    }

    #[geam_macros::function]
    async fn rebox_async<Item>(
        #[geam_macros::call] call: &mut Call<BigInt>,
        boxed: BoxInput<Item>,
    ) -> BoxValue<Item> {
        let value = call.restore(boxed.value());
        std::future::ready(()).await;
        BoxValue {
            value: call.store(value),
        }
    }

    #[geam_macros::function]
    async fn structured(value: BigInt) -> (BigInt, Result<BigInt, BigInt>, Option<BigInt>) {
        std::future::ready(()).await;
        (value.clone(), Ok(value.clone() + 1), Some(value + 2))
    }

    #[geam_macros::function]
    async fn numbers(value: BigInt) -> Vec<BigInt> {
        std::future::ready(()).await;
        vec![value.clone(), value + 1]
    }

    #[geam_macros::function]
    fn new_counter(value: BigInt) -> Counter {
        Counter {
            value: Cell::new(usize::try_from(value).expect("test counter value")),
        }
    }

    #[geam_macros::function]
    fn read_counter(counter: &Counter) -> BigInt {
        counter.value.get().into()
    }

    #[geam_macros::function]
    async fn increment_counter(counter: &Counter) -> BigInt {
        let before = counter.with(|counter| counter.value.get());
        let mut pending = true;
        poll_fn(move |context| {
            if pending {
                pending = false;
                context.waker().wake_by_ref();
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await;
        let observed = counter.with(|counter| counter.value.get());
        assert_eq!(observed, before);
        (observed + 1).into()
    }

    #[geam_macros::function]
    fn first_counter(counters: geam_core::List<Counter>) -> BigInt {
        counters
            .get(0)
            .expect("the test List contains a counter")
            .value
            .get()
            .into()
    }

    #[geam_macros::function]
    async fn increment_first_counter(
        counters: geam_core::List<Counter>,
    ) -> Vec<Counter> {
        let counter = counters.get(0).expect("the test List contains a counter");
        let before = counter.with(|counter| counter.value.get());
        let mut pending = true;
        poll_fn(move |context| {
            if pending {
                pending = false;
                context.waker().wake_by_ref();
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await;
        assert_eq!(counter.with(|counter| counter.value.get()), before);
        vec![Counter { value: Cell::new(before + 1) }]
    }

    #[geam_macros::function]
    fn wrap_counter(value: BigInt) -> CounterEnvelope {
        CounterEnvelope::Wrapped(Counter {
            value: Cell::new(usize::try_from(value).expect("test counter value")),
        })
    }

    #[geam_macros::function]
    fn read_counter_envelope(value: CounterEnvelopeInput) -> BigInt {
        let CounterEnvelopeInput::Wrapped(counter) = value;
        counter.value.get().into()
    }

    #[geam_macros::function]
    async fn increment_counter_envelope(value: CounterEnvelopeInput) -> BigInt {
        let CounterEnvelopeInput::Wrapped(counter) = value;
        let before = counter.with(|counter| counter.value.get());
        std::future::ready(()).await;
        assert_eq!(counter.with(|counter| counter.value.get()), before);
        (before + 1).into()
    }
}
