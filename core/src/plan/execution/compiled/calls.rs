use super::CompiledCheckpoint;
use crate::plan::HostCallSite;
use crate::plan::execution::function::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BoolFunctionFunctionId, BoolFunctionId,
    FloatFunctionFunctionId, FloatFunctionId, FunctionReturnFamily, IntFunctionFunctionId,
    IntFunctionId, NilFunctionFunctionId, NilFunctionId, StringFunctionFunctionId,
    StringFunctionId, UtfCodepointFunctionFunctionId, UtfCodepointFunctionId,
};
use crate::plan::execution::graph::{
    BitArrayFunctionLocalId, BoolFunctionLocalId, FloatFunctionLocalId, FunctionCapture,
    FunctionTarget, IntFunctionLocalId, NilFunctionLocalId, ParamLocal, StringFunctionLocalId,
    UtfCodepointFunctionLocalId,
};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;
use crate::plan::execution::type_::FunctionType;
use crate::runtime::compiled::calls::CallStart;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallTarget {
    Int(IntFunctionId),
    Bool(BoolFunctionId),
    IntFunction(IntFunctionFunctionId),
    BoolFunction(BoolFunctionFunctionId),
    Float(FloatFunctionId),
    String(StringFunctionId),
    BitArray(BitArrayFunctionId),
    UtfCodepoint(UtfCodepointFunctionId),
    Nil(NilFunctionId),
    FloatFunction(FloatFunctionFunctionId),
    StringFunction(StringFunctionFunctionId),
    BitArrayFunction(BitArrayFunctionFunctionId),
    UtfCodepointFunction(UtfCodepointFunctionFunctionId),
    NilFunction(NilFunctionFunctionId),
}

pub struct FunctionCallsImplementation {
    pub root: bool,
    pub entry: usize,
    pub checkpoints: Table<CompiledCheckpoint>,
    pub locals: Table<Table<ParamLocal>>,
    pub calls: Table<CallContract>,
    pub creations: Table<CreationContract>,
    pub returns: Table<ReturnContract>,
    pub tails: Table<TailContract>,
    pub start: CallStart,
}

#[derive(Clone, PartialEq, Eq)]
pub struct CallContract {
    pub point: usize,
    pub output: ParamLocal,
    pub target: CallContractTarget,
    pub args: Table<ParamLocal>,
    pub site: HostCallSite,
}

#[derive(Clone, PartialEq, Eq)]
pub enum CallContractTarget {
    Static(CallTarget),
    IntValue(IntFunctionLocalId),
    BoolValue(BoolFunctionLocalId),
    FloatValue(FloatFunctionLocalId),
    StringValue(StringFunctionLocalId),
    BitArrayValue(BitArrayFunctionLocalId),
    UtfCodepointValue(UtfCodepointFunctionLocalId),
    NilValue(NilFunctionLocalId),
}

#[derive(Clone, PartialEq, Eq)]
pub struct CreationContract {
    pub point: usize,
    pub output: ParamLocal,
    pub target: FunctionTarget,
    pub type_: FunctionType,
    pub reference: bool,
    pub captures: Table<FunctionCapture>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct ReturnContract {
    pub point: usize,
    pub value: ParamLocal,
}

#[derive(Clone, PartialEq, Eq)]
pub struct TailContract {
    pub point: usize,
    pub target: CallTarget,
    pub args: Table<ParamLocal>,
    pub site: HostCallSite,
}

impl CallTarget {
    pub(crate) fn family(self) -> FunctionReturnFamily {
        match self {
            Self::Int(_) => FunctionReturnFamily::Int,
            Self::Float(_) => FunctionReturnFamily::Float,
            Self::String(_) => FunctionReturnFamily::String,
            Self::BitArray(_) => FunctionReturnFamily::BitArray,
            Self::UtfCodepoint(_) => FunctionReturnFamily::UtfCodepoint,
            Self::Bool(_) => FunctionReturnFamily::Bool,
            Self::Nil(_) => FunctionReturnFamily::Nil,
            Self::IntFunction(_)
            | Self::FloatFunction(_)
            | Self::StringFunction(_)
            | Self::BitArrayFunction(_)
            | Self::UtfCodepointFunction(_)
            | Self::BoolFunction(_)
            | Self::NilFunction(_) => FunctionReturnFamily::Function,
        }
    }

