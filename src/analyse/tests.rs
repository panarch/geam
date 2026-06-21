use super::{AnalyseErrorType, NameKind, analyse_module};
use crate::ast::{CallArg, Definition, Pattern, Publicity, Statement, TypedExpr, TypedModule};
use crate::test_support::{analyse_result, io_imports, parse};
use crate::type_::{
    ImportableModules, ModuleInterface, Type, TypeConstructor, ValueConstructor, fn_, int, list,
    named, nil, string, tuple, var,
};
use ecow::EcoString;
use std::collections::HashMap;
use std::sync::Arc;

fn analyse(src: &str) -> TypedModule {
    let module = analyse_result(src).expect("module should analyse");
    assert_no_infer_in_module(&module);
    assert_module_interface_matches_public_function_signatures(&module);
    module
}

fn assert_invalid_name(src: &str, kind: NameKind, expected_name: &str) {
    let error = analyse_module(parse(src), &ImportableModules::new())
        .expect_err("invalid Gleam name should be rejected");

    assert_eq!(
        error.error,
        AnalyseErrorType::InvalidName {
            kind,
            name: expected_name.into()
        }
    );
}

fn helper_imports() -> ImportableModules {
    let mut imports = ImportableModules::new();
    let mut helper = ModuleInterface::new("helper");
    helper.values.insert(
        "make_adder".into(),
        ValueConstructor::module_fn(Publicity::Public, fn_(vec![int()], fn_(vec![int()], int()))),
    );
    imports.insert("helper".into(), helper);
    imports
}

fn option_imports() -> ImportableModules {
    let mut imports = ImportableModules::new();
    let mut option = ModuleInterface::new("option");
    let option_type = named("option", "Option", vec![var("a")]);
    option.types.insert(
        "Option".into(),
        TypeConstructor {
            publicity: Publicity::Public,
            parameters: vec!["a".into()],
            type_: option_type.clone(),
        },
    );
    option.values.insert(
        "Some".into(),
        ValueConstructor::record(Publicity::Public, fn_(vec![var("a")], option_type.clone()))
            .with_parameter_labels(vec![None]),
    );
    option.values.insert(
        "None".into(),
        ValueConstructor::record(Publicity::Public, option_type),
    );
    imports.insert("option".into(), option);
    imports
}

fn uppercase_function_imports() -> ImportableModules {
    let mut imports = ImportableModules::new();
    let mut option = ModuleInterface::new("option");
    option.values.insert(
        "Some".into(),
        ValueConstructor::module_fn(Publicity::Public, fn_(vec![], nil())),
    );
    imports.insert("option".into(), option);
    imports
}

fn function_type(module: &TypedModule, name: &str) -> Arc<Type> {
    module
        .type_info
        .values
        .get(name)
        .unwrap_or_else(|| panic!("{name} should be exported"))
        .type_
        .clone()
}

fn function_return_type(module: &TypedModule, name: &str) -> Arc<Type> {
    module
        .definitions
        .functions
        .iter()
        .find(|function| {
            function
                .name
                .as_ref()
                .is_some_and(|(_, function_name)| function_name.as_str() == name)
        })
        .unwrap_or_else(|| panic!("{name} should be analysed"))
        .return_type
        .clone()
}

fn function_argument_type(
    module: &TypedModule,
    function_name: &str,
    argument_name: &str,
) -> Arc<Type> {
    module
        .definitions
        .functions
        .iter()
        .find(|function| {
            function
                .name
                .as_ref()
                .is_some_and(|(_, name)| name.as_str() == function_name)
        })
        .unwrap_or_else(|| panic!("{function_name} should be analysed"))
        .arguments
        .iter()
        .find(|argument| argument.name.1.as_str() == argument_name)
        .unwrap_or_else(|| panic!("{argument_name} should be analysed"))
        .type_
        .clone()
}

fn first_assignment_pattern_type(module: &TypedModule, function_name: &str) -> Arc<Type> {
    let function = module
        .definitions
        .functions
        .iter()
        .find(|function| {
            function
                .name
                .as_ref()
                .is_some_and(|(_, name)| name.as_str() == function_name)
        })
        .unwrap_or_else(|| panic!("{function_name} should be analysed"));
    let Statement::Assignment(assignment) = &function.body[0] else {
        panic!("{function_name} first statement should be an assignment");
    };
    let Pattern::Variable { type_, .. } = &assignment.pattern else {
        panic!("{function_name} first assignment should use a variable pattern");
    };
    type_.clone()
}

fn first_expression<'a>(module: &'a TypedModule, function_name: &str) -> &'a TypedExpr {
    let function = module
        .definitions
        .functions
        .iter()
        .find(|function| {
            function
                .name
                .as_ref()
                .is_some_and(|(_, name)| name.as_str() == function_name)
        })
        .unwrap_or_else(|| panic!("{function_name} should be analysed"));
    let Statement::Expression(expression) = &function.body[0] else {
        panic!("{function_name} first statement should be an expression");
    };
    expression
}

fn assert_no_infer_in_module(module: &TypedModule) {
    for constructor in module.type_info.types.values() {
        assert_no_infer_type(&constructor.type_);
    }
    for constructor in module.type_info.values.values() {
        assert_no_infer_type(&constructor.type_);
    }
    for custom_type in &module.definitions.custom_types {
        assert_no_infer_type(&custom_type.type_);
        for constructor in &custom_type.constructors {
            for argument in &constructor.arguments {
                assert_no_infer_type(&argument.type_);
            }
        }
    }
    for alias in &module.definitions.type_aliases {
        assert_no_infer_type(&alias.type_);
    }
    for function in &module.definitions.functions {
        for argument in &function.arguments {
            assert_no_infer_type(&argument.type_);
        }
        assert_no_infer_type(&function.return_type);
        for statement in &function.body {
            assert_no_infer_statement(statement);
        }
    }
}

fn assert_module_interface_matches_public_function_signatures(module: &TypedModule) {
    for function in &module.definitions.functions {
        if !function.publicity.is_public() {
            continue;
        }
        let Some((_, name)) = &function.name else {
            continue;
        };
        let signature = fn_(
            function
                .arguments
                .iter()
                .map(|argument| argument.type_.clone())
                .collect(),
            function.return_type.clone(),
        );
        let interface_type = module
            .type_info
            .values
            .get(name)
            .unwrap_or_else(|| panic!("{name} should be exported"))
            .type_
            .clone();
        assert_eq!(interface_type, signature);
    }
}

fn assert_no_infer_type(type_: &Arc<Type>) {
    match type_.as_ref() {
        Type::Infer(_) => panic!("typed module should not contain unresolved inference types"),
        Type::Named { arguments, .. } => {
            for argument in arguments {
                assert_no_infer_type(argument);
            }
        }
        Type::Fn { arguments, return_ } => {
            for argument in arguments {
                assert_no_infer_type(argument);
            }
            assert_no_infer_type(return_);
        }
        Type::Tuple { elements } => {
            for element in elements {
                assert_no_infer_type(element);
            }
        }
        Type::Var { .. } => {}
    }
}

