use crate::plan::execution::storage::Storage;
use std::convert::Infallible;
use std::marker::PhantomData;
use std::ops::Range;

pub(crate) trait Emit {
    fn emit(&self, output: &mut Rust);
}

pub(crate) struct Rust {
    output: String,
    indentation: usize,
}

impl Rust {
    pub(crate) fn expression(value: &impl Emit) -> String {
        let mut output = Self {
            output: String::new(),
            indentation: 0,
        };
        value.emit(&mut output);
        output.output
    }

    pub(crate) fn path(&mut self, path: &str) {
        self.output.push_str("data::");
        self.output.push_str(path);
    }

    pub(crate) fn structure(&mut self, path: &str, fields: &[(&str, &dyn Emit)]) {
        self.path(path);
        self.output.push_str(" {");
        self.indentation += 1;
        for (name, value) in fields {
            self.newline();
            self.output.push_str(name);
            self.output.push_str(": ");
            value.emit(self);
            self.output.push(',');
        }
        self.indentation -= 1;
        if !fields.is_empty() {
            self.newline();
        }
        self.output.push('}');
    }

    pub(crate) fn variant(&mut self, path: &str, fields: &[&dyn Emit]) {
        self.path(path);
        self.output.push('(');
        self.tuple_fields(fields);
        self.output.push(')');
    }

    pub(crate) fn call(&mut self, path: &str, arguments: &[&dyn Emit]) {
        self.variant(path, arguments);
    }

    fn tuple_fields(&mut self, fields: &[&dyn Emit]) {
        for (index, value) in fields.iter().enumerate() {
            if index != 0 {
                self.output.push_str(", ");
            }
            value.emit(self);
        }
    }

    fn newline(&mut self) {
        self.output.push('\n');
        for _ in 0..self.indentation {
            self.output.push_str("    ");
        }
    }
}

impl<Value: Emit + ?Sized + 'static> Emit for Storage<Value> {
    fn emit(&self, output: &mut Rust) {
        output.path("Storage::Static");
        output.output.push_str("(&");
        self.as_ref().emit(output);
        output.output.push(')');
    }
}

impl<Value: Emit> Emit for [Value] {
    fn emit(&self, output: &mut Rust) {
        output.output.push('[');
        output.indentation += 1;
        for value in self {
            output.newline();
            value.emit(output);
            output.output.push(',');
        }
        output.indentation -= 1;
        if !self.is_empty() {
            output.newline();
        }
        output.output.push(']');
    }
}

impl<Value: Emit, const SIZE: usize> Emit for [Value; SIZE] {
    fn emit(&self, output: &mut Rust) {
        self.as_slice().emit(output);
    }
}

impl<Value: Emit> Emit for Option<Value> {
    fn emit(&self, output: &mut Rust) {
        match self {
            Some(value) => {
                output.output.push_str("Some(");
                value.emit(output);
                output.output.push(')');
            }
            None => output.output.push_str("None"),
        }
    }
}

impl<Value: Emit> Emit for Range<Value> {
    fn emit(&self, output: &mut Rust) {
        self.start.emit(output);
        output.output.push_str("..");
        self.end.emit(output);
    }
}

impl<Left: Emit, Right: Emit> Emit for (Left, Right) {
    fn emit(&self, output: &mut Rust) {
        output.output.push('(');
        output.tuple_fields(&[&self.0, &self.1]);
        output.output.push(')');
    }
}

impl<Value: ?Sized> Emit for PhantomData<Value> {
    fn emit(&self, output: &mut Rust) {
        output.output.push_str("::core::marker::PhantomData");
    }
}

impl Emit for Infallible {
    fn emit(&self, _: &mut Rust) {
        match *self {}
    }
}

impl Emit for usize {
    fn emit(&self, output: &mut Rust) {
        output.output.push_str(&self.to_string());
    }
}

impl Emit for u32 {
    fn emit(&self, output: &mut Rust) {
        output.output.push_str(&self.to_string());
    }
}

