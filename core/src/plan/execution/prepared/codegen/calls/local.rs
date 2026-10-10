use super::shape::{CallLocal, Capture};
use crate::plan::execution::prepared::rust::Rust;

pub(super) fn fields(locals: &[CallLocal]) -> String {
    format!(
        " {{ {} }}",
        locals
            .iter()
            .map(|local| format!("{}: {}", local_name(local), local_type(local)))
            .collect::<Vec<_>>()
            .join(", ")
    )
}
pub(super) fn pattern(locals: &[CallLocal]) -> String {
    format!(
        " {{ {} }}",
        locals
            .iter()
            .map(|local| {
                if matches!(local, CallLocal::Nil(_)) {
                    format!("{}: ()", local_name(local))
                } else {
                    local_name(local)
                }
            })
            .collect::<Vec<_>>()
            .join(", ")
    )
}
pub(super) fn local_type(local: &CallLocal) -> &'static str {
    match local {
        CallLocal::Nullary(_) => "CallNullary",
        CallLocal::Custom(_) => "CallCustom",
        CallLocal::Tuple { .. } => "CallTuple",
        CallLocal::Int(..) => "i128",
        CallLocal::IntList { .. } => "IntList",
        CallLocal::IntFunction { .. } => "IntCallable",
        CallLocal::Bool(..) => "bool",
        CallLocal::BoolList { .. } => "BoolList",
        CallLocal::BoolFunction { .. } => "BoolCallable",
        CallLocal::Float(..) => "f64",
        CallLocal::FloatList { .. } => "FloatList",
        CallLocal::FloatFunction { .. } => "FloatCallable",
        CallLocal::String(..) => "StringValue",
        CallLocal::StringList { .. } => "StringList",
        CallLocal::StringFunction { .. } => "StringCallable",
        CallLocal::BitArray(..) => "CallBitArray",
        CallLocal::BitArrayList { .. } => "BitArrayList",
        CallLocal::BitArrayFunction { .. } => "BitArrayCallable",
        CallLocal::UtfCodepoint(..) => "char",
        CallLocal::UtfCodepointList { .. } => "UtfCodepointList",
        CallLocal::UtfCodepointFunction { .. } => "UtfCodepointCallable",
        CallLocal::Nil(..) => "()",
        CallLocal::NilList { .. } => "NilList",
        CallLocal::NilFunction { .. } => "NilCallable",
    }
}

fn local_column(local: &CallLocal) -> &'static str {
    match local {
        CallLocal::Nullary(_) | CallLocal::Custom(_) => "customs",
        CallLocal::Tuple { .. } => "tuples",
        CallLocal::Int(..) => "ints",
        CallLocal::IntList { .. } => "int_lists",
        CallLocal::IntFunction { .. } => "int_functions",
        CallLocal::Bool(..) => "bools",
        CallLocal::BoolList { .. } => "bool_lists",
        CallLocal::BoolFunction { .. } => "bool_functions",
        CallLocal::Float(..) => "floats",
        CallLocal::FloatList { .. } => "float_lists",
        CallLocal::FloatFunction { .. } => "float_functions",
        CallLocal::String(..) => "strings",
        CallLocal::StringList { .. } => "string_lists",
        CallLocal::StringFunction { .. } => "string_functions",
        CallLocal::BitArray(..) => "bit_arrays",
        CallLocal::BitArrayList { .. } => "bit_array_lists",
        CallLocal::BitArrayFunction { .. } => "bit_array_functions",
        CallLocal::UtfCodepoint(..) => "utf_codepoints",
        CallLocal::UtfCodepointList { .. } => "utf_codepoint_lists",
        CallLocal::UtfCodepointFunction { .. } => "utf_codepoint_functions",
        CallLocal::Nil(..) => "nils",
        CallLocal::NilList { .. } => "nil_lists",
        CallLocal::NilFunction { .. } => "nil_functions",
    }
}

pub(super) fn capture_method(local: &CallLocal) -> Option<&'static str> {
    Some(match local {
        CallLocal::Nullary(_) | CallLocal::Custom(_) | CallLocal::Tuple { .. } => return None,
        CallLocal::Int(..) => "int",
        CallLocal::IntList { .. } => "int_list",
        CallLocal::IntFunction { .. } => "int_function",
        CallLocal::Bool(..) => "bool",
        CallLocal::BoolList { .. } => "bool_list",
        CallLocal::BoolFunction { .. } => "bool_function",
        CallLocal::Float(..) => "float",
        CallLocal::FloatList { .. } => "float_list",
        CallLocal::FloatFunction { .. } => "float_function",
        CallLocal::String(..) => "string",
        CallLocal::StringList { .. } => "string_list",
        CallLocal::StringFunction { .. } => "string_function",
        CallLocal::BitArray(..) => "bit_array",
        CallLocal::BitArrayList { .. } => "bit_array_list",
        CallLocal::BitArrayFunction { .. } => "bit_array_function",
        CallLocal::UtfCodepoint(..) => "utf_codepoint",
        CallLocal::UtfCodepointList { .. } => "utf_codepoint_list",
        CallLocal::UtfCodepointFunction { .. } => "utf_codepoint_function",
        CallLocal::Nil(..) => "nil",
        CallLocal::NilList { .. } => "nil_list",
        CallLocal::NilFunction { .. } => "nil_function",
    })
}

