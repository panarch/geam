use geam_core::embedding::{
    BigInt, BitArrayValue, CustomType, FunctionDeclaration, HostPreparation, NamedTypeSchema,
    PreparedHostedModule, StringValue,
};
use geam_core::execution::ExecutionUnit;
use geam_core::{
    HostCall, HostCallCompletion, HostCallContinuation, HostCallError, HostConstructions,
    HostCustomConstructorAt, HostCustomConstructorDefinition, HostCustomConstructorList,
    HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList, HostCustomFieldListEnd,
    HostCustomIndex0, HostCustomIndexNext, HostCustomSchema, HostCustomType,
    HostCustomTypeArgument, HostFailure, HostOwnedCompletion, HostProfile, HostProvider,
    HostProviderModule, HostProviderSet, HostTupleType, HostTypeIndex0, HostTypeIndexNext,
    HostTypeList, HostTypeListEnd, ModuleSource, PackageSource,
};

pub struct Profile;
impl HostProfile for Profile {
    type RunState = Audit;
    type ExternalStores = ();
    type ExecutionState = ();
}
impl HostProvider<Profile> for Profile {
    type State = Audit;
    fn project(state: &mut Audit) -> &mut Audit {
        state
    }
}
#[derive(Default)]
pub struct Audit {
    pub strings: Vec<StringValue>,
    pub integers: Vec<BigInt>,
    pub integer_result: Option<BigInt>,
    pub fail_at: Option<usize>,
    pub panic_at: Option<usize>,
    pub cancel_at: Option<usize>,
    pub unit: Option<ExecutionUnit>,
}
struct ResultSchema;
struct Success;
struct Failure;
struct SuccessValue;
struct FailureValue;
impl HostCustomSchema for ResultSchema {
    const PACKAGE: &'static str = "";
    const MODULE: &'static str = "gleam";
    const NAME: &'static str = "Result";
    const PARAMETER_COUNT: usize = 2;
    type Constructors = HostCustomConstructorList<
        Success,
        HostCustomConstructorList<Failure, HostCustomConstructorListEnd>,
    >;
}
impl HostCustomConstructorDefinition for Success {
    const NAME: &'static str = "Ok";
    type Fields = HostCustomFieldList<SuccessValue, HostCustomFieldListEnd>;
}
impl HostCustomConstructorDefinition for Failure {
    const NAME: &'static str = "Error";
    type Fields = HostCustomFieldList<FailureValue, HostCustomFieldListEnd>;
}
impl HostCustomField for SuccessValue {
    const LABEL: Option<&'static str> = None;
    type Type = HostCustomTypeArgument<HostTypeIndex0>;
}
impl HostCustomField for FailureValue {
    const LABEL: Option<&'static str> = None;
    type Type = HostCustomTypeArgument<HostTypeIndexNext<HostTypeIndex0>>;
}
type ResultType<Value> =
    HostCustomType<ResultSchema, HostTypeList<Value, HostTypeList<(), HostTypeListEnd>>>;
type StringPairElements = HostTypeList<StringValue, HostTypeList<StringValue, HostTypeListEnd>>;
type StringPair = HostTupleType<StringPairElements>;
type PairReturn = HostTupleType<HostTypeList<bool, HostTypeList<StringPair, HostTypeListEnd>>>;
type PairConstructions = HostTypeList<StringPair, HostTypeListEnd>;
type StringResult = ResultType<StringPair>;
type IntegerResult = ResultType<BigInt>;
type StringSuccess = HostCustomConstructorAt<StringResult, HostCustomIndex0, Success>;
type StringFailure =
    HostCustomConstructorAt<StringResult, HostCustomIndexNext<HostCustomIndex0>, Failure>;
type IntegerSuccess = HostCustomConstructorAt<IntegerResult, HostCustomIndex0, Success>;
type IntegerFailure =
    HostCustomConstructorAt<IntegerResult, HostCustomIndexNext<HostCustomIndex0>, Failure>;