impl Emit for u8 {
    fn emit(&self, output: &mut Rust) {
        output.output.push_str(&self.to_string());
    }
}

impl Emit for f64 {
    fn emit(&self, output: &mut Rust) {
        output.output.push_str("f64::from_bits(");
        output.output.push_str(&self.to_bits().to_string());
        output.output.push(')');
    }
}

impl Emit for bool {
    fn emit(&self, output: &mut Rust) {
        output.output.push_str(if *self { "true" } else { "false" });
    }
}

impl Emit for str {
    fn emit(&self, output: &mut Rust) {
        output.output.push_str(&format!("{self:?}"));
    }
}

/// Multi-line text emitted as a raw string literal whose first line break only
/// separates the opening delimiter from the text.
pub(crate) struct TextBlock<'text> {
    text: &'text str,
    hashes: usize,
}

impl<'text> TextBlock<'text> {
    /// Accepts multi-line text only when every character appears as itself and
    /// the literal needs at most the 255 `#` delimiters Rust permits. Other text
    /// keeps the escaped string form: rustc would normalize a CRLF line ending,
    /// reject a lone carriage return or a direction control, and leave invisible
    /// characters unreadable.
    pub(crate) fn new(text: &'text str) -> Option<Self> {
        let hashes = raw_string_hashes(text);
        (text.contains('\n')
            && hashes <= 255
            && text.chars().all(|character| {
                matches!(character, '\n' | '\t' | '"' | '\'' | '\\')
                    || character.escape_debug().eq([character])
            }))
        .then_some(Self { text, hashes })
    }
}

impl Emit for TextBlock<'_> {
    fn emit(&self, output: &mut Rust) {
        let hashes = "#".repeat(self.hashes);
        output.output.push('r');
        output.output.push_str(&hashes);
        output.output.push_str("\"\n");
        output.output.push_str(self.text);
        output.output.push('"');
        output.output.push_str(&hashes);
    }
}

/// Counts one more `#` than the longest run following a quote in the text.
fn raw_string_hashes(text: &str) -> usize {
    text.split('"')
        .skip(1)
        .map(|after| after.len() - after.trim_start_matches('#').len())
        .max()
        .map_or(1, |run| run + 1)
}

impl<Value: Emit + ?Sized> Emit for &Value {
    fn emit(&self, output: &mut Rust) {
        (**self).emit(output);
    }
}

impl<Value: Emit + ?Sized> Emit for std::sync::Arc<Value> {
    fn emit(&self, output: &mut Rust) {
        self.as_ref().emit(output);
    }
}

impl Emit for char {
    fn emit(&self, output: &mut Rust) {
        output.output.push_str(&format!("{self:?}"));
    }
}

impl Emit for num_bigint::Sign {
    fn emit(&self, output: &mut Rust) {
        output.path(match self {
            Self::Minus => "Sign::Minus",
            Self::NoSign => "Sign::NoSign",
            Self::Plus => "Sign::Plus",
        });
    }
}

impl Emit for crate::plan::TypeParameterId {
    fn emit(&self, output: &mut Rust) {
        output.call("type_::parameter_id", &[&self.index()]);
    }
}

impl Emit for crate::plan::ModuleId {
    fn emit(&self, output: &mut Rust) {
        output.call("source::module_id", &[&self.index()]);
    }
}

#[cfg(test)]
mod tests {
    use super::{PhantomData, Rust, TextBlock};
    use crate::plan::execution::storage::{Node, Table};

