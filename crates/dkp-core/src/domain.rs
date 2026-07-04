use crate::error::{DkpError, DkpResult};

pub const MIN_SLUG_LEN: usize = 2;
pub const MAX_SLUG_LEN: usize = 40;

/// Hardcoded, versioned-with-deploys denylist. Not DB-backed by design —
/// domain moderation is a manual-SQL operator runbook, not an API surface.
/// Distinct from the unrelated `BLOCKED_SCOPES` denylist (scope/namespace
/// names are a different concept from domain slugs).
///
/// This is registry moderation policy, not structural validity — it's only
/// enforced via `validate_domain_slug_for_registry`, not the plain
/// `validate_domain_slug` used for local/offline pack builds.
const RESERVED_DOMAIN_SLUGS: &[&str] = &[
    "all",
    "none",
    "null",
    "undefined",
    "admin",
    "root",
    "system",
    "test",
    "example",
    "unknown",
    "n-a",
    "na",
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

/// Validate an already-computed slug: length, charset, hyphenation. This is
/// structural validity only — it does not enforce the registry's
/// reserved-word denylist, so it's safe to run for purely local/offline pack
/// builds (e.g. `dkp build`) where there's no registry moderation concern.
/// Called independently of `slugify_domain` (e.g. by backfill scripts
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
    Ok(())
}

/// Validate a slug for registry publication: structural validity plus the
/// reserved-word denylist. The registry must validate defensively rather
/// than trust the CLI, so this is what the publish route calls.
pub fn validate_domain_slug_for_registry(slug: &str) -> DkpResult<()> {
    validate_domain_slug(slug)?;
    if RESERVED_DOMAIN_SLUGS.contains(&slug) {
        return Err(DkpError::ManifestInvalid {
            reason: format!("domain '{slug}' is a reserved word and cannot be used"),
        });
    }
    Ok(())
}

/// Convenience: slugify + validate (structural only) in one call. Used by
/// the CLI loader (`Pack::open`) for local/offline commands like `dkp build`.
pub fn derive_and_validate_domain_slug(display: &str) -> DkpResult<String> {
    let slug = slugify_domain(display);
    validate_domain_slug(&slug)?;
    Ok(slug)
}

/// Convenience: slugify + validate (structural + reserved-word) in one call.
/// Used by the registry publish handler.
pub fn derive_and_validate_domain_slug_for_registry(display: &str) -> DkpResult<String> {
    let slug = slugify_domain(display);
    validate_domain_slug_for_registry(&slug)?;
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
    fn reserved_words_rejected_for_registry() {
        for w in ["all", "none", "admin", "undefined", "null"] {
            assert!(
                validate_domain_slug_for_registry(w).is_err(),
                "{w} should be reserved"
            );
        }
    }

    #[test]
    fn reserved_words_allowed_locally() {
        // Local/offline builds shouldn't be blocked by registry moderation
        // policy — only registry publication enforces the denylist.
        for w in ["all", "none", "admin", "undefined", "null", "test"] {
            assert!(
                validate_domain_slug(w).is_ok(),
                "{w} should be allowed for local validation"
            );
            assert!(derive_and_validate_domain_slug(w).is_ok());
        }
    }

    #[test]
    fn support_is_not_reserved() {
        // "support" is real example data (dkps/*/manifest.json) and is
        // blocked as a *scope* name elsewhere, but that's an unrelated
        // denylist — it must remain allowed as a domain.
        assert!(validate_domain_slug("support").is_ok());
        assert!(validate_domain_slug_for_registry("support").is_ok());
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
        assert_eq!(
            derive_and_validate_domain_slug("Startups").unwrap(),
            "startups"
        );
        assert_eq!(derive_and_validate_domain_slug("Law").unwrap(), "law");
        assert_eq!(
            derive_and_validate_domain_slug("support").unwrap(),
            "support"
        );
        assert!(derive_and_validate_domain_slug_for_registry("Admin").is_err());
    }

    #[test]
    fn slugify_empty_and_punctuation_only() {
        assert_eq!(slugify_domain(""), "");
        assert_eq!(slugify_domain("   "), "");
        assert_eq!(slugify_domain("!!!"), "");
    }

    #[test]
    fn slugify_unicode_dropped_as_separator() {
        // Non-ASCII chars are dropped as separators, not preserved.
        assert_eq!(slugify_domain("Café Law"), "caf-law");
    }

    #[test]
    fn validate_rejects_all_reserved_words_for_registry() {
        for w in [
            "all",
            "none",
            "null",
            "undefined",
            "admin",
            "root",
            "system",
            "test",
            "example",
            "unknown",
            "n-a",
            "na",
        ] {
            assert!(
                validate_domain_slug_for_registry(w).is_err(),
                "{w} should be reserved"
            );
        }
    }

    #[test]
    fn validate_rejects_uppercase() {
        assert!(validate_domain_slug("Startups").is_err());
    }

    #[test]
    fn validate_rejects_non_ascii() {
        assert!(validate_domain_slug("café").is_err());
    }

    #[test]
    fn derive_and_validate_reserved_word_collision_for_registry() {
        // A display name that slugifies straight into a reserved word must
        // fail registry validation, but remain fine for local validation.
        assert!(derive_and_validate_domain_slug_for_registry("Root").is_err());
        assert!(derive_and_validate_domain_slug_for_registry("  Test  ").is_err());
        assert!(derive_and_validate_domain_slug("Root").is_ok());
        assert!(derive_and_validate_domain_slug("  Test  ").is_ok());
    }

    #[test]
    fn derive_and_validate_too_short_after_slugify() {
        // Punctuation-only input slugifies to empty, which is below MIN_SLUG_LEN.
        assert!(derive_and_validate_domain_slug("!!!").is_err());
        assert!(derive_and_validate_domain_slug("a").is_err());
    }
}
