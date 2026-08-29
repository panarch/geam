#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RustIdentifier(String);

impl RustIdentifier {
    pub(super) fn parse(value: &str) -> Result<Self, String> {
        let mut characters = value.chars();
        let Some(first) = characters.next() else {
            return Err("identifier is empty".to_owned());
        };
        if !(first == '_' || first.is_ascii_alphabetic())
            || !characters.all(|character| character == '_' || character.is_ascii_alphanumeric())
            || value == "_"
        {
            return Err(format!("`{value}` is not a representable Rust identifier"));
        }
        if matches!(value, "crate" | "self" | "Self" | "super") {
            return Err(format!("`{value}` cannot be used as a Rust raw identifier"));
        }

        let rendered = if is_rust_keyword(value) {
            format!("r#{value}")
        } else {
            value.to_owned()
        };
        Ok(Self(rendered))
    }

    pub(super) fn crate_alias(value: &str) -> Result<Self, String> {
        Self::parse(&value.replace('-', "_"))
    }

    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}

fn is_rust_keyword(value: &str) -> bool {
    matches!(
        value,
        "abstract"
            | "as"
            | "async"
            | "await"
            | "become"
            | "box"
            | "break"
            | "const"
            | "continue"
            | "do"
            | "dyn"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "final"
            | "fn"
            | "for"
            | "gen"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "macro"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "override"
            | "priv"
            | "pub"
            | "ref"
            | "return"
            | "static"
            | "struct"
            | "trait"
            | "true"
            | "try"
            | "type"
            | "typeof"
            | "union"
            | "unsafe"
            | "unsized"
            | "use"
            | "virtual"
            | "where"
            | "while"
            | "yield"
    )
}

#[cfg(test)]
mod tests {
    use super::RustIdentifier;

    #[test]
    fn preserves_identifiers_and_escapes_rust_keywords() {
        assert_eq!(
            RustIdentifier::parse("normalize")
                .expect("ordinary identifier should be accepted")
                .as_str(),
            "normalize",
        );
        assert_eq!(
            RustIdentifier::parse("async")
                .expect("raw identifier should be accepted")
                .as_str(),
            "r#async",
        );
        assert_eq!(
            RustIdentifier::crate_alias("my-geam")
                .expect("Cargo alias should normalize")
                .as_str(),
            "my_geam",
        );
    }

    #[test]
    fn rejects_unrepresentable_identifiers() {
        for value in ["", "_", "two-parts", "한글", "9starts", "self", "Self"] {
            assert!(
                RustIdentifier::parse(value).is_err(),
                "{value:?} should be rejected",
            );
        }
    }
}
