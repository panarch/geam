use crate::embedding::boundary::{DataType, FunctionBinding, PlainBindings};

pub(super) fn push_function_field(output: &mut String, index: usize, function: &FunctionBinding) {
    let type_ = TypeExpression::Apply(
        "Function",
        vec![
            TypeExpression::Tuple(function.arguments.iter().map(DataType::rust_type).collect()),
            function.return_type.rust_type(),
            TypeExpression::Name(format!("Function{index}Input")),
        ],
    );
    let prefix = format!("    pub {}:", function.rust_name.as_str());
    let inline = type_.inline();
    let can_inline = type_.can_inline();
    if can_inline && prefix.len() + 1 + inline.len() < 100 {
        output.push_str(&format!("{prefix} {inline},\n"));
    } else if can_inline && 8 + inline.len() < 100 {
        output.push_str(&format!("{prefix}\n        {inline},\n"));
    } else if prefix.len() + " Function<".len() <= 100 {
        output.push_str(&format!("{prefix} "));
        type_.push(output, 4, prefix.len() + 1, 100);
        output.push_str(",\n");
    } else {
        output.push_str(&format!("{prefix}\n        "));
        type_.push(output, 8, 8, 100);
        output.push_str(",\n");
    }
}

pub(super) fn push_input_shapes(output: &mut String, bindings: &PlainBindings) {
    for (index, function) in bindings.functions().enumerate() {
        output.push_str(&format!("pub struct Function{index}Input;\n\n"));
        let mut parameters = Vec::new();
        let input = TypeExpression::Tuple(
            function
                .arguments
                .iter()
                .map(|type_| type_.input_type(&mut parameters))
                .collect(),
        );
        let generics = if parameters.is_empty() {
            String::new()
        } else {
            format!("<{}>", parameters.join(", "))
        };
        let relation = TypeExpression::Apply("InputShape", vec![input]);
        let line = format!(
            "impl{generics} {} for Function{index}Input {{}}\n",
            relation.inline()
        );
        if relation.can_inline() && line.trim_end().len() <= 100 {
            output.push_str(&line);
        } else {
            if 4 + generics.len() <= 100 {
                output.push_str(&format!("impl{generics}\n    "));
            } else {
                // Rustfmt style editions disagree on this impl's generic indentation.
                output.push_str("#[rustfmt::skip]\nimpl<\n");
                for parameter in parameters {
                    output.push_str(&format!("    {parameter},\n"));
                }
                output.push_str(">\n    ");
            }
            let suffix = format!(" for Function{index}Input");
            relation.push(output, 4, 4, 100 - suffix.len());
            output.push_str(&suffix);
            output.push_str("\n{\n}\n");
        }
        output.push('\n');
    }
}

impl DataType {
    fn rust_type(&self) -> TypeExpression {
        match self {
            Self::Int => TypeExpression::Name("BigInt".to_owned()),
            Self::Float => TypeExpression::Name("f64".to_owned()),
            Self::String => TypeExpression::Name("EcoString".to_owned()),
            Self::BitArray => TypeExpression::Name("BitArrayValue".to_owned()),
            Self::UtfCodepoint => TypeExpression::Name("char".to_owned()),
            Self::Bool => TypeExpression::Name("bool".to_owned()),
            Self::Nil => TypeExpression::Name("()".to_owned()),
            Self::Tuple(elements) => {
                TypeExpression::Tuple(elements.iter().map(Self::rust_type).collect())
            }
            Self::Result(ok, error) => {
                TypeExpression::Apply("Result", vec![ok.rust_type(), error.rust_type()])
            }
            Self::Option(item) => TypeExpression::Apply("Option", vec![item.rust_type()]),
            Self::List(item) => TypeExpression::Apply("List", vec![item.rust_type()]),
        }
    }

