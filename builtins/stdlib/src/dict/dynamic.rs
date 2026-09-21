use super::{DictOf, ExactDynamicDictOutput, create_dynamic_dict_with};
use crate::dynamic::{Dynamic, DynamicPayload};
use crate::{GleamStdlibProviderProfile, HostProvider, HostType};
use geam_core::HostCall;
use geam_core::provider::advanced::NativeMap;
use geam_core::provider::{
    ProviderConstruction, ProviderConstructionIndex0, ProviderConstructionIndexNext,
    ProviderConstructionList, ProviderConstructions, ProviderNoConstructions, ProviderOutputValue,
    ProviderValue, ProviderValueForms,
};

pub struct DynamicDictOutput {
    value: Output,
}

enum Output {
    Exact(ExactDynamicDictOutput),
    Native(NativeMap),
}

impl DynamicDictOutput {
    pub(crate) fn exact(value: ExactDynamicDictOutput) -> Self {
        Self {
            value: Output::Exact(value),
        }
    }

    pub(crate) fn native(value: NativeMap) -> Self {
        Self {
            value: Output::Native(value),
        }
    }
}

impl ProviderValue for DynamicDictOutput {
    type Host = DictOf<Dynamic, Dynamic>;
    type OutputRequirements = ProviderConstructionList<
        ProviderConstruction<Self::Host>,
        ProviderConstructionList<ProviderConstruction<Dynamic>, ProviderNoConstructions>,
    >;
    type RootRequirements = Self::OutputRequirements;
}

impl ProviderValueForms for DynamicDictOutput {
    type InvocationRequirements = ();
    type ImmediateListDecoder = geam_core::provider::MissingListContext;
    type OwnedListDecoder = geam_core::provider::MissingListContext;
    type Runtime<Profile: geam_core::HostProfile> =
        geam_core::provider::ProviderStaticValueForms<Self>;
    type Output = Self;
    type ImmediateInput = Self;
    type ImmediateListInput = Self;
    type OwnedInput = Self;
    type OwnedListInput = Self;
}

impl<Profile, Provider, Return> ProviderOutputValue<Profile, Provider, Return> for DynamicDictOutput
where
    Profile: GleamStdlibProviderProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    type Error = std::convert::Infallible;

    fn into_host<'call>(
        self,
        call: &mut HostCall<'call, Profile, Provider, Return>,
        constructions: &ProviderConstructions<'call, Self::OutputRequirements>,
    ) -> Result<<Self::Host as HostType>::Value<'call>, Self::Error> {
        let dict = constructions.select::<ProviderConstructionIndex0>();
        Ok(match self.value {
            Output::Exact(value) => value.into_host_infallible(call, &dict),
            Output::Native(value) => {
                let dynamic = constructions
                    .select::<ProviderConstructionIndexNext<ProviderConstructionIndex0>>();
                create_dynamic_dict_with(call, dict.token(), value.entries(), |call, entry| {
                    let key =
                        DynamicPayload::from_native(entry.key).into_host_infallible(call, &dynamic);
                    let value = DynamicPayload::from_native(entry.value)
                        .into_host_infallible(call, &dynamic);
                    (key, value)
                })
            }
        })
    }
}
