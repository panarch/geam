use geam_core::{
    HostModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource, Value,
    compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;

#[test]
fn executes_package_qualified_rust_host_functions() {
    let math = HostModule::new("host_support", "host/math")
        .expect("host module should be valid")
        .with_function("subtract", |left: BigInt, right: BigInt| left - right)
        .expect("host function should be valid")
        .with_function("add", |left: BigInt, right: BigInt| left + right)
        .expect("host function should be valid")
        .with_function("unused", |left: BigInt, right: BigInt| left + right)
        .expect("host function should be valid");
    let hosts = HostProviderSet::new([math]).expect("host modules should be unique");
    let source = r#"
import host/math.{add, subtract}

fn apply(function: fn(Int, Int) -> Int, left: Int, right: Int) {
  function(left, right)
}

fn tail(left: Int, right: Int) {
  add(left, right)
}

fn left() {
  echo 100000000000000000000000000000000000000 as "left"
}

fn right() {
  echo 3 as "right"
}

pub fn main() {
  let qualified = math.add
  #(
    subtract(left(), right()),
    apply(qualified, 20, 22),
    tail(40, 2),
    qualified == add,
    add == add,
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
    assert_eq!(
        plan.modules()[0]
            .functions()
            .iter()
            .map(|function| {
                function
                    .host_template()
                    .expect("dependency should retain host functions")
                    .name()
                    .as_str()
            })
            .collect::<Vec<_>>(),
        ["subtract", "add", "unused"],
    );
    let execution =
        HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
    let expected = Value::Tuple(vec![
        Value::Int(
            "99999999999999999999999999999999999997"
                .parse()
                .expect("expected value should be an Int"),
        ),
        Value::Int(42.into()),
        Value::Int(42.into()),
        Value::Bool(true),
        Value::Bool(true),
    ]);

    let mut first_echoes = Vec::new();
    assert_eq!(
        execution.run_main(&mut (), &mut first_echoes),
        Ok(expected.clone()),
    );
    assert_eq!(
        first_echoes
            .iter()
            .map(|output| output.message().map(|message| message.as_str()))
            .collect::<Vec<_>>(),
        [Some("left"), Some("right")],
    );

    let mut second_echoes = Vec::new();
    assert_eq!(
        execution.run_main(&mut (), &mut second_echoes),
        Ok(expected),
    );
    assert_eq!(
        second_echoes
            .iter()
            .map(|output| output.message().map(|message| message.as_str()))
            .collect::<Vec<_>>(),
        [Some("left"), Some("right")],
    );

    let expected_explanation = r#"
module main
main tuple#0

function int#0
  host host_support::host/math.add signature=fn(Int, Int) -> Int

function int#1
  entry b0 params=[] captures=[]
  block b0 params=[]
    %int#0:shape#0(Int) = int.value 100000000000000000000000000000000000000
    %string#0:shape#4(String) = string.value "left"
    echo subject=%int#0 message=%string#0 site=main::left@196..250 next=b1(%int#0)
  block b1 params=[%int#0:shape#0(Int)]
    return %int#0

function int#2
  entry b0 params=[] captures=[]
  block b0 params=[]
    %int#0:shape#0(Int) = int.value 3
    %string#0:shape#4(String) = string.value "right"
    echo subject=%int#0 message=%string#0 site=main::right@269..286 next=b1(%int#0)
  block b1 params=[%int#0:shape#0(Int)]
    return %int#0

function int#3
  host host_support::host/math.subtract signature=fn(Int, Int) -> Int

function int#4
  entry b0 params=[%function.int#0:shape#1(fn(Int, Int) -> Int), %int#0:shape#0(Int), %int#1:shape#0(Int)] captures=[]
  block b0 params=[%function.int#0:shape#1(fn(Int, Int) -> Int), %int#0:shape#0(Int), %int#1:shape#0(Int)]
    %int#2:shape#0(Int) = int.function_call %function.int#0 args=[%int#0, %int#1]
    return %int#2

function int#5
  entry b0 params=[%int#0:shape#0(Int), %int#1:shape#0(Int)] captures=[]
  block b0 params=[%int#0:shape#0(Int), %int#1:shape#0(Int)]
    tail int#0 args=[%int#0, %int#1]

function tuple#0
  entry b0 params=[] captures=[]
  block b0 params=[]
    %function.int#0:shape#1(fn(Int, Int) -> Int) = function[Int] reference int#0
    %int#0:shape#0(Int) = int.call int#1 args=[]
    %int#1:shape#0(Int) = int.call int#2 args=[]
    %int#2:shape#0(Int) = int.call int#3 args=[%int#0, %int#1]
    %int#3:shape#0(Int) = int.value 20
    %int#4:shape#0(Int) = int.value 22
    %int#5:shape#0(Int) = int.call int#4 args=[%function.int#0, %int#3, %int#4]
    %int#6:shape#0(Int) = int.value 40
    %int#7:shape#0(Int) = int.value 2
    %int#8:shape#0(Int) = int.call int#5 args=[%int#6, %int#7]
    %function.int#1:shape#1(fn(Int, Int) -> Int) = function[Int] reference int#0
    %bool#0:shape#2(Bool) = bool.equal %function.int#0 %function.int#1
    %function.int#2:shape#1(fn(Int, Int) -> Int) = function[Int] reference int#0
    %function.int#3:shape#1(fn(Int, Int) -> Int) = function[Int] reference int#0
    %bool#1:shape#2(Bool) = bool.equal %function.int#2 %function.int#3
    %tuple#0:shape#3(#(Int, Int, Int, Bool, Bool)) = tuple.value elements=[%int#2, %int#5, %int#8, %bool#0, %bool#1]
    return %tuple#0
"#;

    assert_eq!(
        execution.explain().to_string().trim(),
        expected_explanation.trim(),
    );
}
