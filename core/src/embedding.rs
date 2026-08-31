//! Statically typed Rust calls into plain or hosted Gleam code.
//!
//! Loading and binding happen once. [`ModuleBuilder`] and
//! [`HostedModuleBuilder`] select the first function into non-empty binding
//! owners, which validate any remaining names and signatures from the selected
//! root before sealing one immutable execution shared by every returned
//! [`Function`] handle. Plain calls supply an echo sink; hosted calls also
//! borrow the caller's provider state explicitly. Both accept only the Rust
//! argument and return shapes that were bound up front.
//!
//! Values include scalars and recursive Rust tuples, standard `Result` and
//! `Option`, and [`List`]. Tuple values have arity one through seven; function
//! argument tuples have arity zero through seven, and `()` remains Gleam Nil.
//! Result and Option map only to the exact prelude and stdlib types.
//! A consumed `Vec` constructs a List; a borrowed same-owner List reuses its
//! retained storage. See [`List`] for lazy reads and ownership restrictions.
//!
//! [`Project`] and [`HostedProject`] retain one source selection until it is
//! compiled into the corresponding existing typed program owner.

mod binding;
mod error;
mod hosted;
mod input;
mod list;
mod project;
mod value;

pub use crate::BitArrayValue;
pub use binding::{BindingError, FunctionDeclaration, ModuleBindings, ModuleBuilder};
pub use ecow::EcoString;
pub use error::CallError;
pub use hosted::{HostedModule, HostedModuleBindings, HostedModuleBuilder};
#[doc(hidden)]
pub use input::InputShape;
pub use list::{Iter, List};
pub use num_bigint::BigInt;
pub use project::{HostedProject, Project};

use self::input::ArgumentsInput;
use self::value::{Arguments, ReturnValue};
use crate::plan::execution::LibraryFunctionEntries;
use crate::{EchoSink, ExecutionPlan};
use std::marker::PhantomData;
use std::sync::Arc;

/// A typed function handle created by a plain or hosted module builder.
///
/// The handle becomes callable only after its binding owner is sealed, and
/// only the resulting [`Module`] or [`HostedModule`] may call it.
pub struct Function<Arguments, Return, Shape = Arguments> {
    name: EcoString,
    slot: usize,
    owner: Arc<()>,
    marker: PhantomData<fn(Arguments, Shape) -> Return>,
}

/// One sealed plain execution shared by all functions selected from a module.
pub struct Module {
    execution: ExecutionPlan,
    entries: LibraryFunctionEntries,
    owner: Arc<()>,
}

impl<Arguments, Return, Shape> Clone for Function<Arguments, Return, Shape> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            slot: self.slot,
            owner: Arc::clone(&self.owner),
            marker: PhantomData,
        }
    }
}

impl<Arguments, Return, Shape> Function<Arguments, Return, Shape> {
    /// Returns the source name selected for this function.
    pub fn name(&self) -> &EcoString {
        &self.name
    }

    /// Attaches a generated inference shape without changing the validated signature.
    #[doc(hidden)]
    pub fn with_input_shape<NextShape>(self) -> Function<Arguments, Return, NextShape> {
        Function {
            name: self.name,
            slot: self.slot,
            owner: self.owner,
            marker: PhantomData,
        }
    }

    fn new(name: EcoString, slot: usize, owner: &Arc<()>) -> Self {
        Self {
            name,
            slot,
            owner: Arc::clone(owner),
            marker: PhantomData,
        }
    }
}

impl Module {
    /// Calls a bound function with Rust values through its prevalidated entry.
    #[allow(private_bounds)]
    pub fn call<Arguments, Return, Input, Shape>(
        &self,
        function: &Function<Arguments, Return, Shape>,
        arguments: Input,
        echo: &mut dyn EchoSink,
    ) -> Result<Return, CallError>
    where
        Arguments: ArgumentsInput<Input>,
        Return: ReturnValue,
        Shape: InputShape<Input>,
    {
        self.check_owner(&function.owner).and_then(|()| {
            if !Arguments::owners_match(&arguments, &self.owner) {
                return Err(CallError::ForeignValue);
            }
            let constructions = Return::input_constructions(&self.entries, function.slot);
            let inputs = Arguments::into_inputs(arguments, constructions);
            Return::call(self, function.slot, inputs, echo).map_err(CallError::Execution)
        })
    }