pub(super) fn capture_input_expression(local: &CallLocal) -> String {
    capture_method(local).map_or_else(
        || "{ return None; }".to_owned(),
        |method| format!("captures.{method}({})?", local_id(local)),
    )
}

pub(super) fn local_name(local: &CallLocal) -> String {
    match local {
        CallLocal::Nullary(value) => format!("nullary{}", value.local.id.0),
        CallLocal::Custom(value) => format!("custom{}", value.local.id.0),
        CallLocal::Tuple { local, .. } => format!("tuple{}", local.0),
        CallLocal::Int(local) => format!("int{}", local.0),
        CallLocal::IntList { local, .. } => format!("int_list{}", local.0),
        CallLocal::IntFunction { local, .. } => format!("int_function{}", local.0),
        CallLocal::Bool(local) => format!("bool{}", local.0),
        CallLocal::BoolList { local, .. } => format!("bool_list{}", local.0),
        CallLocal::BoolFunction { local, .. } => format!("bool_function{}", local.0),
        CallLocal::Float(local) => format!("float{}", local.0),
        CallLocal::FloatList { local, .. } => format!("float_list{}", local.0),
        CallLocal::FloatFunction { local, .. } => format!("float_function{}", local.0),
        CallLocal::String(local) => format!("string{}", local.0),
        CallLocal::StringList { local, .. } => format!("string_list{}", local.0),
        CallLocal::StringFunction { local, .. } => format!("string_function{}", local.0),
        CallLocal::BitArray(local) => format!("bit_array{}", local.0),
        CallLocal::BitArrayList { local, .. } => format!("bit_array_list{}", local.0),
        CallLocal::BitArrayFunction { local, .. } => format!("bit_array_function{}", local.0),
        CallLocal::UtfCodepoint(local) => format!("utf_codepoint{}", local.0),
        CallLocal::UtfCodepointList { local, .. } => format!("utf_codepoint_list{}", local.0),
        CallLocal::UtfCodepointFunction { local, .. } => {
            format!("utf_codepoint_function{}", local.0)
        }
        CallLocal::Nil(local) => format!("nil{}", local.0),
        CallLocal::NilList { local, .. } => format!("nil_list{}", local.0),
        CallLocal::NilFunction { local, .. } => format!("nil_function{}", local.0),
    }
}

pub(super) fn local_id(local: &CallLocal) -> String {
    match local {
        CallLocal::Nullary(value) | CallLocal::Custom(value) => Rust::expression(&value.local.id),
        CallLocal::Tuple { local, .. } => Rust::expression(local),
        CallLocal::Int(local) => Rust::expression(local),
        CallLocal::IntList { local, .. } => Rust::expression(local),
        CallLocal::IntFunction { local, .. } => Rust::expression(local),
        CallLocal::Bool(local) => Rust::expression(local),
        CallLocal::BoolList { local, .. } => Rust::expression(local),
        CallLocal::BoolFunction { local, .. } => Rust::expression(local),
        CallLocal::Float(local) => Rust::expression(local),
        CallLocal::FloatList { local, .. } => Rust::expression(local),
        CallLocal::FloatFunction { local, .. } => Rust::expression(local),
        CallLocal::String(local) => Rust::expression(local),
        CallLocal::StringList { local, .. } => Rust::expression(local),
        CallLocal::StringFunction { local, .. } => Rust::expression(local),
        CallLocal::BitArray(local) => Rust::expression(local),
        CallLocal::BitArrayList { local, .. } => Rust::expression(local),
        CallLocal::BitArrayFunction { local, .. } => Rust::expression(local),
        CallLocal::UtfCodepoint(local) => Rust::expression(local),
        CallLocal::UtfCodepointList { local, .. } => Rust::expression(local),
        CallLocal::UtfCodepointFunction { local, .. } => Rust::expression(local),
        CallLocal::Nil(local) => Rust::expression(local),
        CallLocal::NilList { local, .. } => Rust::expression(local),
        CallLocal::NilFunction { local, .. } => Rust::expression(local),
    }
}