    fn input_type(&self, parameters: &mut Vec<String>) -> TypeExpression {
        match self {
            Self::List(_) => {
                let name = format!("Input{}", parameters.len());
                parameters.push(name.clone());
                TypeExpression::Name(name)
            }
            Self::Tuple(elements) => TypeExpression::Tuple(
                elements
                    .iter()
                    .map(|element| element.input_type(parameters))
                    .collect(),
            ),
            Self::Result(ok, error) => TypeExpression::Apply(
                "Result",
                vec![ok.input_type(parameters), error.input_type(parameters)],
            ),
            Self::Option(item) => {
                TypeExpression::Apply("Option", vec![item.input_type(parameters)])
            }
            _ => self.rust_type(),
        }
    }
}

enum TypeExpression {
    Name(String),
    Tuple(Vec<TypeExpression>),
    Apply(&'static str, Vec<TypeExpression>),
}

impl TypeExpression {
    fn inline(&self) -> String {
        match self {
            Self::Name(name) => name.clone(),
            Self::Tuple(elements) => {
                let fields = elements
                    .iter()
                    .map(Self::inline)
                    .collect::<Vec<_>>()
                    .join(", ");
                if elements.len() == 1 {
                    format!("({fields},)")
                } else {
                    format!("({fields})")
                }
            }
            Self::Apply(name, arguments) => format!(
                "{name}<{}>",
                arguments
                    .iter()
                    .map(Self::inline)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }

    fn push(&self, output: &mut String, indent: usize, column: usize, width: usize) -> usize {
        let (opening, elements, closing) = match self {
            Self::Name(name) => {
                output.push_str(name);
                return column + name.len();
            }
            Self::Tuple(elements) => ("(".to_owned(), elements, ")"),
            Self::Apply(name, arguments) => (format!("{name}<"), arguments, ">"),
        };
        let inline = self.inline();
        if self.can_inline() && column + inline.len() <= width {
            output.push_str(&inline);
            return column + inline.len();
        }
        if let Self::Apply(name, arguments) = self
            && let [tuple @ Self::Tuple(_)] = arguments.as_slice()
        {
            output.push_str(&opening);
            let last_column = tuple.push(output, indent, column + name.len() + 1, width - 1);
            output.push('>');
            return last_column + 1;
        }
        output.push_str(&opening);
        output.push('\n');
        for element in elements {
            output.push_str(&" ".repeat(indent + 4));
            element.push(output, indent + 4, indent + 4, 99);
            output.push_str(",\n");
        }
        output.push_str(&" ".repeat(indent));
        output.push_str(closing);
        indent + closing.len()
    }

    fn can_inline(&self) -> bool {
        match self {
            Self::Name(_) => true,
            Self::Tuple(_) => {
                // Rustfmt limits tuple contents to its default 60-column call width.
                self.inline().len() <= 62
            }
            Self::Apply(_, arguments) => arguments.iter().all(Self::can_inline),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{TypeExpression, push_function_field, push_input_shapes};
    use crate::embedding::boundary::{DataType, FunctionBinding, PlainBindings};
    use crate::embedding::identifier::RustIdentifier;
    use std::fs;

    #[test]
    fn renders_recursive_types_and_independent_input_carriers_exactly() {
        let data = DataType::Tuple(vec![
            DataType::Result(
                Box::new(DataType::List(Box::new(DataType::List(Box::new(
                    DataType::Int,
                ))))),
                Box::new(DataType::String),
            ),
            DataType::Option(Box::new(DataType::List(Box::new(DataType::Bool)))),
            DataType::Tuple(vec![DataType::BitArray]),
        ]);
        assert_eq!(
            data.rust_type().inline(),
            "(Result<List<List<BigInt>>, EcoString>, Option<List<bool>>, (BitArrayValue,))"
        );
        let mut parameters = Vec::new();
        assert_eq!(
            data.input_type(&mut parameters).inline(),
            "(Result<Input0, EcoString>, Option<Input1>, (BitArrayValue,))"
        );
        assert_eq!(parameters, ["Input0", "Input1"]);
        assert_eq!(TypeExpression::Tuple(Vec::new()).inline(), "()");

        let bindings = PlainBindings {
            geam_alias: RustIdentifier::parse("runtime").expect("fixture alias"),
            root_module: "boundary".to_owned(),
            first: FunctionBinding {
                gleam_name: "rows".to_owned(),
                rust_name: RustIdentifier::parse("rows").expect("fixture field"),
                arguments: vec![
                    DataType::List(Box::new(DataType::String)),
                    DataType::Option(Box::new(DataType::List(Box::new(DataType::Int)))),
                    DataType::String,
                ],
                return_type: DataType::Nil,
            },
            remaining: Vec::new(),
        };
        let mut output = String::new();
        push_input_shapes(&mut output, &bindings);
        assert_eq!(
            output,
            "pub struct Function0Input;\n\nimpl<Input0, Input1> InputShape<(Input0, Option<Input1>, EcoString)> for Function0Input {}\n\n"
        );
        assert_rustfmt_stable(&output);
    }

    #[test]
    fn renders_the_longest_scalar_function_field_exactly() {
        let short = FunctionBinding {
            gleam_name: "all_bits".to_owned(),
            rust_name: RustIdentifier::parse("all_bits").expect("fixture function should be valid"),
            arguments: vec![DataType::BitArray; 7],
            return_type: DataType::BitArray,
        };
        let long_name = "function_with_a_deliberately_long_but_representable_name_that_remains_part_of_the_public_boundary";
        let long = FunctionBinding {
            gleam_name: long_name.to_owned(),
            rust_name: RustIdentifier::parse(long_name)
                .expect("long fixture function should be valid"),
            arguments: vec![DataType::BitArray; 7],
            return_type: DataType::BitArray,
        };
        let mut field = String::new();
        push_function_field(&mut field, 0, &short);
        push_function_field(&mut field, 1, &long);
        assert_eq!(
            field,
            r#"    pub all_bits: Function<
        (
            BitArrayValue,
            BitArrayValue,
            BitArrayValue,
            BitArrayValue,
            BitArrayValue,
            BitArrayValue,
            BitArrayValue,
        ),
        BitArrayValue,
        Function0Input,
    >,
    pub function_with_a_deliberately_long_but_representable_name_that_remains_part_of_the_public_boundary:
        Function<
            (
                BitArrayValue,
                BitArrayValue,
                BitArrayValue,
                BitArrayValue,
                BitArrayValue,
                BitArrayValue,
                BitArrayValue,
            ),
            BitArrayValue,
            Function1Input,
        >,
"#
        );
        assert_rustfmt_stable(&format!("pub struct Functions {{\n{field}}}\n"));
    }

    #[test]
    fn reserves_the_trailing_comma_at_the_field_width_boundary() {
        let name = "normalize_inventory_code_before_exporting";
        let function = FunctionBinding {
            gleam_name: name.to_owned(),
            rust_name: RustIdentifier::parse(name).expect("fixture function"),
            arguments: vec![DataType::String],
            return_type: DataType::String,
        };
        let mut source = "pub struct Functions {\n".to_owned();
        push_function_field(&mut source, 0, &function);
        source.push_str("}\n");
        assert_eq!(
            source,
            "pub struct Functions {\n    pub normalize_inventory_code_before_exporting:\n        Function<(EcoString,), EcoString, Function0Input>,\n}\n"
        );
        assert_rustfmt_stable(&source);
    }

    #[test]
    fn reserves_the_trailing_comma_inside_a_recursive_field() {
        let function = FunctionBinding {
            gleam_name: "parse_frames".to_owned(),
            rust_name: RustIdentifier::parse("parse_frames").expect("fixture function"),
            arguments: Vec::new(),
            return_type: DataType::Result(
                Box::new(DataType::Tuple(vec![
                    DataType::BitArray,
                    DataType::BitArray,
                    DataType::BitArray,
                    DataType::BitArray,
                    DataType::String,
                ])),
                Box::new(DataType::Option(Box::new(DataType::Float))),
            ),
        };
        let mut source = "pub struct Functions {\n".to_owned();
        push_function_field(&mut source, 0, &function);
        source.push_str("}\n");
        assert_eq!(
            source,
            r#"pub struct Functions {
    pub parse_frames: Function<
        (),
        Result<
            (
                BitArrayValue,
                BitArrayValue,
                BitArrayValue,
                BitArrayValue,
                EcoString,
            ),
            Option<f64>,
        >,
        Function0Input,
    >,
}
"#
        );
        assert_rustfmt_stable(&source);
    }

    #[test]
    fn follows_the_tuple_content_width_through_nested_types() {
        let short = FunctionBinding {
            gleam_name: "short".to_owned(),
            rust_name: RustIdentifier::parse("short").expect("fixture function"),
            arguments: Vec::new(),
            return_type: DataType::Tuple(vec![
                DataType::BitArray,
                DataType::BitArray,
                DataType::BitArray,
                DataType::Bool,
                DataType::Bool,
                DataType::Float,
            ]),
        };
        let long = FunctionBinding {
            gleam_name: "long".to_owned(),
            rust_name: RustIdentifier::parse("long").expect("fixture function"),
            arguments: Vec::new(),
            return_type: DataType::Tuple(vec![
                DataType::BitArray,
                DataType::BitArray,
                DataType::BitArray,
                DataType::Bool,
                DataType::Bool,
                DataType::UtfCodepoint,
            ]),
        };
        let optional = FunctionBinding {
            gleam_name: "optional".to_owned(),
            rust_name: RustIdentifier::parse("optional").expect("fixture function"),
            arguments: Vec::new(),
            return_type: DataType::Option(Box::new(DataType::Tuple(vec![DataType::BitArray; 5]))),
        };
        let mut source = "pub struct Functions {\n".to_owned();
        push_function_field(&mut source, 0, &short);
        push_function_field(&mut source, 1, &long);
        push_function_field(&mut source, 2, &optional);
        source.push_str("}\n");
        assert_eq!(
            source,
            r#"pub struct Functions {
    pub short: Function<
        (),
        (BitArrayValue, BitArrayValue, BitArrayValue, bool, bool, f64),
        Function0Input,
    >,
    pub long: Function<
        (),
        (
            BitArrayValue,
            BitArrayValue,
            BitArrayValue,
            bool,
            bool,
            char,
        ),
        Function1Input,
    >,
    pub optional: Function<
        (),
        Option<(
            BitArrayValue,
            BitArrayValue,
            BitArrayValue,
            BitArrayValue,
            BitArrayValue,
        )>,
        Function2Input,
    >,
}
"#
        );
        assert_rustfmt_stable(&source);
    }

    #[test]
    fn keeps_recursive_fields_and_long_input_shapes_formatted() {
        let bindings = PlainBindings {
            geam_alias: RustIdentifier::parse("runtime").expect("fixture alias"),
            root_module: "boundary".to_owned(),
            first: FunctionBinding {
                gleam_name: "recursive".to_owned(),
                rust_name: RustIdentifier::parse("recursive").expect("fixture field"),
                arguments: vec![DataType::Result(
                    Box::new(DataType::Tuple(vec![DataType::BitArray; 7])),
                    Box::new(DataType::Tuple(vec![
                        DataType::List(Box::new(DataType::Int));
                        7
                    ])),
                )],
                return_type: DataType::List(Box::new(DataType::Option(Box::new(DataType::Tuple(
                    vec![DataType::BitArray; 7],
                ))))),
            },
            remaining: Vec::new(),
        };
        let mut source = "pub struct Functions {\n".to_owned();
        push_function_field(&mut source, 0, &bindings.first);
        source.push_str("}\n\n");
        push_input_shapes(&mut source, &bindings);
        assert_eq!(
            source,
            r#"pub struct Functions {
    pub recursive: Function<
        (
            Result<
                (
                    BitArrayValue,
                    BitArrayValue,
                    BitArrayValue,
                    BitArrayValue,
                    BitArrayValue,
                    BitArrayValue,
                    BitArrayValue,
                ),
                (
                    List<BigInt>,
                    List<BigInt>,
                    List<BigInt>,
                    List<BigInt>,
                    List<BigInt>,
                    List<BigInt>,
                    List<BigInt>,
                ),
            >,
        ),
        List<
            Option<(
                BitArrayValue,
                BitArrayValue,
                BitArrayValue,
                BitArrayValue,
                BitArrayValue,
                BitArrayValue,
                BitArrayValue,
            )>,
        >,
        Function0Input,
    >,
}

pub struct Function0Input;

impl<Input0, Input1, Input2, Input3, Input4, Input5, Input6>
    InputShape<(
        Result<
            (
                BitArrayValue,
                BitArrayValue,
                BitArrayValue,
                BitArrayValue,
                BitArrayValue,
                BitArrayValue,
                BitArrayValue,
            ),
            (Input0, Input1, Input2, Input3, Input4, Input5, Input6),
        >,
    )> for Function0Input
{
}

"#
        );
        assert_rustfmt_stable(&source);
    }

    #[test]
    fn formats_independent_list_carriers_across_nested_tuples() {
        let bindings = PlainBindings {
            geam_alias: RustIdentifier::parse("runtime").expect("fixture alias"),
            root_module: "boundary".to_owned(),
            first: FunctionBinding {
                gleam_name: "many_lists".to_owned(),
                rust_name: RustIdentifier::parse("many_lists").expect("fixture field"),
                arguments: vec![
                    DataType::Tuple(vec![DataType::List(Box::new(DataType::Bool)); 7]);
                    2
                ],
                return_type: DataType::Nil,
            },
            remaining: Vec::new(),
        };
        let mut source = String::new();
        push_input_shapes(&mut source, &bindings);
        assert_eq!(
            source,
            r#"pub struct Function0Input;

#[rustfmt::skip]
impl<
    Input0,
    Input1,
    Input2,
    Input3,
    Input4,
    Input5,
    Input6,
    Input7,
    Input8,
    Input9,
    Input10,
    Input11,
    Input12,
    Input13,
>
    InputShape<(
        (Input0, Input1, Input2, Input3, Input4, Input5, Input6),
        (Input7, Input8, Input9, Input10, Input11, Input12, Input13),
    )> for Function0Input
{
}

"#
        );
        assert_rustfmt_stable(&source);
    }

    #[test]
    fn wraps_wide_fixed_inputs_with_the_owner_suffix() {
        let bindings = PlainBindings {
            geam_alias: RustIdentifier::parse("runtime").expect("fixture alias"),
            root_module: "boundary".to_owned(),
            first: FunctionBinding {
                gleam_name: "bits".to_owned(),
                rust_name: RustIdentifier::parse("bits").expect("fixture field"),
                arguments: vec![DataType::BitArray; 5],
                return_type: DataType::Nil,
            },
            remaining: Vec::new(),
        };
        let mut source = String::new();
        push_input_shapes(&mut source, &bindings);
        assert_eq!(
            source,
            "pub struct Function0Input;\n\nimpl\n    InputShape<(\n        BitArrayValue,\n        BitArrayValue,\n        BitArrayValue,\n        BitArrayValue,\n        BitArrayValue,\n    )> for Function0Input\n{\n}\n\n"
        );
        assert_rustfmt_stable(&source);
    }

    fn assert_rustfmt_stable(source: &str) {
        let directory = tempfile::tempdir().expect("temporary directory");
        let path = directory.path().join("bindings.rs");
        fs::write(&path, format!("{}\n", source.trim_end())).expect("write generated types");
        for style_edition in ["2015", "2024"] {
            let output = std::process::Command::new("rustfmt")
                .args([
                    "--edition",
                    "2024",
                    "--style-edition",
                    style_edition,
                    "--check",
                ])
                .arg(&path)
                .output()
                .expect("rustfmt should start");
            let details = format!(
                "generated types should remain formatted ({style_edition}):\n{}\n{}\n{source}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            assert!(output.status.success(), "{details}",);
        }
    }
}