struct EnvelopeSchema;
struct Wrapped;
struct Empty;
struct EnvelopeValue;
impl HostCustomSchema for EnvelopeSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "compound_native_types";
    const NAME: &'static str = "Envelope";
    const PARAMETER_COUNT: usize = 0;
    type Constructors = HostCustomConstructorList<
        Wrapped,
        HostCustomConstructorList<Empty, HostCustomConstructorListEnd>,
    >;
}
impl HostCustomConstructorDefinition for Wrapped {
    const NAME: &'static str = "Wrapped";
    type Fields = HostCustomFieldList<EnvelopeValue, HostCustomFieldListEnd>;
}
impl HostCustomConstructorDefinition for Empty {
    const NAME: &'static str = "Empty";
    type Fields = HostCustomFieldListEnd;
}
type EnvelopePair = HostTupleType<HostTypeList<bool, HostTypeList<StringResult, HostTypeListEnd>>>;
impl HostCustomField for EnvelopeValue {
    const LABEL: Option<&'static str> = None;
    type Type = EnvelopePair;
}
type Envelope = HostCustomType<EnvelopeSchema, HostTypeListEnd>;
type EnvelopeWrapped = HostCustomConstructorAt<Envelope, HostCustomIndex0, Wrapped>;
type EnvelopeEmpty =
    HostCustomConstructorAt<Envelope, HostCustomIndexNext<HostCustomIndex0>, Empty>;
type EnvelopeConstructions = HostTypeList<
    StringPair,
    HostTypeList<StringResult, HostTypeList<EnvelopePair, HostTypeListEnd>>,
>;
struct MarkerSchema;
struct Found;
impl HostCustomSchema for MarkerSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "compound_native_types";
    const NAME: &'static str = "Marker";
    const PARAMETER_COUNT: usize = 0;
    type Constructors = HostCustomConstructorList<Found, HostCustomConstructorListEnd>;
}
impl HostCustomConstructorDefinition for Found {
    const NAME: &'static str = "Found";
    type Fields = HostCustomFieldListEnd;
}
pub struct MarkerType;
impl NamedTypeSchema for MarkerType {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "compound_native_types";
    const NAME: &'static str = "Marker";
}
type Marker = HostCustomType<MarkerSchema, HostTypeListEnd>;
type MarkerFound = HostCustomConstructorAt<Marker, HostCustomIndex0, Found>;
pub type Primitives = (BigInt, f64, bool, (), char, StringValue, BitArrayValue);
type PrimitiveTuple = HostTupleType<
    HostTypeList<
        BigInt,
        HostTypeList<
            f64,
            HostTypeList<
                bool,
                HostTypeList<
                    (),
                    HostTypeList<
                        char,
                        HostTypeList<StringValue, HostTypeList<BitArrayValue, HostTypeListEnd>>,
                    >,
                >,
            >,
        >,
    >,
>;

