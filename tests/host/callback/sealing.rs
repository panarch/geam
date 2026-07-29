use ecow::EcoString;
use geam::{
    BitArrayValue, HostCall, HostCallCompletion, HostCallError, HostCustom,
    HostCustomConstructorDefinition, HostCustomConstructorList, HostCustomConstructorListEnd,
    HostCustomField, HostCustomFieldList, HostCustomFieldListEnd, HostCustomSchema, HostCustomType,
    HostFunctionType, HostList, HostListType, HostProvider, HostProviderModule, HostProviderSet,
    HostSpecializationErrorReason, HostTuple, HostTupleType, HostTypeList, HostTypeListEnd,
    HostTypeParameter, HostedExecution, ModuleSource, PackageSource, StatelessHostProfile,
    compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;

struct Provider;

impl HostProvider<StatelessHostProfile> for Provider {
    type State = ();

    fn project(state: &mut ()) -> &mut Self::State {
        state
    }
}

type GenericArgument = HostTypeParameter<0>;
type GenericCallbackArguments = HostTypeList<GenericArgument, HostTypeListEnd>;
type GenericCallback = HostFunctionType<GenericCallbackArguments, BigInt>;
type CallbackList = HostListType<GenericCallback>;
type CallbackTupleElements = HostTypeList<GenericCallback, HostTypeList<BigInt, HostTypeListEnd>>;
type CallbackTuple = HostTupleType<CallbackTupleElements>;

fn accept_list<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, BigInt>,
    _values: HostList<'call, GenericCallback>,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
    Ok(call.return_value(1.into()))
}

fn accept_tuple<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, BigInt>,
    _value: HostTuple<'call, CallbackTupleElements>,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
    Ok(call.return_value(1.into()))
}

struct NeverSchema;

impl HostCustomSchema for NeverSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "Never";
    const PARAMETER_COUNT: usize = 0;

    type Constructors = HostCustomConstructorListEnd;
}

type Never = HostCustomType<NeverSchema>;
type UninhabitedTupleElements = HostTypeList<Never, HostTypeList<GenericCallback, HostTypeListEnd>>;
type UninhabitedTuple = HostTupleType<UninhabitedTupleElements>;

fn accept_uninhabited_tuple<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, BigInt>,
    _value: HostTuple<'call, UninhabitedTupleElements>,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
    Ok(call.return_value(1.into()))
}