pub(super) fn load_value(local: &CallLocal) -> String {
    match local {
        CallLocal::Nullary(value) => format!(
            "values.nullary({}, &{})?",
            value.local.id.0,
            Rust::expression(value.constructors.as_slice())
        ),
        CallLocal::Custom(value) => format!("values.custom({})?", value.local.id.0),
        CallLocal::Tuple { local, .. } => format!("values.tuple({})?", local.0),
        CallLocal::Int(local) => format!("values.int({})?", local.0),
        CallLocal::IntList { local, .. } => format!("values.int_list({})?", local.0),
        CallLocal::IntFunction { local, .. } => format!("values.int_function({})?", local.0),
        CallLocal::Bool(local) => format!("values.bool({})?", local.0),
        CallLocal::BoolList { local, .. } => format!("values.bool_list({})?", local.0),
        CallLocal::BoolFunction { local, .. } => format!("values.bool_function({})?", local.0),
        CallLocal::Float(local) => format!("values.float({})?", local.0),
        CallLocal::FloatList { local, .. } => format!("values.float_list({})?", local.0),
        CallLocal::FloatFunction { local, .. } => format!("values.float_function({})?", local.0),
        CallLocal::String(local) => format!("values.string({})?", local.0),
        CallLocal::StringList { local, .. } => format!("values.string_list({})?", local.0),
        CallLocal::StringFunction { local, .. } => format!("values.string_function({})?", local.0),
        CallLocal::BitArray(local) => format!("values.bit_array({})?", local.0),
        CallLocal::BitArrayList { local, .. } => format!("values.bit_array_list({})?", local.0),
        CallLocal::BitArrayFunction { local, .. } => {
            format!("values.bit_array_function({})?", local.0)
        }
        CallLocal::UtfCodepoint(local) => format!("values.utf_codepoint({})?", local.0),
        CallLocal::UtfCodepointList { local, .. } => {
            format!("values.utf_codepoint_list({})?", local.0)
        }
        CallLocal::UtfCodepointFunction { local, .. } => {
            format!("values.utf_codepoint_function({})?", local.0)
        }
        CallLocal::Nil(..) => "()".to_owned(),
        CallLocal::NilList { local, .. } => format!("values.nil_list({})?", local.0),
        CallLocal::NilFunction { local, .. } => format!("values.nil_function({})?", local.0),
    }
}

pub(super) fn local_expression(local: &CallLocal, clone: bool) -> String {
    if matches!(local, CallLocal::Nil(_)) {
        return "()".to_owned();
    }
    let name = local_name(local);
    if clone
        && !matches!(
            local,
            CallLocal::Nullary(_)
                | CallLocal::Int(_)
                | CallLocal::Bool(_)
                | CallLocal::Float(_)
                | CallLocal::UtfCodepoint(_)
                | CallLocal::Nil(_)
        )
    {
        format!("{name}.clone()")
    } else {
        name
    }
}
pub(super) fn field_assignment(parameter: &CallLocal, argument: &CallLocal) -> String {
    let name = local_name(parameter);
    let expression = if matches!(
        (parameter, argument),
        (CallLocal::Custom(_), CallLocal::Nullary(_))
    ) {
        format!("{}.into()", local_expression(argument, true))
    } else {
        local_expression(argument, true)
    };
    if name == expression {
        name
    } else {
        format!("{name}: {expression}")
    }
}
pub(super) fn values(locals: &[CallLocal], clone: bool) -> String {
    values_with_result(locals, clone, None)
}

pub(super) fn values_with_result(
    locals: &[CallLocal],
    clone: bool,
    result: Option<&CallLocal>,
) -> String {
    format!("Box::new({})", owned_values(locals, clone, result))
}

fn owned_values(locals: &[CallLocal], clone: bool, result: Option<&CallLocal>) -> String {
    let columns = [
        "customs",
        "tuples",
        "ints",
        "int_lists",
        "int_functions",
        "bools",
        "bool_lists",
        "bool_functions",
        "floats",
        "float_lists",
        "float_functions",
        "strings",
        "string_lists",
        "string_functions",
        "bit_arrays",
        "bit_array_lists",
        "bit_array_functions",
        "utf_codepoints",
        "utf_codepoint_lists",
        "utf_codepoint_functions",
        "nil_lists",
        "nil_functions",
    ];
    let fields = columns
        .into_iter()
        .filter_map(|column| {
            let values = locals
                .iter()
                .filter(|local| local_column(local) == column)
                .map(|local| {
                    let value = local_expression(local, clone);
                    if matches!(local, CallLocal::Nullary(_) | CallLocal::Int(_))
                        && result != Some(local)
                    {
                        format!("{value}.into()")
                    } else {
                        value
                    }
                })
                .collect::<Vec<_>>();
            (!values.is_empty()).then(|| format!("{column}: vec![{}]", values.join(", ")))
        })
        .collect::<Vec<_>>()
        .join(", ");
    if fields.is_empty() {
        "CallValues::default()".to_owned()
    } else {
        format!("CallValues {{ {fields}, ..CallValues::default() }}")
    }
}