fn assert_no_infer_statement(statement: &Statement<Arc<Type>, TypedExpr>) {
    match statement {
        Statement::Expression(expression) => assert_no_infer_expr(expression),
        Statement::Assignment(assignment) => {
            assert_no_infer_pattern(&assignment.pattern);
            assert_no_infer_expr(&assignment.value);
        }
    }
}

fn assert_no_infer_expr(expression: &TypedExpr) {
    assert_no_infer_type(&expression.type_());
    match expression {
        TypedExpr::Block { statements, .. } => {
            for statement in statements {
                assert_no_infer_statement(statement);
            }
        }
        TypedExpr::List { elements, .. } | TypedExpr::Tuple { elements, .. } => {
            for element in elements {
                assert_no_infer_expr(element);
            }
        }
        TypedExpr::Call { fun, arguments, .. } => {
            assert_no_infer_expr(fun);
            assert_no_infer_expr_call_args(arguments);
        }
        TypedExpr::BinOp { left, right, .. } => {
            assert_no_infer_expr(left);
            assert_no_infer_expr(right);
        }
        TypedExpr::Case {
            subjects, clauses, ..
        } => {
            for subject in subjects {
                assert_no_infer_expr(subject);
            }
            for clause in clauses {
                for pattern in &clause.pattern {
                    assert_no_infer_pattern(pattern);
                }
                for alternative in &clause.alternative_patterns {
                    for pattern in alternative {
                        assert_no_infer_pattern(pattern);
                    }
                }
                if let Some(guard) = &clause.guard {
                    assert_no_infer_expr(&guard.expression);
                }
                assert_no_infer_expr(&clause.then);
            }
        }
        TypedExpr::FieldAccess { container, .. } => assert_no_infer_expr(container),
        TypedExpr::TupleIndex { tuple, .. } => assert_no_infer_expr(tuple),
        TypedExpr::NegateBool { value, .. } | TypedExpr::NegateInt { value, .. } => {
            assert_no_infer_expr(value);
        }
        TypedExpr::Int { .. }
        | TypedExpr::Float { .. }
        | TypedExpr::String { .. }
        | TypedExpr::Var { .. }
        | TypedExpr::ModuleSelect { .. } => {}
    }
}

fn assert_no_infer_expr_call_args(arguments: &[CallArg<TypedExpr>]) {
    for argument in arguments {
        assert_no_infer_expr(&argument.value);
    }
}

fn assert_no_infer_pattern(pattern: &Pattern<Arc<Type>>) {
    match pattern {
        Pattern::Variable { type_, .. } | Pattern::Discard { type_, .. } => {
            assert_no_infer_type(type_);
        }
        Pattern::Assign { pattern, .. } => assert_no_infer_pattern(pattern),
        Pattern::List {
            elements, type_, ..
        } => {
            assert_no_infer_type(type_);
            for element in elements {
                assert_no_infer_pattern(element);
            }
        }
        Pattern::Constructor {
            arguments, type_, ..
        } => {
            assert_no_infer_type(type_);
            for argument in arguments {
                assert_no_infer_pattern(&argument.value);
            }
        }
        Pattern::Tuple { elements, .. } => {
            for element in elements {
                assert_no_infer_pattern(element);
            }
        }
        Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::StringPrefix { .. } => {}
    }
}

#[test]
fn analyse_module_records_imports_and_exports() {
    let module = analyse_module(
        parse(
            r#"
import gleam/io as logger

pub type UserId = Int

pub fn main(message: String) {
  logger.println(message)
}
"#,
        ),
        &io_imports(),
    )
    .expect("module should analyse");

    assert_eq!(module.definitions.imports.len(), 1);
    assert_eq!(module.definitions.type_aliases.len(), 1);
    assert_eq!(module.definitions.functions.len(), 1);
    assert!(module.type_info.types.contains_key("UserId"));
    assert!(module.type_info.values.contains_key("main"));
}

#[test]
fn analyse_module_exports_type_aliases() {
    let module = analyse(
        r#"
pub type UserId = Int

pub fn main(value: UserId) {
  value
}
"#,
    );

    let alias = module
        .type_info
        .types
        .get("UserId")
        .expect("type alias should be exported");
    assert_eq!(alias.publicity, Publicity::Public);
    assert_eq!(alias.parameters, Vec::<EcoString>::new());
    assert_eq!(alias.type_, int());
    assert_eq!(function_type(&module, "main"), fn_(vec![int()], int()));
}

#[test]
fn analyse_module_exports_custom_types_and_constructors() {
    let module = analyse(
        r#"
pub type Option(a) {
  Some(a)
  None
}
"#,
    );

    let option = named("main", "Option", vec![var("a")]);
    let custom_type = module
        .type_info
        .types
        .get("Option")
        .expect("custom type should be exported");
    assert_eq!(custom_type.publicity, Publicity::Public);
    assert_eq!(custom_type.parameters, vec![EcoString::from("a")]);
    assert_eq!(custom_type.type_, option);
    assert_eq!(
        function_type(&module, "Some"),
        fn_(vec![var("a")], named("main", "Option", vec![var("a")]))
    );
    assert_eq!(
        function_type(&module, "None"),
        named("main", "Option", vec![var("a")])
    );
}

#[test]
fn analyse_module_accepts_imported_record_constructor_pattern() {
    let module = analyse_module(
        parse(
            r#"
import option

pub fn main(value: option.Option(Int)) {
  case value {
    option.Some(inner) -> inner
    option.None -> 0
  }
}
"#,
        ),
        &option_imports(),
    )
    .expect("imported record constructors should be usable in patterns");

    assert_no_infer_in_module(&module);
    assert_eq!(function_return_type(&module, "main"), int());
}

#[test]
fn analyse_module_rejects_module_function_as_constructor_pattern() {
    let error = analyse_module(
        parse(
            r#"
import option

pub fn main(value: Nil) {
  case value {
    option.Some -> 1
    _ -> 0
  }
}
"#,
        ),
        &uppercase_function_imports(),
    )
    .expect_err("module functions should not be accepted as constructor patterns");

    assert_eq!(
        error.error,
        AnalyseErrorType::NotConstructor {
            name: "Some".into()
        }
    );
}

#[test]
fn analyse_module_registers_custom_type_header_before_constructor_arguments() {
    let module = analyse(
        r#"
pub type Box {
  Box(Box)
}
"#,
    );

    assert_eq!(
        function_type(&module, "Box"),
        fn_(
            vec![named("main", "Box", vec![])],
            named("main", "Box", vec![])
        )
    );
}

#[test]
fn analyse_module_preregisters_annotated_functions_before_bodies() {
    let module = analyse(
        r#"
pub fn main(value: Int) {
  later(value)
}

pub fn later(value: Int) -> Int {
  value
}
"#,
    );

    assert_eq!(function_return_type(&module, "main"), int());
    assert_eq!(function_type(&module, "later"), fn_(vec![int()], int()));
}

#[test]
fn analyse_module_preregisters_unannotated_functions_before_bodies() {
    let module = analyse(
        r#"
pub fn main(value: Int) {
  later(value)
}

pub fn later(value) {
  value
}
"#,
    );

    assert_eq!(function_return_type(&module, "main"), int());
    assert_eq!(function_return_type(&module, "later"), var("a"));
    assert_eq!(
        function_type(&module, "later"),
        fn_(vec![var("a")], var("a"))
    );
}

