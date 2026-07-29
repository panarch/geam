use ecow::EcoString;
use geam::{
    BitArrayValue, HostCall, HostCallCompletion, HostCallError, HostCustom,
    HostCustomConstructorAt, HostCustomConstructorDefinition, HostCustomConstructorList,
    HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList, HostCustomFieldListEnd,
    HostCustomIndex0, HostCustomIndexNext, HostCustomSchema, HostCustomType, HostFailure, HostList,
    HostListType, HostLocation, HostModule, HostProvider, HostProviderModule, HostProviderSet,
    HostTuple, HostTupleType, HostTypeList, HostTypeListEnd, HostTypeParameter, HostValue,
    HostedExecution, ListValue, ModuleSource, PackageSource, StatelessHostProfile, Value,
    compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;

struct Identity;

impl HostProvider<StatelessHostProfile> for Identity {
    type State = ();

    fn project(state: &mut ()) -> &mut Self::State {
        state
    }
}

struct BoxedSchema;

struct EmptyDefinition;

impl HostCustomConstructorDefinition for EmptyDefinition {
    const NAME: &'static str = "Empty";

    type Fields = HostCustomFieldListEnd;
}

struct BoxedValueField;

impl HostCustomField for BoxedValueField {
    const LABEL: Option<&'static str> = Some("value");

    type Type = HostTypeParameter<0>;
}

struct BoxedEnabledField;

impl HostCustomField for BoxedEnabledField {
    const LABEL: Option<&'static str> = Some("enabled");

    type Type = bool;
}

struct BoxedDefinition;

impl HostCustomConstructorDefinition for BoxedDefinition {
    const NAME: &'static str = "Boxed";

    type Fields = HostCustomFieldList<
        BoxedValueField,
        HostCustomFieldList<BoxedEnabledField, HostCustomFieldListEnd>,
    >;
}

impl HostCustomSchema for BoxedSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "Boxed";
    const PARAMETER_COUNT: usize = 1;

    type Constructors = HostCustomConstructorList<
        EmptyDefinition,
        HostCustomConstructorList<BoxedDefinition, HostCustomConstructorListEnd>,
    >;
}

struct NeverSchema;

impl HostCustomSchema for NeverSchema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "main";
    const NAME: &'static str = "Never";
    const PARAMETER_COUNT: usize = 0;

    type Constructors = HostCustomConstructorListEnd;
}

type BoxedArguments = HostTypeList<HostTypeParameter<0>, HostTypeListEnd>;
type Boxed = HostCustomType<BoxedSchema, BoxedArguments>;
type Empty = HostCustomConstructorAt<Boxed, HostCustomIndex0, EmptyDefinition>;
type BoxedValue =
    HostCustomConstructorAt<Boxed, HostCustomIndexNext<HostCustomIndex0>, BoxedDefinition>;

#[test]
fn specializes_one_generic_provider_for_each_concrete_call_shape() {
    type ValueType = HostTypeParameter<0>;

    fn identity<'call>(
        call: HostCall<'call, StatelessHostProfile, Identity, ValueType>,
        value: HostValue<'call, ValueType>,
    ) -> Result<HostCallCompletion<'call, ValueType>, HostCallError> {
        Ok(call.return_value(value))
    }

    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Identity, (ValueType,), ValueType, _>("identity", identity)
        .expect("generic provider should be valid");
    let source = r#"
@external(erlang, "host", "identity")
fn identity(value: value) -> value

pub fn main() {
  #(identity(42), identity(2), identity(True))
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
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host source should compile");
    let plan = plan_host_program(typed).expect("host source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
    let expected_explanation = r#"
module main
main tuple#0

function int#0
  host application::main.identity signature=fn(Int) -> Int

function bool#0
  host application::main.identity signature=fn(Bool) -> Bool

function tuple#0
  entry b0 params=[] captures=[]
  block b0 params=[]
    %int#0:shape#0(Int) = int.value 42
    %int#1:shape#0(Int) = int.call int#0 args=[%int#0]
    %int#2:shape#0(Int) = int.value 2
    %int#3:shape#0(Int) = int.call int#0 args=[%int#2]
    %bool#0:shape#1(Bool) = bool.value True
    %bool#1:shape#1(Bool) = bool.call bool#0 args=[%bool#0]
    %tuple#0:shape#2(#(Int, Int, Bool)) = tuple.value elements=[%int#1, %int#3, %bool#1]
    return %tuple#0
"#
    .trim();

    assert_eq!(execution.explain().to_string().trim(), expected_explanation);
    assert_eq!(
        execution.run_main(&mut (), &mut Vec::new()),
        Ok(Value::Tuple(vec![
            Value::Int(42.into()),
            Value::Int(2.into()),
            Value::Bool(true),
        ])),
    );
}

#[test]
fn passes_function_values_through_generic_provider_specialization() {
    type ValueType = HostTypeParameter<0>;

    fn identity<'call>(
        call: HostCall<'call, StatelessHostProfile, Identity, ValueType>,
        value: HostValue<'call, ValueType>,
    ) -> Result<HostCallCompletion<'call, ValueType>, HostCallError> {
        Ok(call.return_value(value))
    }

    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Identity, (ValueType,), ValueType, _>("identity", identity)
        .expect("generic provider should be valid");
    let source = r#"