pub(super) fn capture_expression(capture: &Capture) -> String {
    match capture {
        Capture::Int { target, source } => format!(
            "CallCapture::int({}, int{})",
            Rust::expression(target),
            source.0
        ),
        Capture::IntList { target, source } => format!(
            "CallCapture::int_list({}, int_list{}.clone())",
            Rust::expression(target),
            source.0
        ),
        Capture::IntFunction { target, source } => format!(
            "CallCapture::int_function({}, int_function{}.clone())",
            Rust::expression(target),
            source.0
        ),
        Capture::Bool { target, source } => format!(
            "CallCapture::bool({}, bool{})",
            Rust::expression(target),
            source.0
        ),
        Capture::BoolList { target, source } => format!(
            "CallCapture::bool_list({}, bool_list{}.clone())",
            Rust::expression(target),
            source.0
        ),
        Capture::BoolFunction { target, source } => format!(
            "CallCapture::bool_function({}, bool_function{}.clone())",
            Rust::expression(target),
            source.0
        ),
        Capture::Float { target, source } => format!(
            "CallCapture::float({}, float{})",
            Rust::expression(target),
            source.0
        ),
        Capture::FloatList { target, source } => format!(
            "CallCapture::float_list({}, float_list{}.clone())",
            Rust::expression(target),
            source.0
        ),
        Capture::FloatFunction { target, source } => format!(
            "CallCapture::float_function({}, float_function{}.clone())",
            Rust::expression(target),
            source.0
        ),
        Capture::String { target, source } => format!(
            "CallCapture::string({}, string{}.clone())",
            Rust::expression(target),
            source.0
        ),
        Capture::StringList { target, source } => format!(
            "CallCapture::string_list({}, string_list{}.clone())",
            Rust::expression(target),
            source.0
        ),
        Capture::StringFunction { target, source } => format!(
            "CallCapture::string_function({}, string_function{}.clone())",
            Rust::expression(target),
            source.0
        ),
        Capture::BitArray { target, source } => format!(
            "CallCapture::bit_array({}, bit_array{}.clone())",
            Rust::expression(target),
            source.0
        ),
        Capture::BitArrayList { target, source } => format!(
            "CallCapture::bit_array_list({}, bit_array_list{}.clone())",
            Rust::expression(target),
            source.0
        ),
        Capture::BitArrayFunction { target, source } => format!(
            "CallCapture::bit_array_function({}, bit_array_function{}.clone())",
            Rust::expression(target),
            source.0
        ),
        Capture::UtfCodepoint { target, source } => format!(
            "CallCapture::utf_codepoint({}, utf_codepoint{})",
            Rust::expression(target),
            source.0
        ),
        Capture::UtfCodepointList { target, source } => format!(
            "CallCapture::utf_codepoint_list({}, utf_codepoint_list{}.clone())",
            Rust::expression(target),
            source.0
        ),
        Capture::UtfCodepointFunction { target, source } => format!(
            "CallCapture::utf_codepoint_function({}, utf_codepoint_function{}.clone())",
            Rust::expression(target),
            source.0
        ),
        Capture::Nil { target, .. } => {
            format!("CallCapture::nil({}, ())", Rust::expression(target))
        }
        Capture::NilList { target, source } => format!(
            "CallCapture::nil_list({}, nil_list{}.clone())",
            Rust::expression(target),
            source.0
        ),
        Capture::NilFunction { target, source } => format!(
            "CallCapture::nil_function({}, nil_function{}.clone())",
            Rust::expression(target),
            source.0
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::super::shape::{CallLocal, Capture};
    use super::{
        capture_expression, capture_method, fields, load_value, local_column, local_id, local_name,
        local_type, pattern, values,
    };
    use crate::plan::execution::graph::{
        BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId, BoolFunctionLocalId,
        BoolListLocalId, BoolLocalId, FloatFunctionLocalId, FloatListLocalId, FloatLocalId,
        IntFunctionLocalId, IntListLocalId, IntLocalId, NilFunctionLocalId, NilListLocalId,
        NilLocalId, StringFunctionLocalId, StringListLocalId, StringLocalId, TupleLocalId,
        UtfCodepointFunctionLocalId, UtfCodepointListLocalId, UtfCodepointLocalId,
    };
    use crate::plan::execution::type_::{
        BitArrayListTypeId, BoolListTypeId, FloatListTypeId, FunctionType, IntListTypeId,
        ListTypeId, NilListTypeId, StringListTypeId, UtfCodepointListTypeId, ValueType,
    };
    #[test]
    fn local_columns_and_captures_emit_each_concrete_value_family() {
        let locals = [
            CallLocal::Int(IntLocalId(0)),
            CallLocal::Bool(BoolLocalId(1)),
            CallLocal::IntList {
                local: IntListLocalId(2),
                type_id: IntListTypeId {
                    list_type: ListTypeId(0),
                },
            },
            CallLocal::IntFunction {
                local: IntFunctionLocalId(3),
                type_: FunctionType::new(vec![ValueType::Int], ValueType::Int),
            },
            CallLocal::BoolFunction {
                local: BoolFunctionLocalId(4),
                type_: FunctionType::new(vec![ValueType::Bool], ValueType::Bool),
            },
        ];
        assert_eq!(
            fields(&locals),
            " { int0: i128, bool1: bool, int_list2: IntList, int_function3: IntCallable, bool_function4: BoolCallable }"
        );
        assert_eq!(
            pattern(&locals),
            " { int0, bool1, int_list2, int_function3, bool_function4 }"
        );
        assert_eq!(
            values(&locals, false),
            "Box::new(CallValues { ints: vec![int0.into()], int_lists: vec![int_list2], int_functions: vec![int_function3], bools: vec![bool1], bool_functions: vec![bool_function4], ..CallValues::default() })"
        );
        assert_eq!(
            values(&locals, true),
            "Box::new(CallValues { ints: vec![int0.into()], int_lists: vec![int_list2.clone()], int_functions: vec![int_function3.clone()], bools: vec![bool1], bool_functions: vec![bool_function4.clone()], ..CallValues::default() })"
        );
        let projections = locals
            .iter()
            .map(|local| {
                (
                    local_type(local),
                    local_column(local),
                    local_name(local),
                    local_id(local),
                    load_value(local),
                    capture_method(local).unwrap(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            projections,
            [
                (
                    "i128",
                    "ints",
                    "int0".into(),
                    "data::graph::IntLocalId(0)".into(),
                    "values.int(0)?".into(),
                    "int"
                ),
                (
                    "bool",
                    "bools",
                    "bool1".into(),
                    "data::graph::BoolLocalId(1)".into(),
                    "values.bool(1)?".into(),
                    "bool"
                ),
                (
                    "IntList",
                    "int_lists",
                    "int_list2".into(),
                    "data::graph::IntListLocalId(2)".into(),
                    "values.int_list(2)?".into(),
                    "int_list"
                ),
                (
                    "IntCallable",
                    "int_functions",
                    "int_function3".into(),
                    "data::graph::IntFunctionLocalId(3)".into(),
                    "values.int_function(3)?".into(),
                    "int_function"
                ),
                (
                    "BoolCallable",
                    "bool_functions",
                    "bool_function4".into(),
                    "data::graph::BoolFunctionLocalId(4)".into(),
                    "values.bool_function(4)?".into(),
                    "bool_function"
                ),
            ]
        );
        let captures = [
            Capture::Int {
                target: IntLocalId(0),
                source: IntLocalId(7),
            },
            Capture::Bool {
                target: BoolLocalId(1),
                source: BoolLocalId(8),
            },
            Capture::IntList {
                target: IntListLocalId(2),
                source: IntListLocalId(9),
            },
            Capture::IntFunction {
                target: IntFunctionLocalId(3),
                source: IntFunctionLocalId(10),
            },
            Capture::BoolFunction {
                target: BoolFunctionLocalId(4),
                source: BoolFunctionLocalId(11),
            },
        ];
        assert_eq!(
            captures.iter().map(capture_expression).collect::<Vec<_>>(),
            [
                "CallCapture::int(data::graph::IntLocalId(0), int7)",
                "CallCapture::bool(data::graph::BoolLocalId(1), bool8)",
                "CallCapture::int_list(data::graph::IntListLocalId(2), int_list9.clone())",
                "CallCapture::int_function(data::graph::IntFunctionLocalId(3), int_function10.clone())",
                "CallCapture::bool_function(data::graph::BoolFunctionLocalId(4), bool_function11.clone())",
            ]
        );
    }

    #[test]
    fn every_primitive_local_has_exact_field_input_and_capture_syntax() {
        let tuple = CallLocal::Tuple {
            local: TupleLocalId(3),
            type_: vec![ValueType::Int, ValueType::Nil].into(),
        };
        assert_eq!(local_id(&tuple), "data::graph::TupleLocalId(3)");
        for (local, expected_type, column, name, id, input, method) in [
            (
                CallLocal::Int(IntLocalId(1)),
                "i128",
                "ints",
                "int1",
                "data::graph::IntLocalId(1)",
                "values.int(1)?",
                "int",
            ),
            (
                CallLocal::IntList {
                    local: IntListLocalId(1),
                    type_id: IntListTypeId {
                        list_type: ListTypeId(0),
                    },
                },
                "IntList",
                "int_lists",
                "int_list1",
                "data::graph::IntListLocalId(1)",
                "values.int_list(1)?",
                "int_list",
            ),
            (
                CallLocal::IntFunction {
                    local: IntFunctionLocalId(1),
                    type_: FunctionType::new(Vec::new(), ValueType::Int),
                },
                "IntCallable",
                "int_functions",
                "int_function1",
                "data::graph::IntFunctionLocalId(1)",
                "values.int_function(1)?",
                "int_function",
            ),
            (
                CallLocal::Bool(BoolLocalId(1)),
                "bool",
                "bools",
                "bool1",
                "data::graph::BoolLocalId(1)",
                "values.bool(1)?",
                "bool",
            ),
            (
                CallLocal::BoolList {
                    local: BoolListLocalId(1),
                    type_id: BoolListTypeId {
                        list_type: ListTypeId(0),
                    },
                },
                "BoolList",
                "bool_lists",
                "bool_list1",
                "data::graph::BoolListLocalId(1)",
                "values.bool_list(1)?",
                "bool_list",
            ),
            (
                CallLocal::BoolFunction {
                    local: BoolFunctionLocalId(1),
                    type_: FunctionType::new(Vec::new(), ValueType::Bool),
                },
                "BoolCallable",
                "bool_functions",
                "bool_function1",
                "data::graph::BoolFunctionLocalId(1)",
                "values.bool_function(1)?",
                "bool_function",
            ),
            (
                CallLocal::Float(FloatLocalId(1)),
                "f64",
                "floats",
                "float1",
                "data::graph::FloatLocalId(1)",
                "values.float(1)?",
                "float",
            ),
            (
                CallLocal::FloatList {
                    local: FloatListLocalId(1),
                    type_id: FloatListTypeId {
                        list_type: ListTypeId(0),
                    },
                },
                "FloatList",
                "float_lists",
                "float_list1",
                "data::graph::FloatListLocalId(1)",
                "values.float_list(1)?",
                "float_list",
            ),
            (
                CallLocal::FloatFunction {
                    local: FloatFunctionLocalId(1),
                    type_: FunctionType::new(Vec::new(), ValueType::Float),
                },
                "FloatCallable",
                "float_functions",
                "float_function1",
                "data::graph::FloatFunctionLocalId(1)",
                "values.float_function(1)?",
                "float_function",
            ),
            (
                CallLocal::String(StringLocalId(1)),
                "StringValue",
                "strings",
                "string1",
                "data::graph::StringLocalId(1)",
                "values.string(1)?",
                "string",
            ),
            (
                CallLocal::StringList {
                    local: StringListLocalId(1),
                    type_id: StringListTypeId {
                        list_type: ListTypeId(0),
                    },
                },
                "StringList",
                "string_lists",
                "string_list1",
                "data::graph::StringListLocalId(1)",
                "values.string_list(1)?",
                "string_list",
            ),
            (
                CallLocal::StringFunction {
                    local: StringFunctionLocalId(1),
                    type_: FunctionType::new(Vec::new(), ValueType::String),
                },
                "StringCallable",
                "string_functions",
                "string_function1",
                "data::graph::StringFunctionLocalId(1)",
                "values.string_function(1)?",
                "string_function",
            ),
            (
                CallLocal::BitArray(BitArrayLocalId(1)),
                "CallBitArray",
                "bit_arrays",
                "bit_array1",
                "data::graph::BitArrayLocalId(1)",
                "values.bit_array(1)?",
                "bit_array",
            ),
            (
                CallLocal::BitArrayList {
                    local: BitArrayListLocalId(1),
                    type_id: BitArrayListTypeId {
                        list_type: ListTypeId(0),
                    },
                },
                "BitArrayList",
                "bit_array_lists",
                "bit_array_list1",
                "data::graph::BitArrayListLocalId(1)",
                "values.bit_array_list(1)?",
                "bit_array_list",
            ),
            (
                CallLocal::BitArrayFunction {
                    local: BitArrayFunctionLocalId(1),
                    type_: FunctionType::new(Vec::new(), ValueType::BitArray),
                },
                "BitArrayCallable",
                "bit_array_functions",
                "bit_array_function1",
                "data::graph::BitArrayFunctionLocalId(1)",
                "values.bit_array_function(1)?",
                "bit_array_function",
            ),
            (
                CallLocal::UtfCodepoint(UtfCodepointLocalId(1)),
                "char",
                "utf_codepoints",
                "utf_codepoint1",
                "data::graph::UtfCodepointLocalId(1)",
                "values.utf_codepoint(1)?",
                "utf_codepoint",
            ),
            (
                CallLocal::UtfCodepointList {
                    local: UtfCodepointListLocalId(1),
                    type_id: UtfCodepointListTypeId {
                        list_type: ListTypeId(0),
                    },
                },
                "UtfCodepointList",
                "utf_codepoint_lists",
                "utf_codepoint_list1",
                "data::graph::UtfCodepointListLocalId(1)",
                "values.utf_codepoint_list(1)?",
                "utf_codepoint_list",
            ),
            (
                CallLocal::UtfCodepointFunction {
                    local: UtfCodepointFunctionLocalId(1),
                    type_: FunctionType::new(Vec::new(), ValueType::UtfCodepoint),
                },
                "UtfCodepointCallable",
                "utf_codepoint_functions",
                "utf_codepoint_function1",
                "data::graph::UtfCodepointFunctionLocalId(1)",
                "values.utf_codepoint_function(1)?",
                "utf_codepoint_function",
            ),
            (
                CallLocal::Nil(NilLocalId(1)),
                "()",
                "nils",
                "nil1",
                "data::graph::NilLocalId(1)",
                "()",
                "nil",
            ),
            (
                CallLocal::NilList {
                    local: NilListLocalId(1),
                    type_id: NilListTypeId {
                        list_type: ListTypeId(0),
                    },
                },
                "NilList",
                "nil_lists",
                "nil_list1",
                "data::graph::NilListLocalId(1)",
                "values.nil_list(1)?",
                "nil_list",
            ),
            (
                CallLocal::NilFunction {
                    local: NilFunctionLocalId(1),
                    type_: FunctionType::new(Vec::new(), ValueType::Nil),
                },
                "NilCallable",
                "nil_functions",
                "nil_function1",
                "data::graph::NilFunctionLocalId(1)",
                "values.nil_function(1)?",
                "nil_function",
            ),
        ] {
            assert_eq!(
                local.native_argument(),
                !expected_type.ends_with("List") && !expected_type.ends_with("Callable")
            );
            assert_eq!(
                (
                    local_type(&local),
                    local_column(&local),
                    local_name(&local),
                    local_id(&local),
                    load_value(&local),
                    capture_method(&local).unwrap()
                ),
                (
                    expected_type,
                    column,
                    name.to_owned(),
                    id.to_owned(),
                    input.to_owned(),
                    method
                )
            );
        }
        for (capture, expected) in [
            (
                Capture::Int {
                    target: IntLocalId(1),
                    source: IntLocalId(2),
                },
                "CallCapture::int(data::graph::IntLocalId(1), int2)",
            ),
            (
                Capture::IntList {
                    target: IntListLocalId(1),
                    source: IntListLocalId(2),
                },
                "CallCapture::int_list(data::graph::IntListLocalId(1), int_list2.clone())",
            ),
            (
                Capture::IntFunction {
                    target: IntFunctionLocalId(1),
                    source: IntFunctionLocalId(2),
                },
                "CallCapture::int_function(data::graph::IntFunctionLocalId(1), int_function2.clone())",
            ),
            (
                Capture::Bool {
                    target: BoolLocalId(1),
                    source: BoolLocalId(2),
                },
                "CallCapture::bool(data::graph::BoolLocalId(1), bool2)",
            ),
            (
                Capture::BoolList {
                    target: BoolListLocalId(1),
                    source: BoolListLocalId(2),
                },
                "CallCapture::bool_list(data::graph::BoolListLocalId(1), bool_list2.clone())",
            ),
            (
                Capture::BoolFunction {
                    target: BoolFunctionLocalId(1),
                    source: BoolFunctionLocalId(2),
                },
                "CallCapture::bool_function(data::graph::BoolFunctionLocalId(1), bool_function2.clone())",
            ),
            (
                Capture::Float {
                    target: FloatLocalId(1),
                    source: FloatLocalId(2),
                },
                "CallCapture::float(data::graph::FloatLocalId(1), float2)",
            ),
            (
                Capture::FloatList {
                    target: FloatListLocalId(1),
                    source: FloatListLocalId(2),
                },
                "CallCapture::float_list(data::graph::FloatListLocalId(1), float_list2.clone())",
            ),
            (
                Capture::FloatFunction {
                    target: FloatFunctionLocalId(1),
                    source: FloatFunctionLocalId(2),
                },
                "CallCapture::float_function(data::graph::FloatFunctionLocalId(1), float_function2.clone())",
            ),
            (
                Capture::String {
                    target: StringLocalId(1),
                    source: StringLocalId(2),
                },
                "CallCapture::string(data::graph::StringLocalId(1), string2.clone())",
            ),
            (
                Capture::StringList {
                    target: StringListLocalId(1),
                    source: StringListLocalId(2),
                },
                "CallCapture::string_list(data::graph::StringListLocalId(1), string_list2.clone())",
            ),
            (
                Capture::StringFunction {
                    target: StringFunctionLocalId(1),
                    source: StringFunctionLocalId(2),
                },
                "CallCapture::string_function(data::graph::StringFunctionLocalId(1), string_function2.clone())",
            ),
            (
                Capture::BitArray {
                    target: BitArrayLocalId(1),
                    source: BitArrayLocalId(2),
                },
                "CallCapture::bit_array(data::graph::BitArrayLocalId(1), bit_array2.clone())",
            ),
            (
                Capture::BitArrayList {
                    target: BitArrayListLocalId(1),
                    source: BitArrayListLocalId(2),
                },
                "CallCapture::bit_array_list(data::graph::BitArrayListLocalId(1), bit_array_list2.clone())",
            ),
            (
                Capture::BitArrayFunction {
                    target: BitArrayFunctionLocalId(1),
                    source: BitArrayFunctionLocalId(2),
                },
                "CallCapture::bit_array_function(data::graph::BitArrayFunctionLocalId(1), bit_array_function2.clone())",
            ),
            (
                Capture::UtfCodepoint {
                    target: UtfCodepointLocalId(1),
                    source: UtfCodepointLocalId(2),
                },
                "CallCapture::utf_codepoint(data::graph::UtfCodepointLocalId(1), utf_codepoint2)",
            ),
            (
                Capture::UtfCodepointList {
                    target: UtfCodepointListLocalId(1),
                    source: UtfCodepointListLocalId(2),
                },
                "CallCapture::utf_codepoint_list(data::graph::UtfCodepointListLocalId(1), utf_codepoint_list2.clone())",
            ),
            (
                Capture::UtfCodepointFunction {
                    target: UtfCodepointFunctionLocalId(1),
                    source: UtfCodepointFunctionLocalId(2),
                },
                "CallCapture::utf_codepoint_function(data::graph::UtfCodepointFunctionLocalId(1), utf_codepoint_function2.clone())",
            ),
            (
                Capture::Nil {
                    target: NilLocalId(1),
                    source: NilLocalId(2),
                },
                "CallCapture::nil(data::graph::NilLocalId(1), ())",
            ),
            (
                Capture::NilList {
                    target: NilListLocalId(1),
                    source: NilListLocalId(2),
                },
                "CallCapture::nil_list(data::graph::NilListLocalId(1), nil_list2.clone())",
            ),
            (
                Capture::NilFunction {
                    target: NilFunctionLocalId(1),
                    source: NilFunctionLocalId(2),
                },
                "CallCapture::nil_function(data::graph::NilFunctionLocalId(1), nil_function2.clone())",
            ),
        ] {
            assert_eq!(capture_expression(&capture), expected);
        }
        assert_eq!(fields(&[CallLocal::Nil(NilLocalId(3))]), " { nil3: () }");
        assert_eq!(pattern(&[CallLocal::Nil(NilLocalId(3))]), " { nil3: () }");
        assert_eq!(
            values(&[CallLocal::Nil(NilLocalId(3))], false),
            "Box::new(CallValues::default())"
        );
    }
    #[test]
    fn fieldless_custom_locals_keep_their_id_and_do_not_claim_a_capture_adapter() {
        use super::super::nullary::CustomLocalShape;
        use crate::plan::execution::graph::{CustomLocal, CustomLocalId};
        use crate::plan::execution::type_::{
            CustomConstructorId, CustomTypeId, CustomValueShape, CustomValueShapeId,
        };
        let shape = CustomLocalShape {
            local: CustomLocal {
                id: CustomLocalId(2),
                shape: CustomValueShape {
                    type_id: CustomTypeId(0),
                    shape_id: CustomValueShapeId(0),
                },
            },
            arguments: Vec::new(),
            constructors: vec![CustomConstructorId {
                type_id: CustomTypeId(0),
                index: 0,
            }],
        };
        assert_eq!(
            local_id(&CallLocal::Custom(shape.clone())),
            "data::graph::CustomLocalId(2)"
        );
        let local = CallLocal::Nullary(shape);
        assert_eq!(local_id(&local), "data::graph::CustomLocalId(2)");
        assert_eq!(capture_method(&local), None);
        assert_eq!(super::capture_input_expression(&local), "{ return None; }");
        assert_eq!(
            values(std::slice::from_ref(&local), false),
            "Box::new(CallValues { customs: vec![nullary2.into()], ..CallValues::default() })"
        );
        assert_eq!(
            super::values_with_result(std::slice::from_ref(&local), false, Some(&local)),
            "Box::new(CallValues { customs: vec![nullary2], ..CallValues::default() })"
        );
    }
}
