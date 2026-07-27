use geam::{
    HostModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource, Value,
    compile_typed_host_program, plan_host_program,
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
