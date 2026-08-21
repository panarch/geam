use super::discovery::{RegistryDiscoveryError, protocol};
use serde::Deserialize;
use std::collections::BTreeSet;

const SEARCH_LIMIT: usize = 100;

pub(super) fn crate_names(
    source: &[u8],
    canonical: &str,
) -> Result<Vec<String>, RegistryDiscoveryError> {
    let response = serde_json::from_slice::<SearchResponse>(source)
        .map_err(|error| protocol("search", error))?;
    let total = response.meta.total.max(response.crates.len());
    if total > SEARCH_LIMIT {
        return Err(RegistryDiscoveryError::SearchLimit {
            query: canonical.to_owned(),
            total,
        });
    }
    let prefix = format!("{canonical}-");
    let mut names = BTreeSet::new();
    for result in response.crates {
        if result.id != canonical && !result.id.starts_with(&prefix) {
            continue;
        }
        if !valid_crate_name(&result.id) {
            return Err(RegistryDiscoveryError::Protocol {
                response: "search",
                reason: format!("invalid crate name {}", result.id),
            });
        }
        if !is_discoverable_crate_name(&result.id) {
            continue;
        }
        names.insert(result.id);
    }
    Ok(names.into_iter().collect())
}

fn valid_crate_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
}

fn is_discoverable_crate_name(name: &str) -> bool {
    name.split('-').all(|segment| {
        !segment.is_empty()
            && segment
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
    })
}

#[derive(Deserialize)]
struct SearchResponse {
    crates: Vec<SearchResult>,
    meta: SearchMetadata,
}

#[derive(Deserialize)]
struct SearchResult {
    id: String,
}

#[derive(Deserialize)]
struct SearchMetadata {
    total: usize,
}

#[cfg(test)]
mod tests {
    use super::{crate_names, is_discoverable_crate_name, valid_crate_name};
    use crate::provider::registry::RegistryDiscoveryError;

    #[test]
    fn keeps_only_exact_namespace_candidates_in_deterministic_order() {
        let source = response(
            10,
            &[
                "other",
                "geam-images-zed",
                "geam-images",
                "prefix-geam-images",
                "geam-images-alt",
                "geam-images-alt",
                "geam_images",
                "geam-images-alt_provider",
                "geam-images-Alt",
                "geam-images--alt",
            ],
        );

        assert_eq!(
            crate_names(&source, "geam-images").expect("bounded response should parse"),
            ["geam-images", "geam-images-alt", "geam-images-zed"],
        );
    }

    #[test]
    fn rejects_unbounded_invalid_and_malformed_search_responses() {
        for (source, total) in [
            (response(101, &[]), 101),
            (response(1, &vec!["geam-images"; 101]), 101),
        ] {
            assert_eq!(
                crate_names(&source, "geam-images")
                    .expect_err("more than one bounded page must require explicit selection"),
                RegistryDiscoveryError::SearchLimit {
                    query: "geam-images".to_owned(),
                    total,
                },
            );
        }

        assert!(matches!(
            crate_names(
                &response(1, &["geam-images-!"]),
                "geam-images",
            ),
            Err(RegistryDiscoveryError::Protocol {
                response: "search",
                ref reason,
            }) if reason == "invalid crate name geam-images-!"
        ));
        assert!(matches!(
            crate_names(b"{", "geam-images"),
            Err(RegistryDiscoveryError::Protocol {
                response: "search",
                reason,
            }) if reason.contains("EOF")
        ));
    }

    #[test]
    fn validates_registry_crate_name_characters() {
        assert!(valid_crate_name("geam_images-2"));
        assert!(!valid_crate_name(""));
        assert!(!valid_crate_name("geam-images-!"));
    }

    #[test]
    fn recognizes_discoverable_kebab_case_names() {
        assert!(is_discoverable_crate_name("geam-images-2"));
        assert!(!is_discoverable_crate_name("geam_images-2"));
        assert!(!is_discoverable_crate_name("geam-images-Alt"));
        assert!(!is_discoverable_crate_name("geam-images--alt"));
    }

    fn response(total: usize, crate_names: &[&str]) -> Vec<u8> {
        serde_json::to_vec(&serde_json::json!({
            "crates": crate_names
                .iter()
                .map(|crate_name| serde_json::json!({ "id": crate_name }))
                .collect::<Vec<_>>(),
            "meta": { "total": total },
        }))
        .expect("search fixture should serialize")
    }
}
