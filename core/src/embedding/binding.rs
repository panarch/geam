mod error;

pub use error::BindingError;

use super::{Arguments, Function, Module, ReturnValue};
use crate::HostProfile;
use crate::plan::{
    FunctionTemplateId, FunctionTemplateSignature, FunctionType, HostedLibraryModulePlan,
    LibraryEntry, LibraryModulePlan, LibraryValueType,
};
use crate::{ExecutionPlan, PlanError, TypedProgram};
use ecow::EcoString;
use gleam_compiler_core::ast::{Publicity, TypedModule};
use std::collections::HashSet;
use std::marker::PhantomData;
use std::sync::Arc;

/// Plans a typed Gleam module or resolved project before selecting its first
/// embedded function.
///
/// An empty builder cannot be sealed. Its first successful [`Self::function`]
/// call returns a non-empty [`ModuleBindings`] owner:
///
/// ```compile_fail
/// use geam_core::embedding::ModuleBuilder;
///
/// fn seal_empty(builder: ModuleBuilder) {
///     let _ = builder.seal();
/// }
/// ```
pub struct ModuleBuilder {
    inner: BindingBuilder<LibraryModulePlan>,
}

/// Collects one or more typed function bindings before sealing their module.
pub struct ModuleBindings {
    inner: Bindings<LibraryModulePlan>,
}

pub(super) struct BindingBuilder<Plan> {
    source: BindingSource<Plan>,
    owner: Arc<()>,
}

pub(super) struct Bindings<Plan> {
    source: BindingSource<Plan>,
    selected_names: HashSet<EcoString>,
    first: LibraryEntry,
    remaining: Vec<LibraryEntry>,
    counts: LibraryEntryCounts,
    owner: Arc<()>,
}

pub(super) struct BindingParts<Plan> {
    pub(super) plan: Plan,
    pub(super) first: LibraryEntry,
    pub(super) remaining: Vec<LibraryEntry>,
    pub(super) owner: Arc<()>,
}

/// A typed declaration for one Gleam function selected for Rust embedding.
///
/// Arguments are represented by Rust tuples with arity `0..=7`. Supported
/// values are [`super::BigInt`], `f64`, [`super::EcoString`],
/// [`super::BitArrayValue`], `char`, `bool`, `()`, Rust tuples with arity
/// `1..=7`, `Result`, `Option`, and [`super::List`]. Compound values may contain
/// one another. A List declaration accepts a consumed Vec or a borrowed List
/// from the same Module, and returns a retained List.
///
/// Unsupported Rust values and argument arities are rejected by Rust type
/// checking:
///
/// ```compile_fail
/// use geam_core::embedding::FunctionDeclaration;
///
/// let _ = FunctionDeclaration::<(i64,), i64>::new("unsupported");
/// ```
///
/// ```compile_fail
/// use geam_core::embedding::FunctionDeclaration;
///
/// let _ = FunctionDeclaration::<((), (), (), (), (), (), (), ()), ()>::new("too_many");
/// ```
///
/// ```compile_fail
/// use geam_core::embedding::FunctionDeclaration;
///
/// let _ = FunctionDeclaration::<(), ((), (), (), (), (), (), (), ())>::new("wide_tuple");
/// ```
pub struct FunctionDeclaration<Arguments, Return> {
    name: EcoString,
    marker: PhantomData<fn(Arguments) -> Return>,
}

struct BindingSource<Plan> {
    plan: Plan,
    public_functions: HashSet<EcoString>,
}

pub(super) trait BindingPlan {
    fn function_signature(&self, name: &EcoString) -> Option<&FunctionTemplateSignature>;

    fn custom_type(
        &self,
        name: &crate::plan::CustomTypeName,
    ) -> Option<&crate::plan::CustomTypeDefinition>;
}

#[derive(Default)]
struct LibraryEntryCounts {
    ints: usize,
    floats: usize,
    strings: usize,
    bit_arrays: usize,
    utf_codepoints: usize,
    customs: usize,
    bools: usize,
    nils: usize,
    tuples: usize,
    lists: usize,
}

impl ModuleBuilder {
    /// Plans every body in a typed module without requiring a `main` function.
    pub fn new(module: TypedModule) -> Result<Self, PlanError> {
        let public_functions = public_function_names(&module);
        let plan = crate::planner::plan_library_module(module)?;
        Ok(Self {
            inner: BindingBuilder::new(plan, public_functions),
        })
    }

