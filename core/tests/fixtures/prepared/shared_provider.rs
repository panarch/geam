use geam_core::embedding::{BigInt, FunctionDeclaration, HostPreparation, PreparedHostedModule};
use geam_core::provider::{ProviderValueContext, Value};
use geam_core::{
    HostCall, HostCallCompletion, HostCallError, HostCustom, HostCustomConstructorAt,
    HostCustomConstructorDefinition, HostCustomConstructorList, HostCustomConstructorListEnd,
    HostCustomField, HostCustomFieldList, HostCustomFieldListEnd, HostCustomIndex0,
    HostCustomSchema, HostCustomType, HostCustomTypeArgument, HostProvider, HostProviderModule,
    HostProviderSet, HostType, HostTypeIndex0, HostTypeList, HostTypeListEnd, HostTypeParameter,
    ModuleSource, PackageSource, StatelessHostProfile,
};

struct Provider;
impl HostProvider<StatelessHostProfile> for Provider {
    type State = ();
    fn project(state: &mut ()) -> &mut () {
        state
    }
}

mod producer {
    use super::{
        BigInt, HostCall, HostCallCompletion, HostCallError, HostCustom, HostCustomConstructorAt,
        HostCustomConstructorDefinition, HostCustomConstructorList, HostCustomConstructorListEnd,
        HostCustomField, HostCustomFieldList, HostCustomFieldListEnd, HostCustomIndex0,
        HostCustomSchema, HostCustomType, HostCustomTypeArgument, HostType, HostTypeIndex0,
        HostTypeList, HostTypeListEnd, Provider, ProviderValueContext, StatelessHostProfile, Value,
    };

    pub struct Schema;
    pub struct Constructor;
    pub struct Field;
    impl HostCustomSchema for Schema {
        const PACKAGE: &'static str = "producer";
        const MODULE: &'static str = "handles";
        const NAME: &'static str = "Handle";
        const PARAMETER_COUNT: usize = 1;
        const SHARED: bool = true;
        type Constructors = HostCustomConstructorList<Constructor, HostCustomConstructorListEnd>;
    }
    impl HostCustomConstructorDefinition for Constructor {
        const NAME: &'static str = "Handle";
        type Fields = HostCustomFieldList<Field, HostCustomFieldListEnd>;
    }
    impl HostCustomField for Field {
        const LABEL: Option<&'static str> = None;
        type Type = HostCustomTypeArgument<HostTypeIndex0>;
    }
    pub type Host<T> = HostCustomType<Schema, HostTypeList<T, HostTypeListEnd>>;
    type IntConstructor = HostCustomConstructorAt<Host<BigInt>, HostCustomIndex0, Constructor>;

    pub struct Handle<T: HostType> {
        value: Value<Self, ProviderValueContext<Host<T>>>,
    }

    impl<T: HostType> Handle<T> {
        pub fn retain<'call>(
            call: &HostCall<'call, StatelessHostProfile, Provider, Host<T>>,
            value: HostCustom<'call, Host<T>>,
        ) -> Self {
            Self {
                value: Value::from_host(call, value),
            }
        }
        pub fn restore<'call>(
            self,
            call: &mut HostCall<'call, StatelessHostProfile, Provider, Host<T>>,
        ) -> HostCustom<'call, Host<T>> {
            self.value.into_host(call)
        }
    }

    pub fn increment<'call>(
        mut call: HostCall<'call, StatelessHostProfile, Provider, Host<BigInt>>,
        value: HostCustom<'call, Host<BigInt>>,
    ) -> Result<HostCallCompletion<'call, Host<BigInt>>, HostCallError> {
        let (value, ()) = call.custom_fields::<IntConstructor>(value).unwrap();
        Ok(call.return_custom::<IntConstructor>((value + BigInt::from(1), ())))
    }
}

fn retain<'call, T: HostType>(
    mut call: HostCall<'call, StatelessHostProfile, Provider, producer::Host<T>>,
    value: HostCustom<'call, producer::Host<T>>,
) -> Result<HostCallCompletion<'call, producer::Host<T>>, HostCallError> {
    let handle = producer::Handle::<T>::retain(&call, value);
    let restored = handle.restore(&mut call);
    assert!(call.equal::<producer::Host<T>>(value, restored));
    Ok(call.return_value(restored))
}

pub fn hosts(shared: bool) -> HostProviderSet {
    let producer = HostProviderModule::new("producer", "handles").unwrap();
    let producer = if shared {
        producer
            .with_shared_custom_type::<producer::Schema>()
            .unwrap()
    } else {
        producer
    };
    HostProviderSet::from_providers([producer,
        HostProviderModule::new("consumer", "consumer").unwrap()
            .with_scoped_function::<Provider, (producer::Host<HostTypeParameter<0>>,), producer::Host<HostTypeParameter<0>>, _>("retain", retain::<HostTypeParameter<0>>).unwrap()
            .with_scoped_function::<Provider, (producer::Host<BigInt>,), producer::Host<BigInt>, _>("increment", producer::increment).unwrap(),
    ]).unwrap()
}

pub fn packages() -> Vec<PackageSource> {
    vec![
        PackageSource::new(
            "producer",
            Vec::<&str>::new(),
            [ModuleSource::new(
                "handles",
                "handles.gleam",
                r#"
pub opaque type Handle(a) { Handle(a) }
pub fn make(value: a) { Handle(value) }
pub fn get(value: Handle(a)) { case value { Handle(item) -> item } }
"#,
            )],
        ),
        PackageSource::new(
            "consumer",
            ["producer"],
            [ModuleSource::new(
                "consumer",
                "consumer.gleam",
                r#"
import handles
@external(erlang, "native", "retain")
fn retain(value: handles.Handle(a)) -> handles.Handle(a)
@external(erlang, "native", "increment")
fn increment(value: handles.Handle(Int)) -> handles.Handle(Int)
pub fn main() {
  let integer = handles.make(41) |> retain |> increment |> handles.get
  let callback = handles.make(fn(value) { value + 2 }) |> retain |> handles.get
  #(integer, callback(40))
}
"#,
            )],
        ),
    ]
}

pub fn prepare() -> PreparedHostedModule {
    let typed = geam_core::compile_declared_host_program(
        "consumer",
        "consumer",
        packages(),
        hosts(true).into_declarations(),
    )
    .unwrap();
    HostPreparation::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(), (BigInt, BigInt)>::new("main"))
        .unwrap()
        .prepare()
        .unwrap()
}