#[test]
fn analyse_module_resolves_forward_call_chains() {
    let module = analyse(
        r#"
pub fn main(value: Int) {
  middle(value)
}

pub fn middle(value: Int) {
  final(value)
}

pub fn final(value: Int) {
  value
}
"#,
    );

    assert_eq!(function_return_type(&module, "main"), int());
    assert_eq!(function_return_type(&module, "middle"), int());
    assert_eq!(function_return_type(&module, "final"), int());
}

#[test]
fn analyse_module_resolves_generic_forward_calls() {
    let module = analyse(
        r#"
pub fn main(value: Int) {
  id(value)
}

pub fn id(value: a) {
  value
}
"#,
    );

    assert_eq!(function_return_type(&module, "main"), int());
    assert_eq!(function_type(&module, "main"), fn_(vec![int()], int()));
    assert_eq!(function_return_type(&module, "id"), var("a"));
    assert_eq!(function_type(&module, "id"), fn_(vec![var("a")], var("a")));
}

#[test]
fn analyse_module_normalises_source_type_variable_names() {
    let module = analyse(
        r#"
pub fn id(value: b) -> b {
  value
}
"#,
    );

    assert_eq!(function_return_type(&module, "id"), var("a"));
    assert_eq!(function_type(&module, "id"), fn_(vec![var("a")], var("a")));
}

#[test]
fn analyse_module_reflects_constraints_on_function_type_variables() {
    let module = analyse(
        r#"
pub fn main(value: a) {
  value + 1
}
"#,
    );

    assert_eq!(function_argument_type(&module, "main", "value"), int());
    assert_eq!(function_return_type(&module, "main"), int());
    assert_eq!(function_type(&module, "main"), fn_(vec![int()], int()));
}

#[test]
fn analyse_module_shares_function_and_let_annotation_variables() {
    let module = analyse(
        r#"
pub fn main(value: a) {
  let other: a = 1
  value
}
"#,
    );

    assert_eq!(function_argument_type(&module, "main", "value"), int());
    assert_eq!(first_assignment_pattern_type(&module, "main"), int());
    assert_eq!(function_return_type(&module, "main"), int());
    assert_eq!(function_type(&module, "main"), fn_(vec![int()], int()));
}