#[test]
fn rejects_a_symbolic_callback_nested_in_a_list() {
    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Provider, (CallbackList,), BigInt, _>("accept_list", accept_list)
        .expect("callback list should register");
    let source = r#"
@external(erlang, "host", "accept_list")
fn accept_list(values: List(fn(value) -> Int)) -> Int

fn generic(_value) {
  1
}

pub fn main() {
  accept_list([generic])
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<&str>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::with_providers(Vec::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("nested callback list source should compile");
    let plan = plan_host_program(typed).expect("nested callback list source should plan");
    let Err(error) = HostedExecution::try_from_module_plan(plan) else {
        panic!("a symbolic callback list should not expose invocation");
    };

    assert_eq!(error.function(), "accept_list");
    assert!(matches!(
        error.reason(),
        HostSpecializationErrorReason::UninhabitedCallbackArguments { .. }
    ));
}

#[test]
fn rejects_a_symbolic_callback_nested_in_a_tuple() {
    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Provider, (CallbackTuple,), BigInt, _>("accept_tuple", accept_tuple)
        .expect("callback tuple should register");
    let source = r#"
@external(erlang, "host", "accept_tuple")
fn accept_tuple(value: #(fn(argument) -> Int, Int)) -> Int

fn generic(_value) {
  1
}

pub fn main() {
  accept_tuple(#(generic, 1))
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<&str>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::with_providers(Vec::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("nested callback tuple source should compile");
    let plan = plan_host_program(typed).expect("nested callback tuple source should plan");
    let Err(error) = HostedExecution::try_from_module_plan(plan) else {
        panic!("a symbolic callback tuple should not expose invocation");
    };

    assert_eq!(error.function(), "accept_tuple");
    let HostSpecializationErrorReason::UninhabitedCallbackArguments { callback } = error.reason()
    else {
        panic!("the tuple callback should own the sealing reason");
    };
    assert!(matches!(
        callback.argument_types()[0],
        geam::ValueType::Parameter(_)
    ));
}

#[test]
fn erases_a_callback_inside_an_uninhabited_tuple() {
    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Provider, (UninhabitedTuple,), BigInt, _>(
            "accept_uninhabited_tuple",
            accept_uninhabited_tuple,
        )
        .expect("uninhabited callback tuple should register");
    let source = r#"
pub type Never

@external(erlang, "host", "accept_uninhabited_tuple")
fn accept_uninhabited_tuple(value: #(Never, fn(argument) -> Int)) -> Int

pub fn main() {
  accept_uninhabited_tuple
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<&str>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::with_providers(Vec::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("uninhabited callback tuple source should compile");
    let plan = plan_host_program(typed).expect("uninhabited callback tuple source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("an uninhabited tuple must not expose its callback");
    let value = execution
        .run_main(&mut (), &mut Vec::new())
        .expect("uninhabited callback tuple reference should materialize");

    assert_eq!(value.inspect().to_string(), "//fn(a) { ... }");
}

struct EndDefinition;

impl HostCustomConstructorDefinition for EndDefinition {
    const NAME: &'static str = "End";

    type Fields = HostCustomFieldListEnd;
}

struct NextField;

impl HostCustomField for NextField {
    const LABEL: Option<&'static str> = None;

    type Type = HostCustomType<RecursiveSchema>;
}

struct NextDefinition;

impl HostCustomConstructorDefinition for NextDefinition {
    const NAME: &'static str = "Next";

    type Fields = HostCustomFieldList<NextField, HostCustomFieldListEnd>;
}

struct RecursiveSchema;

impl HostCustomSchema for RecursiveSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "Recursive";
    const PARAMETER_COUNT: usize = 0;

    type Constructors = HostCustomConstructorList<
        EndDefinition,
        HostCustomConstructorList<NextDefinition, HostCustomConstructorListEnd>,
    >;
}

struct ParameterField;
struct IntField;
struct FloatField;
struct StringField;
struct BitArrayField;
struct UtfCodepointField;
struct BoolField;
struct NilField;
struct ListField;
struct TupleField;
struct RecursiveField;
struct WrappedField;
struct FunctionField;

impl HostCustomField for ParameterField {
    const LABEL: Option<&'static str> = None;

    type Type = HostTypeParameter<0>;
}

impl HostCustomField for IntField {
    const LABEL: Option<&'static str> = None;

    type Type = BigInt;
}

impl HostCustomField for FloatField {
    const LABEL: Option<&'static str> = None;

    type Type = f64;
}

impl HostCustomField for StringField {
    const LABEL: Option<&'static str> = None;

    type Type = EcoString;
}

impl HostCustomField for BitArrayField {
    const LABEL: Option<&'static str> = None;

    type Type = BitArrayValue;
}

impl HostCustomField for UtfCodepointField {
    const LABEL: Option<&'static str> = None;

    type Type = char;
}

impl HostCustomField for BoolField {
    const LABEL: Option<&'static str> = None;

    type Type = bool;
}

impl HostCustomField for NilField {
    const LABEL: Option<&'static str> = None;

    type Type = ();
}

impl HostCustomField for ListField {
    const LABEL: Option<&'static str> = None;

    type Type = HostListType<BigInt>;
}

impl HostCustomField for TupleField {
    const LABEL: Option<&'static str> = None;

    type Type = HostTupleType<HostTypeList<BigInt, HostTypeList<bool, HostTypeListEnd>>>;
}

impl HostCustomField for RecursiveField {
    const LABEL: Option<&'static str> = None;

    type Type = HostCustomType<RecursiveSchema>;
}

struct WrappedValueField;

impl HostCustomField for WrappedValueField {
    const LABEL: Option<&'static str> = None;

    type Type = HostTypeParameter<0>;
}

struct WrappedDefinition;

impl HostCustomConstructorDefinition for WrappedDefinition {
    const NAME: &'static str = "Wrapped";

    type Fields = HostCustomFieldList<WrappedValueField, HostCustomFieldListEnd>;
}

struct WrappedSchema;

impl HostCustomSchema for WrappedSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "Wrapped";
    const PARAMETER_COUNT: usize = 1;

    type Constructors = HostCustomConstructorList<WrappedDefinition, HostCustomConstructorListEnd>;
}

impl HostCustomField for WrappedField {
    const LABEL: Option<&'static str> = None;

    type Type = HostCustomType<WrappedSchema, HostTypeList<HostTypeParameter<0>, HostTypeListEnd>>;
}

impl HostCustomField for FunctionField {
    const LABEL: Option<&'static str> = None;

    type Type = HostFunctionType<HostTypeList<HostTypeParameter<1>, HostTypeListEnd>, BigInt>;
}

type EnvelopeFields = HostCustomFieldList<
    ParameterField,
    HostCustomFieldList<
        IntField,
        HostCustomFieldList<
            FloatField,
            HostCustomFieldList<
                StringField,
                HostCustomFieldList<
                    BitArrayField,
                    HostCustomFieldList<
                        UtfCodepointField,
                        HostCustomFieldList<
                            BoolField,
                            HostCustomFieldList<
                                NilField,
                                HostCustomFieldList<
                                    ListField,
                                    HostCustomFieldList<
                                        TupleField,
                                        HostCustomFieldList<
                                            RecursiveField,
                                            HostCustomFieldList<
                                                WrappedField,
                                                HostCustomFieldList<
                                                    FunctionField,
                                                    HostCustomFieldListEnd,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >,
>;

struct EnvelopeDefinition;

impl HostCustomConstructorDefinition for EnvelopeDefinition {
    const NAME: &'static str = "Envelope";

    type Fields = EnvelopeFields;
}

struct EnvelopeSchema;

impl HostCustomSchema for EnvelopeSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "Envelope";
    const PARAMETER_COUNT: usize = 2;

    type Constructors = HostCustomConstructorList<EnvelopeDefinition, HostCustomConstructorListEnd>;
}

type EnvelopeArguments = HostTypeList<BigInt, HostTypeList<HostTypeParameter<0>, HostTypeListEnd>>;
type Envelope = HostCustomType<EnvelopeSchema, EnvelopeArguments>;

fn accept_envelope<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, BigInt>,
    _value: HostCustom<'call, Envelope>,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
    Ok(call.return_value(1.into()))
}

#[test]
fn rejects_a_symbolic_callback_nested_in_a_complete_recursive_custom_schema() {
    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Provider, (Envelope,), BigInt, _>(
            "accept_envelope",
            accept_envelope,
        )
        .expect("custom callback envelope should register");
    let source = r#"
pub type Recursive {
  End
  Next(Recursive)
}

pub type Wrapped(value) {
  Wrapped(value)
}

pub type Envelope(first, callback_argument) {
  Envelope(
    first,
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    List(Int),
    #(Int, Bool),
    Recursive,
    Wrapped(first),
    fn(callback_argument) -> Int,
  )
}

@external(erlang, "host", "accept_envelope")
fn accept_envelope(value: Envelope(Int, callback_argument)) -> Int

fn generic(_value) {
  1
}

pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  accept_envelope(
    Envelope(
      0,
      1,
      1.5,
      "text",
      <<1>>,
      codepoint,
      True,
      Nil,
      [1],
      #(1, False),
      End,
      Wrapped(1),
      generic,
    ),
  )
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<&str>::new(),
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::with_providers(Vec::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("recursive custom callback source should compile");
    let plan = plan_host_program(typed).expect("recursive custom callback source should plan");
    let Err(error) = HostedExecution::try_from_module_plan(plan) else {
        panic!("a custom-contained symbolic callback should not expose invocation");
    };

    assert_eq!(error.function(), "accept_envelope");
    let HostSpecializationErrorReason::UninhabitedCallbackArguments { callback } = error.reason()
    else {
        panic!("the custom-contained callback should own the sealing reason");
    };
    assert!(matches!(
        callback.argument_types()[0],
        geam::ValueType::Parameter(_)
    ));
    assert_eq!(callback.return_(), &geam::ValueType::Int);
}