    fn check_owner(&self, owner: &Arc<()>) -> Result<(), CallError> {
        if Arc::ptr_eq(&self.owner, owner) {
            Ok(())
        } else {
            Err(CallError::ForeignFunction)
        }
    }

    fn from_parts(
        execution: ExecutionPlan,
        entries: LibraryFunctionEntries,
        owner: Arc<()>,
    ) -> Self {
        Self {
            execution,
            entries,
            owner,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Arguments, CallError, Function, FunctionDeclaration, ModuleBindings, ModuleBuilder,
        ReturnValue,
    };
    use crate::{
        BitArrayValue, ExecutionError, ModuleSource, PackageSource, PanicKind, PanicSite,
        SourceContext, SourceSpan, TypedProgram, Value, compile_typed_module,
        compile_typed_package_program, compile_typed_program,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;

    fn compile(source: &str) -> gleam_compiler_core::ast::TypedModule {
        compile_typed_module("library", "library.gleam", source).expect("source should compile")
    }

    fn compile_program(root_source: &str, support_source: &str) -> TypedProgram {
        compile_typed_program(
            "library",
            [
                ModuleSource::new("support", "support.gleam", support_source),
                ModuleSource::new("library", "library.gleam", root_source),
            ],
        )
        .expect("library program should compile")
    }

    fn compile_option_program(root_source: &str) -> TypedProgram {
        compile_typed_package_program(
            "application",
            "library",
            [
                PackageSource::new(
                    "gleam_stdlib",
                    Vec::<EcoString>::new(),
                    [ModuleSource::new(
                        "gleam/option",
                        "gleam_stdlib/src/gleam/option.gleam",
                        "pub type Option(value) { Some(value) None }",
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["gleam_stdlib"],
                    [ModuleSource::new(
                        "library",
                        "src/library.gleam",
                        root_source,
                    )],
                ),
            ],
        )
        .expect("stdlib Option program should compile")
    }

    fn bind<ArgumentsType, Return>(
        bindings: &mut ModuleBindings,
        name: &str,
    ) -> Function<ArgumentsType, Return>
    where
        ArgumentsType: Arguments,
        Return: ReturnValue,
    {
        bindings
            .function(FunctionDeclaration::<ArgumentsType, Return>::new(name))
            .expect("function should bind")
    }

    #[test]
    fn seals_multiple_named_entries_without_main_and_calls_them_repeatedly() {
        let typed = compile(
            r#"
fn decorate(prefix: String, value: String) { prefix <> value }

pub fn label(prefix: String, value: String) { decorate(prefix, value) }

pub fn double(value: Int) { value * 2 }

pub fn choose(enabled: Bool, left: Float, right: Float) {
  case enabled { True -> left False -> right }
}
"#,
        );
        let builder = ModuleBuilder::new(typed).expect("library should plan without main");
        let (mut bindings, label) = builder
            .function(FunctionDeclaration::<(EcoString, EcoString), EcoString>::new("label"))
            .expect("first function should bind");
        let double = bind::<(BigInt,), BigInt>(&mut bindings, "double");
        let choose = bind::<(bool, f64, f64), f64>(&mut bindings, "choose");
        assert_eq!(label.name(), "label");
        assert_eq!(label.clone().name(), "label");
        let module = bindings.seal();
        let mut echo = Vec::new();

        for (prefix, value, expected) in
            [("SKU:", "AB-12", "SKU:AB-12"), ("BIN:", "C-4", "BIN:C-4")]
        {
            assert_eq!(
                module.call(&label, (prefix.into(), value.into()), &mut echo),
                Ok(expected.into()),
            );
        }
        assert_eq!(
            module.call(&double, (BigInt::from(21),), &mut echo),
            Ok(BigInt::from(42)),
        );
        assert_eq!(module.call(&choose, (true, 12.5, 9.0), &mut echo), Ok(12.5),);
        assert!(echo.is_empty());
    }

    #[test]
    fn seals_cross_module_entries_once_and_calls_them_repeatedly() {
        let program = compile_program(
            r#"
import support

pub fn label(value: String) { support.decorate("SKU:", value) }
pub fn double(value: Int) { support.double(value) }
"#,
            r#"
pub fn decorate(prefix: String, value: String) { prefix <> value }
pub fn double(value: Int) { value * 2 }
"#,
        );
        let builder =
            ModuleBuilder::from_program(program).expect("library program should plan without main");
        let (mut bindings, label) = builder
            .function(FunctionDeclaration::<(EcoString,), EcoString>::new("label"))
            .expect("first root function should bind");
        let double = bind::<(BigInt,), BigInt>(&mut bindings, "double");
        let module = bindings.seal();
        let mut echo = Vec::new();

        for (value, expected) in [("AB-12", "SKU:AB-12"), ("C-4", "SKU:C-4")] {
            assert_eq!(
                module.call(&label, (value.into(),), &mut echo),
                Ok(expected.into()),
            );
        }
        assert_eq!(
            module.call(&double, (BigInt::from(21),), &mut echo),
            Ok(BigInt::from(42)),
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn sends_each_call_echo_to_the_caller_owned_sink() {
        let typed = compile(
            r#"
pub fn announce(value: String) {
  echo value as "embedded"
}
"#,
        );
        let builder = ModuleBuilder::new(typed).expect("echo library should plan");
        let (bindings, announce) = builder
            .function(FunctionDeclaration::<(EcoString,), EcoString>::new(
                "announce",
            ))
            .expect("first function should bind");
        let module = bindings.seal();
        let mut first_echo = Vec::new();
        let mut second_echo = Vec::new();

        assert_eq!(
            module.call(&announce, ("first".into(),), &mut first_echo),
            Ok("first".into()),
        );
        assert_eq!(
            module.call(&announce, ("second".into(),), &mut second_echo),
            Ok("second".into()),
        );
        assert_eq!(first_echo.len(), 1);
        assert_eq!(
            first_echo[0].message().map(EcoString::as_str),
            Some("embedded")
        );
        assert_eq!(first_echo[0].value(), &Value::String("first".into()));
        assert_eq!(second_echo.len(), 1);
        assert_eq!(
            second_echo[0].message().map(EcoString::as_str),
            Some("embedded")
        );
        assert_eq!(second_echo[0].value(), &Value::String("second".into()));
    }

    #[test]
    fn moves_every_scalar_family_through_exact_typed_entries() {
        let typed = compile(
            r#"
pub fn keep_int(value: Int) { value }
pub fn keep_float(value: Float) { value }
pub fn keep_string(value: String) { value }
pub fn keep_bits(value: BitArray) { value }
pub fn keep_codepoint(value: UtfCodepoint) { value }
pub fn keep_bool(value: Bool) { value }
pub fn keep_nil(value: Nil) { value }

pub fn mixed(
  _int: Int,
  _float: Float,
  _string: String,
  _bits: BitArray,
  _codepoint: UtfCodepoint,
  value: Bool,
  _nil: Nil,
) {
  value
}
"#,
        );
        let builder = ModuleBuilder::new(typed).expect("scalar library should plan");
        let (mut bindings, int) = builder
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("keep_int"))
            .expect("first function should bind");
        let float = bind::<(f64,), f64>(&mut bindings, "keep_float");
        let string = bind::<(EcoString,), EcoString>(&mut bindings, "keep_string");
        let bits = bind::<(BitArrayValue,), BitArrayValue>(&mut bindings, "keep_bits");
        let codepoint = bind::<(char,), char>(&mut bindings, "keep_codepoint");
        let bool_ = bind::<(bool,), bool>(&mut bindings, "keep_bool");
        let nil = bind::<((),), ()>(&mut bindings, "keep_nil");
        let mixed = bind::<(BigInt, f64, EcoString, BitArrayValue, char, bool, ()), bool>(
            &mut bindings,
            "mixed",
        );
        let module = bindings.seal();
        let mut echo = Vec::new();
        let bit_value = BitArrayValue::try_from_parts(vec![0b1010_0000], 3)
            .expect("three bits should fit in one byte");

        assert_eq!(
            module.call(&int, (BigInt::from(123),), &mut echo),
            Ok(BigInt::from(123)),
        );
        assert_eq!(module.call(&float, (1.25,), &mut echo), Ok(1.25));
        assert_eq!(
            module.call(&string, ("value".into(),), &mut echo),
            Ok("value".into()),
        );
        assert_eq!(
            module.call(&bits, (bit_value.clone(),), &mut echo),
            Ok(bit_value.clone()),
        );
        assert_eq!(module.call(&codepoint, ('한',), &mut echo), Ok('한'));
        assert_eq!(module.call(&bool_, (true,), &mut echo), Ok(true));
        assert_eq!(module.call(&nil, ((),), &mut echo), Ok(()));
        assert_eq!(
            module.call(
                &mixed,
                (
                    BigInt::from(1),
                    2.0,
                    "three".into(),
                    bit_value,
                    '四',
                    false,
                    (),
                ),
                &mut echo,
            ),
            Ok(false),
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn moves_every_tuple_arity_and_recursive_result_through_one_sealed_module() {
        let typed = compile(
            r#"
pub fn scalar(value: Int) { value }
pub fn tuple1(value: #(Int)) { value }
pub fn tuple2(value: #(Int, String)) { value }
pub fn tuple3(value: #(Int, String, Bool)) { value }
pub fn tuple4(value: #(Int, String, Bool, Nil)) { value }
pub fn tuple5(value: #(Int, String, Bool, Nil, Float)) { value }
pub fn tuple6(value: #(Int, String, Bool, Nil, Float, UtfCodepoint)) { value }
pub fn tuple7(value: #(Int, String, Bool, Nil, Float, UtfCodepoint, BitArray)) { value }

pub fn swap_result(
  value: Result(#(Int, String), #(Bool, Nil)),
) -> Result(#(Bool, Nil), #(Int, String)) {
  case value {
    Ok(pair) -> Error(pair)
    Error(pair) -> Ok(pair)
  }
}
"#,
        );
        let builder = ModuleBuilder::new(typed).expect("recursive library should plan");
        let (mut bindings, tuple1) = builder
            .function(FunctionDeclaration::<((BigInt,),), (BigInt,)>::new(
                "tuple1",
            ))
            .expect("tuple should bind first");
        let scalar = bind::<(BigInt,), BigInt>(&mut bindings, "scalar");
        let tuple2 = bind::<((BigInt, EcoString),), (BigInt, EcoString)>(&mut bindings, "tuple2");
        let tuple3 = bind::<((BigInt, EcoString, bool),), (BigInt, EcoString, bool)>(
            &mut bindings,
            "tuple3",
        );
        let tuple4 = bind::<((BigInt, EcoString, bool, ()),), (BigInt, EcoString, bool, ())>(
            &mut bindings,
            "tuple4",
        );
        let tuple5 = bind::<
            ((BigInt, EcoString, bool, (), f64),),
            (BigInt, EcoString, bool, (), f64),
        >(&mut bindings, "tuple5");
        let tuple6 = bind::<
            ((BigInt, EcoString, bool, (), f64, char),),
            (BigInt, EcoString, bool, (), f64, char),
        >(&mut bindings, "tuple6");
        let tuple7 = bind::<
            ((BigInt, EcoString, bool, (), f64, char, BitArrayValue),),
            (BigInt, EcoString, bool, (), f64, char, BitArrayValue),
        >(&mut bindings, "tuple7");
        let swap_result = bind::<
            (Result<(BigInt, EcoString), (bool, ())>,),
            Result<(bool, ()), (BigInt, EcoString)>,
        >(&mut bindings, "swap_result");
        let module = bindings.seal();
        let mut echo = Vec::new();
        let bits = BitArrayValue::try_from_parts(vec![0b1010_0000], 3)
            .expect("three bits should fit in one byte");

        assert_eq!(
            module.call(&scalar, (BigInt::from(9),), &mut echo),
            Ok(BigInt::from(9)),
        );
        assert_eq!(
            module.call(&tuple1, ((BigInt::from(1),),), &mut echo),
            Ok((BigInt::from(1),)),
        );
        assert_eq!(
            module.call(&tuple2, ((2.into(), "two".into()),), &mut echo),
            Ok((2.into(), "two".into())),
        );
        assert_eq!(
            module.call(&tuple3, ((3.into(), "three".into(), true),), &mut echo),
            Ok((3.into(), "three".into(), true)),
        );
        assert_eq!(
            module.call(&tuple4, ((4.into(), "four".into(), false, ()),), &mut echo,),
            Ok((4.into(), "four".into(), false, ())),
        );
        assert_eq!(
            module.call(
                &tuple5,
                ((5.into(), "five".into(), true, (), 5.5),),
                &mut echo,
            ),
            Ok((5.into(), "five".into(), true, (), 5.5)),
        );
        assert_eq!(
            module.call(
                &tuple6,
                ((6.into(), "six".into(), false, (), 6.5, '六'),),
                &mut echo,
            ),
            Ok((6.into(), "six".into(), false, (), 6.5, '六')),
        );
        assert_eq!(
            module.call(
                &tuple7,
                ((7.into(), "seven".into(), true, (), 7.5, '七', bits.clone()),),
                &mut echo,
            ),
            Ok((7.into(), "seven".into(), true, (), 7.5, '七', bits)),
        );
        assert_eq!(
            module.call(&swap_result, (Ok((8.into(), "ok".into())),), &mut echo,),
            Ok(Err((8.into(), "ok".into()))),
        );
        assert_eq!(
            module.call(&swap_result, (Err((true, ())),), &mut echo),
            Ok(Ok((true, ()))),
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn preserves_exact_option_identity_aliases_and_recursive_variants() {
        let program = compile_option_program(
            r#"
import gleam/option.{type Option as Maybe}

pub fn keep_option(value: Maybe(#(Int, Result(String, Bool)))) { value }

pub fn keep_nested(
  value: #(Result(Maybe(Int), String), Maybe(Result(Bool, Nil))),
) {
  value
}
"#,
        );
        let builder =
            ModuleBuilder::from_program(program).expect("Option library should plan without main");
        let (mut bindings, keep_option) = builder
            .function(FunctionDeclaration::<
                (Option<(BigInt, Result<EcoString, bool>)>,),
                Option<(BigInt, Result<EcoString, bool>)>,
            >::new("keep_option"))
            .expect("aliased Option should bind");
        let keep_nested = bind::<
            ((Result<Option<BigInt>, EcoString>, Option<Result<bool, ()>>),),
            (Result<Option<BigInt>, EcoString>, Option<Result<bool, ()>>),
        >(&mut bindings, "keep_nested");
        let module = bindings.seal();
        let mut echo = Vec::new();

        let some_ok = Some((BigInt::from(1), Ok("one".into())));
        assert_eq!(
            module.call(&keep_option, (some_ok.clone(),), &mut echo),
            Ok(some_ok),
        );
        assert_eq!(module.call(&keep_option, (None,), &mut echo), Ok(None));

        let nested = (Ok(Some(BigInt::from(2))), Some(Err(())));
        assert_eq!(
            module.call(&keep_nested, (nested.clone(),), &mut echo),
            Ok(nested),
        );
        let opposite = (Err("stopped".into()), Some(Ok(false)));
        assert_eq!(
            module.call(&keep_nested, (opposite.clone(),), &mut echo),
            Ok(opposite),
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn seals_each_scalar_return_family_as_an_independent_entry() {
        let source = r#"
pub fn int_value() { 1 }
pub fn float_value() { 1.5 }
pub fn string_value() { "value" }
pub fn bit_array_value() { <<1, 2>> }
pub fn codepoint_value() {
  let assert <<value:utf8_codepoint>> = <<"한":utf8>>
  value
}
pub fn bool_value() { True }
pub fn nil_value() { Nil }
"#;

        fn call_only<Return>(source: &str, name: &str) -> Return
        where
            Return: ReturnValue,
        {
            let builder = ModuleBuilder::new(compile(source)).expect("library should plan");
            let (bindings, function) = builder
                .function(FunctionDeclaration::<(), Return>::new(name))
                .expect("first function should bind");
            let module = bindings.seal();
            module
                .call(&function, (), &mut Vec::new())
                .expect("single entry should run")
        }

        assert_eq!(call_only::<BigInt>(source, "int_value"), BigInt::from(1));
        assert_eq!(call_only::<f64>(source, "float_value"), 1.5);
        assert_eq!(
            call_only::<EcoString>(source, "string_value"),
            EcoString::from("value"),
        );
        assert_eq!(
            call_only::<BitArrayValue>(source, "bit_array_value"),
            BitArrayValue::from_bytes(vec![1, 2]),
        );
        assert_eq!(call_only::<char>(source, "codepoint_value"), '한');
        assert!(call_only::<bool>(source, "bool_value"));
        assert_eq!(call_only::<()>(source, "nil_value"), ());
    }

    #[test]
    fn supports_every_argument_arity_through_seven() {
        let typed = compile(
            r#"
pub fn arity0() { 0 }
pub fn arity1(a: Int) { a }
pub fn arity2(a: Int, b: Int) { a + b }
pub fn arity3(a: Int, b: Int, c: Int) { a + b + c }
pub fn arity4(a: Int, b: Int, c: Int, d: Int) { a + b + c + d }
pub fn arity5(a: Int, b: Int, c: Int, d: Int, e: Int) { a + b + c + d + e }
pub fn arity6(a: Int, b: Int, c: Int, d: Int, e: Int, f: Int) { a + b + c + d + e + f }
pub fn arity7(a: Int, b: Int, c: Int, d: Int, e: Int, f: Int, g: Int) {
  a + b + c + d + e + f + g
}
"#,
        );
        let builder = ModuleBuilder::new(typed).expect("arity library should plan");
        let (mut bindings, arity0) = builder
            .function(FunctionDeclaration::<(), BigInt>::new("arity0"))
            .expect("first function should bind");
        let arity1 = bind::<(BigInt,), BigInt>(&mut bindings, "arity1");
        let arity2 = bind::<(BigInt, BigInt), BigInt>(&mut bindings, "arity2");
        let arity3 = bind::<(BigInt, BigInt, BigInt), BigInt>(&mut bindings, "arity3");
        let arity4 = bind::<(BigInt, BigInt, BigInt, BigInt), BigInt>(&mut bindings, "arity4");
        let arity5 =
            bind::<(BigInt, BigInt, BigInt, BigInt, BigInt), BigInt>(&mut bindings, "arity5");
        let arity6 = bind::<(BigInt, BigInt, BigInt, BigInt, BigInt, BigInt), BigInt>(
            &mut bindings,
            "arity6",
        );
        let arity7 = bind::<(BigInt, BigInt, BigInt, BigInt, BigInt, BigInt, BigInt), BigInt>(
            &mut bindings,
            "arity7",
        );
        let module = bindings.seal();
        let mut echo = Vec::new();

        assert_eq!(module.call(&arity0, (), &mut echo), Ok(BigInt::from(0)));
        assert_eq!(
            module.call(&arity1, (1.into(),), &mut echo),
            Ok(BigInt::from(1)),
        );
        assert_eq!(
            module.call(&arity2, (1.into(), 2.into()), &mut echo),
            Ok(BigInt::from(3)),
        );
        assert_eq!(
            module.call(&arity3, (1.into(), 2.into(), 3.into()), &mut echo),
            Ok(BigInt::from(6)),
        );
        assert_eq!(
            module.call(&arity4, (1.into(), 2.into(), 3.into(), 4.into()), &mut echo),
            Ok(BigInt::from(10)),
        );
        assert_eq!(
            module.call(
                &arity5,
                (1.into(), 2.into(), 3.into(), 4.into(), 5.into()),
                &mut echo,
            ),
            Ok(BigInt::from(15)),
        );
        assert_eq!(
            module.call(
                &arity6,
                (1.into(), 2.into(), 3.into(), 4.into(), 5.into(), 6.into(),),
                &mut echo,
            ),
            Ok(BigInt::from(21)),
        );
        assert_eq!(
            module.call(
                &arity7,
                (
                    1.into(),
                    2.into(),
                    3.into(),
                    4.into(),
                    5.into(),
                    6.into(),
                    7.into(),
                ),
                &mut echo,
            ),
            Ok(BigInt::from(28)),
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn rejects_a_function_handle_from_another_module() {
        let source = "pub fn identity(value: String) { value }";
        let first = ModuleBuilder::new(compile(source)).expect("first module should plan");
        let (first, first_identity) = first
            .function(FunctionDeclaration::<(EcoString,), EcoString>::new(
                "identity",
            ))
            .expect("first identity should bind");
        let first = first.seal();
        let second = ModuleBuilder::new(compile(source)).expect("second module should plan");
        let (second, second_identity) = second
            .function(FunctionDeclaration::<(EcoString,), EcoString>::new(
                "identity",
            ))
            .expect("second identity should bind");
        let second = second.seal();

        assert_eq!(
            second.call(&first_identity, ("value".into(),), &mut Vec::new()),
            Err(CallError::ForeignFunction),
        );
        assert_eq!(
            first.call(&first_identity, ("first".into(),), &mut Vec::new()),
            Ok("first".into()),
        );
        assert_eq!(
            second.call(&second_identity, ("second".into(),), &mut Vec::new()),
            Ok("second".into()),
        );
    }

    #[test]
    fn propagates_source_execution_failure_from_a_bound_entry() {
        let typed = compile(
            r#"
pub fn explode(_value: String) -> String { panic as "stopped" }
"#,
        );
        let builder = ModuleBuilder::new(typed).expect("library should plan");
        let (bindings, explode) = builder
            .function(FunctionDeclaration::<(EcoString,), EcoString>::new(
                "explode",
            ))
            .expect("first function should bind");
        let module = bindings.seal();

        let error = module
            .call(&explode, ("value".into(),), &mut Vec::new())
            .expect_err("source panic should cross the embedding boundary");
        assert_eq!(
            error,
            CallError::Execution(ExecutionError::source_panic(
                None,
                PanicKind::Panic,
                Some("stopped".into()),
                PanicSite::new("library".into(), "explode".into(), SourceSpan::new(44, 62),),
            )),
        );
    }

    #[test]
    fn preserves_imported_source_context_for_execution_failure() {
        let support_source = r#"
pub fn explode(_value: String) -> String {
  panic as "dependency stopped"
}
"#;
        let program = compile_program(
            r#"
import support

pub fn explode(value: String) { support.explode(value) }
"#,
            support_source,
        );
        let builder = ModuleBuilder::from_program(program).expect("library program should plan");
        let (bindings, explode) = builder
            .function(FunctionDeclaration::<(EcoString,), EcoString>::new(
                "explode",
            ))
            .expect("root function should bind");
        let module = bindings.seal();
        let panic_expression = "panic as \"dependency stopped\"";
        let start = support_source
            .find(panic_expression)
            .expect("fixture should contain the panic expression");

        assert_eq!(
            module.call(&explode, ("value".into(),), &mut Vec::new()),
            Err(CallError::Execution(ExecutionError::source_panic(
                Some(&SourceContext::new("support.gleam", support_source)),
                PanicKind::Panic,
                Some("dependency stopped".into()),
                PanicSite::new(
                    "support".into(),
                    "explode".into(),
                    SourceSpan::new(start, start + panic_expression.len()),
                ),
            ))),
        );
    }
}
