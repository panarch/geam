use ecow::EcoString;
use geam::{
    BitArrayValue, ExecutionError, HostFailure, HostLocation, HostModule, HostProviderSet,
    HostedExecution, ModuleSource, PackageSource, Value, ValueType, compile_typed_host_program,
    plan_host_program,
};
use num_bigint::BigInt;

#[test]
fn executes_zero_through_seven_mixed_int_and_bool_signatures() {
    let threshold = BigInt::from(0);
    let math = HostModule::new("host_support", "host/math")
        .expect("host module should be valid")
        .with_function("ready", || true)
        .expect("host function should be valid")
        .with_function(
            "choose",
            |condition: bool, left: BigInt, right: BigInt| {
                if condition { left } else { right }
            },
        )
        .expect("host function should be valid")
        .with_function("is_positive", move |value: BigInt| value > threshold)
        .expect("host function should be valid")
        .with_function(
            "all",
            |a: bool, b: bool, c: bool, d: bool, e: bool, f: bool, g: bool| {
                a && b && c && d && e && f && g
            },
        )
        .expect("host function should be valid");
    let hosts = HostProviderSet::new([math]).expect("host modules should be unique");
    let source = r#"
import host/math.{all, is_positive, ready}

fn apply(predicate: fn(Int) -> Bool, value: Int) {
  predicate(value)
}

fn non_tail(value: Int) {
  case math.is_positive(value) {
    True -> 1
    False -> 0
  }
}

fn tail(value: Int) {
  is_positive(value)
}

pub fn main() {
  #(
    math.choose(False, 10, 20),
    ready(),
    all(True, True, True, True, True, True, True),
    non_tail(1),
    tail(1),
    apply(math.is_positive, -1),
    math.is_positive == is_positive,
  )
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            ["host_support"],
            [ModuleSource::new("main", "main.gleam", source)],
        )],
        hosts,
    )
    .expect("host program should compile");
    let plan = plan_host_program(typed).expect("host program should plan");
    let execution = HostedExecution::from_module_plan(plan);
    let expected = Value::Tuple(vec![
        Value::Int(20.into()),
        Value::Bool(true),
        Value::Bool(true),
        Value::Int(1.into()),
        Value::Bool(true),
        Value::Bool(false),
        Value::Bool(true),
    ]);

    assert_eq!(execution.run_main(&mut (), &mut Vec::new()), Ok(expected));
}

#[test]
fn executes_every_scalar_family_through_direct_tail_and_function_value_calls() {
    let scalars = HostModule::new("host_support", "host/scalars")
        .expect("host module should be valid")
        .with_function("int", |value: BigInt| value + 1)
        .expect("host function should be valid")
        .with_function("float", |value: f64| value + 0.5)
        .expect("host function should be valid")
        .with_function("string", |value: EcoString| -> EcoString {
            value.to_uppercase()
        })
        .expect("host function should be valid")
        .with_function("bit_array", |value: BitArrayValue| value)
        .expect("host function should be valid")
        .with_function("utf_codepoint", |value: char| value)
        .expect("host function should be valid")
        .with_function("bool", |value: bool| !value)
        .expect("host function should be valid")
        .with_function("nil", |(): ()| ())
        .expect("host function should be valid");
    let hosts = HostProviderSet::new([scalars]).expect("host modules should be unique");
    let source = r#"
import host/scalars

fn apply_int(function: fn(Int) -> Int, value: Int) { function(value) }
fn apply_float(function: fn(Float) -> Float, value: Float) { function(value) }
fn apply_string(function: fn(String) -> String, value: String) { function(value) }
fn apply_bit_array(function: fn(BitArray) -> BitArray, value: BitArray) { function(value) }
fn apply_utf_codepoint(
  function: fn(UtfCodepoint) -> UtfCodepoint,
  value: UtfCodepoint,
) {
  function(value)
}
fn apply_bool(function: fn(Bool) -> Bool, value: Bool) { function(value) }
fn apply_nil(function: fn(Nil) -> Nil, value: Nil) { function(value) }

fn tail_int(value: Int) { scalars.int(value) }
fn tail_float(value: Float) { scalars.float(value) }
fn tail_string(value: String) { scalars.string(value) }
fn tail_bit_array(value: BitArray) { scalars.bit_array(value) }
fn tail_utf_codepoint(value: UtfCodepoint) { scalars.utf_codepoint(value) }
fn tail_bool(value: Bool) { scalars.bool(value) }
fn tail_nil(value: Nil) { scalars.nil(value) }

pub fn main() {
  let assert <<codepoint:utf8_codepoint>> = <<"A":utf8>>
  #(
    scalars.int(1),
    scalars.float(1.0),
    scalars.string("one"),
    scalars.bit_array(<<1, 2>>),
    scalars.utf_codepoint(codepoint),
    scalars.bool(True),
    scalars.nil(Nil),
    tail_int(2),
    tail_float(2.0),
    tail_string("two"),
    tail_bit_array(<<3>>),
    tail_utf_codepoint(codepoint),
    tail_bool(False),
    tail_nil(Nil),
    apply_int(scalars.int, 3),
    apply_float(scalars.float, 3.0),
    apply_string(scalars.string, "three"),
    apply_bit_array(scalars.bit_array, <<4>>),
    apply_utf_codepoint(scalars.utf_codepoint, codepoint),
    apply_bool(scalars.bool, True),
    apply_nil(scalars.nil, Nil),
    scalars.int == scalars.int,
    scalars.float == scalars.float,
    scalars.string == scalars.string,
    scalars.bit_array == scalars.bit_array,
    scalars.utf_codepoint == scalars.utf_codepoint,
    scalars.bool == scalars.bool,
    scalars.nil == scalars.nil,
  )
}
"#;
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            ["host_support"],
            [ModuleSource::new("main", "main.gleam", source)],
        )],
        hosts,
    )
    .expect("host program should compile");
    let plan = plan_host_program(typed).expect("host program should plan");
    let execution = HostedExecution::from_module_plan(plan);
    let expected = Value::Tuple(vec![
        Value::Int(2.into()),
        Value::Float(1.5),
        Value::String("ONE".into()),
        Value::BitArray(BitArrayValue::from_bytes(vec![1, 2])),
        Value::UtfCodepoint('A'),
        Value::Bool(false),
        Value::Nil,
        Value::Int(3.into()),
        Value::Float(2.5),
        Value::String("TWO".into()),
        Value::BitArray(BitArrayValue::from_bytes(vec![3])),
        Value::UtfCodepoint('A'),
        Value::Bool(true),
        Value::Nil,
        Value::Int(4.into()),
        Value::Float(3.5),
        Value::String("THREE".into()),
        Value::BitArray(BitArrayValue::from_bytes(vec![4])),
        Value::UtfCodepoint('A'),
        Value::Bool(false),
        Value::Nil,
        Value::Bool(true),
        Value::Bool(true),
        Value::Bool(true),
        Value::Bool(true),
        Value::Bool(true),
        Value::Bool(true),
        Value::Bool(true),
    ]);

    assert_eq!(execution.run_main(&mut (), &mut Vec::new()), Ok(expected));
}

