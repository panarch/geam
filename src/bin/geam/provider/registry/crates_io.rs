use super::archive::MAX_COMPRESSED_SIZE;
use super::{ProviderRegistry, RegistryAccessError};
use std::time::Duration;

const SEARCH_BODY_LIMIT: usize = 1024 * 1024;
const INDEX_BODY_LIMIT: usize = 16 * 1024 * 1024;
const CONFIG_BODY_LIMIT: usize = 64 * 1024;
const USER_AGENT: &str = concat!("geam/", env!("CARGO_PKG_VERSION"));

pub(in crate::provider) struct CratesIoRegistry {
    agent: ureq::Agent,
}

impl Default for CratesIoRegistry {
    fn default() -> Self {
        let config = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(30)))
            .timeout_connect(Some(Duration::from_secs(10)))
            .https_only(true)
            .build();
        Self {
            agent: config.into(),
        }
    }
}

impl ProviderRegistry for CratesIoRegistry {
    fn search(&self, query: &str) -> Result<Vec<u8>, RegistryAccessError> {
        access(
            "search crates.io",
            self.request::<SEARCH_BODY_LIMIT>(&search_url(query)),
        )
    }

    fn index(&self, crate_name: &str) -> Result<Vec<u8>, RegistryAccessError> {
        access(
            format!("read crates.io sparse index for {crate_name}"),
            self.request::<INDEX_BODY_LIMIT>(&index_url(crate_name)),
        )
    }

    fn configuration(&self) -> Result<Vec<u8>, RegistryAccessError> {
        access(
            "read crates.io registry configuration",
            self.request::<CONFIG_BODY_LIMIT>("https://index.crates.io/config.json"),
        )
    }

    fn download(&self, url: &str) -> Result<Vec<u8>, RegistryAccessError> {
        access(
            format!("download provider archive from {url}"),
            self.request::<{ MAX_COMPRESSED_SIZE + 1 }>(url),
        )
    }
}

impl CratesIoRegistry {
    fn request<const LIMIT: usize>(&self, url: &str) -> Result<Vec<u8>, ureq::Error> {
        self.agent
            .get(url)
            .header("User-Agent", USER_AGENT)
            .call()
            .and_then(read_response::<LIMIT>)
    }
}

fn access(
    operation: impl Into<String>,
    result: Result<Vec<u8>, ureq::Error>,
) -> Result<Vec<u8>, RegistryAccessError> {
    result.map_err(|error| RegistryAccessError::new(operation, error.to_string()))
}

fn read_response<const LIMIT: usize>(
    mut response: ureq::http::Response<ureq::Body>,
) -> Result<Vec<u8>, ureq::Error> {
    response
        .body_mut()
        .with_config()
        .limit(LIMIT as u64)
        .read_to_vec()
}

fn search_url(query: &str) -> String {
    format!("https://crates.io/api/v1/crates?q={query}&per_page=100")
}

fn index_url(crate_name: &str) -> String {
    format!("https://index.crates.io/{}", sparse_index_path(crate_name))
}

pub(super) fn sparse_index_path(crate_name: &str) -> String {
    let crate_name = crate_name.to_ascii_lowercase();
    match crate_name.len() {
        1 => format!("1/{crate_name}"),
        2 => format!("2/{crate_name}"),
        3 => format!("3/{}/{crate_name}", &crate_name[..1]),
        _ => format!("{}/{}/{crate_name}", &crate_name[..2], &crate_name[2..4]),
    }
}

#[cfg(test)]
mod tests {
    use super::{USER_AGENT, access, index_url, read_response, search_url, sparse_index_path};
    use crate::provider::registry::{CratesIoRegistry, ProviderRegistry, RegistryAccessError};
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
    use ureq::unversioned::{
        resolver::{ResolvedSocketAddrs, Resolver},
        transport::{ConnectionDetails, Connector, NextTimeout},
    };
    use ureq::{Agent, Error, config::Config, http::Uri};

    #[test]
    fn constructs_exact_crates_io_endpoints_and_user_agent() {
        assert_eq!(
            search_url("geam-company-image"),
            "https://crates.io/api/v1/crates?q=geam-company-image&per_page=100",
        );
        assert_eq!(
            index_url("Geam-Images"),
            "https://index.crates.io/ge/am/geam-images"
        );
        assert_eq!(
            ["a", "ab", "abc", "abcd"].map(sparse_index_path),
            ["1/a", "2/ab", "3/a/abc", "ab/cd/abcd"],
        );
        assert_eq!(USER_AGENT, format!("geam/{}", env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn bounds_response_bodies_and_preserves_request_failures() {
        let response = ureq::http::Response::new(ureq::Body::builder().data(b"body".to_vec()));
        assert_eq!(
            read_response::<5>(response).expect("bounded response should read"),
            b"body",
        );

        let response = ureq::http::Response::new(ureq::Body::builder().data(b"large".to_vec()));
        assert!(read_response::<4>(response).is_err());
        assert!(
            CratesIoRegistry::default()
                .request::<4>("not a URL")
                .is_err()
        );
        assert_eq!(
            access("request", Ok(b"body".to_vec()))
                .expect("successful response should pass through"),
            b"body",
        );
        assert_eq!(
            access("request", Err(ureq::Error::ConnectionFailed))
                .expect_err("request failure should become an owned registry error"),
            RegistryAccessError::new("request", "connection failed"),
        );
    }

    #[test]
    fn maps_each_crates_io_request_boundary_without_network_access() {
        let registry = CratesIoRegistry {
            agent: Agent::with_parts(Config::default(), FailingConnector, FixedResolver),
        };

        assert_eq!(
            registry.search("geam-images"),
            Err(RegistryAccessError::new(
                "search crates.io",
                "connection failed",
            )),
        );
        assert_eq!(
            registry.index("geam-images"),
            Err(RegistryAccessError::new(
                "read crates.io sparse index for geam-images",
                "connection failed",
            )),
        );
        assert_eq!(
            registry.configuration(),
            Err(RegistryAccessError::new(
                "read crates.io registry configuration",
                "connection failed",
            )),
        );
        assert_eq!(
            registry.download("https://downloads.example/archive"),
            Err(RegistryAccessError::new(
                "download provider archive from https://downloads.example/archive",
                "connection failed",
            )),
        );
    }

    #[derive(Debug)]
    struct FixedResolver;

    impl Resolver for FixedResolver {
        fn resolve(
            &self,
            _: &Uri,
            _: &Config,
            _: NextTimeout,
        ) -> Result<ResolvedSocketAddrs, Error> {
            let mut addresses = self.empty();
            addresses.push(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 443)));
            Ok(addresses)
        }
    }

    #[derive(Debug)]
    struct FailingConnector;

    impl Connector for FailingConnector {
        type Out = ();

        fn connect(
            &self,
            _: &ConnectionDetails<'_>,
            _: Option<()>,
        ) -> Result<Option<Self::Out>, Error> {
            Err(Error::ConnectionFailed)
        }
    }
}