@external(erlang, "host", "identity")
fn identity(value: value) -> value

fn increment(value: Int) -> Int {
  value + 1
}

pub fn main() {
  let returned = identity(increment)
  returned(41)
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
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host source should compile");
    let plan = plan_host_program(typed).expect("host source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");

    assert_eq!(
        execution.run_main(&mut (), &mut Vec::new()),
        Ok(Value::Int(42.into())),
    );
}

#[test]
fn preserves_every_function_return_family_through_generic_provider_specialization() {
    type ValueType = HostTypeParameter<0>;

    fn identity<'call>(
        call: HostCall<'call, StatelessHostProfile, Identity, ValueType>,
        value: HostValue<'call, ValueType>,
    ) -> Result<HostCallCompletion<'call, ValueType>, HostCallError> {
        Ok(call.return_value(value))
    }

    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Identity, (ValueType,), ValueType, _>("identity", identity)
        .expect("generic provider should be valid");
    let source = r#"
pub type Marker {
  Marker(Int)
}

@external(erlang, "host", "identity")
fn identity(value: value) -> value

fn generic(value) { value }
fn never() -> value { panic }
fn int_value() { 1 }
fn float_value() { 1.5 }
fn string_value() { "text" }
fn bit_array_value() { <<1>> }
fn utf_codepoint_value() -> UtfCodepoint {
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  codepoint
}
fn custom_value() { Marker(6) }
fn bool_value() { True }
fn nil_value() { Nil }
fn tuple_value() { #(7, False) }
fn parameter_list() -> List(value) { [] }
fn parameter_list_list() -> List(List(value)) { [] }
fn int_list() { [1] }
fn string_list() { ["text"] }
fn bit_array_list() { [<<1>>] }
fn utf_codepoint_list() -> List(UtfCodepoint) {
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  [codepoint]
}
fn custom_list() { [Marker(6)] }
fn float_list() { [1.5] }
fn bool_list() { [True] }
fn nil_list() { [Nil] }
fn tuple_list() { [#(7, False)] }
fn list_list() { [[8, 9]] }
fn function_list() { [int_value] }
fn function_value() { int_value }

pub fn main() {
  let returned_generic = identity(generic)
  let returned_never = identity(never)
  let returned_int = identity(int_value)
  let returned_float = identity(float_value)
  let returned_string = identity(string_value)
  let returned_bit_array = identity(bit_array_value)
  let returned_utf_codepoint = identity(utf_codepoint_value)
  let returned_custom = identity(custom_value)
  let returned_bool = identity(bool_value)
  let returned_nil = identity(nil_value)
  let returned_tuple = identity(tuple_value)
  let returned_parameter_list = identity(parameter_list)
  let returned_parameter_list_list = identity(parameter_list_list)
  let returned_int_list = identity(int_list)
  let returned_string_list = identity(string_list)
  let returned_bit_array_list = identity(bit_array_list)
  let returned_utf_codepoint_list = identity(utf_codepoint_list)
  let returned_custom_list = identity(custom_list)
  let returned_float_list = identity(float_list)
  let returned_bool_list = identity(bool_list)
  let returned_nil_list = identity(nil_list)
  let returned_tuple_list = identity(tuple_list)
  let returned_list_list = identity(list_list)
  let returned_function_list = identity(function_list)
  let returned_function = identity(function_value)

  let assert 1 = returned_int()
  let assert 1.5 = returned_float()
  let assert "text" = returned_string()
  let assert <<1>> = returned_bit_array()
  let assert <<65>> = <<returned_utf_codepoint():utf8_codepoint>>
  let assert Marker(6) = returned_custom()
  let assert True = returned_bool()
  let assert Nil = returned_nil()
  let assert #(7, False) = returned_tuple()
  let assert [] = returned_parameter_list()
  let assert [] = returned_parameter_list_list()
  let assert [1] = returned_int_list()
  let assert ["text"] = returned_string_list()
  let assert [<<1>>] = returned_bit_array_list()
  let assert [codepoint] = returned_utf_codepoint_list()
  let assert <<65>> = <<codepoint:utf8_codepoint>>
  let assert [Marker(6)] = returned_custom_list()
  let assert [1.5] = returned_float_list()
  let assert [True] = returned_bool_list()
  let assert [Nil] = returned_nil_list()
  let assert [#(7, False)] = returned_tuple_list()
  let assert [[8, 9]] = returned_list_list()
  let assert [returned_int_function] = returned_function_list()
  let assert 1 = returned_int_function()
  let assert 1 = returned_function()()

  #(returned_generic, returned_never)
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
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host source should compile");
    let plan = plan_host_program(typed).expect("host source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
    let value = execution
        .run_main(&mut (), &mut Vec::new())
        .expect("every returned function family should execute");

    assert_eq!(
        value.inspect().to_string(),
        "#(//fn(a) { ... }, //fn() { ... })",
    );
}

#[test]
fn preserves_symbolic_host_function_references_without_runtime_callbacks() {
    type ValueType = HostTypeParameter<0>;

    fn ignore<'call>(
        call: HostCall<'call, StatelessHostProfile, Identity, BigInt>,
        _value: HostValue<'call, ValueType>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        Ok(call.return_value(0.into()))
    }

    fn identity<'call>(
        call: HostCall<'call, StatelessHostProfile, Identity, ValueType>,
        value: HostValue<'call, ValueType>,
    ) -> Result<HostCallCompletion<'call, ValueType>, HostCallError> {
        Ok(call.return_value(value))
    }

    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Identity, (ValueType,), BigInt, _>("ignore", ignore)
        .expect("symbolic provider should be valid")
        .with_scoped_function::<Identity, (ValueType,), ValueType, _>("identity", identity)
        .expect("generic identity provider should be valid");
    let source = r#"
@external(erlang, "host", "ignore")
fn ignore(value: value) -> Int

@external(erlang, "host", "identity")
fn identity(value: value) -> value

pub fn main() {
  #(ignore, identity)
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
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host source should compile");
    let plan = plan_host_program(typed).expect("host source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
    let value = execution
        .run_main(&mut (), &mut Vec::new())
        .expect("symbolic host references should materialize");

    assert_eq!(
        value.inspect().to_string(),
        "#(//fn(a) { ... }, //fn(a) { ... })",
    );
}

#[test]
fn erases_a_host_callback_whose_argument_is_uninhabited() {
    type Never = HostCustomType<NeverSchema>;

    fn accept<'call>(
        call: HostCall<'call, StatelessHostProfile, Identity, BigInt>,
        _value: HostCustom<'call, Never>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        Ok(call.return_value(1.into()))
    }

    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Identity, (Never,), BigInt, _>("accept", accept)
        .expect("uninhabited provider should be valid");
    let source = r#"
pub type Never

@external(erlang, "host", "accept")
fn accept(value: Never) -> Int

pub fn main() {
  accept
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
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host source should compile");
    let plan = plan_host_program(typed).expect("host source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("uninhabited host callback should be erased");
    let value = execution
        .run_main(&mut (), &mut Vec::new())
        .expect("uninhabited function reference should materialize");

    assert_eq!(value.inspect().to_string(), "//fn(a) { ... }");
}

#[test]
fn passes_unresolved_parameter_lists_without_fabricating_item_values() {
    type Item = HostTypeParameter<0>;
    type Values = HostListType<Item>;
    type Nested = HostListType<Values>;

    fn empty<'call>(
        call: HostCall<'call, StatelessHostProfile, Identity, Values>,
    ) -> Result<HostCallCompletion<'call, Values>, HostCallError> {
        Ok(call.return_list([]))
    }

    fn nested_empty<'call>(
        call: HostCall<'call, StatelessHostProfile, Identity, Nested>,
    ) -> Result<HostCallCompletion<'call, Nested>, HostCallError> {
        Ok(call.return_list([]))
    }

    fn count<'call>(
        mut call: HostCall<'call, StatelessHostProfile, Identity, BigInt>,
        values: HostList<'call, Item>,
        nested: HostList<'call, Values>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        if call.list_item(values, 0).is_some() {
            return Err(
                HostFailure::new("an unresolved empty list must not fabricate an item").into(),
            );
        }
        let inner = call
            .list_item(nested, 0)
            .ok_or_else(|| HostFailure::new("nested list should contain one value"))?;
        let count = call.list_len(values) + call.list_len(inner) + 1;
        Ok(call.return_value(count.into()))
    }

    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Identity, (), Values, _>("empty", empty)
        .expect("parameter-list provider should be valid")
        .with_scoped_function::<Identity, (), Nested, _>("nested_empty", nested_empty)
        .expect("nested parameter-list provider should be valid")
        .with_scoped_function::<Identity, (Values, Nested), BigInt, _>("count", count)
        .expect("parameter-list argument provider should be valid");
    let source = r#"
