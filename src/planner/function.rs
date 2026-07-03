mod return_body;

use self::return_body::function_return_expr;
use crate::plan::{
    CaptureArg, FunctionPlan, Param, ParamBinding, ReturnExpr, RuntimeFunctionId, Step, ValueType,
};
use crate::planner::context::{AnonymousFunctions, FunctionInfo, FunctionParam, PlanContext};
use crate::planner::error::{
    InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError, UnsupportedFunctionReason,
};
use crate::planner::statement::plan_steps_and_return;
use ecow::EcoString;
use gleam_core::ast::{TypedFunction, TypedStatement};
use std::collections::HashMap;
use vec1::Vec1;

pub(super) struct PlannedFunctionBody {
    pub(super) params: Vec<Param>,
    pub(super) steps: Vec<Step>,
    pub(super) return_: ReturnExpr,
    pub(super) captures: Vec<CaptureArg>,
}

pub(super) fn plan_function(
    info: FunctionInfo,
    module_name: &EcoString,
    functions: &HashMap<EcoString, FunctionInfo>,
    function: TypedFunction,
    anonymous_functions: &mut AnonymousFunctions,
) -> Result<FunctionPlan, PlanError> {
    let name = function_name(&function)?;

    if function.external_erlang.is_some() || function.external_javascript.is_some() {
        return Err(PlanError::UnsupportedFunction {
            name,
            reason: UnsupportedFunctionReason::External,
        });
    }

    let mut context = PlanContext::new(module_name, functions, anonymous_functions);
    let params = define_params(&info.params, &mut context);
    let planned = plan_steps_and_return(
        function.body,
        &mut context,
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::FunctionShape {
                name: name.clone(),
                reason: InvalidFunctionShapeReason::EmptyBody,
            },
        },
    )?;
    let return_ = function_return_expr(
        &name,
        &info.return_type(),
        &info.runtime_id,
        planned.return_,
    )?;

    Ok(FunctionPlan::new(
        info.id,
        name,
        params,
        planned.steps,
        return_,
    ))
}

pub(super) fn plan_anonymous_function_body(
    name: &EcoString,
    return_type: &ValueType,
    runtime_id: &RuntimeFunctionId,
    params: &[FunctionParam],
    captures: Vec<crate::planner::context::CaptureBinding>,
    body: Vec1<TypedStatement>,
    context: &mut PlanContext<'_>,
) -> Result<PlannedFunctionBody, PlanError> {
    let params = define_params(params, context);
    let captures = context.define_captures(captures)?;
    let planned = crate::planner::statement::plan_non_empty_steps_and_return(body, context)?;
    let return_ = function_return_expr(name, return_type, runtime_id, planned.return_)?;

    Ok(PlannedFunctionBody {
        params,
        steps: planned.steps,
        return_,
        captures,
    })
}

pub(super) fn anonymous_function_plan(
    info: FunctionInfo,
    name: EcoString,
    planned: PlannedFunctionBody,
) -> FunctionPlan {
    FunctionPlan::new(
        info.id,
        name,
        planned.params,
        planned.steps,
        planned.return_,
    )
}

fn define_params(params: &[FunctionParam], context: &mut PlanContext<'_>) -> Vec<Param> {
    params
        .iter()
        .map(|param| match &param.binding {
            ParamBinding::Named(name) => {
                context.define_existing_param(name.clone(), &param.local);
                Param::named(param.local.clone(), name.clone())
            }
            ParamBinding::Discard => Param::discard(param.local.clone()),
        })
        .collect()
}

