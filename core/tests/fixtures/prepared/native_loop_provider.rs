use geam_core::__macro_support::{HostRetainedValue, HostTypeParameter};
use geam_core::embedding::{
    BigInt, BitArrayValue, FunctionDeclaration, HostPreparation, List, PreparedHostedModule,
    StringValue,
};
use geam_core::execution::ExecutionUnit;
use geam_core::{
    HostCall, HostCallCompletion, HostCallError, HostFailure, HostProvider, HostProviderModule,
    HostProviderSet, ModuleSource, PackageSource, StatelessHostProfile,
};
use std::sync::{
    Mutex,
    atomic::{AtomicUsize, Ordering},
};

#[cfg(test)]
pub static TEST_LOCK: Mutex<()> = Mutex::new(());
pub static KEEP_CALLS: AtomicUsize = AtomicUsize::new(0);
pub static KEEP_RETAINED: AtomicUsize = AtomicUsize::new(0);

struct Provider;

impl HostProvider<StatelessHostProfile> for Provider {
    type State = ();

    fn project(state: &mut ()) -> &mut () {
        state
    }
}

#[derive(Default)]
pub struct Audit {
    pub inputs: Vec<BigInt>,
    pub retained: usize,
    pub fail_at: Option<usize>,
    pub panic_at: Option<usize>,
    pub cancel_at: Option<usize>,
    pub unit: Option<ExecutionUnit>,
}

pub static AUDIT: Mutex<Audit> = Mutex::new(Audit {
    inputs: Vec::new(),
    retained: 0,
    fail_at: None,
    panic_at: None,
    cancel_at: None,
    unit: None,
});

fn observe(value: BigInt, retained: bool) -> Result<BigInt, HostFailure> {
    let (index, fail, panic, cancel) = {
        let mut audit = AUDIT.lock().unwrap();
        audit.inputs.push(value.clone());
        audit.retained += usize::from(retained);
        let index = audit.inputs.len();
        (
            index,
            audit.fail_at == Some(index),
            audit.panic_at == Some(index),
            (audit.cancel_at == Some(index))
                .then(|| audit.unit.clone())
                .flatten(),
        )
    };
    if let Some(unit) = cancel {
        unit.cancel();
    }
    if fail {
        return Err(HostFailure::new("observed native failure"));
    }
    if panic {
        panic!("observed Rust panic");
    }
    Ok(value + index)
}

fn scoped_observe<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, BigInt>,
    value: BigInt,
) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
    let value = observe(value, false)?;
    Ok(call.return_value(value))
}

fn scoped_keep<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, HostTypeParameter<0>>,
    value: geam_core::HostValue<'call, HostTypeParameter<0>>,
) -> Result<HostCallCompletion<'call, HostTypeParameter<0>>, HostCallError> {
    KEEP_CALLS.fetch_add(1, Ordering::Relaxed);
    Ok(call.return_value(value))
}

fn scoped_begin<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, ()>,
) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
    AUDIT.lock().unwrap().unit = call.execution_unit();
    Ok(call.return_value(()))
}

pub fn hosts(retained: bool) -> HostProviderSet {
    let module = HostProviderModule::new("application", "native_loop").unwrap();
    let module = if retained {
        module.with_scoped_retained_function::<Provider, BigInt, BigInt, _, _>(
            "observe", scoped_observe, |value| observe(value, true).map_err(Into::into),
        ).unwrap()
        .with_scoped_retained_function::<Provider, HostTypeParameter<0>, HostTypeParameter<0>, _, _>(
            "keep", scoped_keep, |value: HostRetainedValue<HostTypeParameter<0>>| { KEEP_CALLS.fetch_add(1, Ordering::Relaxed); KEEP_RETAINED.fetch_add(1, Ordering::Relaxed); Ok(value) },
        ).unwrap()
    } else {
        module
            .with_scoped_function::<Provider, (BigInt,), BigInt, _>("observe", scoped_observe)
            .unwrap()
            .with_scoped_function::<Provider, (HostTypeParameter<0>,), HostTypeParameter<0>, _>(
                "keep",
                scoped_keep,
            )
            .unwrap()
    };
    let module = primitive_hosts(module, retained);
    HostProviderSet::from_providers([module
        .with_scoped_function::<Provider, (), (), _>("begin", scoped_begin)
        .unwrap()])
    .unwrap()
}