@external(erlang, "host", "empty")
fn empty() -> List(value)

@external(erlang, "host", "nested_empty")
fn nested_empty() -> List(List(value))

@external(erlang, "host", "count")
fn count(values: List(value), nested: List(List(value))) -> Int

pub fn main() {
  #(empty(), nested_empty(), count([], [[]]))
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
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host source should compile");
    let plan = plan_host_program(typed).expect("host source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
    let value = execution
        .run_main(&mut (), &mut Vec::new())
        .expect("unresolved empty lists should remain representable");

    assert_eq!(value.inspect().to_string(), "#([], [], 1)");
}

#[test]
fn embeds_parameter_and_stored_lists_in_typed_tuple_returns() {
    type Item = HostTypeParameter<0>;
    type Values = HostListType<Item>;
    type Return = HostTupleType<HostTypeList<Values, HostTypeListEnd>>;

    fn wrap<'call>(
        call: HostCall<'call, StatelessHostProfile, Identity, Return>,
        values: HostList<'call, Item>,
    ) -> Result<HostCallCompletion<'call, Return>, HostCallError> {
        Ok(call.return_tuple((values, ())))
    }

    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Identity, (Values,), Return, _>("wrap", wrap)
        .expect("list tuple provider should be valid");
    let source = r#"
@external(erlang, "host", "wrap")
fn wrap(values: List(value)) -> #(List(value))

