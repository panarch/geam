use crate::plan::execution::{NeverFunctionId, NeverFunctionLocal, ParamLocal};

pub(crate) enum NeverCallTarget {
    Direct(NeverFunctionId),
    Value(NeverFunctionLocal),
}

pub(crate) struct NeverCall {
    function: NeverCallTarget,
    args: Box<[ParamLocal]>,
}

impl NeverCall {
    pub(in crate::plan::execution) fn new(
        function: NeverCallTarget,
        args: Box<[ParamLocal]>,
    ) -> Self {
        Self { function, args }
    }

    pub(crate) fn function(&self) -> &NeverCallTarget {
        &self.function
    }

    pub(crate) fn args(&self) -> &[ParamLocal] {
        &self.args
    }
}
