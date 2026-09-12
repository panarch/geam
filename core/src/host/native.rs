use crate::host::{
    HostAbiType, HostCall, HostCallCompletion, HostConstruction, HostConstructions, HostExternal,
    HostExternalSchema, HostExternalType, HostProfile, HostProvider, HostScopedValue, HostType,
    HostTypeAt, HostTypeDescriptor, HostTypeIndex0, HostTypeList, HostTypeListEnd,
    HostTypeSequence,
};
use crate::plan::execution::host::{NativeConversionId, NativeConversionKind, NativeConversions};
use crate::runtime::NativeValue;
use std::collections::HashSet;
use std::marker::PhantomData;
use std::sync::Arc;

mod callback;

pub use crate::runtime::NativeValues;
pub use callback::NativeCallable;

/// Native external conversions paired with a single host registration.
///
/// Exact retained values pass through unchanged. A conversion creates the
/// declared source view; it does not change the original value's type or kind.
pub struct NativeRules<Profile: HostProfile, Provider: HostProvider<Profile>, Return: HostType> {
    rules: Vec<NativeRule<Profile, Provider, Return>>,
    custom_schemas: Vec<crate::host::HostCustomTypeSchema>,
    visited: HashSet<(ecow::EcoString, ecow::EcoString, ecow::EcoString)>,
}

/// Active access to the native conversions sealed for one host function.
pub struct NativeCall<'call, Profile, Provider, Return, Targets>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Targets: HostTypeSequence,
{
    call: HostCall<'call, Profile, Provider, Return>,
    rules: Arc<[Box<Decode<Profile, Provider, Return>>]>,
    targets: PhantomData<fn() -> Targets>,
}

#[doc(hidden)]
pub struct NativeFunction<Profile, Provider, Return, Targets, Function>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Targets: HostTypeSequence,
{
    pub(in crate::host) rules: Arc<[Box<Decode<Profile, Provider, Return>>]>,
    pub(in crate::host) function: Function,
    targets: PhantomData<fn() -> Targets>,
}

type Decode<Profile, Provider, Return> = dyn for<'call> Fn(
        &mut HostCall<'call, Profile, Provider, Return>,
        NativeValue,
    ) -> Option<HostScopedValue>
    + Send
    + Sync;

struct NativeRule<Profile: HostProfile, Provider: HostProvider<Profile>, Return: HostType> {
    descriptor: HostTypeDescriptor,
    decode: Box<Decode<Profile, Provider, Return>>,
}

pub(in crate::host) struct NativeRegistration {
    pub(in crate::host) descriptors: Box<[HostTypeDescriptor]>,
    pub(in crate::host) custom_schemas: Box<[crate::host::HostCustomTypeSchema]>,
}

pub(crate) trait NativeTargetIndex {
    const INDEX: usize;
}

impl NativeTargetIndex for HostTypeIndex0 {
    const INDEX: usize = 0;
}

impl<Index: NativeTargetIndex> NativeTargetIndex for crate::host::HostTypeIndexNext<Index> {
    const INDEX: usize = Index::INDEX + 1;
}

impl<Profile, Provider, Return> Default for NativeRules<Profile, Provider, Return>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    fn default() -> Self {
        Self {
            rules: Vec::new(),
            custom_schemas: Vec::new(),
            visited: HashSet::new(),
        }
    }
}

impl<Profile, Provider, Return> NativeRules<Profile, Provider, Return>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    /// Registers construction of one external source view from native data.
    pub fn external<Schema, Arguments>(
        mut self,
        decode: impl for<'call> Fn(
            &mut HostCall<'call, Profile, Provider, Return>,
            HostConstruction<'call, HostExternalType<Schema, Arguments>>,
            NativeValue,
        )
            -> Option<HostExternal<'call, HostExternalType<Schema, Arguments>>>
        + Send
        + Sync
        + 'static,
    ) -> Self
    where
        Schema: HostExternalSchema,
        Arguments: HostTypeSequence,
    {
        <HostExternalType<Schema, Arguments> as HostAbiType>::collect_custom_schemas(
            &mut self.custom_schemas,
            &mut self.visited,
        );
        self.rules.push(NativeRule {
            descriptor: HostTypeDescriptor::of::<HostExternalType<Schema, Arguments>>(),
            decode: Box::new(move |call, value| {
                let construction = HostConstructions::<
                    HostTypeList<HostExternalType<Schema, Arguments>, HostTypeListEnd>,
                >::new();
                decode(call, construction.at::<HostTypeIndex0>(), value)
                    .map(crate::host::type_::into_scoped::<HostExternalType<Schema, Arguments>>)
            }),
        });
        self
    }
}

impl<Profile, Provider, Return, Targets, Function>
    NativeFunction<Profile, Provider, Return, Targets, Function>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Targets: HostTypeSequence,
{
    pub(in crate::host) fn new(
        rules: NativeRules<Profile, Provider, Return>,
        function: Function,
    ) -> (Self, NativeRegistration) {
        let (descriptors, decoders): (Vec<_>, Vec<_>) = rules
            .rules
            .into_iter()
            .map(|rule| (rule.descriptor, rule.decode))
            .unzip();
        (
            Self {
                rules: decoders.into(),
                function,
                targets: PhantomData,
            },
            NativeRegistration {
                descriptors: descriptors.into_boxed_slice(),
                custom_schemas: rules.custom_schemas.into_boxed_slice(),
            },
        )
    }
}