pub fn main() {
  #(wrap([]), wrap([1]))
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
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host source should compile");
    let plan = plan_host_program(typed).expect("host source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");

    assert_eq!(
        execution
            .run_main(&mut (), &mut Vec::new())
            .expect("list tuple provider should run")
            .inspect()
            .to_string(),
        "#(#([]), #([1]))",
    );
}

#[test]
fn constructs_nested_lists_from_the_specialized_generic_item_shape() {
    type Item = HostTypeParameter<0>;
    type Return = HostListType<Item>;

    fn wrap<'call>(
        call: HostCall<'call, StatelessHostProfile, Identity, Return>,
        prefix: HostList<'call, BigInt>,
        value: HostValue<'call, Item>,
    ) -> Result<HostCallCompletion<'call, Return>, HostCallError> {
        if call.list_len(prefix) != 1 {
            return Err(HostFailure::new("prefix should contain one value").into());
        }
        if !call.equal::<HostListType<BigInt>>(prefix, prefix) {
            return Err(HostFailure::new("list should equal itself").into());
        }
        Ok(call.return_list([value]))
    }

    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Identity, (HostListType<BigInt>, Item), Return, _>("wrap", wrap)
        .expect("generic provider should be valid");
    let source = r#"
@external(erlang, "host", "wrap")
fn wrap(prefix: List(Int), value: value) -> List(value)

pub fn main() {
  #(wrap([99], 42), wrap([99], [1, 2]))
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
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host source should compile");
    let plan = plan_host_program(typed).expect("host source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
    let nested = ListValue::try_list(
        geam::ValueType::Int,
        vec![ListValue::int(vec![1.into(), 2.into()])],
    )
    .expect("nested list item type should match");

    let expected = Value::Tuple(vec![
        Value::List(ListValue::int(vec![42.into()])),
        Value::List(nested),
    ]);

    assert_eq!(
        execution.run_main(&mut (), &mut Vec::new()),
        Ok(expected.clone()),
    );
    assert_eq!(execution.run_main(&mut (), &mut Vec::new()), Ok(expected),);
}

#[test]
fn returns_existing_list_handles_without_rebuilding_their_items() {
    type Ints = HostListType<BigInt>;

    fn identity<'call>(
        call: HostCall<'call, StatelessHostProfile, Identity, Ints>,
        values: HostList<'call, BigInt>,
    ) -> Result<HostCallCompletion<'call, Ints>, HostCallError> {
        Ok(call.return_value(values))
    }

    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Identity, (Ints,), Ints, _>("identity", identity)
        .expect("list identity provider should be valid");
    let source = r#"
@external(erlang, "host", "identity")
fn identity(values: List(Int)) -> List(Int)

pub fn main() {
  identity([1, 2, 3])
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
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host source should compile");
    let plan = plan_host_program(typed).expect("host source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");

    assert_eq!(
        execution.run_main(&mut (), &mut Vec::new()),
        Ok(Value::List(ListValue::int(vec![
            1.into(),
            2.into(),
            3.into()
        ]))),
    );
}

#[test]
fn constructs_lists_for_every_concrete_runtime_item_family() {
    type Item = HostTypeParameter<0>;
    type Return = HostListType<Item>;

    fn wrap<'call>(
        call: HostCall<'call, StatelessHostProfile, Identity, Return>,
        value: HostValue<'call, Item>,
    ) -> Result<HostCallCompletion<'call, Return>, HostCallError> {
        Ok(call.return_list([value]))
    }

    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Identity, (Item,), Return, _>("wrap", wrap)
        .expect("generic list provider should be valid");
    let source = r#"
pub type Marker {
  Marker(Int)
}

@external(erlang, "host", "wrap")
fn wrap(value: value) -> List(value)

fn increment(value: Int) -> Int {
  value + 1
}

pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  let assert [1] = wrap(1)
  let assert [1.5] = wrap(1.5)
  let assert ["text"] = wrap("text")
  let assert [<<1>>] = wrap(<<1>>)
  let assert [returned_codepoint] = wrap(codepoint)
  let assert [Marker(6)] = wrap(Marker(6))
  let assert [True] = wrap(True)
  let assert [Nil] = wrap(Nil)
  let assert [#(7, False)] = wrap(#(7, False))
  let assert [[8, 9]] = wrap([8, 9])
  let assert [returned_function] = wrap(increment)
  assert returned_codepoint == codepoint
  returned_function(41)
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
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host source should compile");
    let plan = plan_host_program(typed).expect("host source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");

    assert_eq!(
        execution.run_main(&mut (), &mut Vec::new()),
        Ok(Value::Int(42.into())),
    );
}

#[test]
fn reports_generic_list_host_failure_at_the_source_call_site() {
    type Item = HostTypeParameter<0>;
    type Values = HostListType<Item>;

    fn first<'call>(
        mut call: HostCall<'call, StatelessHostProfile, Identity, Item>,
        values: HostList<'call, Item>,
    ) -> Result<HostCallCompletion<'call, Item>, HostCallError> {
        let value = call
            .list_item(values, 0)
            .ok_or_else(|| HostFailure::new("list is empty"))?;
        Ok(call.return_value(value))
    }

    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Identity, (Values,), Item, _>("first", first)
        .expect("generic provider should be valid");
    let source = r#"
