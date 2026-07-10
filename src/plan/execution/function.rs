use crate::plan::{FrameLayout, Step};

pub(crate) struct ExecutableFunction<Return> {
    frame_layout: FrameLayout,
    steps: Vec<Step>,
    return_: Return,
}

impl<Return> ExecutableFunction<Return> {
    pub(super) fn new(frame_layout: FrameLayout, steps: Vec<Step>, return_: Return) -> Self {
        Self {
            frame_layout,
            steps,
            return_,
        }
    }

    pub(crate) fn frame_layout(&self) -> FrameLayout {
        self.frame_layout.clone()
    }

    pub(crate) fn steps(&self) -> &[Step] {
        &self.steps
    }

    pub(crate) fn return_(&self) -> &Return {
        &self.return_
    }
}

#[cfg(test)]
mod tests {
    use super::ExecutableFunction;
    use crate::plan::{FrameLayout, IntExpr, IntLocalId};
    use num_bigint::BigInt;

    #[test]
    fn executable_function_accessors() {
        let return_ = IntExpr::value(BigInt::from(1));
        let mut layout = FrameLayout::default();
        layout.include_int(IntLocalId(0));
        let function = ExecutableFunction::new(layout, Vec::new(), return_);

        assert_eq!(function.frame_layout().ints(), 1);
        assert_eq!(function.steps(), &[]);
        assert_eq!(function.return_(), &IntExpr::value(BigInt::from(1)));
    }
}