    pub(crate) fn key(self) -> (usize, usize) {
        let family = match self {
            Self::Int(_) => 0,
            Self::Bool(_) => 1,
            Self::IntFunction(_) => 2,
            Self::BoolFunction(_) => 3,
            Self::Float(_) => 4,
            Self::String(_) => 5,
            Self::BitArray(_) => 6,
            Self::UtfCodepoint(_) => 7,
            Self::Nil(_) => 8,
            Self::FloatFunction(_) => 9,
            Self::StringFunction(_) => 10,
            Self::BitArrayFunction(_) => 11,
            Self::UtfCodepointFunction(_) => 12,
            Self::NilFunction(_) => 13,
        };
        (family, self.index())
    }

    pub(crate) fn index(self) -> usize {
        match self {
            Self::Int(id) => id.0,
            Self::Bool(id) => id.0,
            Self::IntFunction(id) => id.0,
            Self::BoolFunction(id) => id.0,
            Self::Float(id) => id.0,
            Self::String(id) => id.0,
            Self::BitArray(id) => id.0,
            Self::UtfCodepoint(id) => id.0,
            Self::Nil(id) => id.0,
            Self::FloatFunction(id) => id.0,
            Self::StringFunction(id) => id.0,
            Self::BitArrayFunction(id) => id.0,
            Self::UtfCodepointFunction(id) => id.0,
            Self::NilFunction(id) => id.0,
        }
    }
}

impl Emit for CallTarget {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Int(id) => output.call("compiled::CallTarget::Int", &[id]),
            Self::Bool(id) => output.call("compiled::CallTarget::Bool", &[id]),
            Self::IntFunction(id) => output.call("compiled::CallTarget::IntFunction", &[id]),
            Self::BoolFunction(id) => output.call("compiled::CallTarget::BoolFunction", &[id]),
            Self::Float(id) => output.call("compiled::CallTarget::Float", &[id]),
            Self::String(id) => output.call("compiled::CallTarget::String", &[id]),
            Self::BitArray(id) => output.call("compiled::CallTarget::BitArray", &[id]),
            Self::UtfCodepoint(id) => output.call("compiled::CallTarget::UtfCodepoint", &[id]),
            Self::Nil(id) => output.call("compiled::CallTarget::Nil", &[id]),
            Self::FloatFunction(id) => output.call("compiled::CallTarget::FloatFunction", &[id]),
            Self::StringFunction(id) => output.call("compiled::CallTarget::StringFunction", &[id]),
            Self::BitArrayFunction(id) => {
                output.call("compiled::CallTarget::BitArrayFunction", &[id])
            }
            Self::UtfCodepointFunction(id) => {
                output.call("compiled::CallTarget::UtfCodepointFunction", &[id])
            }
            Self::NilFunction(id) => output.call("compiled::CallTarget::NilFunction", &[id]),
        }
    }
}

impl Emit for CallContractTarget {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Static(target) => output.call("compiled::CallContractTarget::Static", &[target]),
            Self::IntValue(local) => {
                output.call("compiled::CallContractTarget::IntValue", &[local])
            }
            Self::BoolValue(local) => {
                output.call("compiled::CallContractTarget::BoolValue", &[local])
            }
            Self::FloatValue(local) => {
                output.call("compiled::CallContractTarget::FloatValue", &[local])
            }
            Self::StringValue(local) => {
                output.call("compiled::CallContractTarget::StringValue", &[local])
            }
            Self::BitArrayValue(local) => {
                output.call("compiled::CallContractTarget::BitArrayValue", &[local])
            }
            Self::UtfCodepointValue(local) => {
                output.call("compiled::CallContractTarget::UtfCodepointValue", &[local])
            }
            Self::NilValue(local) => {
                output.call("compiled::CallContractTarget::NilValue", &[local])
            }
        }
    }
}

impl Emit for CallContract {
    fn emit(&self, output: &mut Rust) {
        output.structure(
            "compiled::CallContract",
            &[
                ("point", &self.point),
                ("output", &self.output),
                ("target", &self.target),
                ("args", &self.args),
                ("site", &self.site),
            ],
        );
    }
}

impl Emit for CreationContract {
    fn emit(&self, output: &mut Rust) {
        output.structure(
            "compiled::CreationContract",
            &[
                ("point", &self.point),
                ("output", &self.output),
                ("target", &self.target),
                ("type_", &self.type_),
                ("reference", &self.reference),
                ("captures", &self.captures),
            ],
        );
    }
}