    /// Plans every body in a resolved typed program without requiring `main`.
    ///
    /// Function selection remains limited to public functions in the selected
    /// root module. Imported modules provide its implementation closure.
    pub fn from_program(program: TypedProgram) -> Result<Self, PlanError> {
        let public_functions = public_function_names(program.root_typed_module());
        let plan = crate::planner::plan_library_program(program)?;
        Ok(Self {
            inner: BindingBuilder::new(plan, public_functions),
        })
    }

    /// Selects the first function and creates a non-empty binding owner.
    #[allow(private_bounds)]
    pub fn function<ArgumentsType, Return>(
        self,
        declaration: FunctionDeclaration<ArgumentsType, Return>,
    ) -> Result<(ModuleBindings, Function<ArgumentsType, Return>), BindingError>
    where
        ArgumentsType: Arguments,
        Return: ReturnValue,
    {
        self.inner
            .function(declaration)
            .map(|(inner, function)| (ModuleBindings { inner }, function))
    }
}

impl ModuleBindings {
    /// Validates and selects another named function for the shared execution.
    #[allow(private_bounds)]
    pub fn function<ArgumentsType, Return>(
        &mut self,
        declaration: FunctionDeclaration<ArgumentsType, Return>,
    ) -> Result<Function<ArgumentsType, Return>, BindingError>
    where
        ArgumentsType: Arguments,
        Return: ReturnValue,
    {
        self.inner.function(declaration)
    }

    /// Seals every selected function into one immutable execution.
    pub fn seal(self) -> Module {
        let BindingParts {
            plan,
            first,
            remaining,
            owner,
        } = self.inner.into_parts();
        let (execution, entries) = ExecutionPlan::from_library_plan(plan, first, remaining);
        Module::from_parts(execution, entries, owner)
    }
}

#[allow(private_bounds)]
impl<ArgumentsType, Return> FunctionDeclaration<ArgumentsType, Return>
where
    ArgumentsType: Arguments,
    Return: ReturnValue,
{
    /// Declares the exact Rust signature expected for a named Gleam function.
    pub fn new(name: impl Into<EcoString>) -> Self {
        Self {
            name: name.into(),
            marker: PhantomData,
        }
    }
}

impl<Plan: BindingPlan> BindingSource<Plan> {
    fn validate(
        &self,
        name: EcoString,
        expected: FunctionType,
        standard_variants: &[crate::plan::StandardVariant],
    ) -> Result<(EcoString, FunctionTemplateId), BindingError> {
        let Some(signature) = self.plan.function_signature(&name) else {
            return Err(BindingError::MissingFunction { name });
        };
        if !self.public_functions.contains(&name) {
            return Err(BindingError::NonPublicFunction { name });
        }
        if !signature.scheme().parameters().is_empty() {
            return Err(BindingError::GenericFunction { name });
        }
        let found = signature.shape().type_();
        if found != expected {
            return Err(BindingError::SignatureMismatch {
                name,
                expected,
                found,
            });
        }
        for variant in standard_variants {
            let type_name = variant.type_name();
            if !variant.has_exact_definition(self.plan.custom_type(&type_name)) {
                return Err(BindingError::standard_type_mismatch(name, type_name));
            }
        }

        Ok((name, signature.id()))
    }

    fn into_plan(self) -> Plan {
        self.plan
    }
}

impl<Plan: BindingPlan> BindingBuilder<Plan> {
    pub(super) fn new(plan: Plan, public_functions: HashSet<EcoString>) -> Self {
        Self {
            source: BindingSource {
                plan,
                public_functions,
            },
            owner: Arc::new(()),
        }
    }

    pub(super) fn function<ArgumentsType, Return>(
        self,
        declaration: FunctionDeclaration<ArgumentsType, Return>,
    ) -> Result<(Bindings<Plan>, Function<ArgumentsType, Return>), BindingError>
    where
        ArgumentsType: Arguments,
        Return: ReturnValue,
    {
        let expected = FunctionType::new(ArgumentsType::value_types(), Return::value_type());
        let input_variants = ArgumentsType::input_variants();
        let mut standard_variants = input_variants.clone();
        standard_variants.extend(Return::standard_variants());
        let (name, template) =
            self.source
                .validate(declaration.name, expected, &standard_variants)?;
        let mut counts = LibraryEntryCounts::default();
        let entry = LibraryEntry::new(
            template,
            Return::library_type(),
            input_variants,
            ArgumentsType::input_lists(),
        );
        let (slot, first) = counts.reserve(entry);
        let mut selected_names = HashSet::new();
        selected_names.insert(name.clone());
        let function = Function::new(name, slot, &self.owner);

        Ok((
            Bindings {
                source: self.source,
                selected_names,
                first,
                remaining: Vec::new(),
                counts,
                owner: self.owner,
            },
            function,
        ))
    }
}

