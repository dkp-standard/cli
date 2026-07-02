use crate::error::{DkpError, DkpResult};

pub const MIN_SLUG_LEN: usize = 2;
pub const MAX_SLUG_LEN: usize = 40;

/// Hardcoded, versioned-with-deploys denylist. Not DB-backed by design —
/// domain moderation is a manual-SQL operator runbook, not an API surface.
/// Distinct from the unrelated `BLOCKED_SCOPES` denylist (scope/namespace
/// names are a different concept from domain slugs).
const RESERVED_DOMAIN_SLUGS: &[&str] = &[
    "all", "none", "null", "undefined", "admin", "root", "system", "test", "example", "unknown",
    "n-a", "na",
];

/// Lowercase, trim, collapse any run of whitespace/punctuation into a single
/// hyphen, strip leading/trailing hyphens. Non-ASCII characters are dropped
/// as separators (ASCII-only slugs).
///
/// Examples:
///   "Startups"      -> "startups"
///   " Startups "    -> "startups"
///   "Health & Law"  -> "health-law"
///   "AI/ML Tools"   -> "ai-ml-tools"
///   "---weird---"   -> "weird"
pub fn slugify_domain(display: &str) -> String {
    let lowered = display.trim().to_lowercase();
    let mut slug = String::with_capacity(lowered.len());
    let mut prev_was_hyphen = true; // suppress leading hyphen
    for ch in lowered.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch);
            prev_was_hyphen = false;
        } else if !prev_was_hyphen {
            slug.push('-');
            prev_was_hyphen = true;
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    slug
}

/// Validate an already-computed slug: length, charset, denylist. Called
/// independently of `slugify_domain` (e.g. by the registry, which must
/// validate defensively rather than trust the CLI, and by backfill scripts
/// re-deriving slugs from stored data).
pub fn validate_domain_slug(slug: &str) -> DkpResult<()> {
    if slug.len() < MIN_SLUG_LEN || slug.len() > MAX_SLUG_LEN {
        return Err(DkpError::ManifestInvalid {
            reason: format!(
                "domain slug '{slug}' must be {MIN_SLUG_LEN}-{MAX_SLUG_LEN} characters (got {})",
                slug.len()
            ),
        });
    }
    if !slug
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return Err(DkpError::ManifestInvalid {
            reason: format!(
                "domain slug '{slug}' must contain only lowercase letters, digits, and hyphens"
            ),
        });
    }
    if slug.starts_with('-') || slug.ends_with('-') || slug.contains("--") {
        return Err(DkpError::ManifestInvalid {
            reason: format!("domain slug '{slug}' has malformed hyphenation"),
        });
    }
    if RESERVED_DOMAIN_SLUGS.contains(&slug) {
        return Err(DkpError::ManifestInvalid {
            reason: format!("domain '{slug}' is a reserved word and cannot be used"),
        });
    }
    Ok(())
}

/// Convenience: slugify + validate in one call. Used by both the CLI loader
/// and the registry publish handler.
pub fn derive_and_validate_domain_slug(display: &str) -> DkpResult<String> {
    let slug = slugify_domain(display);
    validate_domain_slug(&slug)?;
    Ok(slug)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_cases() {
        assert_eq!(slugify_domain("Startups"), "startups");
        assert_eq!(slugify_domain(" Startups "), "startups");
        assert_eq!(slugify_domain("Health & Law"), "health-law");
        assert_eq!(slugify_domain("AI/ML Tools"), "ai-ml-tools");
        assert_eq!(slugify_domain("---weird---"), "weird");
        assert_eq!(slugify_domain("support"), "support");
    }

    #[test]
    fn length_bounds() {
        assert!(validate_domain_slug("a").is_err());
        assert!(validate_domain_slug(&"a".repeat(41)).is_err());
        assert!(validate_domain_slug(&"a".repeat(40)).is_ok());
        assert!(validate_domain_slug("ab").is_ok());
    }

    #[test]
    fn reserved_words_rejected() {
        for w in ["all", "none", "admin", "undefined", "null"] {
            assert!(validate_domain_slug(w).is_err(), "{w} should be reserved");
        }
    }

    #[test]
    fn support_is_not_reserved() {
        // "support" is real example data (dkps/*/manifest.json) and is
        // blocked as a *scope* name elsewhere, but that's an unrelated
        // denylist — it must remain allowed as a domain.
        assert!(validate_domain_slug("support").is_ok());
    }

    #[test]
    fn charset_rejected_if_hand_constructed_badly() {
        assert!(validate_domain_slug("Has_Underscore").is_err());
        assert!(validate_domain_slug("-leading").is_err());
        assert!(validate_domain_slug("trailing-").is_err());
        assert!(validate_domain_slug("double--hyphen").is_err());
    }

    #[test]
    fn derive_round_trip() {
        assert_eq!(derive_and_validate_domain_slug("Startups").unwrap(), "startups");
        assert_eq!(derive_and_validate_domain_slug("Law").unwrap(), "law");
        assert_eq!(derive_and_validate_domain_slug("support").unwrap(), "support");
        assert!(derive_and_validate_domain_slug("Admin").is_err());
    }
}