@external(erlang, "host", "first")
fn first(values: List(value)) -> value

fn from_empty() -> Int {
  first([])
}

pub fn main() {
  from_empty()
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
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host source should compile");
    let plan = plan_host_program(typed).expect("host source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
    let error = execution
        .run_main(&mut (), &mut Vec::new())
        .expect_err("empty generic list should fail");
    let geam::ExecutionError::Host(error) = error else {
        panic!("generic list provider should produce a host error");
    };

    assert_eq!(error.package(), "application");
    assert_eq!(error.module(), "main");
    assert_eq!(error.function(), "first");
    assert_eq!(error.failure().message(), "list is empty");
    assert_eq!(
        error.signature().argument_types(),
        [geam::ValueType::List(Box::new(geam::ValueType::Int))],
    );
    assert_eq!(error.signature().return_(), &geam::ValueType::Int);
    let HostLocation::Resolved { site, path, line } = error.location() else {
        panic!("source-backed compound failure should resolve its call site");
    };
    assert_eq!(site.module(), "main");
    assert_eq!(site.function(), "from_empty");
    assert_eq!(path.as_str(), "src/main.gleam");
    assert_eq!(*line, 6);
}

#[test]
fn rejects_a_reachable_generic_list_return_without_a_concrete_item_family() {
    type Item = HostTypeParameter<0>;
    type Values = HostListType<Item>;

    fn first<'call>(
        mut call: HostCall<'call, StatelessHostProfile, Identity, Item>,
        values: HostList<'call, Item>,
    ) -> Result<HostCallCompletion<'call, Item>, HostCallError> {
        let value = call
            .list_item(values, 0)
            .ok_or_else(|| HostFailure::new("list is empty"))?;
        Ok(call.return_value(value))
    }

    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Identity, (Values,), Item, _>("first", first)
        .expect("generic provider should be valid");
    let source = r#"
@external(erlang, "host", "first")
fn first(values: List(value)) -> value

pub fn main() {
  first([])
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
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host source should compile");
    let plan = plan_host_program(typed).expect("host source should plan");
    let Err(error) = HostedExecution::try_from_module_plan(plan) else {
        panic!("unresolved reachable host return should not seal");
    };

    assert_eq!(error.package(), "application");
    assert_eq!(error.module(), "main");
    assert_eq!(error.function(), "first");
    let [argument] = error.signature().argument_types() else {
        panic!("first should have one argument");
    };
    let geam::ValueType::List(item) = argument else {
        panic!("first should accept a list");
    };
    let geam::ValueType::Parameter(argument) = item.as_ref() else {
        panic!("the list item should remain generic");
    };
    let geam::ValueType::Parameter(return_) = error.signature().return_() else {
        panic!("the return should remain generic");
    };
    assert_eq!(argument, return_);
}

#[test]
fn rejects_a_reachable_value_producer_without_a_concrete_return_family() {
    type Item = HostTypeParameter<0>;

    fn produce<'call>(
        _call: HostCall<'call, StatelessHostProfile, Identity, Item>,
    ) -> Result<HostCallCompletion<'call, Item>, HostCallError> {
        Err(HostFailure::new("produce should not run").into())
    }

    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Identity, (), Item, _>("produce", produce)
        .expect("generic provider should be valid");
    let source = r#"
@external(erlang, "host", "produce")
fn produce() -> value

pub fn main() {
  produce()
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
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host source should compile");
    let plan = plan_host_program(typed).expect("host source should plan");
    let Err(error) = HostedExecution::try_from_module_plan(plan) else {
        panic!("unresolved reachable host return should not seal");
    };

    assert_eq!(error.package(), "application");
    assert_eq!(error.module(), "main");
    assert_eq!(error.function(), "produce");
    assert!(error.signature().argument_types().is_empty());
    assert!(matches!(
        error.signature().return_(),
        geam::ValueType::Parameter(_)
    ));
}

#[test]
fn reports_the_first_unrepresentable_specialization_in_source_evaluation_order() {
    type Item = HostTypeParameter<0>;

    fn first<'call>(
        _call: HostCall<'call, StatelessHostProfile, Identity, Item>,
    ) -> Result<HostCallCompletion<'call, Item>, HostCallError> {
        Err(HostFailure::new("first should not run").into())
    }

    fn second<'call>(
        _call: HostCall<'call, StatelessHostProfile, Identity, Item>,
    ) -> Result<HostCallCompletion<'call, Item>, HostCallError> {
        Err(HostFailure::new("second should not run").into())
    }

    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Identity, (), Item, _>("first", first)
        .expect("first generic provider should be valid")
        .with_scoped_function::<Identity, (), Item, _>("second", second)
        .expect("second generic provider should be valid");
    let source = r#"
@external(erlang, "host", "first")
fn first() -> first

@external(erlang, "host", "second")
fn second() -> second

pub fn main() {
  first()
  second()
  1
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
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host source should compile");
    let plan = plan_host_program(typed).expect("host source should plan");
    let Err(error) = HostedExecution::try_from_module_plan(plan) else {
        panic!("the first unresolved reachable host return should not seal");
    };

    assert_eq!(error.package(), "application");
    assert_eq!(error.module(), "main");
    assert_eq!(error.function(), "first");
    assert!(error.signature().argument_types().is_empty());
    assert!(matches!(
        error.signature().return_(),
        geam::ValueType::Parameter(_)
    ));
}

