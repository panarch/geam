use super::CompiledCheckpoint;
use crate::plan::HostCallSite;
use crate::plan::execution::function::{
    BoolFunctionFunctionId, BoolFunctionId, IntFunctionFunctionId, IntFunctionId,
};
use crate::plan::execution::graph::{
    BoolFunctionLocalId, FunctionCapture, FunctionTarget, IntFunctionLocalId, ParamLocal,
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
    pub(crate) fn key(self) -> (usize, usize) {
        let family = match self {
            Self::Int(_) => 0,
            Self::Bool(_) => 1,
            Self::IntFunction(_) => 2,
            Self::BoolFunction(_) => 3,
        };
        (family, self.index())
    }

    pub(crate) fn index(self) -> usize {
        match self {
            Self::Int(id) => id.0,
            Self::Bool(id) => id.0,
            Self::IntFunction(id) => id.0,
            Self::BoolFunction(id) => id.0,
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
        BoolFunctionFunctionId, BoolFunctionId, IntFunctionFunctionId, IntFunctionId,
    };
    use crate::plan::execution::graph::{
        BoolFunctionLocalId, BoolLocalId, FunctionCapture, FunctionTarget, IntFunctionLocalId,
        IntLocalId, ParamLocal,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::{FunctionType, ValueType};
    use crate::plan::{HostCallSite, SourceSpan};

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
