use crate::embedding::boundary::{ClosedType, NamedKind, NamedType};

pub(super) fn push_types(output: &mut String, alias: &str, types: &[NamedType]) {
    for (index, type_) in types.iter().enumerate() {
        let kind = match type_.kind {
            NamedKind::Custom => "Custom",
            NamedKind::External => "External",
        };
        output.push_str(&format!(
            "/// Opaque `{package}:{module}.{name}` in this execution domain.\npub type Type{index} = {alias}::embedding::{kind}Type<Type{index}Schema>;\n\npub struct Type{index}Schema;\n\nimpl {alias}::embedding::NamedTypeSchema for Type{index}Schema {{\n    const PACKAGE: &'static str = {package:?};\n    const MODULE: &'static str = {module:?};\n    const NAME: &'static str = {name:?};\n",
            package = type_.package, module = type_.module, name = type_.name,
        ));
        if !type_.arguments.is_empty() {
            output.push_str(&format!(
                "\n    fn arguments() -> Vec<{alias}::ValueType> {{\n"
            ));
            output.push_str(&format!("        use {alias}::ValueType;\n\n"));
            let mut expressions = Expressions {
                output,
                alias,
                next: 0,
            };
            let arguments = expressions.values(&type_.arguments);
            expressions.push_call("        ", "vec!", &arguments, '[', ']', "");
            output.push_str("    }\n");
        }
        output.push_str("}\n\n");
    }
}

// Emit compound metadata bottom-up, keeping recursive schemas readable and
// line wrapping independent of the depth of their generic arguments.
struct Expressions<'output, 'alias> {
    output: &'output mut String,
    alias: &'alias str,
    next: usize,
}

impl Expressions<'_, '_> {
    fn value(&mut self, type_: &ClosedType) -> String {
        match type_ {
            ClosedType::Int => "ValueType::Int".to_owned(),
            ClosedType::Float => "ValueType::Float".to_owned(),
            ClosedType::String => "ValueType::String".to_owned(),
            ClosedType::BitArray => "ValueType::BitArray".to_owned(),
            ClosedType::UtfCodepoint => "ValueType::UtfCodepoint".to_owned(),
            ClosedType::Bool => "ValueType::Bool".to_owned(),
            ClosedType::Nil => "ValueType::Nil".to_owned(),
            ClosedType::List(item) => {
                let item = self.value(item);
                self.bind("ValueType::List", &[format!("Box::new({item})")])
            }
            ClosedType::Tuple(items) => {
                let items = self.vector(items);
                self.bind("ValueType::Tuple", &[items])
            }
            ClosedType::Function(arguments, return_) => {
                let arguments = self.vector(arguments);
                let return_ = self.value(return_);
                let function = self.bind(
                    &format!("{}::FunctionType::new", self.alias),
                    &[arguments, return_],
                );
                self.bind("ValueType::Function", &[format!("Box::new({function})")])
            }
            ClosedType::Named(type_) => {
                let kind = match type_.kind {
                    NamedKind::Custom => "Custom",
                    NamedKind::External => "External",
                };
                let name = self.bind(
                    &format!("{}::plan::{kind}TypeName::new", self.alias),
                    &[
                        format!("{:?}.into()", type_.package),
                        format!("{:?}.into()", type_.module),
                        format!("{:?}.into()", type_.name),
                    ],
                );
                let arguments = self.vector(&type_.arguments);
                let named = self.bind(
                    &format!("{}::plan::{kind}Type::new", self.alias),
                    &[name, arguments],
                );
                self.bind(&format!("ValueType::{kind}"), &[named])
            }
        }
    }

    fn values(&mut self, types: &[ClosedType]) -> Vec<String> {
        types.iter().map(|type_| self.value(type_)).collect()
    }

    fn vector(&mut self, types: &[ClosedType]) -> String {
        let arguments = self.values(types);
        let name = self.name();
        self.push_call(
            &format!("        let {name} = "),
            "vec!",
            &arguments,
            '[',
            ']',
            ";",
        );
        name
    }

    fn name(&mut self) -> String {
        let name = format!("type_{}", self.next);
        self.next += 1;
        name
    }

    fn bind(&mut self, function: &str, arguments: &[String]) -> String {
        let name = self.name();
        self.push_call(
            &format!("        let {name} = "),
            function,
            arguments,
            '(',
            ')',
            ";",
        );
        name
    }