#[test]
fn leaves_an_unused_unrepresentable_provider_outside_execution_sealing() {
    type Item = HostTypeParameter<0>;

    fn produce<'call>(
        _call: HostCall<'call, StatelessHostProfile, Identity, Item>,
    ) -> Result<HostCallCompletion<'call, Item>, HostCallError> {
        Err(HostFailure::new("unused provider should not run").into())
    }

    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Identity, (), Item, _>("produce", produce)
        .expect("generic provider should be valid");
    let source = r#"
@external(erlang, "host", "produce")
fn produce() -> value

pub fn main() {
  42
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
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host source should compile");
    let plan = plan_host_program(typed).expect("host source should plan");
    let execution = HostedExecution::try_from_module_plan(plan)
        .expect("unused provider should not block sealing");

    assert_eq!(
        execution.run_main(&mut (), &mut Vec::new()),
        Ok(Value::Int(42.into())),
    );
}

#[test]
fn applies_specialization_sealing_to_dependency_package_providers() {
    type Item = HostTypeParameter<0>;
    type Values = HostListType<Item>;

    fn first<'call>(
        mut call: HostCall<'call, StatelessHostProfile, Identity, Item>,
        values: HostList<'call, Item>,
    ) -> Result<HostCallCompletion<'call, Item>, HostCallError> {
        let value = call
            .list_item(values, 0)
            .ok_or_else(|| HostFailure::new("list is empty"))?;
        Ok(call.return_value(value))
    }

    let provider = HostProviderModule::<StatelessHostProfile>::new("host_support", "host/generic")
        .expect("provider module should be valid")
        .with_scoped_function::<Identity, (Values,), Item, _>("first", first)
        .expect("generic provider should be valid");
    let dependency_source = r#"
@external(erlang, "host", "first")
pub fn first(values: List(value)) -> value
"#;
    let main_source = r#"
import host/generic

pub fn main() {
  generic.first([])
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [
            PackageSource::new(
                "host_support",
                Vec::<&str>::new(),
                [ModuleSource::new(
                    "host/generic",
                    "host_support/src/host/generic.gleam",
                    dependency_source,
                )],
            ),
            PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new("main", "src/main.gleam", main_source)],
            ),
        ],
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host source should compile");
    let plan = plan_host_program(typed).expect("host source should plan");
    let Err(error) = HostedExecution::try_from_module_plan(plan) else {
        panic!("dependency provider should use the same sealing boundary");
    };

    assert_eq!(error.package(), "host_support");
    assert_eq!(error.module(), "host/generic");
    assert_eq!(error.function(), "first");
    assert!(matches!(
        error.signature().return_(),
        geam::ValueType::Parameter(_)
    ));
}

#[test]
fn reads_generic_list_items_through_call_scoped_handles() {
    type Item = HostTypeParameter<0>;
    type Values = HostListType<Item>;

    fn first<'call>(
        mut call: HostCall<'call, StatelessHostProfile, Identity, Item>,
        values: HostList<'call, Item>,
    ) -> Result<HostCallCompletion<'call, Item>, HostCallError> {
        let value = call
            .list_item(values, 0)
            .ok_or_else(|| HostFailure::new("list is empty"))?;
        Ok(call.return_value(value))
    }

    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Identity, (Values,), Item, _>("first", first)
        .expect("generic provider should be valid");
    let source = r#"
pub type Marker {
  Marker(Int)
}

@external(erlang, "host", "first")
fn first(values: List(value)) -> value

fn increment(value: Int) -> Int {
  value + 1
}

pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  let assert 42 = first([42])
  let assert 1.5 = first([1.5])
  let assert "text" = first(["text"])
  let assert <<1>> = first([<<1>>])
  assert first([codepoint]) == codepoint
  let assert Marker(6) = first([Marker(6)])
  let assert True = first([True])
  let assert Nil = first([Nil])
  let assert #(7, False) = first([#(7, False)])
  let assert [] = first([[]])
  let assert [1, 2] = first([[1, 2]])
  let returned = first([increment])
  returned(41)
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
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host source should compile");
    let plan = plan_host_program(typed).expect("host source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");

    assert_eq!(
        execution.run_main(&mut (), &mut Vec::new()),
        Ok(Value::Int(42.into())),
    );
}

#[test]
fn compiles_source_less_list_and_tuple_host_interfaces_from_recursive_schema() {
    type PairElements = HostTypeList<BigInt, HostTypeList<bool, HostTypeListEnd>>;
    type Pair = HostTupleType<PairElements>;

    fn count<'call>(
        call: HostCall<'call, StatelessHostProfile, Identity, BigInt>,
        values: HostList<'call, BigInt>,
        pair: HostTuple<'call, PairElements>,
    ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
        let count = call.list_len(values) + call.tuple_len(pair);
        Ok(call.return_value(count.into()))
    }

    let host = HostModule::new("host_support", "host/compound")
        .expect("host module should be valid")
        .with_scoped_function::<Identity, (HostListType<BigInt>, Pair), BigInt, _>("count", count)
        .expect("compound host function should be valid");
    let source = r#"
