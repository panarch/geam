mod expression;
mod function;
mod module;

pub(crate) fn host_call_site(
    source: &str,
    function: &str,
    call: &str,
) -> crate::plan::HostCallSite {
    host_call_site_in(source, function, call, call)
}

pub(crate) fn host_call_site_in(
    source: &str,
    function: &str,
    context: &str,
    call: &str,
) -> crate::plan::HostCallSite {
    let function_start = if function.starts_with('<') {
        0
    } else {
        let function_declaration = format!("fn {function}(");
        let declaration_start = source
            .find(&function_declaration)
            .unwrap_or_else(|| panic!("function `{function}` should appear in the source"));
        source[declaration_start..]
            .find('{')
            .map(|body| declaration_start + body + 1)
            .unwrap_or_else(|| panic!("function `{function}` should have a body"))
    };
    let context_start = source[function_start..]
        .find(context)
        .map(|start| function_start + start)
        .unwrap_or_else(|| panic!("context `{context}` should appear in the source"));
    let call_start = context
        .find(call)
        .unwrap_or_else(|| panic!("call `{call}` should appear in context `{context}`"));
    let start = context_start + call_start;
    crate::plan::HostCallSite::new(
        "main".into(),
        function.into(),
        crate::plan::SourceSpan::new(start, start + call.len()),
    )
}

pub(crate) use expression::{
    bit_array, bit_array_function_ref, block_function, block_int, block_int_function, bool_,
    bool_arg, bool_case_int_function, bool_function_ref, call_bool_at, call_float_at, call_int_at,
    call_int_function_at, call_int_returning_function, call_list, capture_int, capture_tuple,
    equal, evaluate_step, float, float_arg, float_function_ref, function_function_closure,
    function_function_ref, function_ref, int, int_arg, int_case_int_function, int_function_arg,
    int_function_call_arg, int_function_closure, int_function_ref, let_bool_function_step,
    let_bool_step, let_float_step, let_int_function_step, let_int_step, let_list_step,
    let_nil_function_step, let_nil_step, let_string_function_step, let_string_step, let_tuple_step,
    let_utf_codepoint_step, list, list_function_ref, list_spread, local_bool, local_float,
    local_int, local_int_function, local_list, local_nil, local_string, local_tuple,
    local_utf_codepoint, nil, nil_arg, nil_function_ref, not_equal, string, string_arg,
    string_function_ref, tuple, tuple_arg, tuple_function_closure, tuple_function_ref,
    utf_codepoint_function_ref,
};
pub(crate) use function::{
    bool_function_return_block, bool_function_return_bool_case, bool_function_return_expr,
    bool_function_return_int_case, bool_function_return_string_case,
    bool_function_return_tail_call, bool_return_block, bool_return_bool_case, bool_return_expr,
    bool_return_float_case, bool_return_int_case, bool_return_string_case,
    bool_return_tail_call_at, float_return_block, float_return_expr, float_return_float_case,
    function, function_function_return_block, function_function_return_expr,
    function_function_return_int_case, function_function_return_string_case,
    function_function_return_tail_call, int_function_return_block, int_function_return_bool_case,
    int_function_return_expr, int_function_return_int_case, int_function_return_string_case,
    int_function_return_tail_call, int_return_block, int_return_bool_case, int_return_expr,
    int_return_float_case, int_return_int_case, int_return_string_case, int_return_tail_call_at,
    list_return_block, list_return_bool_case, list_return_expr, list_return_float_case,
    list_return_int_case, list_return_string_case, nil_function_return_block,
    nil_function_return_bool_case, nil_function_return_expr, nil_function_return_int_case,
    nil_function_return_string_case, nil_function_return_tail_call, nil_return_block,
    nil_return_bool_case, nil_return_expr, nil_return_float_case, nil_return_int_case,
    nil_return_string_case, nil_return_tail_call_at, return_bool_function,
    return_function_function, return_int_function, return_list, return_nil_function,
    return_string_function, string_function_return_block, string_function_return_bool_case,
    string_function_return_expr, string_function_return_int_case,
    string_function_return_string_case, string_function_return_tail_call, string_return_block,
    string_return_bool_case, string_return_expr, string_return_float_case, string_return_int_case,
    string_return_string_case, string_return_tail_call_at, utf_codepoint_return_block,
    utf_codepoint_return_expr,
};
pub(crate) use module::{module, module_with_anonymous};

#[cfg(test)]
mod tests {
    use super::{host_call_site, host_call_site_in};

    #[test]
    fn host_call_site_uses_exact_source_bytes() {
        let source = "pub fn main() { add(1, 2) }";

        let site = host_call_site(source, "main", "add(1, 2)");

        assert_eq!(site.module(), "main");
        assert_eq!(site.function(), "main");
        assert_eq!(site.span(), crate::plan::SourceSpan::new(16, 25));
    }

    #[test]
    #[should_panic(expected = "context `missing()` should appear in the source")]
    fn host_call_site_rejects_missing_call_text() {
        host_call_site("pub fn main() { 1 }", "main", "missing()");
    }

    #[test]
    #[should_panic(expected = "function `missing` should appear in the source")]
    fn host_call_site_rejects_missing_function() {
        host_call_site("pub fn main() { 1 }", "missing", "1");
    }

    #[test]
    #[should_panic(expected = "function `main` should have a body")]
    fn host_call_site_rejects_missing_function_body() {
        host_call_site("pub fn main()", "main", "main");
    }

    #[test]
    fn host_call_site_in_uses_the_visible_context() {
        let source = "pub fn main() { let f = 1\n1 |> f }";

        let site = host_call_site_in(source, "main", "1 |> f", "f");

        assert_eq!(site.span(), crate::plan::SourceSpan::new(31, 32));
    }

    #[test]
    #[should_panic(expected = "context `missing` should appear in the source")]
    fn host_call_site_in_rejects_missing_context() {
        host_call_site_in("pub fn main() { 1 }", "main", "missing", "1");
    }

    #[test]
    #[should_panic(expected = "call `missing` should appear in context `1`")]
    fn host_call_site_in_rejects_call_outside_context() {
        host_call_site_in("pub fn main() { 1 }", "main", "1", "missing");
    }
}
