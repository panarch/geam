use super::{CaptureArg, ParamLocal};

pub(crate) struct FunctionReference<Function> {
    function: Function,
    params: Vec<ParamLocal>,
}

pub(crate) struct ClosureTemplate<Function> {
    function: Function,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureArg>,
}

impl<Function> FunctionReference<Function> {
    pub(super) fn new(function: Function, params: Vec<ParamLocal>) -> Self {
        Self { function, params }
    }

    pub(crate) fn function(&self) -> &Function {
        &self.function
    }

    pub(crate) fn params(&self) -> &[ParamLocal] {
        &self.params
    }
}

impl<Function> ClosureTemplate<Function> {
    pub(super) fn new(
        function: Function,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureArg>,
    ) -> Self {
        Self {
            function,
            params,
            captures,
        }
    }

    pub(crate) fn function(&self) -> &Function {
        &self.function
    }

    pub(crate) fn params(&self) -> &[ParamLocal] {
        &self.params
    }

    pub(crate) fn captures(&self) -> &[CaptureArg] {
        &self.captures
    }
}