#[test]
fn analyse_module_rejects_let_annotation_type_variable_return_mismatch() {
    let error = analyse_module(
        parse(
            r#"
pub fn main() -> String {
  let value: a = 1
  value
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("let annotation type variables should keep inferred constraints");

    assert!(matches!(error.error, AnalyseErrorType::TypeMismatch { .. }));
}

#[test]
fn analyse_module_rejects_reused_let_annotation_type_variable_mismatch() {
    let error = analyse_module(
        parse(
            r#"
pub fn main() {
  let pair: #(a, a) = #(1, "x")
  pair
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("same type variable name in one function body should share constraints");

    assert!(matches!(error.error, AnalyseErrorType::TypeMismatch { .. }));
}

#[test]
fn analyse_module_resolves_mutually_recursive_return_types_after_group_analysis() {
    let module = analyse(
        r#"
pub fn a(value: Int) {
  case value {
    0 -> 0
    _ -> b(value)
  }
}

pub fn b(value: Int) {
  a(value)
}
"#,
    );

    assert_eq!(function_return_type(&module, "a"), int());
    assert_eq!(function_return_type(&module, "b"), int());
    assert_eq!(function_type(&module, "a"), fn_(vec![int()], int()));
    assert_eq!(function_type(&module, "b"), fn_(vec![int()], int()));
}

#[test]
fn analyse_module_generalises_unconstrained_recursive_return_type() {
    let module = analyse(
        r#"
pub fn forever(value: Int) {
  forever(value)
}
"#,
    );

    assert_eq!(
        function_type(&module, "forever"),
        fn_(vec![int()], var("a"))
    );
}

#[test]
fn analyse_module_prioritises_signature_type_variables_before_body_locals() {
    let module = analyse(
        r#"
pub fn forever(value: Int) {
  let xs: List(a) = []
  forever(value)
}
"#,
    );

    assert_eq!(
        function_type(&module, "forever"),
        fn_(vec![int()], var("a"))
    );
    assert_eq!(function_return_type(&module, "forever"), var("a"));
    assert_eq!(
        first_assignment_pattern_type(&module, "forever"),
        list(var("b"))
    );
}

#[test]
fn analyse_module_treats_import_alias_field_access_as_module_select() {
    let module = analyse_module(
        parse(
            r#"
import gleam/io

pub type User {
  User
}

pub fn main(io: User) {
  io.println("hi")
}
"#,
        ),
        &io_imports(),
    )
    .expect("imported module alias should take field access syntax");

    let TypedExpr::Call { fun, type_, .. } = first_expression(&module, "main") else {
        panic!("main should call imported println");
    };
    assert_eq!(type_, &nil());
    assert!(matches!(
        fun.as_ref(),
        TypedExpr::ModuleSelect {
            module_name,
            module_alias,
            label,
            ..
        } if module_name == "gleam/io" && module_alias == "io" && label == "println"
    ));
}

#[test]
fn analyse_module_treats_import_alias_before_local_let_in_field_access() {
    let module = analyse_module(
        parse(
            r#"
import gleam/io

pub fn main() {
  let io = 1
  io.println("hi")
}
"#,
        ),
        &io_imports(),
    )
    .expect("imported module alias should take field access syntax");

    let function = module
        .definitions
        .functions
        .iter()
        .find(|function| {
            function
                .name
                .as_ref()
                .is_some_and(|(_, name)| name.as_str() == "main")
        })
        .expect("main should be analysed");
    let Statement::Expression(TypedExpr::Call { fun, .. }) = &function.body[1] else {
        panic!("main should call imported println after the assignment");
    };
    assert!(matches!(
        fun.as_ref(),
        TypedExpr::ModuleSelect {
            module_name,
            module_alias,
            label,
            ..
        } if module_name == "gleam/io" && module_alias == "io" && label == "println"
    ));
}

#[test]
fn analyse_module_reports_unknown_module_value_for_import_alias_field_access() {
    let error = analyse_module(
        parse(
            r#"
import gleam/io

pub fn main() {
  let io = 1
  io.missing
}
"#,
        ),
        &io_imports(),
    )
    .expect_err("imported module alias field access should look up module values");

    assert_eq!(
        error.error,
        AnalyseErrorType::UnknownModuleValue {
            module: "gleam/io".into(),
            value: "missing".into()
        }
    )
}

#[test]
fn function_dependency_collection_ignores_field_access_container() {
    let module = parse(
        r#"
import gleam/io

fn io(value) {
  value
}

pub fn main() {
  io.println("hi")
}
"#,
    );
    let functions = module
        .definitions
        .into_iter()
        .filter_map(|definition| match definition {
            Definition::Function(function) => Some(function),
            _ => None,
        })
        .collect::<Vec<_>>();
    let function_names = functions
        .iter()
        .enumerate()
        .filter_map(|(index, function)| {
            function
                .name
                .as_ref()
                .map(|(_, name)| (name.clone(), index))
        })
        .collect::<HashMap<_, _>>();
    let main = functions
        .iter()
        .position(|function| {
            function
                .name
                .as_ref()
                .is_some_and(|(_, name)| name.as_str() == "main")
        })
        .expect("main should be present");

    assert_eq!(
        super::function_dependencies(&functions[main], &function_names),
        Vec::<usize>::new()
    );
}

#[test]
fn analyse_module_rejects_import_interface_with_unresolved_infer() {
    let mut imports = ImportableModules::new();
    let mut bad = ModuleInterface::new("bad");
    bad.values.insert(
        "value".into(),
        ValueConstructor::module_fn(Publicity::Public, crate::type_::infer(0)),
    );
    imports.insert("bad".into(), bad);

    let error = analyse_module(
        parse(
            r#"
import bad

pub fn main() {
  bad.value
}
"#,
        ),
        &imports,
    )
    .expect_err("import interfaces should not expose unresolved infer types");

    assert!(matches!(
        error.error,
        AnalyseErrorType::UnresolvedType { .. }
    ));
}

#[test]
fn analyse_module_rejects_import_record_constructor_without_exported_type() {
    let mut imports = ImportableModules::new();
    let mut bad = ModuleInterface::new("bad");
    bad.values.insert(
        "Bad".into(),
        ValueConstructor::record(Publicity::Public, int()),
    );
    imports.insert("bad".into(), bad);

    let error = analyse_module(
        parse(
            r#"
import bad

pub fn main() {
  bad.Bad
}
"#,
        ),
        &imports,
    )
    .expect_err("record constructors in an import interface should return an exported type");

    assert_eq!(
        error.error,
        AnalyseErrorType::InvalidModuleInterface {
            module: "bad".into()
        }
    );
}

#[test]
fn analyse_module_rejects_import_record_constructor_with_unbound_argument_variable() {
    let mut imports = ImportableModules::new();
    let mut bad = ModuleInterface::new("bad");
    let option_type = named("bad", "Option", vec![var("a")]);
    bad.types.insert(
        "Option".into(),
        TypeConstructor {
            publicity: Publicity::Public,
            parameters: vec!["a".into()],
            type_: option_type.clone(),
        },
    );
    bad.values.insert(
        "Some".into(),
        ValueConstructor::record(Publicity::Public, fn_(vec![var("b")], option_type))
            .with_parameter_labels(vec![None]),
    );
    imports.insert("bad".into(), bad);

    let error = analyse_module(
        parse(
            r#"
import bad

pub fn main() {
  bad.Some(1)
}
"#,
        ),
        &imports,
    )
    .expect_err("record constructor arguments should use return type variables");

    assert_eq!(
        error.error,
        AnalyseErrorType::InvalidModuleInterface {
            module: "bad".into()
        }
    );
}

#[test]
fn analyse_module_rejects_import_record_constructor_for_alias_like_type() {
    let mut imports = ImportableModules::new();
    let mut bad = ModuleInterface::new("bad");
    bad.types.insert(
        "Bad".into(),
        TypeConstructor {
            publicity: Publicity::Public,
            parameters: vec!["a".into()],
            type_: list(var("a")),
        },
    );
    bad.values.insert(
        "Bad".into(),
        ValueConstructor::record(
            Publicity::Public,
            fn_(vec![var("a")], named("bad", "Bad", vec![var("a")])),
        )
        .with_parameter_labels(vec![None]),
    );
    imports.insert("bad".into(), bad);

    let error = analyse_module(
        parse(
            r#"
import bad

pub fn main() {
  bad.Bad(1)
}
"#,
        ),
        &imports,
    )
    .expect_err("record constructors should point at custom type-shaped exports");

    assert_eq!(
        error.error,
        AnalyseErrorType::InvalidModuleInterface {
            module: "bad".into()
        }
    );
}

#[test]
fn analyse_module_rejects_import_type_constructor_free_type_variable() {
    let mut imports = ImportableModules::new();
    let mut bad = ModuleInterface::new("bad");
    bad.types.insert(
        "Box".into(),
        TypeConstructor {
            publicity: Publicity::Public,
            parameters: vec!["a".into()],
            type_: named("bad", "Box", vec![var("b")]),
        },
    );
    imports.insert("bad".into(), bad);

    let error = analyse_module(
        parse(
            r#"
import bad

pub fn main() {
  Nil
}
"#,
        ),
        &imports,
    )
    .expect_err("import type constructors should not contain undeclared type variables");

    assert_eq!(
        error.error,
        AnalyseErrorType::InvalidModuleInterface {
            module: "bad".into()
        }
    );
}

#[test]
fn analyse_module_rejects_import_type_constructor_duplicate_parameter() {
    let mut imports = ImportableModules::new();
    let mut bad = ModuleInterface::new("bad");
    bad.types.insert(
        "Box".into(),
        TypeConstructor {
            publicity: Publicity::Public,
            parameters: vec!["a".into(), "a".into()],
            type_: named("bad", "Box", vec![var("a")]),
        },
    );
    imports.insert("bad".into(), bad);

    let error = analyse_module(
        parse(
            r#"
import bad

pub fn main() {
  Nil
}
"#,
        ),
        &imports,
    )
    .expect_err("import type constructors should not repeat type parameters");

    assert_eq!(
        error.error,
        AnalyseErrorType::InvalidModuleInterface {
            module: "bad".into()
        }
    );
}

#[test]
fn analyse_module_rejects_import_type_constructor_unused_parameter() {
    let mut imports = ImportableModules::new();
    let mut bad = ModuleInterface::new("bad");
    bad.types.insert(
        "Box".into(),
        TypeConstructor {
            publicity: Publicity::Public,
            parameters: vec!["a".into(), "b".into()],
            type_: named("bad", "Box", vec![var("a")]),
        },
    );
    imports.insert("bad".into(), bad);

    let error = analyse_module(
        parse(
            r#"
import bad

pub fn main() {
  Nil
}
"#,
        ),
        &imports,
    )
    .expect_err("import type constructor parameters should all appear in the type");

    assert_eq!(
        error.error,
        AnalyseErrorType::InvalidModuleInterface {
            module: "bad".into()
        }
    );
}

#[test]
fn analyse_module_rejects_import_record_constructor_type_parameter_order_mismatch() {
    let mut imports = ImportableModules::new();
    let mut bad = ModuleInterface::new("bad");
    bad.types.insert(
        "Pair".into(),
        TypeConstructor {
            publicity: Publicity::Public,
            parameters: vec!["a".into(), "b".into()],
            type_: named("bad", "Pair", vec![var("b"), var("a")]),
        },
    );
    bad.values.insert(
        "Pair".into(),
        ValueConstructor::record(
            Publicity::Public,
            fn_(
                vec![var("a"), var("b")],
                named("bad", "Pair", vec![var("a"), var("b")]),
            ),
        )
        .with_parameter_labels(vec![None, None]),
    );
    imports.insert("bad".into(), bad);

    let error = analyse_module(
        parse(
            r#"
import bad

pub fn main() {
  bad.Pair(1, "two")
}
"#,
        ),
        &imports,
    )
    .expect_err("record constructor type exports should preserve parameter order");

    assert_eq!(
        error.error,
        AnalyseErrorType::InvalidModuleInterface {
            module: "bad".into()
        }
    );
}

#[test]
fn analyse_module_rejects_import_record_constructor_return_parameter_order_mismatch() {
    let mut imports = ImportableModules::new();
    let mut bad = ModuleInterface::new("bad");
    bad.types.insert(
        "Pair".into(),
        TypeConstructor {
            publicity: Publicity::Public,
            parameters: vec!["a".into(), "b".into()],
            type_: named("bad", "Pair", vec![var("a"), var("b")]),
        },
    );
    bad.values.insert(
        "Pair".into(),
        ValueConstructor::record(
            Publicity::Public,
            fn_(
                vec![var("a"), var("b")],
                named("bad", "Pair", vec![var("b"), var("a")]),
            ),
        )
        .with_parameter_labels(vec![None, None]),
    );
    imports.insert("bad".into(), bad);

    let error = analyse_module(
        parse(
            r#"
import bad

pub fn main() {
  bad.Pair(1, "two")
}
"#,
        ),
        &imports,
    )
    .expect_err("record constructor return types should preserve parameter order");

    assert_eq!(
        error.error,
        AnalyseErrorType::InvalidModuleInterface {
            module: "bad".into()
        }
    );
}

#[test]
fn analyse_module_rejects_import_value_label_arity_mismatch() {
    let mut imports = ImportableModules::new();
    let mut bad = ModuleInterface::new("bad");
    bad.values.insert(
        "run".into(),
        ValueConstructor::module_fn(Publicity::Public, fn_(vec![string()], nil()))
            .with_parameter_labels(vec![Some("first".into()), Some("second".into())]),
    );
    imports.insert("bad".into(), bad);

    let error = analyse_module(
        parse(
            r#"
import bad

pub fn main() {
  bad.run("x")
}
"#,
        ),
        &imports,
    )
    .expect_err("import value labels should be empty or match the function arity");

    assert_eq!(
        error.error,
        AnalyseErrorType::InvalidModuleInterface {
            module: "bad".into()
        }
    );
}

#[test]
fn analyse_module_rejects_import_module_fn_with_non_function_type() {
    let mut imports = ImportableModules::new();
    let mut bad = ModuleInterface::new("bad");
    bad.values.insert(
        "value".into(),
        ValueConstructor::module_fn(Publicity::Public, int()),
    );
    imports.insert("bad".into(), bad);

    let error = analyse_module(
        parse(
            r#"
import bad

pub fn main() {
  bad.value
}
"#,
        ),
        &imports,
    )
    .expect_err("module function constructors should have function types");

    assert_eq!(
        error.error,
        AnalyseErrorType::InvalidModuleInterface {
            module: "bad".into()
        }
    );
}

#[test]
fn analyse_module_resolves_forward_type_aliases() {
    let module = analyse(
        r#"
pub type UserId = Id
pub type Id = Int

pub fn main(value: UserId) {
  value
}
"#,
    );

    let alias = module
        .type_info
        .types
        .get("UserId")
        .expect("forward type alias should be exported");
    assert_eq!(alias.type_, int());
    assert_eq!(function_type(&module, "main"), fn_(vec![int()], int()));
}

#[test]
fn analyse_module_rejects_recursive_type_aliases() {
    let error = analyse_module(
        parse(
            r#"
pub type A = B
pub type B = A
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("recursive type aliases should be rejected");

    assert!(matches!(
        error.error,
        AnalyseErrorType::RecursiveTypeAlias { .. }
    ));
}

#[test]
fn analyse_module_rejects_unused_type_alias_parameter() {
    let error = analyse_module(
        parse(
            r#"
pub type Mapper(a, b) = fn(a) -> List(Int)
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("type alias parameters must be used by the alias body");

    assert_eq!(
        error.error,
        AnalyseErrorType::UnusedTypeAliasParameter { name: "b".into() }
    );
}

#[test]
fn analyse_module_accepts_nested_type_alias_parameters() {
    let module = analyse(
        r#"
pub type Mapper(a, b) = fn(a) -> List(b)

pub fn main(callback: Mapper(Int, String), value: Int) {
  callback(value)
}
"#,
    );

    assert_eq!(function_return_type(&module, "main"), list(string()));
}

#[test]
fn analyse_module_rejects_duplicate_type_names() {
    let error = analyse_module(
        parse(
            r#"
pub type UserId = Int
pub type UserId = String
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("duplicate type aliases should be rejected");

    assert_eq!(
        error.error,
        AnalyseErrorType::DuplicateName {
            name: "UserId".into(),
            previous_location: crate::ast::SrcSpan::new(10, 16)
        }
    );
}

#[test]
fn analyse_module_rejects_duplicate_value_names() {
    let error = analyse_module(
        parse(
            r#"
pub fn main() {
  1
}

pub fn main() {
  2
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("duplicate functions should be rejected");

    assert_eq!(
        error.error,
        AnalyseErrorType::DuplicateName {
            name: "main".into(),
            previous_location: crate::ast::SrcSpan::new(8, 12)
        }
    );
}

#[test]
fn analyse_module_rejects_duplicate_constructor_names() {
    let error = analyse_module(
        parse(
            r#"
pub type First {
  Same
}

pub type Second {
  Same
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("duplicate constructors should be rejected");

    assert_eq!(
        error.error,
        AnalyseErrorType::DuplicateName {
            name: "Same".into(),
            previous_location: crate::ast::SrcSpan::new(20, 24)
        }
    );
}

#[test]
fn analyse_module_rejects_duplicate_function_argument_names() {
    let error = analyse_module(
        parse(
            r#"
pub fn main(value: Int, value: String) {
  value
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("duplicate function argument names should be rejected");

    assert!(matches!(
        error.error,
        AnalyseErrorType::DuplicateName { name, .. } if name == "value"
    ));
}

#[test]
fn analyse_module_rejects_duplicate_type_alias_parameters() {
    let error = analyse_module(
        parse(
            r#"
pub type Pair(a, a) = #(a, a)
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("duplicate type alias parameters should be rejected");

    assert!(matches!(
        error.error,
        AnalyseErrorType::DuplicateName { name, .. } if name == "a"
    ));
}

#[test]
fn analyse_module_rejects_duplicate_custom_type_parameters() {
    let error = analyse_module(
        parse(
            r#"
pub type Box(a, a) {
  Box(a)
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("duplicate custom type parameters should be rejected");

    assert!(matches!(
        error.error,
        AnalyseErrorType::DuplicateName { name, .. } if name == "a"
    ));
}

#[test]
fn analyse_module_rejects_duplicate_let_pattern_bindings() {
    let error = analyse_module(
        parse(
            r#"
pub fn main(pair: #(Int, Int)) {
  let #(x, x) = pair
  x
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("duplicate let pattern bindings should be rejected");

    assert_eq!(
        error.error,
        AnalyseErrorType::DuplicateVarInPattern { name: "x".into() }
    );
}

#[test]
fn analyse_module_rejects_duplicate_import_aliases() {
    let mut imports = ImportableModules::new();
    imports.insert("gleam/io".into(), ModuleInterface::new("gleam/io"));
    imports.insert("gleam/string".into(), ModuleInterface::new("gleam/string"));

    let error = analyse_module(
        parse(
            r#"
import gleam/io as logger
import gleam/string as logger

pub fn main() {
  1
}
"#,
        ),
        &imports,
    )
    .expect_err("duplicate import aliases should be rejected");

    assert!(matches!(
        error.error,
        AnalyseErrorType::DuplicateName { name, .. } if name == "logger"
    ));
}

#[test]
fn analyse_module_rejects_invalid_function_name() {
    assert_invalid_name(
        r#"
pub fn doStuff() {
  Nil
}
"#,
        NameKind::Function,
        "doStuff",
    );
}

#[test]
fn analyse_module_rejects_invalid_function_argument_name() {
    assert_invalid_name(
        r#"
pub fn add(numA: Int, num_b: Int) {
  num_b
}
"#,
        NameKind::Argument,
        "numA",
    );
}

#[test]
fn analyse_module_rejects_invalid_type_and_constructor_names() {
    assert_invalid_name(
        "type Boxed_value { Box(Int) }",
        NameKind::Type,
        "Boxed_value",
    );
    assert_invalid_name(
        "type MyType { Int_Value(Int) }",
        NameKind::CustomTypeVariant,
        "Int_Value",
    );
    assert_invalid_name("type Fancy_Bool = Bool", NameKind::TypeAlias, "Fancy_Bool");
}

#[test]
fn analyse_module_rejects_invalid_labels() {
    assert_invalid_name(
        "type IntWrapper { IntWrapper(innerInt: Int) }",
        NameKind::Label,
        "innerInt",
    );
    assert_invalid_name(
        r#"
pub type User {
  User(name: String)
}

pub fn main() {
  User(userName: "Lucy")
}
"#,
        NameKind::Label,
        "userName",
    );
}

#[test]
fn analyse_module_rejects_invalid_type_variable_names() {
    assert_invalid_name(
        "type Wrapper(innerType) {}",
        NameKind::TypeVariable,
        "innerType",
    );
    assert_invalid_name(
        r#"
pub fn identity(value: someType) {
  value
}
"#,
        NameKind::TypeVariable,
        "someType",
    );
}

#[test]
fn analyse_module_rejects_invalid_pattern_names() {
    assert_invalid_name(
        r#"
pub fn main() {
  let theAnswer = 42
  theAnswer
}
"#,
        NameKind::Variable,
        "theAnswer",
    );
    assert_invalid_name(
        r#"
pub fn main() {
  let _boringNumber = 72
  Nil
}
"#,
        NameKind::Discard,
        "_boringNumber",
    );
    assert_invalid_name(
        r#"
pub fn main(text: String) {
  case text {
    "prefix" as thePrefix <> _suffix -> thePrefix
    _ -> text
  }
}
"#,
        NameKind::Variable,
        "thePrefix",
    );
}

#[test]
fn analyse_module_rejects_invalid_import_alias_name() {
    assert_invalid_name(
        "import gleam/io as ioLogger",
        NameKind::Variable,
        "ioLogger",
    );
}

#[test]
fn analyse_module_exports_only_public_types_and_values() {
    let module = analyse(
        r#"
type Secret = Int

type Hidden {
  Hidden
}

fn helper(value: Int) {
  value
}

pub fn main(value: Int) {
  helper(value)
}
"#,
    );

    assert!(!module.type_info.types.contains_key("Secret"));
    assert!(!module.type_info.types.contains_key("Hidden"));
    assert!(!module.type_info.values.contains_key("Hidden"));
    assert!(!module.type_info.values.contains_key("helper"));
    assert!(module.type_info.values.contains_key("main"));
}

#[test]
fn analyse_module_rejects_public_function_returning_private_type() {
    let error = analyse_module(
        parse(
            r#"
type PrivateType {
  PrivateType
}

pub fn leak_type() {
  PrivateType
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("public function return type should not leak a private type");

    assert_eq!(
        error.error,
        AnalyseErrorType::PrivateTypeLeak {
            type_: named("main", "PrivateType", vec![])
        }
    );
}

#[test]
fn analyse_module_rejects_public_function_argument_private_type() {
    let error = analyse_module(
        parse(
            r#"
type PrivateType {
  PrivateType
}

pub fn leak_type(value: PrivateType) {
  1
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("public function argument type should not leak a private type");

    assert_eq!(
        error.error,
        AnalyseErrorType::PrivateTypeLeak {
            type_: named("main", "PrivateType", vec![])
        }
    );
}

#[test]
fn analyse_module_rejects_private_type_leak_from_private_helper() {
    let error = analyse_module(
        parse(
            r#"
type PrivateType {
  PrivateType
}

fn go() {
  PrivateType
}

pub fn leak_type() {
  go()
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("public function inferred return type should not leak a private type");

    assert_eq!(
        error.error,
        AnalyseErrorType::PrivateTypeLeak {
            type_: named("main", "PrivateType", vec![])
        }
    );
}

#[test]
fn analyse_module_rejects_nested_private_type_leak() {
    let error = analyse_module(
        parse(
            r#"
type PrivateType {
  PrivateType
}

fn go() {
  PrivateType
}

pub fn leak_type() {
  [go()]
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("public function inferred nested return type should not leak a private type");

    assert_eq!(
        error.error,
        AnalyseErrorType::PrivateTypeLeak {
            type_: named("main", "PrivateType", vec![])
        }
    );
}

#[test]
fn analyse_module_rejects_public_constructor_argument_private_type() {
    let error = analyse_module(
        parse(
            r#"
type PrivateType {
  PrivateType
}

pub type LeakType {
  Variant(PrivateType)
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("public constructor argument should not leak a private type");

    assert_eq!(
        error.error,
        AnalyseErrorType::PrivateTypeLeak {
            type_: named("main", "PrivateType", vec![])
        }
    );
}

#[test]
fn analyse_module_rejects_private_imported_values() {
    let mut imports = ImportableModules::new();
    let mut io = ModuleInterface::new("gleam/io");
    io.values.insert(
        "secret".into(),
        ValueConstructor::module_fn(Publicity::Private, fn_(vec![string()], nil())),
    );
    imports.insert("gleam/io".into(), io);

    let error = analyse_module(
        parse(
            r#"
import gleam/io

pub fn main(message: String) {
  io.secret(message)
}
"#,
        ),
        &imports,
    )
    .expect_err("private imported value should not be visible");

    assert_eq!(
        error.error,
        AnalyseErrorType::UnknownModuleValue {
            module: "gleam/io".into(),
            value: "secret".into()
        }
    );
}

#[test]
fn analyse_module_rejects_private_imported_types() {
    let mut imports = ImportableModules::new();
    let mut secret = ModuleInterface::new("gleam/secret");
    secret.types.insert(
        "Secret".into(),
        TypeConstructor {
            publicity: Publicity::Private,
            parameters: vec![],
            type_: named("gleam/secret", "Secret", vec![]),
        },
    );
    imports.insert("gleam/secret".into(), secret);

    let error = analyse_module(
        parse(
            r#"
import gleam/secret

pub fn main(value: secret.Secret) {
  value
}
"#,
        ),
        &imports,
    )
    .expect_err("private imported type should not be visible");

    assert_eq!(
        error.error,
        AnalyseErrorType::UnknownType {
            name: "Secret".into()
        }
    );
}

#[test]
fn analyse_module_rejects_undeclared_type_alias_parameter() {
    let error = analyse_module(
        parse(
            r#"
pub type Bad = List(a)
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("type alias parameters must be declared");

    assert_eq!(
        error.error,
        AnalyseErrorType::UnknownType { name: "a".into() }
    );
}

#[test]
fn analyse_module_rejects_undeclared_custom_type_parameter() {
    let error = analyse_module(
        parse(
            r#"
pub type Box {
  Box(a)
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("custom type parameters must be declared");

    assert_eq!(
        error.error,
        AnalyseErrorType::UnknownType { name: "a".into() }
    );
}

#[test]
fn analyse_module_accepts_alternative_patterns_with_same_bindings() {
    let module = analyse(
        r#"
pub type Thing {
  A(Int)
  B(Int)
}

pub fn main(thing: Thing) {
  case thing {
    A(value) | B(value) -> value
  }
}
"#,
    );

    assert_eq!(function_return_type(&module, "main"), int());
}

#[test]
fn analyse_module_rejects_extra_alternative_pattern_binding() {
    let error = analyse_module(
        parse(
            r#"
pub type Thing {
  A(Int)
  B(Int)
}

pub fn main(thing: Thing) {
  case thing {
    A(x) | B(y) -> x
  }
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("alternative patterns must bind the same variables");

    assert_eq!(
        error.error,
        AnalyseErrorType::ExtraVarInAlternativePattern { name: "y".into() }
    );
}

#[test]
fn analyse_module_rejects_missing_alternative_pattern_binding() {
    let error = analyse_module(
        parse(
            r#"
pub type Thing {
  A(Int)
  B
}

pub fn main(thing: Thing) {
  case thing {
    A(x) | B -> x
  }
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("alternative patterns must bind the same variables");

    assert_eq!(
        error.error,
        AnalyseErrorType::MissingVarInAlternativePattern { name: "x".into() }
    );
}

#[test]
fn analyse_module_infers_missing_function_argument_annotations() {
    let module = analyse(
        r#"
pub fn main(value) {
  value + 1
}
"#,
    );

    assert_eq!(function_argument_type(&module, "main", "value"), int());
    assert_eq!(function_return_type(&module, "main"), int());
}

#[test]
fn analyse_module_rejects_unknown_import_before_function_body() {
    let error = analyse_module(
        parse(
            r#"
import gleam/io

pub fn main() {
  missing
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("module should reject unknown import");

    assert_eq!(
        error.error,
        AnalyseErrorType::UnknownModule {
            module: "gleam/io".into()
        }
    );
}

#[test]
fn analyse_module_instantiates_generic_constructors_at_each_use_site() {
    let module = analyse(
        r#"
pub type Box(a) {
  Empty
}

pub fn main() -> #(Box(Int), Box(String)) {
  #(Empty, Empty)
}
"#,
    );

    assert_eq!(
        function_return_type(&module, "main"),
        tuple(vec![
            named("main", "Box", vec![int()]),
            named("main", "Box", vec![string()])
        ])
    );
}

#[test]
fn analyse_module_uses_constructor_argument_labels() {
    let module = analyse(
        r#"
pub type User {
  User(name: String, age: Int)
}

pub fn main() {
  User(age: 42, name: "Tae")
}

pub fn name(user: User) {
  case user {
    User(age: age, name: name) -> name
  }
}
"#,
    );

    assert_eq!(
        function_return_type(&module, "main"),
        named("main", "User", vec![])
    );
    assert_eq!(function_return_type(&module, "name"), string());
}

#[test]
fn analyse_module_reorders_labelled_constructor_arguments() {
    let module = analyse(
        r#"
pub type User {
  User(name: String, age: Int)
}

pub fn main() {
  User(age: 42, name: "Tae")
}
"#,
    );

    let TypedExpr::Call { arguments, .. } = first_expression(&module, "main") else {
        panic!("main should call the constructor");
    };
    let labels = arguments
        .iter()
        .map(|argument| argument.label.as_ref().map(|(_, label)| label.as_str()))
        .collect::<Vec<_>>();

    assert_eq!(labels, vec![Some("name"), Some("age")]);
    assert_eq!(arguments[0].value.type_(), string());
    assert_eq!(arguments[1].value.type_(), int());
}

#[test]
fn analyse_module_rejects_unknown_constructor_argument_label() {
    let error = analyse_module(
        parse(
            r#"
pub type User {
  User(name: String, age: Int)
}

pub fn main() {
  User(foo: "Tae", age: 42)
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("unknown constructor call labels should be rejected");

    assert_eq!(
        error.error,
        AnalyseErrorType::UnknownArgumentLabel {
            label: "foo".into()
        }
    );
}

#[test]
fn analyse_module_rejects_unlabelled_argument_after_labelled_constructor_call() {
    let error = analyse_module(
        parse(
            r#"
pub type User {
  User(name: String, age: Int)
}

pub fn main() {
  User(name: "Tae", 42)
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("unlabelled constructor call arguments should not follow labelled arguments");

    assert_eq!(
        error.error,
        AnalyseErrorType::UnlabelledArgumentAfterLabelled
    );
}

#[test]
fn analyse_module_rejects_labelled_plain_function_call() {
    let error = analyse_module(
        parse(
            r#"
pub fn id(value: Int) {
  value
}

pub fn main() {
  id(value: 1)
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("labelled function calls without parameter label metadata should be rejected");

    assert_eq!(
        error.error,
        AnalyseErrorType::UnknownArgumentLabel {
            label: "value".into()
        }
    );
}

#[test]
fn analyse_module_rejects_empty_case() {
    let error = analyse_module(
        parse(
            r#"
pub fn main() {
  case True {
  }
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("case expressions should have at least one clause");

    assert_eq!(error.error, AnalyseErrorType::EmptyCase);
}

#[test]
fn analyse_module_rejects_unknown_constructor_pattern_label() {
    let error = analyse_module(
        parse(
            r#"
pub type User {
  User(name: String, age: Int)
}

pub fn main(user: User) {
  case user {
    User(foo: name, age: age) -> name
  }
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("unknown constructor pattern labels should be rejected");

    assert_eq!(
        error.error,
        AnalyseErrorType::UnknownArgumentLabel {
            label: "foo".into()
        }
    );
}

#[test]
fn analyse_module_rejects_unlabelled_argument_after_labelled_constructor_pattern() {
    let error = analyse_module(
        parse(
            r#"
pub type User {
  User(name: String, age: Int)
}

pub fn main(user: User) {
  case user {
    User(name: name, _) -> name
  }
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("unlabelled constructor pattern arguments should not follow labelled arguments");

    assert_eq!(
        error.error,
        AnalyseErrorType::UnlabelledArgumentAfterLabelled
    );
}

#[test]
fn analyse_module_rejects_duplicate_constructor_call_label() {
    let error = analyse_module(
        parse(
            r#"
pub type User {
  User(name: String, age: Int)
}

pub fn main() {
  User(name: "Tae", name: "Kim")
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("duplicate constructor call labels should be rejected");

    assert_eq!(
        error.error,
        AnalyseErrorType::DuplicateArgumentLabel {
            label: "name".into()
        }
    );
}

#[test]
fn analyse_module_rejects_duplicate_constructor_field_labels() {
    let error = analyse_module(
        parse(
            r#"
pub type User {
  User(name: String, name: Int)
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("duplicate constructor field labels should be rejected");

    assert!(matches!(
        error.error,
        AnalyseErrorType::DuplicateName { name, .. } if name == "name"
    ));
}

#[test]
fn analyse_module_rejects_unlabelled_constructor_field_after_labelled_field() {
    let error = analyse_module(
        parse(
            r#"
pub type User {
  User(name: String, Int)
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("unlabelled constructor fields should not follow labelled fields");

    assert_eq!(
        error.error,
        AnalyseErrorType::UnlabelledArgumentAfterLabelled
    );
}

#[test]
fn analyse_module_list_pattern_constrains_inferred_subject_type() {
    let module = analyse(
        r#"
pub fn main(value: a) {
  case value {
    [x] -> x + 1
  }
}
"#,
    );

    assert_eq!(
        function_argument_type(&module, "main", "value"),
        list(int())
    );
    assert_eq!(function_return_type(&module, "main"), int());
}

#[test]
fn analyse_module_tuple_pattern_constrains_inferred_subject_type() {
    let module = analyse(
        r#"
pub fn main(value: a) {
  case value {
    #(x, y) -> x + y
  }
}
"#,
    );

    assert_eq!(
        function_argument_type(&module, "main", "value"),
        tuple(vec![int(), int()])
    );
    assert_eq!(function_return_type(&module, "main"), int());
}

#[test]
fn analyse_module_tuple_index_uses_resolved_inferred_type() {
    let module = analyse(
        r#"
pub fn main(value) {
  let pair: #(Int, Int) = value
  value.0
}
"#,
    );

    assert_eq!(
        function_argument_type(&module, "main", "value"),
        tuple(vec![int(), int()])
    );
    assert_eq!(function_return_type(&module, "main"), int());
}

#[test]
fn analyse_module_rejects_tuple_index_on_unbound_type() {
    let error = analyse_module(
        parse(
            r#"
pub fn main(value) {
  value.0
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("unbound value should not be inferred as a tuple from index access alone");

    assert!(matches!(error.error, AnalyseErrorType::NotTuple { .. }));
}

#[test]
fn analyse_module_uses_annotations_for_empty_list_expressions() {
    let module = analyse(
        r#"
pub fn assigned() {
  let values: List(Int) = []
  values
}

pub fn returned() -> List(String) {
  []
}
"#,
    );

    assert_eq!(function_return_type(&module, "assigned"), list(int()));
    assert_eq!(
        first_assignment_pattern_type(&module, "assigned"),
        list(int())
    );
    assert_eq!(function_return_type(&module, "returned"), list(string()));
}

#[test]
fn analyse_module_uses_case_result_annotation_for_empty_list_branches() {
    let module = analyse(
        r#"
pub fn main() -> List(Int) {
  case True {
    True -> []
    False -> []
  }
}
"#,
    );

    assert_eq!(function_return_type(&module, "main"), list(int()));
}

#[test]
fn analyse_module_inserts_pipeline_value_as_first_call_argument() {
    let module = analyse(
        r#"
pub fn add(left: Int, right: Int) {
  left + right
}

pub fn main(value: Int) {
  value |> add(1)
}
"#,
    );

    assert_eq!(function_return_type(&module, "main"), int());
}

#[test]
fn analyse_module_applies_pipeline_value_to_returned_function() {
    let module = analyse_module(
        parse(
            r#"
import helper

pub fn main(value: Int) {
  value |> helper.make_adder(1)
}
"#,
        ),
        &helper_imports(),
    )
    .expect("pipeline callback should analyse");

    assert_eq!(function_return_type(&module, "main"), int());
}

#[test]
fn analyse_module_uses_case_clause_result_type_for_empty_list_branches() {
    let module = analyse(
        r#"
pub fn first_empty() {
  case True {
    True -> []
    False -> [1]
  }
}

pub fn later_empty() {
  case True {
    True -> [1]
    False -> []
  }
}
"#,
    );

    assert_eq!(function_return_type(&module, "first_empty"), list(int()));
    assert_eq!(function_return_type(&module, "later_empty"), list(int()));
}

#[test]
fn analyse_module_generalises_empty_list_returns() {
    let module = analyse(
        r#"
pub fn empty() {
  []
}

pub fn nested() {
  [[], []]
}
"#,
    );

    assert_eq!(function_return_type(&module, "empty"), list(var("a")));
    assert_eq!(
        function_return_type(&module, "nested"),
        list(list(var("a")))
    );
}

#[test]
fn analyse_module_constrains_empty_list_from_later_use() {
    let module = analyse(
        r#"
pub fn takes(xs: List(Int)) {
  xs
}

pub fn assigned() {
  let xs = []
  takes(xs)
}

pub fn piped() {
  [] |> takes
}

pub fn case_piped() {
  case True {
    True -> []
    False -> []
  } |> takes
}
"#,
    );

    assert_eq!(function_return_type(&module, "assigned"), list(int()));
    assert_eq!(function_return_type(&module, "piped"), list(int()));
    assert_eq!(function_return_type(&module, "case_piped"), list(int()));
}

#[test]
fn analyse_module_constrains_unannotated_function_argument_calls() {
    let module = analyse(
        r#"
pub fn apply(f, x) {
  f(x)
}

pub fn inc(value: Int) {
  value + 1
}

pub fn use_apply(value: Int) {
  apply(inc, value)
}
"#,
    );

    assert_eq!(
        function_type(&module, "apply"),
        fn_(vec![fn_(vec![var("a")], var("b")), var("a")], var("b"))
    );
    assert_eq!(first_expression(&module, "apply").type_(), var("b"));
    assert_eq!(function_return_type(&module, "use_apply"), int());
}

#[test]
fn analyse_module_rejects_case_result_annotation_mismatch() {
    let error = analyse_module(
        parse(
            r#"
pub fn main() -> List(Int) {
  case True {
    True -> []
    False -> ["bad"]
  }
}
"#,
        ),
        &ImportableModules::new(),
    )
    .expect_err("case branch results should match the expected result annotation");

    assert!(matches!(error.error, AnalyseErrorType::TypeMismatch { .. }));
}

#[test]
fn analyse_module_generalises_contextless_case_with_only_empty_list_branches() {
    let module = analyse(
        r#"
pub fn main() {
  case True {
    True -> []
    False -> []
  }
}
"#,
    );

    assert_eq!(function_return_type(&module, "main"), list(var("a")));
}

#[test]
fn analyse_module_generalises_empty_list_without_context() {
    let module = analyse(
        r#"
pub fn main() {
  []
}
"#,
    );

    assert_eq!(function_return_type(&module, "main"), list(var("a")));
}
