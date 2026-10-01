use super::instruction::DraftIntegerOperand;
use super::{DraftBool, DraftFloat, DraftList, DraftOperand, DraftString, DraftUse, DraftValueRef};

pub(in crate::plan::execution::lowering) enum DraftBoolTest {
    Not(DraftBool),
    EqualInt {
        left: DraftIntegerOperand,
        right: DraftIntegerOperand,
    },
    NotEqualInt {
        left: DraftIntegerOperand,
        right: DraftIntegerOperand,
    },
    LtInt {
        left: DraftIntegerOperand,
        right: DraftIntegerOperand,
    },
    LtEqInt {
        left: DraftIntegerOperand,
        right: DraftIntegerOperand,
    },
    GtInt {
        left: DraftIntegerOperand,
        right: DraftIntegerOperand,
    },
    GtEqInt {
        left: DraftIntegerOperand,
        right: DraftIntegerOperand,
    },
    LtFloat {
        left: DraftFloat,
        right: DraftFloat,
    },
    LtEqFloat {
        left: DraftFloat,
        right: DraftFloat,
    },
    GtFloat {
        left: DraftFloat,
        right: DraftFloat,
    },
    GtEqFloat {
        left: DraftFloat,
        right: DraftFloat,
    },
    Equal {
        left: DraftValueRef,
        right: DraftValueRef,
    },
    NotEqual {
        left: DraftValueRef,
        right: DraftValueRef,
    },
    StringStartsWith {
        value: DraftString,
        prefix: ecow::EcoString,
    },
    ListLengthEquals {
        value: DraftList,
        length: usize,
    },
    ListLengthAtLeast {
        value: DraftList,
        length: usize,
    },
}

impl DraftBoolTest {
    pub(in crate::plan::execution::lowering::graph) fn uses(
        &self,
        values: &mut Vec<impl DraftUse>,
    ) {
        match self {
            Self::Not(value) => value.push_operand(values),
            Self::EqualInt { left, right }
            | Self::NotEqualInt { left, right }
            | Self::LtInt { left, right }
            | Self::LtEqInt { left, right }
            | Self::GtInt { left, right }
            | Self::GtEqInt { left, right } => {
                left.push_operand(values);
                right.push_operand(values);
            }
            Self::LtFloat { left, right }
            | Self::LtEqFloat { left, right }
            | Self::GtFloat { left, right }
            | Self::GtEqFloat { left, right } => {
                left.push_operand(values);
                right.push_operand(values);
            }
            Self::Equal { left, right } | Self::NotEqual { left, right } => {
                left.push_operand(values);
                right.push_operand(values);
            }
            Self::StringStartsWith { value, .. } => value.push_operand(values),
            Self::ListLengthEquals { value, .. } | Self::ListLengthAtLeast { value, .. } => {
                value.push_operand(values);
            }
        }
    }
}