impl<'call, Profile, Provider, Return, Targets>
    NativeCall<'call, Profile, Provider, Return, Targets>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Targets: HostTypeSequence,
{
    pub(in crate::host) fn new(
        call: HostCall<'call, Profile, Provider, Return>,
        rules: Arc<[Box<Decode<Profile, Provider, Return>>]>,
    ) -> Self {
        Self {
            call,
            rules,
            targets: PhantomData,
        }
    }

    pub fn call(&mut self) -> &mut HostCall<'call, Profile, Provider, Return> {
        &mut self.call
    }

    /// Requires an invocation while retaining this call's sealed conversions.
    pub fn with_execution_unit<Output>(
        self,
        operation: impl FnOnce(
            Self,
            crate::execution::ExecutionUnit,
        ) -> Result<Output, crate::host::HostCallError>,
    ) -> Result<Output, crate::host::HostCallError> {
        let Self {
            call,
            rules,
            targets: _,
        } = self;
        call.with_execution_unit(move |call, unit| operation(Self::new(call, rules), unit))
    }

    /// Ends native conversion and returns the original typed call and its
    /// registered construction permissions for an owned continuation.
    pub fn into_call(
        self,
    ) -> (
        HostCall<'call, Profile, Provider, Return>,
        HostConstructions<'call, Targets>,
    ) {
        (self.call, HostConstructions::new())
    }

    /// Retains the original value for structural native access.
    pub fn source<Type: HostType>(&self, value: Type::Value<'call>) -> NativeValue {
        self.call.native_value::<Type>(value)
    }

    /// Converts incoming data to the preselected registered target.
    #[allow(private_bounds)]
    pub fn convert<Index>(
        &mut self,
        value: &NativeValue,
    ) -> Option<<<Targets as HostTypeAt<Index>>::Type as HostType>::Value<'call>>
    where
        Targets: HostTypeAt<Index>,
        Index: NativeTargetIndex,
    {
        let scope = self.call.runtime.codec_scope();
        let conversions = scope.function().constructions().natives();
        let value = self.convert_value(conversions, conversions.root(Index::INDEX), value)?;
        let token = self.call.runtime.complete(value);
        Some(crate::host::type_::from_runtime_token::<
            <Targets as HostTypeAt<Index>>::Type,
            _,
        >(self.call.runtime, token))
    }

    pub fn finish(self, value: Return::Value<'call>) -> HostCallCompletion<'call, Return> {
        self.call.return_value(value)
    }

    fn convert_value(
        &mut self,
        conversions: &NativeConversions,
        id: NativeConversionId,
        value: &NativeValue,
    ) -> Option<HostScopedValue> {
        let conversion = conversions.get(id);
        if let Some(source) = value.find_source(|source| {
            (self.call.runtime.owns_stored(source) && source.type_() == conversion.type_())
                .then(|| source.clone_retained())
        }) {
            return Some(HostScopedValue::Value(
                self.call.runtime.restore_stored(&source),
            ));
        }
        match conversion.kind() {
            NativeConversionKind::Exact => None,
            NativeConversionKind::Int => value.as_int().map(HostScopedValue::Int),
            NativeConversionKind::Float => value.as_float().map(HostScopedValue::Float),
            NativeConversionKind::String => value.as_string().map(HostScopedValue::String),
            NativeConversionKind::BitArray => value.as_bit_array().map(HostScopedValue::BitArray),
            NativeConversionKind::UtfCodepoint => {
                let integer = value.as_int()?;
                let codepoint = u32::try_from(&integer).ok()?;
                char::from_u32(codepoint).map(HostScopedValue::UtfCodepoint)
            }
            NativeConversionKind::Bool => match value.as_symbol().as_deref() {
                Some("true") => Some(HostScopedValue::Bool(true)),
                Some("false") => Some(HostScopedValue::Bool(false)),
                _ => None,
            },
            NativeConversionKind::Nil => {
                (value.as_symbol().as_deref() == Some("nil")).then_some(HostScopedValue::Nil)
            }
            NativeConversionKind::Tuple(items) => {
                let values = value.convert_tuple(items.len(), |index, value| {
                    self.convert_value(conversions, items[index], &value)
                })?;
                Some(HostScopedValue::Value(
                    self.call.runtime.build_tuple(values),
                ))
            }
            NativeConversionKind::List { storage, item } => {
                let values =
                    value.convert_list(|value| self.convert_value(conversions, *item, &value))?;
                Some(HostScopedValue::Value(
                    self.call.runtime.build_native_list(*storage, values),
                ))
            }
            NativeConversionKind::Custom(constructors) => {
                let (constructor, fields) = value.convert_custom(constructors, |id, value| {
                    self.convert_value(conversions, id, &value)
                })?;
                Some(HostScopedValue::Value(
                    self.call.runtime.build_native_custom(constructor, fields),
                ))
            }
            NativeConversionKind::External { rule } => {
                (self.rules[*rule])(&mut self.call, value.clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{NativeCall, NativeRules, NativeValue};
    use crate::host::{
        HostCall, HostCallCompletion, HostCallError, HostCallable, HostConstruction,
        HostCustomConstructorDefinition, HostCustomConstructorList, HostCustomConstructorListEnd,
        HostCustomField, HostCustomFieldList, HostCustomFieldListEnd, HostCustomSchema,
        HostCustomType, HostCustomTypeArgument, HostExternal, HostExternalBinding,
        HostExternalEquality, HostExternalHashing, HostExternalInspection, HostExternalSchema,
        HostExternalStorage, HostExternalStore, HostExternalType, HostFunctionType, HostListType,
        HostProfile, HostProvider, HostProviderModule, HostProviderSet, HostType, HostTypeIndex0,
        HostTypeList, HostTypeListEnd, HostTypeParameter, HostTypeSequence, HostValue,
        StatelessHostProfile,
    };
    use ecow::EcoString;

    struct Converter;

    impl HostProvider<StatelessHostProfile> for Converter {
        type State = ();

        fn project(state: &mut ()) -> &mut () {
            state
        }
    }

    type Source = HostTypeParameter<0>;
    type Target = HostTypeParameter<1>;
    // Source schemes number the return parameter before argument-only parameters.
    type Output = HostTypeParameter<0>;
    type CallbackSource = HostTypeParameter<1>;
    type CallbackTarget = HostTypeParameter<2>;
    type CallbackTargets = HostTypeList<CallbackTarget, HostTypeListEnd>;
    type Owned<Type> = crate::provider::Value<Type, crate::provider::ProviderValueContext<Type>>;

    fn equal_native<'call, Profile: HostProfile, Extra: HostTypeSequence>(
        mut call: NativeCall<'call, Profile, Converter, bool, HostTypeList<Target, Extra>>,
        source: HostValue<'call, Source>,
        target: HostValue<'call, Target>,
    ) -> Result<HostCallCompletion<'call, bool>, HostCallError>
    where
        Converter: HostProvider<Profile>,
    {
        let source = call.source::<Source>(source);
        let same = call
            .convert::<HostTypeIndex0>(&source)
            .is_some_and(|converted| {
                let view = call.source::<Target>(converted);
                assert!(call.call().native_equal(&source, &view));
                call.call().equal::<Target>(converted, target)
            });
        Ok(call.finish(same))
    }

    fn apply_native<'call, Extra: HostTypeSequence>(
        mut call: NativeCall<
            'call,
            StatelessHostProfile,
            Converter,
            Output,
            HostTypeList<CallbackTarget, Extra>,
        >,
        source: HostValue<'call, CallbackSource>,
        callback: HostCallable<'call, CallbackTargets, Output>,
        fallback: HostValue<'call, Output>,
    ) -> Result<crate::host::HostCallContinuation<'call, Output>, HostCallError> {
        let source = call.source::<CallbackSource>(source);
        let input = call
            .convert::<HostTypeIndex0>(&source)
            .map(|value| Owned::<CallbackTarget>::from_host(call.call(), value));
        let fallback = Owned::<Output>::from_host(call.call(), fallback);
        let (call, constructions) = call.into_call();
        let callback = call.owned_callable(callback, &constructions);
        Ok(call.resume(constructions, move |context| {
            Box::pin(async move {
                let result = match input {
                    Some(value) => {
                        callback
                            .invoke(
                                &context,
                                move |mut call, _| (value.into_host(&mut call), ()),
                                |call, _, value| Ok(Owned::<Output>::from_host(&call, value)),
                            )
                            .await?
                    }
                    None => fallback,
                };
                Ok(crate::host::HostOwnedCompletion::new(move |mut call, _| {
                    let value = result.into_host(&mut call);
                    Ok(call.return_value(value))
                }))
            })
        }))
    }

    fn run<Extra: HostTypeSequence>(source: &str) -> crate::Value {
        let provider = HostProviderModule::new("application", "main")
            .unwrap()
            .with_native_function::<Converter, (Source, Target), bool, HostTypeList<Target, Extra>, _>(
                "equal_native",
                NativeRules::default(),
                equal_native::<StatelessHostProfile, Extra>,
            )
            .unwrap()
            .with_resumable_native_function::<Converter, (
                CallbackSource,
                HostFunctionType<CallbackTargets, Output>,
                Output,
            ), Output, HostTypeList<CallbackTarget, Extra>, _>(
                "apply_native", NativeRules::default(), apply_native::<Extra>
            )
            .unwrap();
        execute(provider, source).unwrap()
    }

    fn execute<Profile: HostProfile>(
        provider: HostProviderModule<Profile>,
        source: &str,
    ) -> Result<crate::Value, crate::ExecutionError>
    where
        Profile::RunState: Default,
    {
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [crate::PackageSource::new(
                "application",
                Vec::<String>::new(),
                [crate::ModuleSource::new("main", "src/main.gleam", source)],
            )],
            HostProviderSet::from_providers([provider]).unwrap(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        crate::execution_fixture::run(
            &mut execution,
            &mut Profile::RunState::default(),
            &mut Vec::new(),
        )
    }

    #[test]
    fn native_targets_keep_declaration_order_including_duplicate_target_types() {
        use num_bigint::BigInt;
        type Targets =
            HostTypeList<EcoString, HostTypeList<BigInt, HostTypeList<BigInt, HostTypeListEnd>>>;
        let provider = HostProviderModule::new("application", "main").unwrap()
            .with_native_function::<Converter, (BigInt,), bool, Targets, _>(
                "targets", NativeRules::default(),
                |mut call: NativeCall<'_, StatelessHostProfile, Converter, bool, Targets>, value| {
                    let source = call.source::<BigInt>(value);
                    assert!(call.convert::<HostTypeIndex0>(&source).is_none());
                    assert_eq!(call.convert::<crate::HostTypeIndexNext<HostTypeIndex0>>(&source), Some(42.into()));
                    assert_eq!(call.convert::<crate::HostTypeIndexNext<crate::HostTypeIndexNext<HostTypeIndex0>>>(&source), Some(42.into()));
                    Ok(call.finish(true))
                },
            ).unwrap();
        assert_eq!(
            execute(
                provider,
                r#"
@external(erlang, "native", "targets")
fn targets(value: Int) -> Bool
pub fn main() { targets(42) }
"#
            )
            .unwrap(),
            crate::Value::Bool(true)
        );
    }

    #[test]
    fn native_targets_convert_scalars_and_preserve_kind_mismatches_as_absence() {
        let value = run::<HostTypeListEnd>(
            r#"
@external(erlang, "native", "equal_native")
fn equal_native(value: a, target: b) -> Bool
@external(erlang, "native", "apply_native")
fn apply_native(value: a, callback: fn(b) -> c, fallback: c) -> c
pub type Flag { True False }
pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  #(
    equal_native(<<"hello":utf8>>, "hello"),
    equal_native("hello", <<"hello":utf8>>),
    equal_native(<<255>>, ""),
    equal_native(<<1:size(1)>>, ""),
    equal_native(42, 42.0),
    equal_native(2.5, 2.5),
    equal_native(True, 1 == 1),
    equal_native(False, 1 == 2),
    equal_native(Nil, Nil),
    equal_native(#(1, 2), [1, 2]),
    equal_native([1, 2], #(1, 2)),
    equal_native(#(1, 2), #(1)),
    equal_native(codepoint, 65),
    equal_native(65, codepoint),
    equal_native(-1, codepoint),
    equal_native(55296, codepoint),
    equal_native(4294967296, codepoint),
    equal_native("A", codepoint),
    equal_native("true", 1 == 1),
    equal_native(#(<<255>>), #("text")),
    equal_native([<<255>>], ["text"]),
  )
}
"#,
        );
        assert_eq!(
            value.inspect().to_string(),
            "#(True, True, False, False, False, True, True, True, True, False, False, False, True, True, False, False, False, False, False, False, False)"
        );
    }

    #[test]
    fn native_recursive_conversion_preserves_generic_arguments_and_invokes_real_callbacks() {
        let value = run::<
            HostTypeList<
                HostCustomType<TreeSchema, HostTypeList<EcoString, HostTypeListEnd>>,
                HostTypeListEnd,
            >,
        >(
            r#"
@external(erlang, "native", "equal_native")
fn equal_native(value: a, target: b) -> Bool
@external(erlang, "native", "apply_native")
fn apply_native(value: a, callback: fn(b) -> c, fallback: c) -> c
pub type Tree(a) { Leaf(a) Branch(List(Tree(a))) }
pub type Empty { Empty }
pub fn main() {
  let source = Branch([Leaf(<<"one":utf8>>), Branch([Leaf(<<"two":utf8>>)])])
  let expected = Branch([Leaf("one"), Branch([Leaf("two")])])
  #(
    equal_native(source, expected),
    equal_native(source, Branch([Leaf(1)])),
    equal_native(#(<<"one":utf8>>, [<<"two":utf8>>]), #("one", ["two"])),
    equal_native(#(Empty), Empty),
    apply_native(<<"hello":utf8>>, fn(value: String) { value <> "!" }, "invalid"),
    apply_native(<<255>>, fn(value: String) { value <> "!" }, "invalid"),
    apply_native(source, fn(value: Tree(String)) { value == expected }, False),
  )
}
"#,
        );
        assert_eq!(
            value.inspect().to_string(),
            "#(True, False, True, False, \"hello!\", \"invalid\", True)"
        );
    }

    #[test]
    fn native_exact_transport_preserves_non_regular_recursive_source_values() {
        let returned = run::<HostTypeListEnd>(
            r#"
@external(erlang, "native", "equal_native")
fn equal_native(value: a, target: b) -> Bool
@external(erlang, "native", "apply_native")
fn apply_native(value: a, callback: fn(b) -> c, fallback: c) -> c
pub type Grow(a) { Stop Grow(Grow(List(a))) }
pub fn main() {
  let value: Grow(Int) = Grow(Stop)
  let other: Grow(String) = Grow(Stop)
  #(
    equal_native(value, value),
    equal_native(value, other),
    apply_native(value, fn(received: Grow(Int)) { received == value }, False),
    apply_native(#(<<"message":utf8>>, value), fn(received: #(String, Grow(Int))) {
      received.0 == "message" && received.1 == value
    }, False),
  )
}
"#,
        );
        assert_eq!(returned.inspect().to_string(), "#(True, False, True, True)");
    }

    #[test]
    fn native_custom_conversion_uses_only_registered_construction_permissions() {
        let source = r#"
@external(erlang, "native", "equal_native")
fn equal_native(value: a, target: b) -> Bool
@external(erlang, "native", "apply_native")
fn apply_native(value: a, callback: fn(b) -> c, fallback: c) -> c
pub type Packet(a) { Packet(a) }
pub fn main() {
  #(
    equal_native(Packet(<<"text":utf8>>), Packet("text")),
    equal_native(Packet("text"), Packet("text")),
    equal_native(Packet(1), Packet("text")),
  )
}
"#;
        assert_eq!(
            run::<HostTypeListEnd>(source).inspect().to_string(),
            "#(False, True, False)"
        );
        assert_eq!(
            run::<
                HostTypeList<
                    HostCustomType<PacketSchema, HostTypeList<EcoString, HostTypeListEnd>>,
                    HostTypeListEnd,
                >,
            >(source)
            .inspect()
            .to_string(),
            "#(True, True, False)"
        );
    }

    #[test]
    fn native_conversion_preserves_a_source_callback_failure() {
        let provider = HostProviderModule::new("application", "main")
            .unwrap()
            .with_resumable_native_function::<Converter, (
                CallbackSource,
                HostFunctionType<CallbackTargets, Output>,
                Output,
            ), Output, HostTypeList<
                CallbackTarget,
                HostTypeList<
                    HostCustomType<TreeSchema, HostTypeList<EcoString, HostTypeListEnd>>,
                    HostTypeListEnd,
                >,
            >, _>(
                "apply_native",
                NativeRules::default(),
                apply_native::<
                    HostTypeList<
                        HostCustomType<TreeSchema, HostTypeList<EcoString, HostTypeListEnd>>,
                        HostTypeListEnd,
                    >,
                >,
            )
            .unwrap();
        let error = execute(
            provider,
            r#"
@external(erlang, "native", "apply_native")
fn apply_native(value: a, callback: fn(b) -> c, fallback: c) -> c
pub type Tree(a) { Leaf(a) Branch(List(Tree(a))) }
pub fn main() {
  apply_native(<<"text":utf8>>, fn(value: String) -> Bool { panic as value }, False)
}
"#,
        )
        .unwrap_err();
        assert_eq!(error.to_string(), "panic: text");
    }

    #[test]
    fn declared_custom_envelopes_retain_generic_recursive_values_and_captures() {
        type Packet = HostCustomType<PacketSchema, HostTypeList<CallbackSource, HostTypeListEnd>>;
        type Targets = HostTypeList<Packet, HostTypeListEnd>;

        fn wrap<'call>(
            mut call: NativeCall<'call, StatelessHostProfile, Converter, Output, Targets>,
            source: HostValue<'call, CallbackSource>,
            callback: HostCallable<'call, Targets, Output>,
        ) -> Result<crate::host::HostCallContinuation<'call, Output>, HostCallError> {
            let source = NativeValue::tuple([
                NativeValue::symbol("packet"),
                call.source::<CallbackSource>(source),
            ]);
            let packet = call
                .convert::<HostTypeIndex0>(&source)
                .expect("declared envelope should preserve its exact generic payload");
            let packet = Owned::<Packet>::from_host(call.call(), packet);
            let (call, constructions) = call.into_call();
            let callback = call.owned_callable(callback, &constructions);
            Ok(call.resume(constructions, move |context| {
                Box::pin(async move {
                    let result = callback
                        .invoke(
                            &context,
                            move |mut call, _| (packet.into_host(&mut call), ()),
                            |call, _, value| Ok(Owned::<Output>::from_host(&call, value)),
                        )
                        .await?;
                    Ok(crate::host::HostOwnedCompletion::new(move |mut call, _| {
                        let value = result.into_host(&mut call);
                        Ok(call.return_value(value))
                    }))
                })
            }))
        }

        let provider = HostProviderModule::new("application", "main")
            .unwrap()
            .with_resumable_native_function::<
                Converter,
                (CallbackSource, HostFunctionType<Targets, Output>),
                Output,
                Targets,
                _,
            >("wrap", NativeRules::default(), wrap)
            .unwrap();
        for name in ["Bad", "wrap"] {
            let result = HostProviderModule::new("application", "main")
                .unwrap()
                .with_resumable_native_function::<Converter, (CallbackSource, HostFunctionType<Targets, Output>), Output, Targets, _>("wrap", NativeRules::default(), wrap)
                .unwrap()
                .with_resumable_native_function::<Converter, (CallbackSource, HostFunctionType<Targets, Output>), Output, Targets, _>(name, NativeRules::default(), wrap);
            let expected = if name == "Bad" {
                crate::HostRegistrationError::InvalidFunctionName {
                    module: "main".into(),
                    function: name.into(),
                }
            } else {
                crate::HostRegistrationError::DuplicateFunction {
                    module: "main".into(),
                    function: name.into(),
                }
            };
            assert_eq!(result.err(), Some(expected));
        }
        let value = execute(
            provider,
            r#"
pub type Grow(a) { Stop Grow(Grow(List(a))) }
pub type Packet(a) { Packet(a) }
@external(erlang, "native", "wrap")
fn wrap(value: a, callback: fn(Packet(a)) -> b) -> b
pub fn main() {
  let original: Grow(Int) = Grow(Stop)
  let capture = [original]
  #(
    wrap(original, fn(packet) {
      let Packet(value) = packet
      value == original
    }),
    wrap(fn() { capture }, fn(packet) {
      let Packet(callback) = packet
      callback() == [original]
    }),
  )
}
"#,
        )
        .unwrap();
        assert_eq!(value.inspect().to_string(), "#(True, True)");

        let provider = HostProviderModule::new("application", "main").unwrap()
            .with_resumable_native_function::<Converter, (
                CallbackSource, HostFunctionType<Targets, Output>,
            ), Output, Targets, _>("wrap", NativeRules::default(), wrap).unwrap();
        let error = execute(
            provider,
            r#"
pub type Packet(a) { Packet(a) }
@external(erlang, "native", "wrap")
fn wrap(value: a, callback: fn(Packet(a)) -> b) -> b
pub fn main() {
  wrap("wrapped callback", fn(packet) -> Bool {
    let Packet(value) = packet
    panic as value
  })
}
"#,
        )
        .unwrap_err();
        assert_eq!(error.to_string(), "panic: wrapped callback");
    }

    struct TreeSchema;
    struct LeafConstructor;
    struct BranchConstructor;
    struct ValueField;
    struct BranchField;
    struct PacketSchema;
    struct PacketConstructor;

    impl HostCustomSchema for TreeSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Tree";
        const PARAMETER_COUNT: usize = 1;
        type Constructors = HostCustomConstructorList<
            LeafConstructor,
            HostCustomConstructorList<BranchConstructor, HostCustomConstructorListEnd>,
        >;
    }

    struct EmptySchema;
    struct EmptyConstructor;

    impl HostCustomSchema for EmptySchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Empty";
        const PARAMETER_COUNT: usize = 1;
        type Constructors =
            HostCustomConstructorList<EmptyConstructor, HostCustomConstructorListEnd>;
    }

    impl HostCustomConstructorDefinition for EmptyConstructor {
        const NAME: &'static str = "Empty";
        type Fields = HostCustomFieldListEnd;
    }

    #[test]
    fn native_custom_conversion_checks_native_tags_arity_and_each_field() {
        type Targets = HostTypeList<
            HostCustomType<EmptySchema, HostTypeList<EcoString, HostTypeListEnd>>,
            HostTypeList<
                HostCustomType<PacketSchema, HostTypeList<EcoString, HostTypeListEnd>>,
                HostTypeListEnd,
            >,
        >;
        let result = run::<Targets>(
            r#"
@external(erlang, "native", "equal_native")
fn equal_native(value: a, target: b) -> Bool
@external(erlang, "native", "apply_native")
fn apply_native(value: a, callback: fn(b) -> c, fallback: c) -> c
pub type Empty(a) { Empty }
pub type Other { Wrong }
pub type Packet(a) { Packet(a) }
pub fn main() {
  let original: Empty(Int) = Empty
  let target: Empty(String) = Empty
  #(
    equal_native(original, target),
    equal_native(#(), target),
    equal_native(#(original), target),
    equal_native(1, target),
    equal_native(Wrong, target),
    equal_native(#(1, "text"), Packet("text")),
    equal_native(#(Wrong, "text"), Packet("text")),
    equal_native(Packet(<<255>>), Packet("text")),
  )
}
"#,
        );
        assert_eq!(
            result.inspect().to_string(),
            "#(True, False, False, False, False, False, False, False)"
        );
    }

    impl HostCustomConstructorDefinition for LeafConstructor {
        const NAME: &'static str = "Leaf";
        type Fields = HostCustomFieldList<ValueField, HostCustomFieldListEnd>;
    }

    impl HostCustomField for ValueField {
        const LABEL: Option<&'static str> = None;
        type Type = HostCustomTypeArgument<HostTypeIndex0>;
    }

    impl HostCustomConstructorDefinition for BranchConstructor {
        const NAME: &'static str = "Branch";
        type Fields = HostCustomFieldList<BranchField, HostCustomFieldListEnd>;
    }

    impl HostCustomField for BranchField {
        const LABEL: Option<&'static str> = None;
        type Type = HostListType<
            HostCustomType<
                TreeSchema,
                HostTypeList<HostCustomTypeArgument<HostTypeIndex0>, HostTypeListEnd>,
            >,
        >;
    }

    impl HostCustomSchema for PacketSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Packet";
        const PARAMETER_COUNT: usize = 1;
        type Constructors =
            HostCustomConstructorList<PacketConstructor, HostCustomConstructorListEnd>;
    }

    impl HostCustomConstructorDefinition for PacketConstructor {
        const NAME: &'static str = "Packet";
        type Fields = HostCustomFieldList<ValueField, HostCustomFieldListEnd>;
    }

    struct NativeProfile;
    struct NameSchema;
    struct BoxSchema;
    struct OpaqueSchema;
    struct EnvelopeSchema;
    struct NativeStorage;

    #[derive(Default)]
    struct NativeStores {
        names: HostExternalStore<NativeValue>,
        boxes: HostExternalStore<NativeValue>,
    }

    impl HostProfile for NativeProfile {
        type RunState = Vec<EcoString>;
        type ExternalStores = NativeStores;
        type ExecutionState = ();
    }

    impl HostProvider<NativeProfile> for Converter {
        type State = Vec<EcoString>;

        fn project(state: &mut Self::State) -> &mut Self::State {
            state
        }
    }

    impl HostExternalSchema for NameSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Name";
        const PARAMETER_COUNT: usize = 0;
    }

    impl HostExternalSchema for BoxSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Box";
        const PARAMETER_COUNT: usize = 0;
    }

    impl HostExternalSchema for OpaqueSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Opaque";
        const PARAMETER_COUNT: usize = 0;
    }

    impl HostExternalSchema for EnvelopeSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Envelope";
        const PARAMETER_COUNT: usize = 1;
    }

    trait NativeStore: HostExternalSchema {
        fn store(stores: &NativeStores) -> &HostExternalStore<NativeValue>;

        fn native_view(value: &NativeValue) -> Option<NativeValue> {
            Some(value.clone())
        }
    }

    impl NativeStore for NameSchema {
        fn store(stores: &NativeStores) -> &HostExternalStore<NativeValue> {
            &stores.names
        }
    }

    impl NativeStore for BoxSchema {
        fn store(stores: &NativeStores) -> &HostExternalStore<NativeValue> {
            &stores.boxes
        }
    }

    impl NativeStore for OpaqueSchema {
        fn store(stores: &NativeStores) -> &HostExternalStore<NativeValue> {
            &stores.boxes
        }

        fn native_view(_: &NativeValue) -> Option<NativeValue> {
            None
        }
    }

    impl NativeStore for EnvelopeSchema {
        fn store(stores: &NativeStores) -> &HostExternalStore<NativeValue> {
            &stores.boxes
        }
    }

    impl<Schema: NativeStore> HostExternalBinding<NativeProfile, Schema> for Converter {
        type Storage = NativeStorage;
    }

    impl<Schema: NativeStore> HostExternalStorage<NativeProfile, Schema> for NativeStorage {
        type Payload = NativeValue;

        fn store(stores: &NativeStores) -> &HostExternalStore<NativeValue> {
            Schema::store(stores)
        }
        fn source_equal(
            context: &HostExternalEquality<'_>,
            left: &NativeValue,
            right: &NativeValue,
        ) -> bool {
            left.source_equal(context, right)
        }
        fn source_hash(context: &HostExternalHashing<'_>, value: &NativeValue) -> u64 {
            value.source_hash(context)
        }
        fn inspect(context: &HostExternalInspection<'_>, value: &NativeValue) -> EcoString {
            value.inspect(context)
        }
        fn native_view(value: &NativeValue) -> Option<NativeValue> {
            Schema::native_view(value)
        }
    }

    fn name<'call>(
        mut call: HostCall<'call, NativeProfile, Converter, HostExternalType<NameSchema>>,
        value: EcoString,
    ) -> Result<HostCallCompletion<'call, HostExternalType<NameSchema>>, HostCallError> {
        let value = call.create_external(NativeValue::symbol(value));
        Ok(call.return_value(value))
    }

    fn boxed<'call>(
        mut call: HostCall<'call, NativeProfile, Converter, HostExternalType<BoxSchema>>,
        value: HostValue<'call, Source>,
    ) -> Result<HostCallCompletion<'call, HostExternalType<BoxSchema>>, HostCallError> {
        let value = NativeValue::from_stored(call.retain_value::<Source>(value));
        let value = call.create_external(value);
        Ok(call.return_value(value))
    }

    #[test]
    fn native_conversion_combines_external_rules_with_structural_targets() {
        type Tree = HostCustomType<TreeSchema, HostTypeList<EcoString, HostTypeListEnd>>;
        type Envelope = HostExternalType<EnvelopeSchema, HostTypeList<Tree, HostTypeListEnd>>;
        type Extra = HostTypeList<
            HostCustomType<PacketSchema, HostTypeList<EcoString, HostTypeListEnd>>,
            HostTypeList<
                HostCustomType<EmptySchema, HostTypeList<EcoString, HostTypeListEnd>>,
                HostTypeListEnd,
            >,
        >;
        fn envelope<'call>(
            mut call: HostCall<'call, NativeProfile, Converter, Envelope>,
            tree: <Tree as HostType>::Value<'call>,
        ) -> Result<HostCallCompletion<'call, Envelope>, HostCallError> {
            let value = NativeValue::from_stored(call.retain_value::<Tree>(tree));
            let value = call.create_external(value);
            Ok(call.return_value(value))
        }
        fn opaque_value<'call>(
            mut call: HostCall<'call, NativeProfile, Converter, HostExternalType<OpaqueSchema>>,
            value: EcoString,
        ) -> Result<HostCallCompletion<'call, HostExternalType<OpaqueSchema>>, HostCallError>
        {
            let value = call.create_external(NativeValue::symbol(value));
            Ok(call.return_value(value))
        }
        let provider = HostProviderModule::new("application", "main").unwrap()
            .with_external_type::<Converter, NameSchema>().unwrap()
            .with_external_type::<Converter, EnvelopeSchema>().unwrap()
            .with_external_type::<Converter, OpaqueSchema>().unwrap()
            .with_scoped_function::<Converter, (EcoString,), HostExternalType<NameSchema>, _>("name", name).unwrap()
            .with_scoped_function::<Converter, (Tree,), Envelope, _>("envelope", envelope).unwrap()
            .with_scoped_function::<Converter, (EcoString,), HostExternalType<OpaqueSchema>, _>(
                "opaque_value",
                opaque_value,
            ).unwrap()
            .with_native_function::<Converter, (Source, Target), bool, HostTypeList<Target, Extra>, _>(
                "equal_native",
                NativeRules::default()
                    .external::<NameSchema, HostTypeListEnd>(|call, token, value| {
                        let symbol = value.as_symbol()?;
                        Some(call.construct_external(token, NativeValue::symbol(symbol)))
                    })
                    .external::<EnvelopeSchema, HostTypeList<Tree, HostTypeListEnd>>(|call, token, value| {
                        if value.kind() != crate::runtime::NativeKind::Tuple {
                            return None;
                        }
                        let value = call.construct_external(token, value);
                        let stored = call.provider_external_view_with::<Converter, EnvelopeSchema, HostTypeList<Tree, HostTypeListEnd>>(value);
                        assert_eq!(stored.index(0).unwrap().as_symbol().as_deref(), Some("branch"));
                        drop(stored);
                        Some(value)
                    }),
                equal_native::<NativeProfile, Extra>,
            ).unwrap();
        let returned = execute(
            provider,
            r#"
pub type Name
pub type Envelope(a)
pub type Opaque
pub type Tree(a) { Leaf(a) Branch(List(Tree(a))) }
pub type Packet(a) { Packet(a) }
pub type Empty(a) { Empty }
pub type Unknown(a) { Unknown(a) }
pub type Tag { Alpha }
@external(erlang, "native", "name")
fn name(value: String) -> Name
@external(erlang, "native", "envelope")
fn envelope(value: Tree(String)) -> Envelope(Tree(String))
@external(erlang, "native", "opaque_value")
fn opaque_value(value: String) -> Opaque
@external(erlang, "native", "equal_native")
fn equal_native(value: a, target: b) -> Bool
pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  let empty: Empty(Int) = Empty
  let target: Empty(String) = Empty
  let tree = Branch([Leaf("one"), Branch([Leaf("two")])])
  let source = Branch([Leaf(<<"one":utf8>>), Branch([Leaf(<<"two":utf8>>)])])
  let assert True = equal_native(42, 42) as "exact integer"
  let assert True = equal_native("one", "one") as "exact string"
  let assert True = equal_native(2.5, 2.5) as "exact float"
  let assert True = equal_native(codepoint, 65) as "codepoint to integer"
  let assert True = !equal_native(1, 1.0) as "integer is not a float"
  let assert True = equal_native(65, codepoint) as "integer to codepoint"
  let assert True = !equal_native(55296, codepoint) as "invalid codepoint"
  let assert True = !equal_native(-1, codepoint) as "negative codepoint"
  let assert True = !equal_native(4294967296, codepoint) as "oversized codepoint"
  let assert True = !equal_native("A", codepoint) as "non-integer codepoint"
  let assert True = equal_native(<<"one":utf8>>, "one") as "binary to string"
  let assert True = equal_native("one", <<"one":utf8>>) as "string to binary"
  let assert True = !equal_native(<<255>>, "one") as "invalid UTF-8"
  let assert True = equal_native(name("true"), True) as "true symbol"
  let assert True = equal_native(name("false"), False) as "false symbol"
  let assert True = !equal_native(name("alpha"), True) as "non-boolean symbol"
  let assert True = equal_native(name("nil"), Nil) as "nil symbol"
  let assert True = equal_native(#(<<"one":utf8>>), #("one")) as "tuple fields"
  let assert True = !equal_native(#(<<255>>), #("one")) as "invalid tuple field"
  let assert True = equal_native([<<"one":utf8>>], ["one"]) as "list elements"
  let assert True = !equal_native([<<255>>], ["one"]) as "invalid list element"
  let assert True = equal_native(source, tree) as "recursive construction from rule schema"
  let assert True = equal_native(empty, target) as "nullary generic constructor"
  let assert True = !equal_native(#(), target) as "empty native tuple"
  let assert True = !equal_native(#(1, "one"), Packet("one")) as "non-symbol tag"
  let assert True = !equal_native(#(name("wrong"), "one"), Packet("one")) as "unknown tag"
  let assert True = !equal_native(#(name("packet"), "one", 2), Packet("one")) as "wrong arity"
  let assert True = !equal_native(Packet(<<255>>), Packet("one")) as "invalid custom field"
  let assert True = !equal_native(Unknown(<<"one":utf8>>), Unknown("one")) as "undeclared construction"
  let assert True = !equal_native(opaque_value("true"), True) as "opaque terminal"
  let assert True = equal_native(Alpha, name("alpha")) as "external construction"
  let assert True = !equal_native("alpha", name("alpha")) as "rejected external input"
  let assert True = equal_native(source, envelope(tree)) as "generic external construction"
  let assert True = !equal_native(1, envelope(tree)) as "rejected generic external input"
  True
}
"#,
        )
        .unwrap();
        assert_eq!(returned, crate::Value::Bool(true));
    }

    fn apply_external<'call, Extra: HostTypeSequence>(
        mut call: NativeCall<
            'call,
            NativeProfile,
            Converter,
            Output,
            HostTypeList<CallbackTarget, Extra>,
        >,
        source: HostValue<'call, CallbackSource>,
        callback: HostCallable<'call, CallbackTargets, Output>,
        fallback: HostValue<'call, Output>,
    ) -> Result<crate::host::HostCallContinuation<'call, Output>, HostCallError> {
        let source = call.source::<CallbackSource>(source);
        let input = call
            .convert::<HostTypeIndex0>(&source)
            .map(|value| Owned::<CallbackTarget>::from_host(call.call(), value));
        let fallback = Owned::<Output>::from_host(call.call(), fallback);
        let (call, constructions) = call.into_call();
        let callback = call.owned_callable(callback, &constructions);
        Ok(call.resume(constructions, move |context| {
            Box::pin(async move {
                let result = match input {
                    Some(value) => {
                        callback
                            .invoke(
                                &context,
                                move |mut call, _| (value.into_host(&mut call), ()),
                                |call, _, value| Ok(Owned::<Output>::from_host(&call, value)),
                            )
                            .await?
                    }
                    None => fallback,
                };
                Ok(crate::host::HostOwnedCompletion::new(move |mut call, _| {
                    let value = result.into_host(&mut call);
                    Ok(call.return_value(value))
                }))
            })
        }))
    }

    fn ready<'call, Targets: HostTypeSequence>(
        call: NativeCall<'call, NativeProfile, Converter, bool, Targets>,
        _source: HostValue<'call, Source>,
        _target: HostValue<'call, Target>,
    ) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
        Ok(call.finish(true))
    }

    fn register_ready(
        rules: NativeRules<NativeProfile, Converter, bool>,
    ) -> Result<HostProviderModule<NativeProfile>, crate::HostRegistrationError> {
        HostProviderModule::new("application", "main")
            .unwrap()
            .with_external_type::<Converter, EnvelopeSchema>()
            .unwrap()
            .with_native_function::<Converter, (Source, Target), bool, HostTypeListEnd, _>(
                "ready",
                rules,
                ready::<HostTypeListEnd>,
            )
    }

    #[test]
    fn native_registration_rejects_duplicate_and_unbound_rules() {
        assert_eq!(
            <Converter as HostProvider<StatelessHostProfile>>::project(&mut ()),
            &()
        );
        let error = HostProviderModule::new("application", "main").unwrap()
            .with_native_function::<Converter, (Source, Target), bool,
                HostTypeList<HostTypeParameter<2>, HostTypeListEnd>, _>(
                    "unbound_target", NativeRules::default(),
                    ready::<HostTypeList<HostTypeParameter<2>, HostTypeListEnd>>,
                ).err().expect("construction targets must be bound by the source signature");
        assert_eq!(
            error,
            crate::HostRegistrationError::UnboundConstructionTypeParameters {
                function: "unbound_target".into(),
                parameters: Box::new([2]),
            }
        );
        let error = register_ready(
            NativeRules::default()
                .external::<EnvelopeSchema, HostTypeList<Source, HostTypeListEnd>>(|_, _, _| None)
                .external::<EnvelopeSchema, HostTypeList<Source, HostTypeListEnd>>(|_, _, _| None),
        )
        .err()
        .expect("duplicate conversion should fail before source compilation");
        assert_eq!(
            error,
            crate::HostRegistrationError::DuplicateNativeConversion {
                function: "ready".into(),
                type_: crate::plan::ValueType::External(crate::plan::ExternalType::new(
                    crate::plan::ExternalTypeName::new(
                        "application".into(),
                        "main".into(),
                        "Envelope".into(),
                    ),
                    vec![crate::plan::ValueType::Parameter(
                        crate::plan::TypeParameterId(0)
                    )],
                )),
            }
        );

        let error = register_ready(
            NativeRules::default()
                .external::<EnvelopeSchema, HostTypeList<HostTypeParameter<2>, HostTypeListEnd>>(
                    |_, _, _| None,
                ),
        )
        .err()
        .expect("native rule parameters must belong to the original source scheme");
        assert_eq!(
            error,
            crate::HostRegistrationError::UnboundConstructionTypeParameters {
                function: "ready".into(),
                parameters: Box::new([2]),
            }
        );
    }

    fn prepare_ready(
        rules: NativeRules<NativeProfile, Converter, bool>,
        source: &str,
    ) -> Result<crate::HostedExecution<NativeProfile>, crate::HostSpecializationError> {
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [crate::PackageSource::new(
                "application",
                Vec::<String>::new(),
                [crate::ModuleSource::new("main", "src/main.gleam", source)],
            )],
            HostProviderSet::from_providers([register_ready(rules).unwrap()]).unwrap(),
        )
        .unwrap();
        crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
    }

    #[test]
    fn native_rule_overlap_is_checked_after_the_original_generic_scheme_is_specialized() {
        let error = prepare_ready(
            NativeRules::default()
                .external::<EnvelopeSchema, HostTypeList<Source, HostTypeListEnd>>(|_, _, _| None)
                .external::<EnvelopeSchema, HostTypeList<Target, HostTypeListEnd>>(|_, _, _| None),
            r#"
pub type Envelope(a)
@external(erlang, "native", "ready")
fn ready(left: a, right: b) -> Bool
pub fn main() { ready(1, 2) }
"#,
        )
        .err()
        .expect("independent parameters may select conflicting concrete rules");
        assert_eq!(error.package(), "application");
        assert_eq!(error.module(), "main");
        assert_eq!(error.function(), "ready");
        assert_eq!(
            error.signature(),
            &crate::plan::FunctionType::new(
                vec![crate::plan::ValueType::Int, crate::plan::ValueType::Int],
                crate::plan::ValueType::Bool,
            )
        );
        assert_eq!(
            error.reason(),
            &crate::HostSpecializationErrorReason::ConflictingNativeConversions {
                type_: crate::plan::ValueType::External(crate::plan::ExternalType::new(
                    crate::plan::ExternalTypeName::new(
                        "application".into(),
                        "main".into(),
                        "Envelope".into(),
                    ),
                    vec![crate::plan::ValueType::Int],
                )),
            }
        );
    }

    #[test]
    fn native_rules_register_nested_recursive_custom_construction_schemas() {
        let mut execution = prepare_ready(
            NativeRules::default().external::<EnvelopeSchema, HostTypeList<
                HostCustomType<TreeSchema, HostTypeList<EcoString, HostTypeListEnd>>,
                HostTypeListEnd,
            >>(|_, _, _| None),
            r#"
pub type Envelope(a)
pub type Tree(a) { Leaf(a) Branch(List(Tree(a))) }
@external(erlang, "native", "ready")
fn ready(left: a, right: b) -> Bool
pub fn main() { ready(1, "two") }
"#,
        )
        .expect("custom schemas referenced only by a native rule must also seal");
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut Vec::new(), &mut Vec::new())
                .unwrap(),
            crate::Value::Bool(true)
        );
    }

    #[test]
    fn native_exact_external_values_remain_valid_only_with_their_hosted_execution() {
        type Name = HostExternalType<NameSchema>;
        type Targets = HostTypeList<Name, HostTypeListEnd>;
        fn execution(
            previous: std::sync::Arc<std::sync::Mutex<Option<NativeValue>>>,
        ) -> crate::HostedExecution<NativeProfile> {
            let provider = HostProviderModule::new("application", "main")
                .unwrap()
                .with_external_type::<Converter, NameSchema>()
                .unwrap()
                .with_scoped_function::<Converter, (EcoString,), Name, _>("name", name)
                .unwrap()
                .with_native_function::<Converter, (Name,), bool, Targets, _>(
                    "restore_previous",
                    NativeRules::default(),
                    move |mut call: NativeCall<'_, NativeProfile, Converter, bool, Targets>, value| {
                        let previous = previous.lock().unwrap().replace(call.source::<Name>(value));
                        let restored = previous.as_ref().is_some_and(|previous| {
                            call.convert::<HostTypeIndex0>(previous).is_some_and(|value| {
                                let payload = call.call()
                                    .provider_external_view_with::<Converter, NameSchema, HostTypeListEnd>(value);
                                payload.as_symbol().as_deref() == Some("key")
                            })
                        });
                        Ok(call.finish(restored))
                    },
                )
                .unwrap();
            let typed = crate::compile_typed_host_program(
                "application",
                "main",
                [crate::PackageSource::new(
                    "application",
                    Vec::<String>::new(),
                    [crate::ModuleSource::new(
                        "main",
                        "src/main.gleam",
                        r#"
pub type Name
@external(erlang, "native", "name")
fn name(value: String) -> Name
@external(erlang, "native", "restore_previous")
fn restore_previous(value: Name) -> Bool
pub fn main() { restore_previous(name("key")) }
"#,
                    )],
                )],
                HostProviderSet::from_providers([provider]).unwrap(),
            )
            .unwrap();
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap()
        }

        let previous = std::sync::Arc::new(std::sync::Mutex::new(None));
        let mut original = execution(previous.clone());
        let mut other = execution(previous);
        for (first, expected) in [
            (true, false),
            (true, true),
            (false, false),
            (false, true),
            (true, false),
        ] {
            let execution = if first { &mut original } else { &mut other };
            assert_eq!(
                crate::execution_fixture::run(execution, &mut Vec::new(), &mut Vec::new()).unwrap(),
                crate::Value::Bool(expected),
                "exact payload access must stay with the original hosted execution and stores",
            );
        }
    }

    #[test]
    fn opaque_values_remain_terminal_and_keep_their_storage_semantics() {
        type Opaque = HostExternalType<OpaqueSchema>;
        type Targets = HostTypeList<Opaque, HostTypeList<bool, HostTypeListEnd>>;
        fn opaque<'call>(
            mut call: HostCall<'call, NativeProfile, Converter, Opaque>,
            name: EcoString,
        ) -> Result<HostCallCompletion<'call, Opaque>, HostCallError> {
            let value = call.create_external(NativeValue::symbol(name));
            Ok(call.return_value(value))
        }
        fn check<'call>(
            mut call: NativeCall<'call, NativeProfile, Converter, bool, Targets>,
            left: HostExternal<'call, Opaque>,
            right: HostExternal<'call, Opaque>,
        ) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
            let first = call.source::<Opaque>(left);
            let second = call.source::<Opaque>(right);
            assert_eq!(first.kind(), crate::runtime::NativeKind::External);
            assert!(first.as_symbol().is_none());
            assert!(first.index(0).is_none());
            assert!(call.convert::<HostTypeIndex0>(&first).is_some());
            assert!(
                call.convert::<crate::HostTypeIndexNext<HostTypeIndex0>>(&first)
                    .is_none()
            );
            assert!(call.call().native_equal(&first, &second));
            assert_eq!(
                call.call().native_hash(&first),
                call.call().native_hash(&second)
            );
            assert_eq!(
                call.call().source_hash::<Opaque>(left),
                call.call().source_hash::<Opaque>(right)
            );
            assert_eq!(call.call().inspect::<Opaque>(left), "Item");
            let inspect = |_: &crate::runtime::RetainedValueRef| EcoString::from("opaque-source");
            let inspection = crate::host::RetainedValueInspection::new(&inspect);
            assert_eq!(
                first.inspect(&HostExternalInspection(&inspection)),
                "opaque-source"
            );
            let payload = call
                .call()
                .provider_external_view_with::<Converter, OpaqueSchema, HostTypeListEnd>(left);
            assert_eq!(payload.as_symbol().as_deref(), Some("item"));
            drop(payload);
            Ok(call.finish(true))
        }
        let provider = HostProviderModule::new("application", "main")
            .unwrap()
            .with_external_type::<Converter, OpaqueSchema>()
            .unwrap()
            .with_scoped_function::<Converter, (EcoString,), Opaque, _>("make_opaque", opaque)
            .unwrap()
            .with_native_function::<Converter, (Opaque, Opaque), bool, Targets, _>(
                "check",
                NativeRules::default(),
                check,
            )
            .unwrap();
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [crate::PackageSource::new(
                "application",
                Vec::<String>::new(),
                [crate::ModuleSource::new(
                    "main",
                    "src/main.gleam",
                    r#"
pub type Opaque
@external(erlang, "native", "make_opaque")
fn make_opaque(name: String) -> Opaque
@external(erlang, "native", "check")
fn check(left: Opaque, right: Opaque) -> Bool
pub fn main() { check(make_opaque("item"), make_opaque("item")) }
"#,
                )],
            )],
            HostProviderSet::from_providers([provider]).unwrap(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut Vec::new(), &mut Vec::new())
                .unwrap(),
            crate::Value::Bool(true)
        );
    }

    #[test]
    fn external_rules_keep_their_sealed_slot_and_native_kind_while_exact_values_pass_through() {
        fn name_rule<'call>(
            call: &mut HostCall<'call, NativeProfile, Converter, Output>,
            token: HostConstruction<'call, HostExternalType<NameSchema>>,
            source: NativeValue,
        ) -> Option<HostExternal<'call, HostExternalType<NameSchema>>> {
            let value = source.as_symbol()?;
            call.state().push("name".into());
            Some(call.construct_external(token, NativeValue::symbol(value)))
        }
        fn box_rule<'call>(
            call: &mut HostCall<'call, NativeProfile, Converter, Output>,
            token: HostConstruction<'call, HostExternalType<BoxSchema>>,
            source: NativeValue,
        ) -> Option<HostExternal<'call, HostExternalType<BoxSchema>>> {
            call.state().push("box".into());
            let value = call.construct_external(token, source);
            let payload =
                call.provider_external_view_with::<Converter, BoxSchema, HostTypeListEnd>(value);
            assert_eq!(payload.kind(), crate::runtime::NativeKind::Symbol);
            drop(payload);
            Some(value)
        }
        for (reverse_rules, fails) in [(false, false), (true, false), (false, true)] {
            let rules = if reverse_rules {
                NativeRules::default()
                    .external::<BoxSchema, HostTypeListEnd>(box_rule)
                    .external::<NameSchema, HostTypeListEnd>(name_rule)
            } else {
                NativeRules::default()
                    .external::<NameSchema, HostTypeListEnd>(name_rule)
                    .external::<BoxSchema, HostTypeListEnd>(box_rule)
            };
            let provider = HostProviderModule::new("application", "main")
                .unwrap()
                .with_external_type::<Converter, NameSchema>()
                .unwrap()
                .with_external_type::<Converter, BoxSchema>()
                .unwrap()
                .with_scoped_function::<Converter, (EcoString,), HostExternalType<NameSchema>, _>(
                    "name", name,
                )
                .unwrap()
                .with_scoped_function::<Converter, (Source,), HostExternalType<BoxSchema>, _>(
                    "boxed", boxed,
                )
                .unwrap()
                .with_resumable_native_function::<Converter, (
                    CallbackSource,
                    HostFunctionType<CallbackTargets, Output>,
                    Output,
                ), Output, HostTypeList<
                    CallbackTarget,
                    HostTypeList<
                        HostCustomType<PacketSchema, HostTypeList<EcoString, HostTypeListEnd>>,
                        HostTypeListEnd,
                    >,
                >, _>(
                    "apply_native",
                    rules,
                    apply_external::<
                        HostTypeList<
                            HostCustomType<PacketSchema, HostTypeList<EcoString, HostTypeListEnd>>,
                            HostTypeListEnd,
                        >,
                    >,
                )
                .unwrap();
            let source = if fails {
                r#"
pub type Name
pub type Box
pub type Packet(a) { Packet(a) }
@external(erlang, "native", "name")
fn name(value: String) -> Name
@external(erlang, "native", "boxed")
fn boxed(value: a) -> Box
@external(erlang, "native", "apply_native")
fn apply_native(value: a, callback: fn(b) -> c, fallback: c) -> c
pub fn main() {
  apply_native(name("alpha"), fn(_value: Name) -> Bool { panic as "external callback" }, False)
}
"#
            } else {
                r#"
pub type Name
pub type Box
pub type Symbol { Alpha }
pub type Packet(a) { Packet(a) }
@external(erlang, "native", "name")
fn name(value: String) -> Name
@external(erlang, "native", "boxed")
fn boxed(value: a) -> Box
@external(erlang, "native", "apply_native")
fn apply_native(value: a, callback: fn(b) -> c, fallback: c) -> c
pub fn main() {
  #(
    apply_native(boxed(name("alpha")), fn(value: Name) { value == name("alpha") }, False),
    apply_native(Alpha, fn(value: Name) { value == name("alpha") }, False),
    apply_native("alpha", fn(value: Name) { value == name("alpha") }, False),
    apply_native(name("alpha"), fn(value: Box) { value == boxed(name("alpha")) }, False),
    apply_native(name("true"), fn(value: Bool) { value }, False),
    apply_native(name("false"), fn(value: Bool) { !value }, False),
    apply_native(name("nil"), fn(value: Nil) { #(value) }, #(Nil)),
    apply_native(boxed(#(name("packet"), <<"one":utf8>>)), fn(value: Packet(String)) { value == Packet("one") }, False),
    apply_native(boxed(#(name("packet"), 1, 2)), fn(value: Packet(Int)) { value == Packet(1) }, False),
  )
}
"#
            };
            let typed = crate::compile_typed_host_program(
                "application",
                "main",
                [crate::PackageSource::new(
                    "application",
                    Vec::<String>::new(),
                    [crate::ModuleSource::new("main", "src/main.gleam", source)],
                )],
                HostProviderSet::from_providers([provider]).unwrap(),
            )
            .unwrap();
            let mut execution = crate::HostedExecution::try_from_module_plan(
                crate::plan_host_program(typed).unwrap(),
            )
            .unwrap();
            let mut state = Vec::new();
            let returned =
                crate::execution_fixture::run(&mut execution, &mut state, &mut Vec::new());
            if fails {
                assert_eq!(
                    returned.unwrap_err().to_string(),
                    "panic: external callback"
                );
                assert!(state.is_empty());
            } else {
                assert_eq!(
                    returned.unwrap().inspect().to_string(),
                    "#(True, True, False, True, True, True, #(Nil), True, False)"
                );
                assert_eq!(state, ["name", "box"]);
            }
        }
    }
}