pub fn packages() -> Vec<PackageSource> {
    vec![PackageSource::new(
        "application",
        Vec::<&str>::new(),
        [ModuleSource::new(
            "native_loop",
            "src/native_loop.gleam",
            include_str!("native_loop.gleam"),
        )],
    )]
}

pub fn prepare() -> PreparedHostedModule {
    let typed = geam_core::compile_declared_host_program(
        "application",
        "native_loop",
        packages(),
        hosts(true).into_declarations(),
    )
    .unwrap();
    let mut bindings = HostPreparation::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
            "captured",
        ))
        .unwrap();
    for name in [
        "computed",
        "computed_cancellable",
        "ordinary_computed",
        "dynamic_computed",
        "dynamic_computed_cancellable",
        "cancellable",
        "retained_value",
        "graph_captured",
    ] {
        bindings
            .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(name))
            .unwrap();
    }
    bindings
        .function(FunctionDeclaration::<(List<BigInt>,), List<BigInt>>::new(
            "compound",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt, f64), f64>::new(
            "captured_float",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt, StringValue), StringValue>::new("captured_string"))
        .unwrap();
    bindings
        .function(
            FunctionDeclaration::<(BigInt, BitArrayValue), BitArrayValue>::new(
                "captured_bit_array",
            ),
        )
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt, char), char>::new(
            "captured_utf_codepoint",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt, bool), bool>::new(
            "captured_bool",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt, bool), bool>::new(
            "computed_bool",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt, ()), ()>::new("captured_nil"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt, f64), bool>::new("mixed"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt,), ()>::new("literal_nil"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt, f64), f64>::new(
            "computed_float",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt, f64), f64>::new(
            "retained_float",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt, StringValue), StringValue>::new("retained_string"))
        .unwrap();
    bindings
        .function(
            FunctionDeclaration::<(BigInt, BitArrayValue), BitArrayValue>::new(
                "retained_bit_array",
            ),
        )
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt,), BigInt>::new(
            "producer_failure",
        ))
        .unwrap();
    bindings.prepare().unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimitiveInput {
    Float(u64),
    String(StringValue),
    BitArray(BitArrayValue),
    UtfCodepoint(char),
    Bool(bool),
    Nil,
}

#[derive(Default)]
pub struct PrimitiveAudit {
    pub inputs: Vec<PrimitiveInput>,
    pub retained: usize,
}
pub static PRIMITIVES: Mutex<PrimitiveAudit> = Mutex::new(PrimitiveAudit {
    inputs: Vec::new(),
    retained: 0,
});

fn record_primitive(value: PrimitiveInput, retained: bool) -> usize {
    let mut audit = PRIMITIVES.lock().unwrap();
    audit.inputs.push(value);
    audit.retained += usize::from(retained);
    audit.inputs.len()
}

fn observe_float(value: f64, retained: bool) -> f64 {
    let index = record_primitive(PrimitiveInput::Float(value.to_bits()), retained);
    value + index as f64
}
fn observe_string(value: StringValue, retained: bool) -> StringValue {
    let index = record_primitive(PrimitiveInput::String(value), retained);
    if index % 2 == 1 {
        "native odd".into()
    } else {
        "native even".into()
    }
}
fn observe_bit_array(value: BitArrayValue, retained: bool) -> BitArrayValue {
    let index = record_primitive(PrimitiveInput::BitArray(value), retained);
    BitArrayValue::try_from_parts(vec![(index % 256) as u8], 8).unwrap()
}
fn observe_utf_codepoint(value: char, retained: bool) -> char {
    let index = record_primitive(PrimitiveInput::UtfCodepoint(value), retained);
    if index % 2 == 1 { 'β' } else { 'λ' }
}
fn observe_bool(value: bool, retained: bool) -> bool {
    let index = record_primitive(PrimitiveInput::Bool(value), retained);
    value ^ (index % 2 == 1)
}
fn observe_nil(_value: (), retained: bool) {
    record_primitive(PrimitiveInput::Nil, retained);
}
fn float_to_bool(value: f64, retained: bool) -> bool {
    record_primitive(PrimitiveInput::Float(value.to_bits()), retained) % 2 == 1
}

fn scoped_observe_float<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, f64>,
    value: f64,
) -> Result<HostCallCompletion<'call, f64>, HostCallError> {
    Ok(call.return_value(observe_float(value, false)))
}

