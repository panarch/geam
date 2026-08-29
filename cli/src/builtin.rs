#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum BuiltInProvider {
    Stdlib,
    Json,
    Time,
}

pub(super) struct BuiltInProviderClosure {
    first: BuiltInProvider,
    remaining: &'static [BuiltInProvider],
}

impl BuiltInProvider {
    pub(super) const ALL: [Self; 3] = [Self::Stdlib, Self::Json, Self::Time];

    pub(super) fn from_package(package: &str) -> Option<Self> {
        match package {
            "gleam_stdlib" => Some(Self::Stdlib),
            "gleam_json" => Some(Self::Json),
            "gleam_time" => Some(Self::Time),
            _ => None,
        }
    }

    pub(super) fn package(self) -> &'static str {
        match self {
            Self::Stdlib => "gleam_stdlib",
            Self::Json => "gleam_json",
            Self::Time => "gleam_time",
        }
    }

    pub(super) fn geam_feature(self) -> &'static str {
        match self {
            Self::Stdlib => "gleam-stdlib",
            Self::Json => "gleam-json",
            Self::Time => "gleam-time",
        }
    }

    pub(super) fn component_closure(self) -> BuiltInProviderClosure {
        match self {
            Self::Stdlib => BuiltInProviderClosure {
                first: Self::Stdlib,
                remaining: &[],
            },
            Self::Json => BuiltInProviderClosure {
                first: Self::Stdlib,
                remaining: &[Self::Json],
            },
            Self::Time => BuiltInProviderClosure {
                first: Self::Stdlib,
                remaining: &[Self::Time],
            },
        }
    }
}

impl BuiltInProviderClosure {
    pub(super) fn first(&self) -> BuiltInProvider {
        self.first
    }

    pub(super) fn remaining(&self) -> impl Iterator<Item = BuiltInProvider> + '_ {
        self.remaining.iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::BuiltInProvider;

    #[test]
    fn owns_exact_package_identity_and_component_dependencies() {
        assert_eq!(BuiltInProvider::ALL.len(), 3);
        assert_eq!(
            BuiltInProvider::ALL.map(BuiltInProvider::package),
            ["gleam_stdlib", "gleam_json", "gleam_time"],
        );
        assert_eq!(
            BuiltInProvider::ALL.map(BuiltInProvider::geam_feature),
            ["gleam-stdlib", "gleam-json", "gleam-time"],
        );
        assert_eq!(
            ["gleam_stdlib", "gleam_json", "gleam_time"].map(BuiltInProvider::from_package),
            BuiltInProvider::ALL.map(Some),
        );
        assert_eq!(
            BuiltInProvider::from_package("gleam_stdlib"),
            Some(BuiltInProvider::Stdlib),
        );
        assert_eq!(BuiltInProvider::from_package("other"), None);
        let json = BuiltInProvider::Json.component_closure();
        assert_eq!(json.first(), BuiltInProvider::Stdlib);
        assert_eq!(
            json.remaining().collect::<Vec<_>>(),
            [BuiltInProvider::Json]
        );
        let time = BuiltInProvider::Time.component_closure();
        assert_eq!(time.first(), BuiltInProvider::Stdlib);
        assert_eq!(
            time.remaining().collect::<Vec<_>>(),
            [BuiltInProvider::Time]
        );
    }
}
