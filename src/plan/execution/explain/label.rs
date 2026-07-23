mod direct;
mod function;
mod list;
mod runtime;

pub(super) use self::direct::ExplainFunctionId;
pub(super) use self::function::function_function_label;
pub(super) use self::list::list_function_label;
pub(super) use self::runtime::runtime_function_label;

#[derive(Clone, Copy)]
pub(super) struct FunctionLabel {
    family: &'static str,
    index: usize,
}

impl FunctionLabel {
    pub(super) fn new(family: &'static str, index: usize) -> Self {
        Self { family, index }
    }

    pub(super) fn push_to(self, output: &mut String) {
        output.push_str(self.family);
        output.push('#');
        output.push_str(&self.index.to_string());
    }
}