fn scoped_observe_string<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, StringValue>,
    value: StringValue,
) -> Result<HostCallCompletion<'call, StringValue>, HostCallError> {
    Ok(call.return_value(observe_string(value, false)))
}

fn scoped_observe_bit_array<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, BitArrayValue>,
    value: BitArrayValue,
) -> Result<HostCallCompletion<'call, BitArrayValue>, HostCallError> {
    Ok(call.return_value(observe_bit_array(value, false)))
}

fn scoped_observe_utf_codepoint<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, char>,
    value: char,
) -> Result<HostCallCompletion<'call, char>, HostCallError> {
    Ok(call.return_value(observe_utf_codepoint(value, false)))
}

fn scoped_observe_bool<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, bool>,
    value: bool,
) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
    Ok(call.return_value(observe_bool(value, false)))
}

fn scoped_observe_nil<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, ()>,
    value: (),
) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
    observe_nil(value, false);
    Ok(call.return_value(()))
}

fn scoped_float_to_bool<'call>(
    call: HostCall<'call, StatelessHostProfile, Provider, bool>,
    value: f64,
) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
    Ok(call.return_value(float_to_bool(value, false)))
}

fn primitive_hosts(mut module: HostProviderModule, retained: bool) -> HostProviderModule {
    module = if retained {
        module
            .with_scoped_retained_function::<Provider, f64, f64, _, _>(
                "observe_float",
                scoped_observe_float,
                |value| Ok(observe_float(value, true)),
            )
            .unwrap()
    } else {
        module
            .with_scoped_function::<Provider, (f64,), f64, _>("observe_float", scoped_observe_float)
            .unwrap()
    };
    module = if retained {
        module
            .with_scoped_retained_function::<Provider, StringValue, StringValue, _, _>(
                "observe_string",
                scoped_observe_string,
                |value| Ok(observe_string(value, true)),
            )
            .unwrap()
    } else {
        module
            .with_scoped_function::<Provider, (StringValue,), StringValue, _>(
                "observe_string",
                scoped_observe_string,
            )
            .unwrap()
    };
    module = if retained {
        module
            .with_scoped_retained_function::<Provider, BitArrayValue, BitArrayValue, _, _>(
                "observe_bit_array",
                scoped_observe_bit_array,
                |value| Ok(observe_bit_array(value, true)),
            )
            .unwrap()
    } else {
        module
            .with_scoped_function::<Provider, (BitArrayValue,), BitArrayValue, _>(
                "observe_bit_array",
                scoped_observe_bit_array,
            )
            .unwrap()
    };
    module = if retained {
        module
            .with_scoped_retained_function::<Provider, char, char, _, _>(
                "observe_utf_codepoint",
                scoped_observe_utf_codepoint,
                |value| Ok(observe_utf_codepoint(value, true)),
            )
            .unwrap()
    } else {
        module
            .with_scoped_function::<Provider, (char,), char, _>(
                "observe_utf_codepoint",
                scoped_observe_utf_codepoint,
            )
            .unwrap()
    };
    module = if retained {
        module
            .with_scoped_retained_function::<Provider, bool, bool, _, _>(
                "observe_bool",
                scoped_observe_bool,
                |value| Ok(observe_bool(value, true)),
            )
            .unwrap()
    } else {
        module
            .with_scoped_function::<Provider, (bool,), bool, _>("observe_bool", scoped_observe_bool)
            .unwrap()
    };
    module = if retained {
        module
            .with_scoped_retained_function::<Provider, (), (), _, _>(
                "observe_nil",
                scoped_observe_nil,
                |value| {
                    observe_nil(value, true);
                    Ok(())
                },
            )
            .unwrap()
    } else {
        module
            .with_scoped_function::<Provider, ((),), (), _>("observe_nil", scoped_observe_nil)
            .unwrap()
    };
    module = if retained {
        module
            .with_scoped_retained_function::<Provider, f64, bool, _, _>(
                "float_to_bool",
                scoped_float_to_bool,
                |value| Ok(float_to_bool(value, true)),
            )
            .unwrap()
    } else {
        module
            .with_scoped_function::<Provider, (f64,), bool, _>(
                "float_to_bool",
                scoped_float_to_bool,
            )
            .unwrap()
    };
    module
}