#[test]
fn reports_every_scalar_host_failure_at_the_source_call_site() {
    let cases = [
        (
            HostModule::new("host_support", "host/scalars")
                .expect("host module should be valid")
                .with_fallible_function("fail", || -> Result<f64, HostFailure> {
                    Err(HostFailure::new("float unavailable"))
                })
                .expect("host function should be valid"),
            ValueType::Float,
            "float unavailable",
        ),
        (
            HostModule::new("host_support", "host/scalars")
                .expect("host module should be valid")
                .with_fallible_function("fail", || -> Result<EcoString, HostFailure> {
                    Err(HostFailure::new("string unavailable"))
                })
                .expect("host function should be valid"),
            ValueType::String,
            "string unavailable",
        ),
        (
            HostModule::new("host_support", "host/scalars")
                .expect("host module should be valid")
                .with_fallible_function("fail", || -> Result<BitArrayValue, HostFailure> {
                    Err(HostFailure::new("bit array unavailable"))
                })
                .expect("host function should be valid"),
            ValueType::BitArray,
            "bit array unavailable",
        ),
        (
            HostModule::new("host_support", "host/scalars")
                .expect("host module should be valid")
                .with_fallible_function("fail", || -> Result<char, HostFailure> {
                    Err(HostFailure::new("codepoint unavailable"))
                })
                .expect("host function should be valid"),
            ValueType::UtfCodepoint,
            "codepoint unavailable",
        ),
        (
            HostModule::new("host_support", "host/scalars")
                .expect("host module should be valid")
                .with_fallible_function("fail", || -> Result<(), HostFailure> {
                    Err(HostFailure::new("nil unavailable"))
                })
                .expect("host function should be valid"),
            ValueType::Nil,
            "nil unavailable",
        ),
    ];
    let source = r#"
import host/scalars

pub fn main() {
  scalars.fail()
}
"#;

    for (module, return_, message) in cases {
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new("main", "src/main.gleam", source)],
            )],
            HostProviderSet::new([module]).expect("host module should be unique"),
        )
        .expect("host program should compile");
        let plan = plan_host_program(typed).expect("host program should plan");
        let execution = HostedExecution::from_module_plan(plan);
        let error = execution
            .run_main(&mut (), &mut Vec::new())
            .expect_err("fallible host function should fail");
        let ExecutionError::Host(error) = error else {
            panic!("fallible host function should produce a host error");
        };

        assert_eq!(error.package(), "host_support");
        assert_eq!(error.module(), "host/scalars");
        assert_eq!(error.function(), "fail");
        assert_eq!(error.signature().argument_types(), []);
        assert_eq!(error.signature().return_(), &return_);
        assert_eq!(error.failure().message(), message);
        assert_eq!(
            error.location(),
            &HostLocation::Resolved {
                site: geam::HostCallSite::new(
                    "main".into(),
                    "main".into(),
                    geam::SourceSpan::new(40, 54),
                ),
                path: "src/main.gleam".into(),
                line: 5,
            },
        );
    }
}