impl<Plan: BindingPlan> Bindings<Plan> {
    pub(super) fn function<ArgumentsType, Return>(
        &mut self,
        declaration: FunctionDeclaration<ArgumentsType, Return>,
    ) -> Result<Function<ArgumentsType, Return>, BindingError>
    where
        ArgumentsType: Arguments,
        Return: ReturnValue,
    {
        let name = declaration.name;
        if self.selected_names.contains(&name) {
            return Err(BindingError::DuplicateFunction { name });
        }
        let expected = FunctionType::new(ArgumentsType::value_types(), Return::value_type());
        let input_variants = ArgumentsType::input_variants();
        let mut standard_variants = input_variants.clone();
        standard_variants.extend(Return::standard_variants());
        let (name, template) = self.source.validate(name, expected, &standard_variants)?;
        let entry = LibraryEntry::new(
            template,
            Return::library_type(),
            input_variants,
            ArgumentsType::input_lists(),
        );
        let (slot, entry) = self.counts.reserve(entry);
        self.selected_names.insert(name.clone());
        self.remaining.push(entry);

        Ok(Function::new(name, slot, &self.owner))
    }

    pub(super) fn into_parts(self) -> BindingParts<Plan> {
        BindingParts {
            plan: self.source.into_plan(),
            first: self.first,
            remaining: self.remaining,
            owner: self.owner,
        }
    }
}

impl BindingPlan for LibraryModulePlan {
    fn function_signature(&self, name: &EcoString) -> Option<&FunctionTemplateSignature> {
        self.functions()
            .iter()
            .find(|function| function.name() == name)
            .map(|function| function.signature())
    }

    fn custom_type(
        &self,
        name: &crate::plan::CustomTypeName,
    ) -> Option<&crate::plan::CustomTypeDefinition> {
        self.custom_type(name)
    }
}

impl<Profile: HostProfile> BindingPlan for HostedLibraryModulePlan<Profile> {
    fn function_signature(&self, name: &EcoString) -> Option<&FunctionTemplateSignature> {
        self.functions()
            .iter()
            .find(|function| function.name() == name)
            .map(|function| function.signature())
    }

    fn custom_type(
        &self,
        name: &crate::plan::CustomTypeName,
    ) -> Option<&crate::plan::CustomTypeDefinition> {
        self.custom_type(name)
    }
}

fn public_function_names(module: &TypedModule) -> HashSet<EcoString> {
    module
        .definitions
        .functions
        .iter()
        .filter(|function| function.publicity == Publicity::Public)
        .filter_map(|function| function.name.as_ref().map(|(_, name)| name.clone()))
        .collect()
}

impl LibraryEntryCounts {
    fn reserve(&mut self, entry: LibraryEntry) -> (usize, LibraryEntry) {
        let count = match entry.return_() {
            LibraryValueType::Int => &mut self.ints,
            LibraryValueType::Float => &mut self.floats,
            LibraryValueType::String => &mut self.strings,
            LibraryValueType::BitArray => &mut self.bit_arrays,
            LibraryValueType::UtfCodepoint => &mut self.utf_codepoints,
            LibraryValueType::Custom(_) => &mut self.customs,
            LibraryValueType::Bool => &mut self.bools,
            LibraryValueType::Nil => &mut self.nils,
            LibraryValueType::Tuple(_) => &mut self.tuples,
            LibraryValueType::List(_) => &mut self.lists,
        };
        let slot = *count;
        *count += 1;
        (slot, entry)
    }
}

