use super::{CaptureArg, ParamLocal, ParamSlot};

pub(crate) struct FunctionReference<Function> {
    function: Function,
    params: Vec<ParamSlot>,
}

pub(crate) struct ClosureTemplate<Function> {
    function: Function,
    params: Vec<ParamSlot>,
    captures: Vec<CaptureArg>,
}

impl<Function> FunctionReference<Function> {
    pub(super) fn new(function: Function, params: Vec<ParamSlot>) -> Self {
        Self { function, params }
    }

    pub(crate) fn function(&self) -> &Function {
        &self.function
    }

    pub(crate) fn params(&self) -> &[ParamSlot] {
        &self.params
    }

    pub(crate) fn param_locals(&self) -> Vec<ParamLocal> {
        self.params
            .iter()
            .map(|param| param.local().clone())
            .collect()
    }
}

impl<Function> ClosureTemplate<Function> {
    pub(super) fn new(
        function: Function,
        params: Vec<ParamSlot>,
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

    pub(crate) fn params(&self) -> &[ParamSlot] {
        &self.params
    }

    pub(crate) fn param_locals(&self) -> Vec<ParamLocal> {
        self.params
            .iter()
            .map(|param| param.local().clone())
            .collect()
    }

    pub(crate) fn captures(&self) -> &[CaptureArg] {
        &self.captures
    }
}