pub(super) fn function_name(function: &TypedFunction) -> Result<EcoString, PlanError> {
    match &function.name {
        Some((_, name)) => Ok(name.clone()),
        None => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::FunctionShape {
                name: "<anonymous>".into(),
                reason: InvalidFunctionShapeReason::Anonymous,
            },
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolLocalId, FunctionFunctionId, FunctionId, FunctionType, IntFunctionFunctionId,
        IntFunctionId, IntLocalId, LocalId, NilLocalId, RuntimeFunctionId, StringLocalId,
        ValueType,
    };
    use crate::planner::context::FunctionInfo;
    use crate::planner::dsl::{
        bool_, bool_arg, bool_function_ref, bool_function_return_block,
        bool_function_return_bool_case, bool_function_return_expr, bool_function_return_int_case,
        bool_function_return_string_case, bool_function_return_tail_call, bool_return_tail_call,
        call_bool, call_int, call_int_function, call_int_returning_function, function,
        function_function_ref, function_function_return_block, function_function_return_expr,
        function_function_return_int_case, function_function_return_string_case,
        function_function_return_tail_call, function_ref, int, int_arg, int_function_arg,
        int_function_call_arg, int_function_ref, int_function_return_block,
        int_function_return_bool_case, int_function_return_expr, int_function_return_int_case,
        int_function_return_string_case, int_function_return_tail_call, int_return_block,
        int_return_bool_case, int_return_expr, int_return_int_case, int_return_tail_call,
        let_int_function_step, let_int_step, local_bool, local_int, local_int_function, local_nil,
        local_string, module, module_with_anonymous, nil, nil_arg, nil_function_ref,
        nil_function_return_block, nil_function_return_bool_case, nil_function_return_expr,
        nil_function_return_int_case, nil_function_return_string_case,
        nil_function_return_tail_call, nil_return_tail_call, return_bool_function,
        return_function_function, return_int_function, return_nil_function, return_string_function,
        string, string_arg, string_function_ref, string_function_return_block,
        string_function_return_bool_case, string_function_return_expr,
        string_function_return_int_case, string_function_return_string_case,
        string_function_return_tail_call, string_return_tail_call,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, compile_minimal_module, expect_plan_error};
    use crate::planner::{
        InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError, UnsupportedArgumentReason,
        UnsupportedFunctionReason,
    };

    #[test]
    fn plan_final_direct_call_as_tail_call() {
        let actual = plan_module(compile(
            r#"
fn add(a: Int, b: Int) {
  a + b
}

pub fn main() {
  add(1, 2)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_tail_call(1, [int_arg(0, int(1)), int_arg(1, int(2))]),
            ),
            [
                function("add", local_int(0, "a").add_int(local_int(1, "b")))
                    .param_int(0, "a")
                    .param_int(1, "b"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_block_case_branches_preserve_tail_call() {
        let actual = plan_module(compile(
            r#"
fn count_down(n: Int, acc: Int) {
  {
    case n {
      0 -> acc
      _ -> count_down(n - 1, acc + 1)
    }
  }
}

pub fn main() {
  count_down(1, 0)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_tail_call(1, [int_arg(0, int(1)), int_arg(1, int(0))]),
            ),
            [function(
                "count_down",
                int_return_block(
                    [],
                    int_return_int_case(
                        local_int(0, "n"),
                        [(0, int_return_expr(local_int(1, "acc")))],
                        int_return_tail_call(
                            1,
                            [
                                int_arg(0, local_int(0, "n").sub_int(int(1))),
                                int_arg(1, local_int(1, "acc").add_int(int(1))),
                            ],
                        ),
                    ),
                ),
            )
            .param_int(0, "n")
            .param_int(1, "acc")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_function_return_family_block_int_case_fallbacks_as_tail_calls() {
        let actual = plan_module(compile(
            r#"
fn int_identity(value: Int) {
  value
}

fn string_identity(value: String) {
  value
}

fn bool_identity(value: Bool) {
  value
}

fn nil_identity(value: Nil) {
  value
}

fn get_int(n: Int) {
  {
    case n {
      0 -> int_identity
      _ -> get_int(n - 1)
    }
  }
}

fn get_string(n: Int) {
  {
    case n {
      0 -> string_identity
      _ -> get_string(n - 1)
    }
  }
}

fn get_bool(n: Int) {
  {
    case n {
      0 -> bool_identity
      _ -> get_bool(n - 1)
    }
  }
}

fn get_nil(n: Int) {
  {
    case n {
      0 -> nil_identity
      _ -> get_nil(n - 1)
    }
  }
}

fn get_getter(n: Int) {
  {
    case n {
      0 -> get_int
      _ -> get_getter(n - 1)
    }
  }
}

pub fn main() {
  get_int(0)(1)
}
"#,
        ))
        .expect("source should plan");
        let int_to_int = function_type([ValueType::Int], ValueType::Int);
        let string_to_string = function_type([ValueType::String], ValueType::String);
        let bool_to_bool = function_type([ValueType::Bool], ValueType::Bool);
        let nil_to_nil = function_type([ValueType::Nil], ValueType::Nil);
        let int_to_int_function = function_type(
            [ValueType::Int],
            ValueType::Function(Box::new(int_to_int.clone())),
        );
        let expected = module(
            "main",
            function(
                "main",
                call_int_function(
                    call_int_returning_function(0, [int_arg(0, int(0))], int_to_int.clone()),
                    [int_function_call_arg(0, int(1))],
                ),
            ),
            [
                function("int_identity", local_int(0, "value")).param_int(0, "value"),
                function("string_identity", local_string(0, "value")).param_string(0, "value"),
                function("bool_identity", local_bool(0, "value")).param_bool(0, "value"),
                function("nil_identity", local_nil(0, "value")).param_nil(0, "value"),
                function(
                    "get_int",
                    return_int_function(
                        int_to_int.clone(),
                        int_function_return_block(
                            [],
                            int_function_return_int_case(
                                local_int(0, "n"),
                                [(
                                    0,
                                    int_function_return_expr(int_function_ref(
                                        1,
                                        [LocalId::Int(IntLocalId(0))],
                                    )),
                                )],
                                int_function_return_tail_call(
                                    0,
                                    [int_arg(0, local_int(0, "n").sub_int(int(1)))],
                                ),
                            ),
                        ),
                    ),
                )
                .param_int(0, "n"),
                function(
                    "get_string",
                    return_string_function(
                        string_to_string.clone(),
                        string_function_return_block(
                            [],
                            string_function_return_int_case(
                                local_int(0, "n"),
                                [(
                                    0,
                                    string_function_return_expr(string_function_ref(
                                        0,
                                        [LocalId::String(StringLocalId(0))],
                                    )),
                                )],
                                string_function_return_tail_call(
                                    0,
                                    [int_arg(0, local_int(0, "n").sub_int(int(1)))],
                                ),
                            ),
                        ),
                    ),
                )
                .param_int(0, "n"),
                function(
                    "get_bool",
                    return_bool_function(
                        bool_to_bool.clone(),
                        bool_function_return_block(
                            [],
                            bool_function_return_int_case(
                                local_int(0, "n"),
                                [(
                                    0,
                                    bool_function_return_expr(bool_function_ref(
                                        0,
                                        [LocalId::Bool(BoolLocalId(0))],
                                    )),
                                )],
                                bool_function_return_tail_call(
                                    0,
                                    [int_arg(0, local_int(0, "n").sub_int(int(1)))],
                                ),
                            ),
                        ),
                    ),
                )
                .param_int(0, "n"),
                function(
                    "get_nil",
                    return_nil_function(
                        nil_to_nil.clone(),
                        nil_function_return_block(
                            [],
                            nil_function_return_int_case(
                                local_int(0, "n"),
                                [(
                                    0,
                                    nil_function_return_expr(nil_function_ref(
                                        0,
                                        [LocalId::Nil(NilLocalId(0))],
                                    )),
                                )],
                                nil_function_return_tail_call(
                                    0,
                                    [int_arg(0, local_int(0, "n").sub_int(int(1)))],
                                ),
                            ),
                        ),
                    ),
                )
                .param_int(0, "n"),
                function(
                    "get_getter",
                    return_function_function(
                        int_to_int_function.clone(),
                        function_function_return_block(
                            [],
                            function_function_return_int_case(
                                local_int(0, "n"),
                                [(
                                    0,
                                    function_function_return_expr(function_function_ref(
                                        FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                                        [LocalId::Int(IntLocalId(0))],
                                        int_to_int.clone(),
                                    )),
                                )],
                                function_function_return_tail_call(
                                    0,
                                    [int_arg(0, local_int(0, "n").sub_int(int(1)))],
                                ),
                            ),
                        ),
                    ),
                )
                .param_int(0, "n"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_function_return_family_bool_case_branches() {
        let actual = plan_module(compile(
            r#"
fn int_identity(value: Int) {
  value
}

fn int_increment(value: Int) {
  value + 1
}

fn string_identity(value: String) {
  value
}

fn string_suffix(value: String) {
  value <> "!"
}

fn bool_true(value: Bool) {
  True
}

fn bool_false(value: Bool) {
  False
}

fn nil_identity(value: Nil) {
  value
}

fn nil_other(value: Nil) {
  Nil
}

fn choose_int(flag: Bool) {
  case flag {
    True -> int_identity
    False -> int_increment
  }
}

fn choose_string(flag: Bool) {
  case flag {
    True -> string_identity
    False -> string_suffix
  }
}

fn choose_bool(flag: Bool) {
  case flag {
    True -> bool_true
    False -> bool_false
  }
}

fn choose_nil(flag: Bool) {
  case flag {
    True -> nil_identity
    False -> nil_other
  }
}

pub fn main() {
  choose_int(True)(1)
}
"#,
        ))
        .expect("source should plan");
        let int_to_int = function_type([ValueType::Int], ValueType::Int);
        let string_to_string = function_type([ValueType::String], ValueType::String);
        let bool_to_bool = function_type([ValueType::Bool], ValueType::Bool);
        let nil_to_nil = function_type([ValueType::Nil], ValueType::Nil);
        let expected = module(
            "main",
            function(
                "main",
                call_int_function(
                    call_int_returning_function(0, [bool_arg(0, bool_(true))], int_to_int.clone()),
                    [int_function_call_arg(0, int(1))],
                ),
            ),
            [
                function("int_identity", local_int(0, "value")).param_int(0, "value"),
                function("int_increment", local_int(0, "value").add_int(int(1)))
                    .param_int(0, "value"),
                function("string_identity", local_string(0, "value")).param_string(0, "value"),
                function(
                    "string_suffix",
                    local_string(0, "value").concatenate(string("!")),
                )
                .param_string(0, "value"),
                function("bool_true", bool_(true)).param_bool(0, "value"),
                function("bool_false", bool_(false)).param_bool(0, "value"),
                function("nil_identity", local_nil(0, "value")).param_nil(0, "value"),
                function("nil_other", nil()).param_nil(0, "value"),
                function(
                    "choose_int",
                    return_int_function(
                        int_to_int.clone(),
                        int_function_return_bool_case(
                            local_bool(0, "flag"),
                            int_function_return_expr(int_function_ref(
                                1,
                                [LocalId::Int(IntLocalId(0))],
                            )),
                            int_function_return_expr(int_function_ref(
                                2,
                                [LocalId::Int(IntLocalId(0))],
                            )),
                        ),
                    ),
                )
                .param_bool(0, "flag"),
                function(
                    "choose_string",
                    return_string_function(
                        string_to_string,
                        string_function_return_bool_case(
                            local_bool(0, "flag"),
                            string_function_return_expr(string_function_ref(
                                0,
                                [LocalId::String(StringLocalId(0))],
                            )),
                            string_function_return_expr(string_function_ref(
                                1,
                                [LocalId::String(StringLocalId(0))],
                            )),
                        ),
                    ),
                )
                .param_bool(0, "flag"),
                function(
                    "choose_bool",
                    return_bool_function(
                        bool_to_bool,
                        bool_function_return_bool_case(
                            local_bool(0, "flag"),
                            bool_function_return_expr(bool_function_ref(
                                0,
                                [LocalId::Bool(BoolLocalId(0))],
                            )),
                            bool_function_return_expr(bool_function_ref(
                                1,
                                [LocalId::Bool(BoolLocalId(0))],
                            )),
                        ),
                    ),
                )
                .param_bool(0, "flag"),
                function(
                    "choose_nil",
                    return_nil_function(
                        nil_to_nil,
                        nil_function_return_bool_case(
                            local_bool(0, "flag"),
                            nil_function_return_expr(nil_function_ref(
                                0,
                                [LocalId::Nil(NilLocalId(0))],
                            )),
                            nil_function_return_expr(nil_function_ref(
                                1,
                                [LocalId::Nil(NilLocalId(0))],
                            )),
                        ),
                    ),
                )
                .param_bool(0, "flag"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_function_return_family_string_case_branches() {
        let actual = plan_module(compile(
            r#"
fn int_identity(value: Int) {
  value
}

fn int_increment(value: Int) {
  value + 1
}

fn string_identity(value: String) {
  value
}

fn string_suffix(value: String) {
  value <> "!"
}

fn bool_true(value: Bool) {
  True
}

fn bool_false(value: Bool) {
  False
}

fn nil_identity(value: Nil) {
  value
}

fn nil_other(value: Nil) {
  Nil
}

fn choose_int(key: String) {
  case key {
    "one" -> int_identity
    _ -> int_increment
  }
}

fn choose_string(key: String) {
  case key {
    "one" -> string_identity
    _ -> string_suffix
  }
}

fn choose_bool(key: String) {
  case key {
    "one" -> bool_true
    _ -> bool_false
  }
}

fn choose_nil(key: String) {
  case key {
    "one" -> nil_identity
    _ -> nil_other
  }
}

fn choose_increment(key: String) {
  int_increment
}

fn choose_getter(key: String) {
  case key {
    "one" -> choose_int
    _ -> choose_increment
  }
}

pub fn main() {
  choose_int("one")(1)
}
"#,
        ))
        .expect("source should plan");
        let int_to_int = function_type([ValueType::Int], ValueType::Int);
        let string_to_string = function_type([ValueType::String], ValueType::String);
        let bool_to_bool = function_type([ValueType::Bool], ValueType::Bool);
        let nil_to_nil = function_type([ValueType::Nil], ValueType::Nil);
        let string_to_int_function = function_type(
            [ValueType::String],
            ValueType::Function(Box::new(int_to_int.clone())),
        );
        let expected = module(
            "main",
            function(
                "main",
                call_int_function(
                    call_int_returning_function(
                        0,
                        [string_arg(0, string("one"))],
                        int_to_int.clone(),
                    ),
                    [int_function_call_arg(0, int(1))],
                ),
            ),
            [
                function("int_identity", local_int(0, "value")).param_int(0, "value"),
                function("int_increment", local_int(0, "value").add_int(int(1)))
                    .param_int(0, "value"),
                function("string_identity", local_string(0, "value")).param_string(0, "value"),
                function(
                    "string_suffix",
                    local_string(0, "value").concatenate(string("!")),
                )
                .param_string(0, "value"),
                function("bool_true", bool_(true)).param_bool(0, "value"),
                function("bool_false", bool_(false)).param_bool(0, "value"),
                function("nil_identity", local_nil(0, "value")).param_nil(0, "value"),
                function("nil_other", nil()).param_nil(0, "value"),
                function(
                    "choose_int",
                    return_int_function(
                        int_to_int.clone(),
                        int_function_return_string_case(
                            local_string(0, "key"),
                            [(
                                "one",
                                int_function_return_expr(int_function_ref(
                                    1,
                                    [LocalId::Int(IntLocalId(0))],
                                )),
                            )],
                            int_function_return_expr(int_function_ref(
                                2,
                                [LocalId::Int(IntLocalId(0))],
                            )),
                        ),
                    ),
                )
                .param_string(0, "key"),
                function(
                    "choose_string",
                    return_string_function(
                        string_to_string,
                        string_function_return_string_case(
                            local_string(0, "key"),
                            [(
                                "one",
                                string_function_return_expr(string_function_ref(
                                    0,
                                    [LocalId::String(StringLocalId(0))],
                                )),
                            )],
                            string_function_return_expr(string_function_ref(
                                1,
                                [LocalId::String(StringLocalId(0))],
                            )),
                        ),
                    ),
                )
                .param_string(0, "key"),
                function(
                    "choose_bool",
                    return_bool_function(
                        bool_to_bool,
                        bool_function_return_string_case(
                            local_string(0, "key"),
                            [(
                                "one",
                                bool_function_return_expr(bool_function_ref(
                                    0,
                                    [LocalId::Bool(BoolLocalId(0))],
                                )),
                            )],
                            bool_function_return_expr(bool_function_ref(
                                1,
                                [LocalId::Bool(BoolLocalId(0))],
                            )),
                        ),
                    ),
                )
                .param_string(0, "key"),
                function(
                    "choose_nil",
                    return_nil_function(
                        nil_to_nil,
                        nil_function_return_string_case(
                            local_string(0, "key"),
                            [(
                                "one",
                                nil_function_return_expr(nil_function_ref(
                                    0,
                                    [LocalId::Nil(NilLocalId(0))],
                                )),
                            )],
                            nil_function_return_expr(nil_function_ref(
                                1,
                                [LocalId::Nil(NilLocalId(0))],
                            )),
                        ),
                    ),
                )
                .param_string(0, "key"),
                function(
                    "choose_increment",
                    return_int_function(
                        int_to_int.clone(),
                        int_function_return_expr(int_function_ref(
                            2,
                            [LocalId::Int(IntLocalId(0))],
                        )),
                    ),
                )
                .param_string(0, "key"),
                function(
                    "choose_getter",
                    return_function_function(
                        string_to_int_function,
                        function_function_return_string_case(
                            local_string(0, "key"),
                            [(
                                "one",
                                function_function_return_expr(function_function_ref(
                                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                                    [LocalId::String(StringLocalId(0))],
                                    int_to_int.clone(),
                                )),
                            )],
                            function_function_return_expr(function_function_ref(
                                FunctionFunctionId::Int(IntFunctionFunctionId(1)),
                                [LocalId::String(StringLocalId(0))],
                                int_to_int,
                            )),
                        ),
                    ),
                )
                .param_string(0, "key"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_bool_case_branches_as_tail_calls() {
        let actual = plan_module(compile(
            r#"
fn positive(value: Int) {
  value
}

fn negative(value: Int) {
  0 - value
}

fn choose(flag: Bool) {
  case flag {
    True -> positive(1)
    False -> negative(1)
  }
}

pub fn main() {
  choose(True)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", int_return_tail_call(3, [bool_arg(0, bool_(true))])),
            [
                function("positive", local_int(0, "value")).param_int(0, "value"),
                function("negative", int(0).sub_int(local_int(0, "value"))).param_int(0, "value"),
                function(
                    "choose",
                    int_return_bool_case(
                        local_bool(0, "flag"),
                        int_return_tail_call(1, [int_arg(0, int(1))]),
                        int_return_tail_call(2, [int_arg(0, int(1))]),
                    ),
                )
                .param_bool(0, "flag"),
            ],
        );

        assert_eq!(actual, expected);
    }

    fn function_type(
        arguments: impl IntoIterator<Item = ValueType>,
        return_: ValueType,
    ) -> FunctionType {
        FunctionType::new(arguments.into_iter().collect(), return_)
    }

    #[test]
    fn plan_non_tail_direct_call_stays_expression_call() {
        let actual = plan_module(compile(
            r#"
fn add(a: Int, b: Int) {
  a + b
}

pub fn main() {
  add(1, 2) + 3
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                call_int(1, [int_arg(0, int(1)), int_arg(1, int(2))]).add_int(int(3)),
            ),
            [
                function("add", local_int(0, "a").add_int(local_int(1, "b")))
                    .param_int(0, "a")
                    .param_int(1, "b"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_call_argument_direct_call_stays_expression_call() {
        let actual = plan_module(compile(
            r#"
fn add(a: Int, b: Int) {
  a + b
}

fn identity(value: Int) {
  value
}

pub fn main() {
  identity(add(1, 2))
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_tail_call(
                    2,
                    [int_arg(
                        0,
                        call_int(1, [int_arg(0, int(1)), int_arg(1, int(2))]),
                    )],
                ),
            ),
            [
                function("add", local_int(0, "a").add_int(local_int(1, "b")))
                    .param_int(0, "a")
                    .param_int(1, "b"),
                function("identity", local_int(0, "value")).param_int(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_expression_statement_direct_call_stays_expression_call() {
        let actual = plan_module(compile(
            r#"
fn add(a: Int, b: Int) {
  a + b
}

pub fn main() {
  add(1, 2)
  3
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", int(3))
                .evaluate(call_int(1, [int_arg(0, int(1)), int_arg(1, int(2))])),
            [
                function("add", local_int(0, "a").add_int(local_int(1, "b")))
                    .param_int(0, "a")
                    .param_int(1, "b"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_let_value_direct_call_stays_expression_call() {
        let actual = plan_module(compile(
            r#"
fn add(a: Int, b: Int) {
  a + b
}

pub fn main() {
  let value = add(1, 2)
  value
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", local_int(0, "value")).let_int(
                0,
                "value",
                call_int(1, [int_arg(0, int(1)), int_arg(1, int(2))]),
            ),
            [
                function("add", local_int(0, "a").add_int(local_int(1, "b")))
                    .param_int(0, "a")
                    .param_int(1, "b"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_short_circuit_rhs_direct_call_stays_expression_call() {
        let actual = plan_module(compile(
            r#"
fn truth() {
  True
}

pub fn main() {
  False && truth()
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", bool_(false).and_bool(call_bool(1, []))),
            [function("truth", bool_(true))],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_final_function_value_call_stays_expression_call() {
        let actual = plan_module(compile(
            r#"
fn add(a: Int, b: Int) {
  a + b
}

pub fn main() {
  let f = add
  f(1, 2)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                call_int_function(
                    local_int_function(0, "f", [ValueType::Int, ValueType::Int]),
                    [
                        int_function_call_arg(0, int(1)),
                        int_function_call_arg(1, int(2)),
                    ],
                ),
            )
            .step(let_int_function_step(
                0,
                "f",
                int_function_ref(
                    1,
                    [LocalId::Int(IntLocalId(0)), LocalId::Int(IntLocalId(1))],
                ),
            )),
            [
                function("add", local_int(0, "a").add_int(local_int(1, "b")))
                    .param_int(0, "a")
                    .param_int(1, "b"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_shadowed_current_function_local_call_stays_function_value_call() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let main = fn() { 0 }
  main()
}
"#,
        ))
        .expect("source should plan");
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                call_int_function(local_int_function(0, "main", Vec::<ValueType>::new()), []),
            )
            .step(let_int_function_step(
                0,
                "main",
                int_function_ref(1, Vec::<LocalId>::new()),
            )),
            [],
            [function("<anonymous:0>", int(0))],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_shadowed_current_function_argument_call_stays_function_value_call() {
        let actual = plan_module(compile(
            r#"
fn one() {
  1
}

fn run(run: fn() -> Int) {
  run()
}

pub fn main() {
  run(one)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_tail_call(
                    2,
                    [int_function_arg(
                        0,
                        int_function_ref(1, Vec::<LocalId>::new()),
                    )],
                ),
            ),
            [
                function("one", int(1)),
                function(
                    "run",
                    call_int_function(local_int_function(0, "run", Vec::<ValueType>::new()), []),
                )
                .param_int_function(0, "run", Vec::<ValueType>::new()),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_final_pipeline_direct_call_preserves_tail_call() {
        let actual = plan_module(compile(
            r#"
fn count_down(n: Int, acc: Int) {
  case n {
    0 -> acc
    _ -> count_down(n - 1, acc + 1)
  }
}

pub fn main() {
  1 |> count_down(0)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_int_step(0, "_pipe", int(1))],
                    int_return_tail_call(
                        1,
                        [int_arg(0, local_int(0, "_pipe")), int_arg(1, int(0))],
                    ),
                ),
            ),
            [function(
                "count_down",
                int_return_int_case(
                    local_int(0, "n"),
                    [(0, int_return_expr(local_int(1, "acc")))],
                    int_return_tail_call(
                        1,
                        [
                            int_arg(0, local_int(0, "n").sub_int(int(1))),
                            int_arg(1, local_int(1, "acc").add_int(int(1))),
                        ],
                    ),
                ),
            )
            .param_int(0, "n")
            .param_int(1, "acc")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_main_returning_function_value() {
        let actual = plan_module(compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                function_ref(
                    RuntimeFunctionId::Int(IntFunctionId(0)),
                    [LocalId::Int(IntLocalId(0))],
                ),
            ),
            [function("identity", local_int(0, "value")).param_int(0, "value")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_main_as_local_function_call() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  1
}

pub fn helper() {
  main()
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", int(1)),
            [function("helper", int_return_tail_call(0, []))],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_typed_local_function_calls() {
        let actual = plan_module(compile(
            r#"
pub fn string_id(value: String) {
  value
}

pub fn bool_id(value: Bool) {
  value
}

pub fn nil_id(value: Nil) {
  value
}

pub fn main() {
  string_id("geam")
}

pub fn bool_main() {
  bool_id(True)
}

pub fn nil_main() {
  nil_id(Nil)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                string_return_tail_call(1, [string_arg(0, string("geam"))]),
            ),
            [
                function("string_id", local_string(0, "value")).param_string(0, "value"),
                function("bool_id", local_bool(0, "value")).param_bool(0, "value"),
                function("nil_id", local_nil(0, "value")).param_nil(0, "value"),
                function(
                    "bool_main",
                    bool_return_tail_call(0, [bool_arg(0, bool_(true))]),
                ),
                function("nil_main", nil_return_tail_call(0, [nil_arg(0, nil())])),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_labelled_arguments() {
        assert_eq!(
            expect_plan_error(
                r#"
fn identity(value value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
            ),
            PlanError::UnsupportedArgument {
                function: "identity".into(),
                reason: UnsupportedArgumentReason::Labelled,
            },
        );
    }

    #[test]
    fn reject_profile_function_shapes() {
        assert_eq!(
            expect_plan_error(
                r#"
@external(erlang, "one", "two")
pub fn main() -> Int
"#,
            ),
            PlanError::UnsupportedFunction {
                name: "main".into(),
                reason: UnsupportedFunctionReason::External,
            },
        );

        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() -> BitArray {
  <<>>
}
"#,
            ),
            PlanError::UnsupportedFunction {
                name: "main".into(),
                reason: UnsupportedFunctionReason::UnsupportedReturnType,
            },
        );
    }

    #[test]
    fn reject_margin_function_shapes() {
        let mut empty_body = compile_minimal_module();
        empty_body.definitions.functions[0].body = Vec::new();
        assert_eq!(
            plan_module(empty_body),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "main".into(),
                    reason: InvalidFunctionShapeReason::EmptyBody,
                },
            }),
        );

        let mut anonymous = compile_minimal_module();
        anonymous.definitions.functions[0].name = None;
        assert_eq!(
            plan_module(anonymous),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "<anonymous>".into(),
                    reason: InvalidFunctionShapeReason::Anonymous,
                },
            }),
        );

        let mut return_type_mismatch = compile_minimal_module();
        return_type_mismatch.definitions.functions[0].return_type = gleam_core::type_::bool();
        assert_eq!(
            plan_module(return_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "main".into(),
                    reason: InvalidFunctionShapeReason::ReturnTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_plan_function_name_shape() {
        let mut module = compile_minimal_module();
        let mut function = module.definitions.functions.remove(0);
        function.name = None;
        let info = FunctionInfo {
            id: FunctionId::new(0),
            runtime_id: RuntimeFunctionId::Int(IntFunctionId(0)),
            return_type: ValueType::Int,
            params: Vec::new(),
        };
        let mut anonymous = crate::planner::context::AnonymousFunctions::default();

        assert_eq!(
            super::plan_function(
                info,
                &"main".into(),
                &Default::default(),
                function,
                &mut anonymous,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "<anonymous>".into(),
                    reason: InvalidFunctionShapeReason::Anonymous,
                },
            }),
        );
    }
}