import host/compound

pub fn main() {
  compound.count([1, 2, 3], #(4, True))
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            ["host_support"],
            [ModuleSource::new("main", "src/main.gleam", source)],
        )],
        HostProviderSet::new([host]).expect("host module should be unique"),
    )
    .expect("source-less compound interface should compile");
    let plan = plan_host_program(typed).expect("host source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("source-less compound host should seal");

    assert_eq!(
        execution.run_main(&mut (), &mut Vec::new()),
        Ok(Value::Int(5.into())),
    );
}

#[test]
fn reads_and_constructs_tuples_beyond_the_function_arity_limit() {
    type Eighth = HostTypeList<BigInt, HostTypeListEnd>;
    type Seventh = HostTypeList<BigInt, Eighth>;
    type Sixth = HostTypeList<BigInt, Seventh>;
    type Fifth = HostTypeList<BigInt, Sixth>;
    type Fourth = HostTypeList<BigInt, Fifth>;
    type Third = HostTypeList<BigInt, Fourth>;
    type Second = HostTypeList<BigInt, Third>;
    type Elements = HostTypeList<BigInt, Second>;
    type Tuple = HostTupleType<Elements>;

    fn reverse<'call>(
        mut call: HostCall<'call, StatelessHostProfile, Identity, Tuple>,
        value: HostTuple<'call, Elements>,
    ) -> Result<HostCallCompletion<'call, Tuple>, HostCallError> {
        if call.tuple_len(value) != 8 {
            return Err(HostFailure::new("tuple should contain eight values").into());
        }
        let (first, (second, (third, (fourth, (fifth, (sixth, (seventh, (eighth, ())))))))) =
            call.tuple_values(value);
        Ok(call.return_tuple((
            eighth,
            (
                seventh,
                (sixth, (fifth, (fourth, (third, (second, (first, ())))))),
            ),
        )))
    }

    fn identity<'call>(
        call: HostCall<'call, StatelessHostProfile, Identity, Tuple>,
        value: HostTuple<'call, Elements>,
    ) -> Result<HostCallCompletion<'call, Tuple>, HostCallError> {
        Ok(call.return_value(value))
    }

    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Identity, (Tuple,), Tuple, _>("reverse", reverse)
        .expect("tuple provider should be valid")
        .with_scoped_function::<Identity, (Tuple,), Tuple, _>("identity", identity)
        .expect("tuple identity provider should be valid");
    let source = r#"
@external(erlang, "host", "reverse")
fn reverse(value: #(Int, Int, Int, Int, Int, Int, Int, Int)) ->
  #(Int, Int, Int, Int, Int, Int, Int, Int)

@external(erlang, "host", "identity")
fn identity(value: #(Int, Int, Int, Int, Int, Int, Int, Int)) ->
  #(Int, Int, Int, Int, Int, Int, Int, Int)

pub fn main() {
  let value = #(1, 2, 3, 4, 5, 6, 7, 8)
  #(reverse(value), identity(value))
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
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host source should compile");
    let plan = plan_host_program(typed).expect("host source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");

    assert_eq!(
        execution.run_main(&mut (), &mut Vec::new()),
        Ok(Value::Tuple(vec![
            Value::Tuple(
                [8, 7, 6, 5, 4, 3, 2, 1]
                    .into_iter()
                    .map(|value| Value::Int(value.into()))
                    .collect(),
            ),
            Value::Tuple(
                [1, 2, 3, 4, 5, 6, 7, 8]
                    .into_iter()
                    .map(|value| Value::Int(value.into()))
                    .collect(),
            ),
        ])),
    );
}

#[test]
fn reads_every_scalar_from_a_tuple_list_item_and_preserves_typed_equality() {
    type Seventh = HostTypeList<(), HostTypeListEnd>;
    type Sixth = HostTypeList<bool, Seventh>;
    type Fifth = HostTypeList<char, Sixth>;
    type Fourth = HostTypeList<BitArrayValue, Fifth>;
    type Third = HostTypeList<EcoString, Fourth>;
    type Second = HostTypeList<f64, Third>;
    type Elements = HostTypeList<BigInt, Second>;
    type Tuple = HostTupleType<Elements>;
    type Values = HostListType<Tuple>;

    fn inspect<'call>(
        mut call: HostCall<'call, StatelessHostProfile, Identity, bool>,
        values: HostList<'call, Tuple>,
    ) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
        let value = call
            .list_item(values, 0)
            .ok_or_else(|| HostFailure::new("tuple list should contain one value"))?;
        if call.tuple_len(value) != 7 || !call.equal::<Tuple>(value, value) {
            return Err(HostFailure::new("tuple shape should be preserved").into());
        }
        let (int, (float, (string, (bits, (codepoint, (bool_, (nil, ()))))))) =
            call.tuple_values(value);
        let matches = call.equal::<BigInt>(int, 1.into())
            && call.equal::<f64>(float, 1.5)
            && call.equal::<EcoString>(string, "text".into())
            && call.equal::<BitArrayValue>(bits, BitArrayValue::from_bytes(vec![1]))
            && call.equal::<char>(codepoint, 'A')
            && call.equal::<bool>(bool_, true)
            && call.equal::<()>(nil, ());
        Ok(call.return_value(matches))
    }

    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Identity, (Values,), bool, _>("inspect", inspect)
        .expect("tuple-list provider should be valid");
    let source = r#"
@external(erlang, "host", "inspect")
fn inspect(
  values: List(#(Int, Float, String, BitArray, UtfCodepoint, Bool, Nil)),
) -> Bool

pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  inspect([#(1, 1.5, "text", <<1>>, codepoint, True, Nil)])
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
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host source should compile");
    let plan = plan_host_program(typed).expect("host source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");

    assert_eq!(
        execution.run_main(&mut (), &mut Vec::new()),
        Ok(Value::Bool(true)),
    );
}

#[test]
fn reads_and_constructs_generic_custom_values_from_the_declared_schema() {
    fn toggle<'call>(
        mut call: HostCall<'call, StatelessHostProfile, Identity, Boxed>,
        value: HostCustom<'call, Boxed>,
    ) -> Result<HostCallCompletion<'call, Boxed>, HostCallError> {
        let constructor = call.custom_constructor(value);
        if !call.equal::<Boxed>(value, value) {
            return Err(HostFailure::new("custom value should equal itself").into());
        }
        match call.custom_fields::<BoxedValue>(value) {
            Some((item, (enabled, ()))) => {
                if constructor != 1 {
                    return Err(HostFailure::new("boxed constructor should have index one").into());
                }
                Ok(call.return_custom::<BoxedValue>((item, (!enabled, ()))))
            }
            None => {
                if constructor != 0 {
                    return Err(HostFailure::new("empty constructor should have index zero").into());
                }
                Ok(call.return_custom::<Empty>(()))
            }
        }
    }

    fn first<'call>(
        mut call: HostCall<'call, StatelessHostProfile, Identity, Boxed>,
        values: HostList<'call, Boxed>,
    ) -> Result<HostCallCompletion<'call, Boxed>, HostCallError> {
        let value = call
            .list_item(values, 0)
            .ok_or_else(|| HostFailure::new("custom list should contain one value"))?;
        Ok(call.return_value(value))
    }

    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Identity, (Boxed,), Boxed, _>("toggle", toggle)
        .expect("custom provider should be valid")
        .with_scoped_function::<Identity, (HostListType<Boxed>,), Boxed, _>("first", first)
        .expect("custom-list provider should be valid");
    let source = r#"