    fn push_call(
        &mut self,
        prefix: &str,
        function: &str,
        arguments: &[String],
        open: char,
        close: char,
        suffix: &str,
    ) {
        let arguments_inline = arguments.join(", ");
        let expression = format!("{function}{open}{arguments_inline}{close}");
        if arguments_inline.len() <= 60 && prefix.len() + expression.len() + suffix.len() <= 100 {
            self.output
                .push_str(&format!("{prefix}{expression}{suffix}\n"));
        } else if arguments_inline.len() <= 60 && 12 + expression.len() + suffix.len() <= 100 {
            self.output.push_str(&format!(
                "{}\n            {expression}{suffix}\n",
                prefix.trim_end()
            ));
        } else {
            self.output.push_str(&format!("{prefix}{function}{open}\n"));
            for argument in arguments {
                self.output.push_str(&format!("            {argument},\n"));
            }
            self.output.push_str(&format!("        {close}{suffix}\n"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Expressions, push_types};
    use crate::embedding::boundary::{ClosedType, NamedKind, NamedType};

    #[test]
    fn emits_exact_schema_kinds_and_scalar_arguments() {
        let types = [
            NamedType {
                package: "application".into(),
                module: "library".into(),
                name: "Resource".into(),
                kind: NamedKind::External,
                arguments: Vec::new(),
            },
            NamedType {
                package: "application".into(),
                module: "library".into(),
                name: "Record".into(),
                kind: NamedKind::Custom,
                arguments: vec![
                    ClosedType::Int,
                    ClosedType::Float,
                    ClosedType::String,
                    ClosedType::BitArray,
                    ClosedType::UtfCodepoint,
                    ClosedType::Bool,
                    ClosedType::Nil,
                ],
            },
        ];
        let mut output = String::new();
        push_types(&mut output, "geam", &types);
        assert_eq!(
            output,
            r#"/// Opaque `application:library.Resource` in this execution domain.
pub type Type0 = geam::embedding::ExternalType<Type0Schema>;

pub struct Type0Schema;

impl geam::embedding::NamedTypeSchema for Type0Schema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "library";
    const NAME: &'static str = "Resource";
}

/// Opaque `application:library.Record` in this execution domain.
pub type Type1 = geam::embedding::CustomType<Type1Schema>;

pub struct Type1Schema;

impl geam::embedding::NamedTypeSchema for Type1Schema {
    const PACKAGE: &'static str = "application";
    const MODULE: &'static str = "library";
    const NAME: &'static str = "Record";

    fn arguments() -> Vec<geam::ValueType> {
        use geam::ValueType;

        vec![
            ValueType::Int,
            ValueType::Float,
            ValueType::String,
            ValueType::BitArray,
            ValueType::UtfCodepoint,
            ValueType::Bool,
            ValueType::Nil,
        ]
    }
}

"#,
        );
    }

    #[test]
    fn emits_exact_compound_metadata_bottom_up() {
        for (type_, expected, result) in [
            (
                ClosedType::List(Box::new(ClosedType::Int)),
                "        let type_0 = ValueType::List(Box::new(ValueType::Int));\n",
                "type_0",
            ),
            (
                ClosedType::Tuple(vec![ClosedType::Int, ClosedType::String]),
                "        let type_0 = vec![ValueType::Int, ValueType::String];\n        let type_1 = ValueType::Tuple(type_0);\n",
                "type_1",
            ),
            (
                ClosedType::Function(vec![ClosedType::Int], Box::new(ClosedType::String)),
                "        let type_0 = vec![ValueType::Int];\n        let type_1 = geam::FunctionType::new(type_0, ValueType::String);\n        let type_2 = ValueType::Function(Box::new(type_1));\n",
                "type_2",
            ),
            (
                ClosedType::Named(NamedType {
                    package: "app".into(),
                    module: "library".into(),
                    name: "Boxed".into(),
                    kind: NamedKind::Custom,
                    arguments: vec![ClosedType::Int],
                }),
                "        let type_0 =\n            geam::plan::CustomTypeName::new(\"app\".into(), \"library\".into(), \"Boxed\".into());\n        let type_1 = vec![ValueType::Int];\n        let type_2 = geam::plan::CustomType::new(type_0, type_1);\n        let type_3 = ValueType::Custom(type_2);\n",
                "type_3",
            ),
            (
                ClosedType::Named(NamedType {
                    package: "app".into(),
                    module: "library".into(),
                    name: "Handle".into(),
                    kind: NamedKind::External,
                    arguments: Vec::new(),
                }),
                "        let type_0 =\n            geam::plan::ExternalTypeName::new(\"app\".into(), \"library\".into(), \"Handle\".into());\n        let type_1 = vec![];\n        let type_2 = geam::plan::ExternalType::new(type_0, type_1);\n        let type_3 = ValueType::External(type_2);\n",
                "type_3",
            ),
        ] {
            let mut output = String::new();
            let mut expressions = Expressions {
                output: &mut output,
                alias: "geam",
                next: 0,
            };
            assert_eq!(expressions.value(&type_), result);
            assert_eq!(output, expected);
        }
    }

    #[test]
    fn wraps_a_long_constructor_after_the_assignment() {
        let mut output = String::new();
        let mut expressions = Expressions {
            output: &mut output,
            alias: "longer_runtime_dependency_with_a_long_name",
            next: 0,
        };
        assert_eq!(
            expressions.value(&ClosedType::Function(Vec::new(), Box::new(ClosedType::Int))),
            "type_2",
        );
        assert_eq!(
            output,
            "        let type_0 = vec![];\n        let type_1 =\n            longer_runtime_dependency_with_a_long_name::FunctionType::new(type_0, ValueType::Int);\n        let type_2 = ValueType::Function(Box::new(type_1));\n",
        );
    }
}