#[cfg(test)]
mod tests {
    use super::{BindingError, FunctionDeclaration, ModuleBuilder};
    use crate::plan::{CustomType, CustomTypeName, StandardVariant};
    use crate::planner::UnsupportedFunctionReason;
    use crate::{
        FunctionType, ModuleSource, PackageSource, PlanError, ValueType, compile_typed_module,
        compile_typed_package_program, compile_typed_program,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;

    fn compile(source: &str) -> gleam_compiler_core::ast::TypedModule {
        compile_typed_module("library", "library.gleam", source).expect("source should compile")
    }

    #[test]
    fn selects_only_public_functions_from_the_program_root() {
        let program = compile_typed_program(
            "library",
            [
                ModuleSource::new(
                    "support",
                    "support.gleam",
                    r#"
pub fn selected(value: Int) { value + 1 }
pub fn support_only(value: String) { value }
"#,
                ),
                ModuleSource::new(
                    "library",
                    "library.gleam",
                    r#"
import support

pub fn selected(value: String) { value <> support.support_only(":root") }
"#,
                ),
            ],
        )
        .expect("program should compile");
        let builder = ModuleBuilder::from_program(program).expect("library program should plan");
        let (mut bindings, selected) = builder
            .function(FunctionDeclaration::<(EcoString,), EcoString>::new(
                "selected",
            ))
            .expect("same-named root function should bind");

        assert_eq!(
            bindings
                .function(FunctionDeclaration::<(EcoString,), EcoString>::new(
                    "support_only",
                ))
                .err(),
            Some(BindingError::MissingFunction {
                name: "support_only".into(),
            }),
        );
        let module = bindings.seal();
        assert_eq!(
            module.call(&selected, ("value".into(),), &mut Vec::new()),
            Ok("value:root".into()),
        );
    }

    #[test]
    fn rejects_an_invalid_first_selection_without_creating_bindings() {
        let builder = ModuleBuilder::new(compile("pub fn number(value: Int) { value }"))
            .expect("library should plan");

        assert_eq!(
            builder
                .function(FunctionDeclaration::<(EcoString,), EcoString>::new(
                    "missing",
                ))
                .err(),
            Some(BindingError::MissingFunction {
                name: "missing".into(),
            }),
        );
    }

    #[test]
    fn keeps_non_empty_bindings_unchanged_after_validation_failures() {
        let typed = compile(
            r#"
fn private(value: String) { value }
pub fn generic(value) { value }
pub fn number(value: Int) { value }
"#,
        );
        let builder = ModuleBuilder::new(typed).expect("library should plan");
        let (mut bindings, number) = builder
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("number"))
            .expect("first function should bind");

        assert_eq!(
            bindings
                .function(FunctionDeclaration::<(EcoString,), EcoString>::new(
                    "missing",
                ))
                .err(),
            Some(BindingError::MissingFunction {
                name: "missing".into(),
            }),
        );
        assert_eq!(
            bindings
                .function(FunctionDeclaration::<(EcoString,), EcoString>::new(
                    "private",
                ))
                .err(),
            Some(BindingError::NonPublicFunction {
                name: "private".into(),
            }),
        );
        assert_eq!(
            bindings
                .function(FunctionDeclaration::<(EcoString,), EcoString>::new(
                    "generic",
                ))
                .err(),
            Some(BindingError::GenericFunction {
                name: "generic".into(),
            }),
        );
        assert_eq!(
            bindings
                .function(FunctionDeclaration::<(EcoString,), EcoString>::new(
                    "number",
                ))
                .err(),
            Some(BindingError::DuplicateFunction {
                name: "number".into(),
            }),
        );

        let module = bindings.seal();
        assert_eq!(
            module.call(&number, (BigInt::from(7),), &mut Vec::new()),
            Ok(BigInt::from(7)),
        );
    }

    #[test]
    fn reports_an_exact_signature_mismatch_without_mutating_bindings() {
        let builder = ModuleBuilder::new(compile(
            r#"
pub fn number(value: Int) { value }
pub fn other_number(value: Int) { value }
pub fn text(value: String) { value }
"#,
        ))
        .expect("library should plan");
        let (mut bindings, number) = builder
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("number"))
            .expect("first function should bind");

        assert_eq!(
            bindings
                .function(FunctionDeclaration::<(EcoString,), EcoString>::new(
                    "other_number",
                ))
                .err(),
            Some(BindingError::SignatureMismatch {
                name: "other_number".into(),
                expected: FunctionType::new(vec![ValueType::String], ValueType::String),
                found: FunctionType::new(vec![ValueType::Int], ValueType::Int),
            }),
        );
        let text = bindings
            .function(FunctionDeclaration::<(EcoString,), EcoString>::new("text"))
            .expect("a valid selection should follow the mismatch");

