use super::crates_io::sparse_index_path;
use super::discovery::{RegistryDiscoveryError, protocol};
use semver::Version;
use serde::Deserialize;

#[derive(Debug)]
pub(super) struct DownloadLocation(String);

impl DownloadLocation {
    pub(super) fn url(
        &self,
        crate_name: &str,
        version: &Version,
        checksum: &[u8; 32],
    ) -> Result<String, RegistryDiscoveryError> {
        let base = self.0.trim_end_matches('/');
        if !base.contains('{') {
            return Ok(format!("{base}/{crate_name}/{version}/download"));
        }
        let index_path = sparse_index_path(crate_name);
        let prefix = index_path.rsplit_once('/').map_or("", |(prefix, _)| prefix);
        let url = base
            .replace("{crate}", crate_name)
            .replace("{version}", &version.to_string())
            .replace("{prefix}", prefix)
            .replace("{lowerprefix}", &prefix.to_ascii_lowercase())
            .replace("{sha256-checksum}", &hex::encode(checksum));
        if url.contains('{') || url.contains('}') {
            return Err(RegistryDiscoveryError::Protocol {
                response: "configuration",
                reason: format!("download URL contains an unsupported marker: {url}"),
            });
        }
        Ok(url)
    }
}

pub(super) fn parse(source: &[u8]) -> Result<DownloadLocation, RegistryDiscoveryError> {
    let configuration = serde_json::from_slice::<RegistryConfiguration>(source)
        .map_err(|error| protocol("configuration", error))?;
    if !configuration.download.starts_with("https://") {
        return Err(RegistryDiscoveryError::Protocol {
            response: "configuration",
            reason: "download URL must use HTTPS".to_owned(),
        });
    }
    Ok(DownloadLocation(configuration.download))
}

#[derive(Deserialize)]
struct RegistryConfiguration {
    #[serde(rename = "dl")]
    download: String,
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::provider::registry::RegistryDiscoveryError;
    use semver::Version;

    #[test]
    fn expands_legacy_and_supported_marker_download_locations() {
        let checksum = [0xab; 32];
        let legacy = parse(br#"{"dl":"https://downloads.example/"}"#)
            .expect("HTTPS legacy configuration should parse");
        assert_eq!(
            legacy
                .url("geam-images", &Version::new(1, 2, 3), &checksum)
                .expect("legacy URL should expand"),
            "https://downloads.example/geam-images/1.2.3/download",
        );

        let markers = parse(
            br#"{"dl":"https://downloads.example/{crate}/{version}/{prefix}/{lowerprefix}/{sha256-checksum}"}"#,
        )
        .expect("marker configuration should parse");
        assert_eq!(
            markers
                .url("geam-images", &Version::new(1, 2, 3), &checksum)
                .expect("supported markers should expand"),
            format!(
                "https://downloads.example/geam-images/1.2.3/ge/am/ge/am/{}",
                "ab".repeat(32),
            ),
        );
    }

    #[test]
    fn rejects_invalid_configuration_and_unsupported_markers() {
        for (source, expected, exact) in [
            (b"{".as_slice(), "EOF", false),
            (
                br#"{"dl":"http://example.test"}"#.as_slice(),
                "download URL must use HTTPS",
                true,
            ),
        ] {
            assert!(matches!(
                parse(source),
                Err(RegistryDiscoveryError::Protocol {
                    response: "configuration",
                    reason,
                }) if if exact {
                    reason == expected
                } else {
                    reason.contains(expected)
                }
            ));
        }

        let unsupported = parse(br#"{"dl":"https://downloads.example/{crate}/{unknown}"}"#)
            .expect("HTTPS configuration should parse");
        assert!(matches!(
            unsupported.url("geam-images", &Version::new(1, 0, 0), &[0; 32]),
            Err(RegistryDiscoveryError::Protocol {
                response: "configuration",
                ref reason,
            }) if reason.contains("unsupported marker")
        ));
    }
}
