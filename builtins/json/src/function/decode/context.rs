use crate::schema::{
    DecodeDictIndex, DecodeDynamicIndex, DecodeListIndex, DecodeRequirements, DynamicDict,
    DynamicList, JsonDynamicResult,
};
use crate::{GleamJsonHostProfile, HostCall, HostExternal, HostProvider};
use ecow::EcoString;
use geam_core::HostType;
use geam_core::provider::ProviderConstructions;
use geam_stdlib::provider_support::{Dynamic, create_dynamic_dict, create_dynamic_value};

pub(super) struct DynamicBuilder<
    'borrow,
    'call,
    Profile: GleamJsonHostProfile,
    Provider: HostProvider<Profile>,
> {
    pub(super) call: &'borrow mut HostCall<'call, Profile, Provider, JsonDynamicResult>,
    pub(super) constructions: &'borrow ProviderConstructions<'call, DecodeRequirements>,
}

impl<'call, Profile, Provider> DynamicBuilder<'_, 'call, Profile, Provider>
where
    Profile: GleamJsonHostProfile,
    Provider: HostProvider<Profile>,
{
    pub(super) fn scalar<Type: HostType>(
        &mut self,
        value: Type::Value<'call>,
    ) -> HostExternal<'call, Dynamic> {
        create_dynamic_value::<Profile, Provider, JsonDynamicResult, Type>(
            self.call,
            self.constructions.select::<DecodeDynamicIndex>().token(),
            value,
        )
    }

    pub(super) fn list(
        &mut self,
        values: Vec<HostExternal<'call, Dynamic>>,
    ) -> HostExternal<'call, Dynamic> {
        let values = self.call.construct_list(
            self.constructions.select::<DecodeListIndex>().token(),
            values,
        );
        self.scalar::<DynamicList>(values)
    }

    pub(super) fn object(
        &mut self,
        entries: Vec<(EcoString, HostExternal<'call, Dynamic>)>,
    ) -> HostExternal<'call, Dynamic> {
        let entries = entries
            .into_iter()
            .map(|(key, value)| (self.scalar::<EcoString>(key), value))
            .collect::<Vec<_>>();
        let dict = create_dynamic_dict(
            self.call,
            self.constructions.select::<DecodeDictIndex>().token(),
            entries,
        );
        self.scalar::<DynamicDict>(dict)
    }
}
