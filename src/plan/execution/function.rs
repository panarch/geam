use super::{FrameLayout, Step};

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

    pub(crate) fn frame_layout(&self) -> &FrameLayout {
        &self.frame_layout
    }

    pub(crate) fn steps(&self) -> &[Step] {
        &self.steps
    }

    pub(crate) fn return_(&self) -> &Return {
        &self.return_
    }
}
