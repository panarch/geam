use crate::plan::execution::storage::Storage;
use std::convert::Infallible;
use std::marker::PhantomData;
use std::ops::Range;

pub(crate) trait Emit {
    fn emit(&self, output: &mut Rust);
}

pub(crate) struct Rust {
    output: String,
}

impl Rust {
    pub(crate) fn expression(value: &impl Emit) -> String {
        let mut output = Self {
            output: String::new(),
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
        for (name, value) in fields {
            self.output.push_str(name);
            self.output.push_str(": ");
            value.emit(self);
            self.output.push(',');
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
        for value in fields {
            value.emit(self);
            self.output.push(',');
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
        for value in self {
            value.emit(output);
            output.output.push(',');
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
    use super::{PhantomData, Rust};
    use crate::plan::execution::storage::{Node, Table};

    #[test]
    fn emits_static_expressions_without_owned_constructors() {
        let values = Table::from(vec![Some((2usize..5, true)), None]);
        assert_eq!(
            Rust::expression(&values),
            "data::Storage::Static(&[Some((2..5,true,)),None,])"
        );
        assert_eq!(
            Rust::expression(&Node::from(Box::new(7u32))),
            "data::Storage::Static(&7)"
        );
        assert_eq!(Rust::expression(&[1usize, 2]), "[1,2,]");
        assert_eq!(
            Rust::expression(&PhantomData::<str>),
            "::core::marker::PhantomData"
        );
        assert_eq!(Rust::expression(&"a\n\"b\""), "\"a\\n\\\"b\\\"\"");
        assert_eq!(Rust::expression(&'\n'), "'\\n'");
        assert_eq!(Rust::expression(&false), "false");
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
