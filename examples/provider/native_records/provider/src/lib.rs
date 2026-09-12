use geam::provider::advanced::{
    Equality, Hashing, Inspection, NativeKind, NativeValue, RetainedExternalPayload,
};
use geam::provider::{BigInt, Call, Callback, EcoString, HostResult, Value};

#[geam::provider(package = "example_native_records", modules = [records])]
pub struct Component;

#[geam::module(path = "example_native_records")]
mod records {
    use super::{
        BigInt, Call, Callback, EcoString, Equality, Hashing, HostResult, Inspection, NativeKind,
        NativeValue, RetainedExternalPayload, Value,
    };

    #[geam::external(name = "Key", retained)]
    struct Key {
        value: NativeValue,
    }

    impl RetainedExternalPayload for Key {
        fn source_equal(&self, context: &Equality<'_>, other: &Self) -> bool {
            self.value.source_equal(context, &other.value)
        }

        fn source_hash(&self, context: &Hashing<'_>) -> u64 {
            self.value.source_hash(context)
        }

        fn inspect(&self, context: &Inspection<'_>) -> EcoString {
            self.value.inspect(context)
        }

        fn native_view(&self) -> Option<NativeValue> {
            Some(self.value.clone())
        }
    }

    #[geam::external(name = "Record", retained)]
    struct Record {
        value: NativeValue,
    }

    impl RetainedExternalPayload for Record {
        fn source_equal(&self, context: &Equality<'_>, other: &Self) -> bool {
            self.value.source_equal(context, &other.value)
        }

        fn source_hash(&self, context: &Hashing<'_>) -> u64 {
            self.value.source_hash(context)
        }

        fn inspect(&self, context: &Inspection<'_>) -> EcoString {
            self.value.inspect(context)
        }

        fn native_view(&self) -> Option<NativeValue> {
            Some(self.value.clone())
        }
    }

    #[geam::function]
    fn key(name: EcoString) -> Key {
        Key {
            value: NativeValue::symbol(name),
        }
    }

    #[geam::function]
    fn record(#[geam::call] call: &mut Call<()>, label: EcoString, count: BigInt) -> Record {
        let label = call.store_dynamic::<_, Record>(label).native_view();
        let count = call.store_dynamic::<_, Record>(count).native_view();
        Record {
            value: NativeValue::tuple([NativeValue::symbol("record"), label, count]),
        }
    }

    #[geam::function]
    fn erase<Item>(
        #[geam::call] call: &mut Call<()>,
        value: Value<Item>,
    ) -> geam::gleam_stdlib::Dynamic {
        let value = call.store_dynamic::<_, Record>(value).native_view();
        geam::gleam_stdlib::Dynamic::from_native(value)
    }

    #[geam::function(await)]
    async fn map<Output>(
        #[geam::call] call: &mut Call<()>,
        value: geam::provider::advanced::External<geam::gleam_stdlib::Dynamic>,
        transform: Callback<fn(EcoString, BigInt) -> Value<Output>>,
    ) -> HostResult<Result<Value<Output>, ()>> {
        let native = value.with(|value| value.native_value().clone());
        drop(value);
        match record_fields(&native) {
            Some(fields) => call.invoke(&transform, fields).await.map(Ok),
            None => Ok(Err(())),
        }
    }

    fn record_fields(value: &NativeValue) -> Option<(EcoString, BigInt)> {
        if value.kind() != NativeKind::Tuple || value.len() != Some(3) {
            return None;
        }
        if value.index(0)?.as_symbol().as_deref() != Some("record") {
            return None;
        }
        Some((value.index(1)?.as_string()?, value.index(2)?.as_int()?))
    }
}