pub type Boxed(value) {
  Empty
  Boxed(value: value, enabled: Bool)
}

@external(erlang, "host", "toggle")
fn toggle(value: Boxed(value)) -> Boxed(value)

@external(erlang, "host", "first")
fn first(values: List(Boxed(value))) -> Boxed(value)

pub fn main() {
  #(
    toggle(Empty),
    toggle(Boxed(value: 42, enabled: True)),
    first([Boxed(value: 9, enabled: False)]),
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
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host source should compile");
    let plan = plan_host_program(typed).expect("host source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
    let value = execution
        .run_main(&mut (), &mut Vec::new())
        .expect("custom host calls should run");

    assert_eq!(
        value.inspect().to_string(),
        "#(Empty, Boxed(value: 42, enabled: False), Boxed(value: 9, enabled: False))",
    );
}

#[test]
fn compares_generic_scalar_list_and_custom_values_with_gleam_equality() {
    type Item = HostTypeParameter<0>;

    fn same<'call>(
        call: HostCall<'call, StatelessHostProfile, Identity, bool>,
        left: HostValue<'call, Item>,
        right: HostValue<'call, Item>,
    ) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
        let equal = call.equal::<Item>(left, right);
        Ok(call.return_value(equal))
    }

    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .expect("provider module should be valid")
        .with_scoped_function::<Identity, (Item, Item), bool, _>("same", same)
        .expect("generic provider should be valid");
    let source = r#"
pub type Boxed(value) {
  Empty
  Boxed(value: value, enabled: Bool)
}

@external(erlang, "host", "same")
fn same(left: value, right: value) -> Bool

fn increment(value: Int) {
  value + 1
}

pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<65>>
  #(
    same(1, 1),
    same(1.5, 1.5),
    same("one", "one"),
    same(<<1>>, <<1>>),
    same(codepoint, codepoint),
    same(True, False),
    same(Nil, Nil),
    same(#(1, True), #(1, True)),
    same([1, 2], [1, 3]),
    same(
      Boxed(value: "one", enabled: True),
      Boxed(value: "one", enabled: True),
    ),
    same(increment, increment),
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
        HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
            .expect("provider module should be unique"),
    )
    .expect("host source should compile");
    let plan = plan_host_program(typed).expect("host source should plan");
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");

    assert_eq!(
        execution.run_main(&mut (), &mut Vec::new()),
        Ok(Value::Tuple(vec![
            Value::Bool(true),
            Value::Bool(true),
            Value::Bool(true),
            Value::Bool(true),
            Value::Bool(true),
            Value::Bool(false),
            Value::Bool(true),
            Value::Bool(true),
            Value::Bool(false),
            Value::Bool(true),
            Value::Bool(true),
        ])),
    );
}
