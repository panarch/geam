use super::{CollectionType, ListDecoderModel, StaticValueType};
use quote::{format_ident, quote};
use syn::Ident;

pub(super) fn register_list_decoder(
    list: &CollectionType,
    decoders: &mut Vec<ListDecoderModel>,
) -> Ident {
    let key = static_value_key(&list.value);
    if let Some(decoder) = decoders
        .iter()
        .find(|decoder: &&ListDecoderModel| decoder.key == key)
    {
        return decoder.ident.clone();
    }
    let index = decoders.len();
    let ident = format_ident!("__GeamListDecoder{index}");
    decoders.push(ListDecoderModel {
        ident: ident.clone(),
        value: list.value.clone(),
        key,
    });
    ident
}

pub(super) fn static_value_key(type_: &StaticValueType) -> String {
    match type_ {
        StaticValueType::Scalar(type_) => format!("scalar:{}", quote!(#type_)),
        StaticValueType::Declared { type_, .. } => {
            format!("declared:{}", quote!(#type_))
        }
        StaticValueType::External { schema, .. } => format!("external:{schema}"),
        StaticValueType::Custom { index, .. } => format!("custom:{index}"),
        StaticValueType::Tuple(elements) => format!(
            "tuple:({})",
            elements
                .iter()
                .map(static_value_key)
                .collect::<Vec<_>>()
                .join(",")
        ),
        StaticValueType::Result { success, failure } => format!(
            "result:<{},{}>",
            static_value_key(success),
            static_value_key(failure),
        ),
        StaticValueType::Option { value } => {
            format!("option:<{}>", static_value_key(value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{register_list_decoder, static_value_key};
    use crate::module::{CollectionType, ListDecoderModel, StaticValueType};

    #[test]
    fn static_value_keys_cover_each_distinct_list_item_family() {
        let scalar = StaticValueType::Scalar(syn::parse_quote!(BigInt));
        let declared = StaticValueType::Declared {
            type_: syn::parse_quote!(crate::Tag),
        };
        let external = StaticValueType::External {
            payload: syn::parse_quote!(Payload),
            schema: syn::parse_quote!(Schema),
            store_field: syn::parse_quote!(store),
        };
        let custom = StaticValueType::Custom { index: 3 };
        let tuple = StaticValueType::Tuple(vec![scalar.clone(), external.clone()]);
        let result = StaticValueType::Result {
            success: Box::new(StaticValueType::Option {
                value: Box::new(scalar.clone()),
            }),
            failure: Box::new(StaticValueType::Option {
                value: Box::new(StaticValueType::Scalar(syn::parse_quote!(EcoString))),
            }),
        };

        assert_eq!(static_value_key(&scalar), "scalar:BigInt");
        assert_eq!(static_value_key(&declared), "declared:crate :: Tag");
        assert_eq!(static_value_key(&external), "external:Schema");
        assert_eq!(static_value_key(&custom), "custom:3");
        assert_eq!(
            static_value_key(&tuple),
            "tuple:(scalar:BigInt,external:Schema)",
        );
        assert_eq!(
            static_value_key(&result),
            "result:<option:<scalar:BigInt>,option:<scalar:EcoString>>",
        );
    }

    #[test]
    fn decoder_registration_deduplicates_exact_item_shapes() {
        let mut decoders: Vec<ListDecoderModel> = Vec::new();
        let integers = CollectionType {
            source: syn::parse_quote!(geam::List<BigInt>),
            item: syn::parse_quote!(BigInt),
            value: StaticValueType::Scalar(syn::parse_quote!(BigInt)),
        };
        let booleans = CollectionType {
            source: syn::parse_quote!(geam::List<bool>),
            item: syn::parse_quote!(bool),
            value: StaticValueType::Scalar(syn::parse_quote!(bool)),
        };

        assert_eq!(
            register_list_decoder(&integers, &mut decoders),
            "__GeamListDecoder0"
        );
        assert_eq!(
            register_list_decoder(&integers, &mut decoders),
            "__GeamListDecoder0"
        );
        assert_eq!(
            register_list_decoder(&booleans, &mut decoders),
            "__GeamListDecoder1"
        );
        assert_eq!(decoders.len(), 2);
        assert_eq!(decoders[0].key, "scalar:BigInt");
        assert_eq!(decoders[1].key, "scalar:bool");
    }
}