        let module = bindings.seal();
        assert_eq!(
            module.call(&number, (BigInt::from(11),), &mut Vec::new()),
            Ok(BigInt::from(11)),
        );
        assert_eq!(
            module.call(&text, (EcoString::from("kept"),), &mut Vec::new()),
            Ok(EcoString::from("kept")),
        );
    }

    #[test]
    fn rejects_a_lookalike_option_before_sealing() {
        let builder = ModuleBuilder::new(compile(
            r#"
pub type Option(value) { Some(value) None }

pub fn number(value: Int) { value }
pub fn fake_option(value: Option(Int)) { value }
pub fn text(value: String) { value }
"#,
        ))
        .expect("lookalike library should plan");
        let (mut bindings, number) = builder
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("number"))
            .expect("first function should bind");

        let error = bindings
            .function(FunctionDeclaration::<(Option<BigInt>,), Option<BigInt>>::new("fake_option"))
            .err()
            .expect("lookalike Option should not bind as the standard type");
        let expected_option =
            ValueType::Custom(StandardVariant::Option.custom_type(vec![ValueType::Int]));
        let found_option = ValueType::Custom(CustomType::new(
            CustomTypeName::new("geam".into(), "library".into(), "Option".into()),
            vec![ValueType::Int],
        ));
        assert_eq!(
            error,
            BindingError::SignatureMismatch {
                name: "fake_option".into(),
                expected: FunctionType::new(vec![expected_option.clone()], expected_option,),
                found: FunctionType::new(vec![found_option.clone()], found_option),
            },
        );

        let text = bindings
            .function(FunctionDeclaration::<(EcoString,), EcoString>::new("text"))
            .expect("a valid selection should follow the mismatch");
        let module = bindings.seal();
        assert_eq!(
            module.call(&number, (BigInt::from(7),), &mut Vec::new()),
            Ok(BigInt::from(7)),
        );
        assert_eq!(
            module.call(&text, ("kept".into(),), &mut Vec::new()),
            Ok("kept".into()),
        );
    }

    #[test]
    fn rejects_a_nonstandard_option_definition_before_sealing() {
        let program = compile_typed_package_program(
            "application",
            "library",
            [
                PackageSource::new(
                    "gleam_stdlib",
                    Vec::<EcoString>::new(),
                    [ModuleSource::new(
                        "gleam/option",
                        "gleam_stdlib/src/gleam/option.gleam",
                        "pub type Option(value) { None Some(value) }",
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["gleam_stdlib"],
                    [ModuleSource::new(
                        "library",
                        "src/library.gleam",
                        r#"
import gleam/option.{type Option}

pub fn number(value: Int) { value }
pub fn optional(value: Option(Int)) { value }
"#,
                    )],
                ),
            ],
        )
        .expect("nonstandard Option package should still type-check");
        let builder = ModuleBuilder::from_program(program).expect("library should plan");
        let (mut bindings, number) = builder
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("number"))
            .expect("unrelated scalar should bind");

        assert_eq!(
            bindings
                .function(
                    FunctionDeclaration::<(Option<BigInt>,), Option<BigInt>>::new("optional",)
                )
                .err(),
            Some(BindingError::StandardTypeMismatch {
                name: "optional".into(),
                package: "gleam_stdlib".into(),
                module: "gleam/option".into(),
                type_name: "Option".into(),
            }),
        );

        let module = bindings.seal();
        assert_eq!(
            module.call(&number, (BigInt::from(8),), &mut Vec::new()),
            Ok(BigInt::from(8)),
        );
    }

    #[test]
    fn rejects_an_unsupported_body_before_selection() {
        let typed = compile(
            r#"
pub fn identity(value: String) { value }

@external(erlang, "unsupported", "call")
fn unsupported(value: String) -> String
"#,
        );

        assert_eq!(
            ModuleBuilder::new(typed).err(),
            Some(PlanError::UnsupportedFunction {
                name: "unsupported".into(),
                reason: UnsupportedFunctionReason::External,
            }),
        );
    }

    #[test]
    fn rejects_an_unsupported_dependency_body_before_selection() {
        let program = compile_typed_program(
            "library",
            [
                ModuleSource::new(
                    "support",
                    "support.gleam",
                    r#"
pub fn keep(value: String) { value }

@external(erlang, "unsupported", "call")
fn unsupported(value: String) -> String
"#,
                ),
                ModuleSource::new(
                    "library",
                    "library.gleam",
                    r#"
import support

pub fn identity(value: String) { support.keep(value) }
"#,
                ),
            ],
        )
        .expect("program should compile");

        assert_eq!(
            ModuleBuilder::from_program(program).err(),
            Some(PlanError::UnsupportedFunction {
                name: "unsupported".into(),
                reason: UnsupportedFunctionReason::External,
            }),
        );
    }

    #[test]
    fn shares_a_specialization_reached_by_another_selected_entry() {
        let typed = compile(
            r#"
pub fn increment(value: Int) { value + 1 }

pub fn twice_incremented(value: Int) { increment(value) * 2 }
"#,
        );
        let builder = ModuleBuilder::new(typed).expect("library should plan");
        let (mut bindings, twice) = builder
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new(
                "twice_incremented",
            ))
            .expect("first function should bind");
        let increment = bindings
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("increment"))
            .expect("second function should bind");
        let module = bindings.seal();

        assert_eq!(
            module.call(&twice, (BigInt::from(20),), &mut Vec::new()),
            Ok(BigInt::from(42)),
        );
        assert_eq!(
            module.call(&increment, (BigInt::from(41),), &mut Vec::new()),
            Ok(BigInt::from(42)),
        );
    }
}