impl Emit for ReturnContract {
    fn emit(&self, output: &mut Rust) {
        output.structure(
            "compiled::ReturnContract",
            &[("point", &self.point), ("value", &self.value)],
        );
    }
}

impl Emit for TailContract {
    fn emit(&self, output: &mut Rust) {
        output.structure(
            "compiled::TailContract",
            &[
                ("point", &self.point),
                ("target", &self.target),
                ("args", &self.args),
                ("site", &self.site),
            ],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CallContract, CallContractTarget, CallTarget, CreationContract, ReturnContract,
        TailContract,
    };
    use crate::plan::execution::function::{
        BitArrayFunctionFunctionId, BitArrayFunctionId, BoolFunctionFunctionId, BoolFunctionId,
        FloatFunctionFunctionId, FloatFunctionId, IntFunctionFunctionId, IntFunctionId,
        NilFunctionFunctionId, NilFunctionId, StringFunctionFunctionId, StringFunctionId,
        UtfCodepointFunctionFunctionId, UtfCodepointFunctionId,
    };
    use crate::plan::execution::graph::{
        BitArrayFunctionLocalId, BoolFunctionLocalId, BoolLocalId, FloatFunctionLocalId,
        FunctionCapture, FunctionTarget, IntFunctionLocalId, IntLocalId, NilFunctionLocalId,
        ParamLocal, StringFunctionLocalId, UtfCodepointFunctionLocalId,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::{FunctionType, ValueType};
    use crate::plan::{HostCallSite, SourceSpan};

    #[test]
    fn targets_preserve_each_exact_return_family() {
        use crate::plan::execution::function::FunctionReturnFamily;
        for (target, expected) in [
            (CallTarget::Int(IntFunctionId(3)), FunctionReturnFamily::Int),
            (
                CallTarget::Bool(BoolFunctionId(3)),
                FunctionReturnFamily::Bool,
            ),
            (
                CallTarget::Float(FloatFunctionId(3)),
                FunctionReturnFamily::Float,
            ),
            (
                CallTarget::String(StringFunctionId(3)),
                FunctionReturnFamily::String,
            ),
            (
                CallTarget::BitArray(BitArrayFunctionId(3)),
                FunctionReturnFamily::BitArray,
            ),
            (
                CallTarget::UtfCodepoint(UtfCodepointFunctionId(3)),
                FunctionReturnFamily::UtfCodepoint,
            ),
            (CallTarget::Nil(NilFunctionId(3)), FunctionReturnFamily::Nil),
            (
                CallTarget::IntFunction(IntFunctionFunctionId(3)),
                FunctionReturnFamily::Function,
            ),
            (
                CallTarget::BoolFunction(BoolFunctionFunctionId(3)),
                FunctionReturnFamily::Function,
            ),
            (
                CallTarget::FloatFunction(FloatFunctionFunctionId(3)),
                FunctionReturnFamily::Function,
            ),
            (
                CallTarget::StringFunction(StringFunctionFunctionId(3)),
                FunctionReturnFamily::Function,
            ),
            (
                CallTarget::BitArrayFunction(BitArrayFunctionFunctionId(3)),
                FunctionReturnFamily::Function,
            ),
            (
                CallTarget::UtfCodepointFunction(UtfCodepointFunctionFunctionId(3)),
                FunctionReturnFamily::Function,
            ),
            (
                CallTarget::NilFunction(NilFunctionFunctionId(3)),
                FunctionReturnFamily::Function,
            ),
        ] {
            assert_eq!(target.family(), expected);
        }
    }

    #[test]
    fn every_call_target_family_has_exact_rust_identity_and_sort_key() {
        for (target, key, expected) in [
            (
                CallTarget::Int(IntFunctionId(7)),
                (0, 7),
                "data::compiled::CallTarget::Int(data::function::IntFunctionId(7))",
            ),
            (
                CallTarget::Bool(BoolFunctionId(8)),
                (1, 8),
                "data::compiled::CallTarget::Bool(data::function::BoolFunctionId(8))",
            ),
            (
                CallTarget::IntFunction(IntFunctionFunctionId(9)),
                (2, 9),
                "data::compiled::CallTarget::IntFunction(data::function::IntFunctionFunctionId(9))",
            ),
            (
                CallTarget::BoolFunction(BoolFunctionFunctionId(10)),
                (3, 10),
                "data::compiled::CallTarget::BoolFunction(data::function::BoolFunctionFunctionId(10))",
            ),
            (
                CallTarget::Float(FloatFunctionId(11)),
                (4, 11),
                "data::compiled::CallTarget::Float(data::function::FloatFunctionId(11))",
            ),
            (
                CallTarget::String(StringFunctionId(12)),
                (5, 12),
                "data::compiled::CallTarget::String(data::function::StringFunctionId(12))",
            ),
            (
                CallTarget::BitArray(BitArrayFunctionId(13)),
                (6, 13),
                "data::compiled::CallTarget::BitArray(data::function::BitArrayFunctionId(13))",
            ),
            (
                CallTarget::UtfCodepoint(UtfCodepointFunctionId(14)),
                (7, 14),
                "data::compiled::CallTarget::UtfCodepoint(data::function::UtfCodepointFunctionId(14))",
            ),
            (
                CallTarget::Nil(NilFunctionId(15)),
                (8, 15),
                "data::compiled::CallTarget::Nil(data::function::NilFunctionId(15))",
            ),
            (
                CallTarget::FloatFunction(FloatFunctionFunctionId(16)),
                (9, 16),
                "data::compiled::CallTarget::FloatFunction(data::function::FloatFunctionFunctionId(16))",
            ),
            (
                CallTarget::StringFunction(StringFunctionFunctionId(17)),
                (10, 17),
                "data::compiled::CallTarget::StringFunction(data::function::StringFunctionFunctionId(17))",
            ),
            (
                CallTarget::BitArrayFunction(BitArrayFunctionFunctionId(18)),
                (11, 18),
                "data::compiled::CallTarget::BitArrayFunction(data::function::BitArrayFunctionFunctionId(18))",
            ),
            (
                CallTarget::UtfCodepointFunction(UtfCodepointFunctionFunctionId(19)),
                (12, 19),
                "data::compiled::CallTarget::UtfCodepointFunction(data::function::UtfCodepointFunctionFunctionId(19))",
            ),
            (
                CallTarget::NilFunction(NilFunctionFunctionId(20)),
                (13, 20),
                "data::compiled::CallTarget::NilFunction(data::function::NilFunctionFunctionId(20))",
            ),
        ] {
            assert_eq!(target.key(), key);
            assert_eq!(target.index(), key.1);
            assert_eq!(Rust::expression(&target), expected);
            assert_eq!(
                Rust::expression(&CallContractTarget::Static(target)),
                format!("data::compiled::CallContractTarget::Static({expected})")
            );
        }
        assert_eq!(
            Rust::expression(&CallContractTarget::IntValue(IntFunctionLocalId(3))),
            "data::compiled::CallContractTarget::IntValue(data::graph::IntFunctionLocalId(3))"
        );
        assert_eq!(
            Rust::expression(&CallContractTarget::BoolValue(BoolFunctionLocalId(4))),
            "data::compiled::CallContractTarget::BoolValue(data::graph::BoolFunctionLocalId(4))"
        );
    }

    #[test]
    fn every_additional_dynamic_target_keeps_its_exact_typed_local_identity() {
        for (target, expected) in [
            (
                CallContractTarget::FloatValue(FloatFunctionLocalId(3)),
                "data::compiled::CallContractTarget::FloatValue(data::graph::FloatFunctionLocalId(3))",
            ),
            (
                CallContractTarget::StringValue(StringFunctionLocalId(3)),
                "data::compiled::CallContractTarget::StringValue(data::graph::StringFunctionLocalId(3))",
            ),
            (
                CallContractTarget::BitArrayValue(BitArrayFunctionLocalId(3)),
                "data::compiled::CallContractTarget::BitArrayValue(data::graph::BitArrayFunctionLocalId(3))",
            ),
            (
                CallContractTarget::UtfCodepointValue(UtfCodepointFunctionLocalId(3)),
                "data::compiled::CallContractTarget::UtfCodepointValue(data::graph::UtfCodepointFunctionLocalId(3))",
            ),
            (
                CallContractTarget::NilValue(NilFunctionLocalId(3)),
                "data::compiled::CallContractTarget::NilValue(data::graph::NilFunctionLocalId(3))",
            ),
        ] {
            assert_eq!(Rust::expression(&target), expected);
        }
    }

    #[test]
    fn call_return_and_tail_contracts_emit_the_exact_local_arguments_and_source_origin() {
        let site = HostCallSite::from_static("example", "calculate", SourceSpan::new(7, 19));
        let call = CallContract {
            point: 3,
            output: ParamLocal::Bool(BoolLocalId(2)),
            target: CallContractTarget::BoolValue(BoolFunctionLocalId(1)),
            args: vec![
                ParamLocal::Int(IntLocalId(4)),
                ParamLocal::Bool(BoolLocalId(0)),
            ]
            .into(),
            site: site.clone(),
        };
        assert_eq!(
            Rust::expression(&call),
            r#"data::compiled::CallContract {
    point: 3,
    output: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
    target: data::compiled::CallContractTarget::BoolValue(data::graph::BoolFunctionLocalId(1)),
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Int(data::graph::IntLocalId(4)),
        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(0)),
    ]),
    site: data::source::HostCallSite::from_static("example", "calculate", data::source::SourceSpan::new(7, 19)),
}"#
        );
        let returning = ReturnContract {
            point: 4,
            value: ParamLocal::Bool(BoolLocalId(2)),
        };
        assert_eq!(
            Rust::expression(&returning),
            r#"data::compiled::ReturnContract {
    point: 4,
    value: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
}"#
        );
        let tail = TailContract {
            point: 5,
            target: CallTarget::Bool(BoolFunctionId(8)),
            args: vec![ParamLocal::Bool(BoolLocalId(2))].into(),
            site,
        };
        assert_eq!(
            Rust::expression(&tail),
            r#"data::compiled::TailContract {
    point: 5,
    target: data::compiled::CallTarget::Bool(data::function::BoolFunctionId(8)),
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
    ]),
    site: data::source::HostCallSite::from_static("example", "calculate", data::source::SourceSpan::new(7, 19)),
}"#
        );
    }

    #[test]
    fn closure_contract_emits_all_four_capture_mappings_without_changing_the_callable_type() {
        let type_ = FunctionType::new(vec![ValueType::Int], ValueType::Bool);
        let creation = CreationContract {
            point: 2,
            output: ParamLocal::BoolFunction {
                local: BoolFunctionLocalId(3),
                type_: type_.clone(),
            },
            target: FunctionTarget::Bool(BoolFunctionId(8)),
            type_,
            reference: false,
            captures: vec![
                FunctionCapture::Int {
                    target: IntLocalId(1),
                    source: IntLocalId(4),
                },
                FunctionCapture::Bool {
                    target: BoolLocalId(0),
                    source: BoolLocalId(2),
                },
                FunctionCapture::IntFunction {
                    target: IntFunctionLocalId(0),
                    source: IntFunctionLocalId(5),
                },
                FunctionCapture::BoolFunction {
                    target: BoolFunctionLocalId(0),
                    source: BoolFunctionLocalId(1),
                },
            ]
            .into(),
        };
        assert_eq!(
            Rust::expression(&creation),
            r#"data::compiled::CreationContract {
    point: 2,
    output: data::graph::ParamLocal::BoolFunction {
        local: data::graph::BoolFunctionLocalId(3),
        type_: data::type_::FunctionType {
            arguments: data::Storage::Static(&[
                data::type_::ValueType::Int,
            ]),
            return_: data::Storage::Static(&data::type_::ValueType::Bool),
        },
    },
    target: data::graph::FunctionTarget::Bool(data::function::BoolFunctionId(8)),
    type_: data::type_::FunctionType {
        arguments: data::Storage::Static(&[
            data::type_::ValueType::Int,
        ]),
        return_: data::Storage::Static(&data::type_::ValueType::Bool),
    },
    reference: false,
    captures: data::Storage::Static(&[
        data::graph::FunctionCapture::Int {
            target: data::graph::IntLocalId(1),
            source: data::graph::IntLocalId(4),
        },
        data::graph::FunctionCapture::Bool {
            target: data::graph::BoolLocalId(0),
            source: data::graph::BoolLocalId(2),
        },
        data::graph::FunctionCapture::IntFunction {
            target: data::graph::IntFunctionLocalId(0),
            source: data::graph::IntFunctionLocalId(5),
        },
        data::graph::FunctionCapture::BoolFunction {
            target: data::graph::BoolFunctionLocalId(0),
            source: data::graph::BoolFunctionLocalId(1),
        },
    ]),
}"#
        );
    }
}
