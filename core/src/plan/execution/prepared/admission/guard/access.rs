use super::super::place::{self, Projection};
use super::{Access, GuardError, Requirement, origin};
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::ProfiledInstructionKind;

pub(super) fn instruction<Graph: ExecutionGraphProfile>(
    value: &ProfiledInstructionKind<Graph>,
) -> Result<Option<Access<'_>>, GuardError> {
    if let Some((local, Projection::List(index))) = place::projected_input(value) {
        let length = index.checked_add(1).ok_or(GuardError::LengthOverflow)?;
        return Ok(Some(Access {
            local,
            requirement: Requirement::Length(length),
        }));
    }
    Ok(match origin::instruction(value) {
        origin::Origin::ListDrop { source, count } => Some(Access {
            local: source,
            requirement: Requirement::Length(count),
        }),
        origin::Origin::TextDrop { source, prefix } => Some(Access {
            local: source,
            requirement: Requirement::Prefix(prefix.into()),
        }),
        origin::Origin::Unknown
        | origin::Origin::List { .. }
        | origin::Origin::Text(_)
        | origin::Origin::Concatenate { .. } => None,
    })
}

#[cfg(test)]
mod tests {
    use super::{Access, GuardError, Requirement, instruction};
    use crate::plan::Text;
    use crate::plan::execution::function::HostedExecutionGraph;
    use crate::plan::execution::graph::{
        ExternalInstruction, ExternalListLocalId, IntInstruction, IntListLocalId, ListInstruction,
        ProfiledInstructionKind, StringInstruction, StringLocalId, TupleLocalId,
        TypedListInstruction,
    };
    use crate::plan::execution::storage::Table;
    use crate::plan::execution::type_::{IntListTypeId, ListTypeId};

    #[test]
    fn projected_elements_require_the_prefix_that_contains_the_requested_index() {
        type Kind = ProfiledInstructionKind<HostedExecutionGraph>;
        for (index, expected_length) in [(0, 1), (7, 8), (usize::MAX - 1, usize::MAX)] {
            assert_eq!(
                instruction(&Kind::Int(IntInstruction::ListIndex {
                    list: IntListLocalId(2),
                    index
                })),
                Ok(Some(Access {
                    local: IntListLocalId(2).into(),
                    requirement: Requirement::Length(expected_length)
                }))
            );
            assert_eq!(
                instruction(&Kind::External(ExternalInstruction::ListIndex {
                    list: ExternalListLocalId(3),
                    index
                })),
                Ok(Some(Access {
                    local: ExternalListLocalId(3).into(),
                    requirement: Requirement::Length(expected_length)
                }))
            );
        }
        assert_eq!(
            instruction(&Kind::Int(IntInstruction::ListIndex {
                list: IntListLocalId(2),
                index: usize::MAX
            })),
            Err(GuardError::LengthOverflow)
        );
    }

    #[test]
    fn suffix_reads_preserve_exact_drop_requirements_and_non_reads_have_none() {
        type Kind = ProfiledInstructionKind<HostedExecutionGraph>;
        let type_id = IntListTypeId::new(ListTypeId(0));
        for count in [0, 7, usize::MAX] {
            assert_eq!(
                instruction(&Kind::List(ListInstruction::Int(
                    type_id,
                    TypedListInstruction::DropFirst {
                        list: IntListLocalId(2),
                        count
                    },
                ))),
                Ok(Some(Access {
                    local: IntListLocalId(2).into(),
                    requirement: Requirement::Length(count)
                }))
            );
        }
        for prefix in ["", "pre", "\u{e9}-"] {
            assert_eq!(
                instruction(&Kind::String(StringInstruction::DropPrefix {
                    value: StringLocalId(3),
                    prefix: Text::Owned(prefix.into()),
                })),
                Ok(Some(Access {
                    local: StringLocalId(3).into(),
                    requirement: Requirement::Prefix(prefix.into())
                }))
            );
        }
        for value in [
            Kind::Int(IntInstruction::Value(num_bigint::BigInt::from(42).into())),
            Kind::Int(IntInstruction::TupleIndex {
                tuple: TupleLocalId(4),
                index: 2,
            }),
            Kind::List(ListInstruction::Int(
                type_id,
                TypedListInstruction::Value(Table::Static(&[])),
            )),
            Kind::String(StringInstruction::Value(Text::Static("text"))),
            Kind::String(StringInstruction::Concatenate {
                left: StringLocalId(0),
                right: StringLocalId(1),
            }),
        ] {
            assert_eq!(instruction(&value), Ok(None));
        }
    }
}
