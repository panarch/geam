use geam_core::embedding::{BigInt, BitArrayValue, StringValue};
use geam_core::host::native::{NativeCall, NativeRules};
use geam_core::{
    HostCall, HostCallCompletion, HostCallContinuation, HostCallError, HostCallable,
    HostConstructions, HostCustomConstructorDefinition, HostCustomConstructorList,
    HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList, HostCustomFieldListEnd,
    HostCustomSchema, HostCustomType, HostCustomTypeArgument, HostFunctionType, HostListType,
    HostOwnedCompletion, HostProvider, HostProviderModule, HostProviderSet, HostTypeIndex0,
    HostTypeList, HostTypeListEnd, HostTypeParameter, HostValue, StatelessHostProfile,
};

struct Provider;
impl HostProvider<StatelessHostProfile> for Provider {
    type State = ();
    fn project(state: &mut ()) -> &mut () {
        state
    }
}

struct TreeSchema;
struct Leaf;
struct Branch;
struct Item;
struct Children;

impl HostCustomSchema for TreeSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "Tree";
    const PARAMETER_COUNT: usize = 1;
    type Constructors = HostCustomConstructorList<
        Leaf,
        HostCustomConstructorList<Branch, HostCustomConstructorListEnd>,
    >;
}

impl HostCustomConstructorDefinition for Leaf {
    const NAME: &'static str = "Leaf";
    type Fields = HostCustomFieldList<Item, HostCustomFieldListEnd>;
}

impl HostCustomField for Item {
    const LABEL: Option<&'static str> = None;
    type Type = HostCustomTypeArgument<HostTypeIndex0>;
}

impl HostCustomConstructorDefinition for Branch {
    const NAME: &'static str = "Branch";
    type Fields = HostCustomFieldList<Children, HostCustomFieldListEnd>;
}

impl HostCustomField for Children {
    const LABEL: Option<&'static str> = None;
    type Type = HostListType<
        HostCustomType<
            TreeSchema,
            HostTypeList<HostCustomTypeArgument<HostTypeIndex0>, HostTypeListEnd>,
        >,
    >;
}

type Source = HostTypeParameter<0>;
type Target = HostTypeParameter<1>;
type TextTree = HostCustomType<TreeSchema, HostTypeList<StringValue, HostTypeListEnd>>;
type Targets = HostTypeList<Target, HostTypeList<TextTree, HostTypeListEnd>>;
type IntArgs = HostTypeList<BigInt, HostTypeListEnd>;
type Callback = HostFunctionType<IntArgs, BigInt>;

fn equal_native<'call>(
    mut call: NativeCall<'call, StatelessHostProfile, Provider, bool, Targets>,
    source: HostValue<'call, Source>,
    target: HostValue<'call, Target>,
) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
    let source = call.source::<Source>(source);
    let same = call
        .convert::<HostTypeIndex0>(&source)
        .is_some_and(|converted| call.call().equal::<Target>(converted, target));
    Ok(call.finish(same))
}

fn fold<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, BigInt>,
    constructions: HostConstructions<'call, HostTypeListEnd>,
    callback: HostCallable<'call, IntArgs, BigInt>,
    initial: BigInt,
) -> Result<HostCallContinuation<'call, BigInt>, HostCallError> {
    let callback = call.owned_callable(callback, &constructions);
    Ok(call.resume(constructions, move |context| {
        Box::pin(async move {
            let mut value = initial;
            for _ in 0..3 {
                value = callback
                    .invoke(&context, move |_, _| (value, ()), |_, _, value| Ok(value))
                    .await?;
            }
            Ok(HostOwnedCompletion::new(move |call, _| {
                Ok(call.return_value(value))
            }))
        })
    }))
}

fn keep_bits<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, BitArrayValue>,
    value: BitArrayValue,
) -> Result<HostCallCompletion<'call, BitArrayValue>, HostCallError> {
    Ok(call.return_value(value))
}

pub fn hosts() -> HostProviderSet {
    HostProviderSet::from_providers([HostProviderModule::new("application", "main")
        .unwrap()
        .with_native_function::<Provider, (Source, Target), bool, Targets, _>(
            "equal_native",
            NativeRules::default(),
            equal_native,
        )
        .unwrap()
        .with_resumable_function::<Provider, (Callback, BigInt), BigInt, HostTypeListEnd, _>(
            "fold", fold,
        )
        .unwrap()
        .with_scoped_function::<Provider, (BitArrayValue,), BitArrayValue, _>(
            "keep_bits",
            keep_bits,
        )
        .unwrap()])
    .unwrap()
}