    #[test]
    fn emits_static_expressions_without_owned_constructors() {
        let values = Table::from(vec![Some((2usize..5, true)), None]);
        assert_eq!(
            Rust::expression(&values),
            r#"
data::Storage::Static(&[
    Some((2..5, true)),
    None,
])"#
            .trim_start_matches('\n')
        );
        assert_eq!(
            Rust::expression(&Node::from(Box::new(7u32))),
            "data::Storage::Static(&7)"
        );
        assert_eq!(Rust::expression(&[1usize, 2]), "[\n    1,\n    2,\n]");
        assert_eq!(
            Rust::expression(&PhantomData::<str>),
            "::core::marker::PhantomData"
        );
        assert_eq!(Rust::expression(&"a\n\"b\""), "\"a\\n\\\"b\\\"\"");
        assert_eq!(Rust::expression(&'\n'), "'\\n'");
        assert_eq!(Rust::expression(&false), "false");
    }

    #[test]
    fn emits_verbatim_multi_line_text_as_a_raw_block_after_one_line_break() {
        let source = TextBlock::new("fn main() {\n  'quoted\\n' <> \"text\"\n}\n").unwrap();
        assert_eq!(
            Rust::expression(&source),
            r##"
r#"
fn main() {
  'quoted\n' <> "text"
}
"#"##
                .trim_start_matches('\n')
        );
        let marker = TextBlock::new("let marker = \"#\"\n").unwrap();
        assert_eq!(
            Rust::expression(&marker),
            r###"
r##"
let marker = "#"
"##"###
                .trim_start_matches('\n')
        );
        let tabbed = TextBlock::new("a\tb\n").unwrap();
        assert_eq!(Rust::expression(&tabbed), "r#\"\na\tb\n\"#");
        let delimited = format!("\"{}\n", "#".repeat(254));
        let hashes = "#".repeat(255);
        assert_eq!(
            Rust::expression(&TextBlock::new(&delimited).unwrap()),
            format!("r{hashes}\"\n{delimited}\"{hashes}")
        );
    }

    #[test]
    fn keeps_single_line_non_verbatim_and_over_delimited_text_out_of_raw_blocks() {
        let over_delimited = format!("\"{}\n", "#".repeat(255));
        for text in [
            "single line",
            "carriage\r\nreturn\n",
            "direction \u{202e}control\n",
            "nul \0\n",
            &over_delimited,
        ] {
            assert!(TextBlock::new(text).is_none(), "{text:?}");
        }
    }

    #[test]
    fn indents_nested_fields_and_arrays_without_changing_literals() {
        let mut output = Rust {
            output: String::new(),
            indentation: 0,
        };
        output.structure(
            "Record",
            &[
                ("rows", &[[1u8, 2], [3, 4]]),
                ("text", &Some("{a, [b]}\n\"c\"\\")),
                ("empty", &[] as &[u8; 0]),
                ("last", &false),
            ],
        );
        assert_eq!(
            output.output,
            r#"
data::Record {
    rows: [
        [
            1,
            2,
        ],
        [
            3,
            4,
        ],
    ],
    text: Some("{a, [b]}\n\"c\"\\"),
    empty: [],
    last: false,
}"#
            .trim_start_matches('\n')
        );
        assert_eq!(output.indentation, 0);
        output.output.clear();
        output.structure("Empty", &[]);
        assert_eq!(output.output, "data::Empty {}");
        output.output.clear();
        output.call("empty", &[]);
        assert_eq!(output.output, "data::empty()");
        output.output.clear();
        output.variant("Pair", &[&1u8, &2u8]);
        assert_eq!(output.output, "data::Pair(1, 2)");
    }

    #[test]
    fn preserves_float_bits_and_integer_signs() {
        assert_eq!(
            Rust::expression(&-0.0f64),
            "f64::from_bits(9223372036854775808)"
        );
        assert_eq!(
            Rust::expression(&f64::from_bits(0x7ff8000000000037)),
            "f64::from_bits(9221120237041090615)"
        );
        assert_eq!(
            Rust::expression(&num_bigint::Sign::Minus),
            "data::Sign::Minus"
        );
        assert_eq!(
            Rust::expression(&num_bigint::Sign::NoSign),
            "data::Sign::NoSign"
        );
        assert_eq!(
            Rust::expression(&num_bigint::Sign::Plus),
            "data::Sign::Plus"
        );
    }
}