fn observe(state: &mut Audit, value: &StringValue) -> Result<(), HostCallError> {
    state.strings.push(value.clone());
    let count = state.strings.len();
    if state.cancel_at == Some(count) {
        state.unit.as_ref().unwrap().cancel();
    }
    if state.fail_at == Some(count) {
        return Err(HostFailure::new("compound native failure").into());
    }
    assert_ne!(state.panic_at, Some(count), "compound native Rust panic");
    Ok(())
}
fn split(value: StringValue) -> Option<(StringValue, StringValue)> {
    let text = value.as_str().unwrap();
    let first = text.chars().next()?;
    let boundary = first.len_utf8();
    Some((text[..boundary].into(), text[boundary..].into()))
}
fn next<'call>(
    mut call: HostCall<'call, Profile, Profile, StringResult>,
    constructions: HostConstructions<'call, PairConstructions>,
    value: StringValue,
) -> Result<HostCallCompletion<'call, StringResult>, HostCallError> {
    let unit = call.execution_unit();
    call.state().unit = unit;
    observe(call.state(), &value)?;
    Ok(finish_next(call, constructions, split(value)))
}
fn continuing_next<'call>(
    mut call: HostCall<'call, Profile, Profile, StringResult>,
    constructions: HostConstructions<'call, PairConstructions>,
    value: StringValue,
) -> Result<HostCallContinuation<'call, StringResult>, HostCallError> {
    let unit = call.execution_unit();
    call.state().unit = unit;
    observe(call.state(), &value)?;
    let returned = split(value);
    Ok(call.resume(constructions, move |_| {
        Box::pin(async move {
            Ok(HostOwnedCompletion::new(move |call, constructions| {
                Ok(finish_next(call, constructions, returned))
            }))
        })
    }))
}
fn finish_next<'call>(
    mut call: HostCall<'call, Profile, Profile, StringResult>,
    constructions: HostConstructions<'call, PairConstructions>,
    value: Option<(StringValue, StringValue)>,
) -> HostCallCompletion<'call, StringResult> {
    match value {
        Some((first, rest)) => {
            let pair =
                call.construct_tuple(constructions.at::<HostTypeIndex0>(), (first, (rest, ())));
            call.return_custom::<StringSuccess>((pair, ()))
        }
        None => call.return_custom::<StringFailure>(((), ())),
    }
}
fn pair<'call>(
    mut call: HostCall<'call, Profile, Profile, PairReturn>,
    constructions: HostConstructions<'call, PairConstructions>,
    value: StringValue,
) -> Result<HostCallCompletion<'call, PairReturn>, HostCallError> {
    let unit = call.execution_unit();
    call.state().unit = unit;
    observe(call.state(), &value)?;
    let (available, (first, rest)) = match split(value) {
        Some(pair) => (true, pair),
        None => (false, ("".into(), "".into())),
    };
    let pair = call.construct_tuple(constructions.at::<HostTypeIndex0>(), (first, (rest, ())));
    Ok(call.return_tuple((available, (pair, ()))))
}
fn envelope<'call>(
    mut call: HostCall<'call, Profile, Profile, Envelope>,
    constructions: HostConstructions<'call, EnvelopeConstructions>,
    value: StringValue,
) -> Result<HostCallCompletion<'call, Envelope>, HostCallError> {
    let unit = call.execution_unit();
    call.state().unit = unit;
    observe(call.state(), &value)?;
    if value.as_str().unwrap() == "!" {
        return Ok(call.return_custom::<EnvelopeEmpty>(()));
    }
    let (available, returned) = if value.as_str().unwrap() == "?" {
        (true, None)
    } else {
        let returned = split(value);
        (returned.is_some(), returned)
    };
    let result = match returned {
        Some((first, rest)) => {
            let pair =
                call.construct_tuple(constructions.at::<HostTypeIndex0>(), (first, (rest, ())));
            call.construct_custom::<StringSuccess>(
                constructions.at::<HostTypeIndexNext<HostTypeIndex0>>(),
                (pair, ()),
            )
        }
        None => call.construct_custom::<StringFailure>(
            constructions.at::<HostTypeIndexNext<HostTypeIndex0>>(),
            ((), ()),
        ),
    };
    let pair = call.construct_tuple(
        constructions.at::<HostTypeIndexNext<HostTypeIndexNext<HostTypeIndex0>>>(),
        (available, (result, ())),
    );
    Ok(call.return_custom::<EnvelopeWrapped>((pair, ())))
}
fn integer<'call>(
    mut call: HostCall<'call, Profile, Profile, IntegerResult>,
    value: BigInt,
) -> Result<HostCallCompletion<'call, IntegerResult>, HostCallError> {
    call.state().integers.push(value.clone());
    let value = call.state().integer_result.clone().unwrap_or(value);
    Ok(if value == BigInt::from(-1) {
        call.return_custom::<IntegerFailure>(((), ()))
    } else {
        call.return_custom::<IntegerSuccess>((value, ()))
    })
}
// Preserve the seven separate source arguments in the actual Native ABI test.
#[allow(clippy::too_many_arguments)]
fn primitives<'call>(
    call: HostCall<'call, Profile, Profile, PrimitiveTuple>,
    i: BigInt,
    f: f64,
    b: bool,
    n: (),
    u: char,
    s: StringValue,
    a: BitArrayValue,
) -> Result<HostCallCompletion<'call, PrimitiveTuple>, HostCallError> {
    Ok(call.return_tuple((i, (f, (b, (n, (u, (s, (a, ())))))))))
}
fn marker<'call>(
    mut call: HostCall<'call, Profile, Profile, Marker>,
) -> Result<HostCallCompletion<'call, Marker>, HostCallError> {
    call.state().strings.push("marker".into());
    Ok(call.return_custom::<MarkerFound>(()))
}
fn mirror<'call>(
    mut call: HostCall<'call, Profile, Profile, StringValue>,
    value: StringValue,
) -> Result<HostCallCompletion<'call, StringValue>, HostCallError> {
    observe(call.state(), &value)?;
    Ok(call.return_value(value))
}
pub fn hosts(continuing: bool) -> HostProviderSet<Profile> {
    let module = HostProviderModule::new("application", "compound_native").unwrap();
    let module = if continuing {
        module
            .with_resumable_function::<Profile, (StringValue,), StringResult, PairConstructions, _>(
                "next",
                continuing_next,
            )
            .unwrap()
    } else {
        module.with_scoped_function_and_constructions::<Profile, (StringValue,), StringResult, PairConstructions, _>("next", next).unwrap()
    };
    let module = module.with_scoped_function_and_constructions::<Profile, (StringValue,), PairReturn, PairConstructions, _>("pair", pair).unwrap();
    let module = module.with_scoped_function_and_constructions::<Profile, (StringValue,), Envelope, EnvelopeConstructions, _>("envelope", envelope).unwrap();
    let module = module
        .with_scoped_function::<Profile, (BigInt,), IntegerResult, _>("integer", integer)
        .unwrap();
    let module = module
        .with_scoped_function::<Profile, Primitives, PrimitiveTuple, _>("primitives", primitives)
        .unwrap();
    let module = module
        .with_scoped_function::<Profile, (), Marker, _>("marker", marker)
        .unwrap();
    let module = module
        .with_scoped_function::<Profile, (StringValue,), StringValue, _>("mirror", mirror)
        .unwrap();
    HostProviderSet::from_providers([module]).unwrap()
}
pub fn packages() -> Vec<PackageSource> {
    vec![PackageSource::new(
        "application",
        Vec::<String>::new(),
        [
            ModuleSource::new(
                "compound_native",
                "src/compound_native.gleam",
                include_str!("compound_native_calls.gleam"),
            ),
            ModuleSource::new(
                "compound_native_types",
                "src/compound_native_types.gleam",
                include_str!("compound_native_types.gleam"),
            ),
        ],
    )]
}
pub fn prepare() -> PreparedHostedModule {
    let typed = geam_core::compile_declared_host_program(
        "application",
        "compound_native",
        packages(),
        hosts(false).into_declarations(),
    )
    .unwrap();
    let mut bindings = HostPreparation::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(StringValue, BigInt), BigInt>::new(
            "walk",
        ))
        .unwrap();
    for name in ["walk_pair", "walk_envelope", "after_native"] {
        bindings
            .function(FunctionDeclaration::<(StringValue, BigInt), BigInt>::new(
                name,
            ))
            .unwrap();
    }
    bindings
        .function(FunctionDeclaration::<(BigInt,), BigInt>::new("guarded"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(), bool>::new("fieldless_control"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(StringValue,), bool>::new(
            "empty_alias",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<((BigInt, ()),), ()>::new("nil_index"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(), ()>::new("nil_field_control"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(StringValue,), BigInt>::new(
            "after_native_panic",
        ))
        .unwrap();
    for name in ["identity_tuple", "all_fields", "custom_field_values"] {
        bindings
            .function(FunctionDeclaration::<Primitives, Primitives>::new(name))
            .unwrap();
    }
    bindings
        .function(FunctionDeclaration::<Primitives, bool>::new(
            "assert_zero_float",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<
            (StringValue,),
            (bool, (StringValue, StringValue)),
        >::new("aliased"))
        .unwrap();
    for name in ["string_native_root", "string_native_after"] {
        bindings
            .function(FunctionDeclaration::<(StringValue,), StringValue>::new(
                name,
            ))
            .unwrap();
    }
    bindings
        .function(FunctionDeclaration::<(), CustomType<MarkerType>>::new(
            "custom_native_root",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(CustomType<MarkerType>,), bool>::new(
            "marker_matches",
        ))
        .unwrap();
    bindings.prepare().unwrap()
}
#[cfg(test)]
pub fn source(continuing: bool) -> geam_core::embedding::HostedModuleBuilder<Profile> {
    geam_core::embedding::HostedModuleBuilder::new(
        geam_core::compile_typed_host_program(
            "application",
            "compound_native",
            packages(),
            hosts(continuing),
        )
        .unwrap(),
    )
    .unwrap()
}
