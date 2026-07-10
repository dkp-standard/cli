use std::path::Path;
use std::str::FromStr;

use semver::{Prerelease, Version};

use crate::error::{DkpError, DkpResult};

/// Default prerelease identifier used for `pre*` bumps, matching npm's default `pre` id.
const DEFAULT_PRERELEASE_ID: &str = "pre";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionBump {
    Major,
    Minor,
    Patch,
    Premajor,
    Preminor,
    Prepatch,
    Prerelease,
    Explicit(Version),
}

impl FromStr for VersionBump {
    type Err = DkpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "major" => Ok(VersionBump::Major),
            "minor" => Ok(VersionBump::Minor),
            "patch" => Ok(VersionBump::Patch),
            "premajor" => Ok(VersionBump::Premajor),
            "preminor" => Ok(VersionBump::Preminor),
            "prepatch" => Ok(VersionBump::Prepatch),
            "prerelease" => Ok(VersionBump::Prerelease),
            other => Version::parse(other)
                .map(VersionBump::Explicit)
                .map_err(|e| DkpError::VersionBumpInvalid {
                    version: other.to_string(),
                    reason: e.to_string(),
                }),
        }
    }
}

/// Computes the version resulting from applying `bump` to `current`.
pub fn bump_version(current: &Version, bump: &VersionBump) -> Version {
    match bump {
        VersionBump::Major => Version::new(current.major + 1, 0, 0),
        VersionBump::Minor => Version::new(current.major, current.minor + 1, 0),
        VersionBump::Patch => Version::new(current.major, current.minor, current.patch + 1),
        VersionBump::Premajor => prerelease_of(Version::new(current.major + 1, 0, 0)),
        VersionBump::Preminor => prerelease_of(Version::new(current.major, current.minor + 1, 0)),
        VersionBump::Prepatch => prerelease_of(Version::new(
            current.major,
            current.minor,
            current.patch + 1,
        )),
        VersionBump::Prerelease => bump_prerelease(current),
        VersionBump::Explicit(v) => v.clone(),
    }
}

fn prerelease_of(mut v: Version) -> Version {
    v.pre = Prerelease::new(&format!("{DEFAULT_PRERELEASE_ID}.0")).expect("valid prerelease");
    v
}

/// If `current` already has a prerelease with a trailing numeral, increments it.
/// Otherwise starts a new prerelease at `.0` on the current version's numbers.
fn bump_prerelease(current: &Version) -> Version {
    if !current.pre.is_empty() {
        if let Some((prefix, num)) = current.pre.as_str().rsplit_once('.') {
            if let Ok(n) = num.parse::<u64>() {
                let mut v = current.clone();
                v.pre = Prerelease::new(&format!("{prefix}.{}", n + 1)).expect("valid prerelease");
                return v;
            }
        }
        // No trailing numeral to increment; restart numbering under the same id.
        let mut v = current.clone();
        v.pre = Prerelease::new(&format!("{}.0", current.pre.as_str())).expect("valid prerelease");
        return v;
    }

    let mut v = Version::new(current.major, current.minor, current.patch + 1);
    v.pre = Prerelease::new(&format!("{DEFAULT_PRERELEASE_ID}.0")).expect("valid prerelease");
    v
}

/// Reads `manifest.json` at `pack_root`, applies `bump`, and writes the result back,
/// mutating only the `"version"` key and leaving all other keys (including any
/// unmodeled/future fields) untouched and in their original order.
///
/// Returns `(old_version, new_version)`. Errors (without writing) if the new version
/// equals the current version and `allow_same_version` is false.
pub fn bump_manifest_version(
    pack_root: &Path,
    bump: &VersionBump,
    allow_same_version: bool,
) -> DkpResult<(Version, Version)> {
    let manifest_path = pack_root.join("manifest.json");
    if !manifest_path.exists() {
        return Err(DkpError::ManifestMissing(pack_root.to_path_buf()));
    }

    let bytes = std::fs::read(&manifest_path)?;
    let mut value: serde_json::Value = serde_json::from_slice(&bytes)?;

    let current_str = value
        .get("version")
        .and_then(|v| v.as_str())
        .ok_or_else(|| DkpError::ManifestFieldMissing { field: "version" })?
        .to_string();

    let current = Version::parse(&current_str).map_err(|e| DkpError::VersionInvalid {
        version: current_str.clone(),
        reason: e.to_string(),
    })?;

    let new_version = bump_version(&current, bump);

    if new_version == current && !allow_same_version {
        return Err(DkpError::VersionUnchanged {
            current: current.to_string(),
            new: new_version.to_string(),
        });
    }

    if let Some(obj) = value.as_object_mut() {
        obj.insert(
            "version".to_string(),
            serde_json::Value::String(new_version.to_string()),
        );
    }

    let serialized = serde_json::to_string_pretty(&value)?;
    std::fs::write(&manifest_path, serialized + "\n")?;

    Ok((current, new_version))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn v(s: &str) -> Version {
        Version::parse(s).unwrap()
    }

    #[test]
    fn patch_minor_major_bumps() {
        assert_eq!(bump_version(&v("1.0.0"), &VersionBump::Patch), v("1.0.1"));
        assert_eq!(bump_version(&v("1.0.0"), &VersionBump::Minor), v("1.1.0"));
        assert_eq!(bump_version(&v("1.0.0"), &VersionBump::Major), v("2.0.0"));
    }

    #[test]
    fn minor_bump_on_major_zero_has_no_special_case() {
        assert_eq!(bump_version(&v("0.1.0"), &VersionBump::Minor), v("0.2.0"));
    }

    #[test]
    fn pre_bumps_start_at_dot_zero() {
        assert_eq!(
            bump_version(&v("1.2.3"), &VersionBump::Premajor),
            v("2.0.0-pre.0")
        );
        assert_eq!(
            bump_version(&v("1.2.3"), &VersionBump::Preminor),
            v("1.3.0-pre.0")
        );
        assert_eq!(
            bump_version(&v("1.2.3"), &VersionBump::Prepatch),
            v("1.2.4-pre.0")
        );
    }

    #[test]
    fn prerelease_bump_increments_existing_numeral() {
        assert_eq!(
            bump_version(&v("1.2.3"), &VersionBump::Prerelease),
            v("1.2.4-pre.0")
        );
        assert_eq!(
            bump_version(&v("1.2.4-pre.0"), &VersionBump::Prerelease),
            v("1.2.4-pre.1")
        );
    }

    #[test]
    fn explicit_bump_uses_given_version() {
        assert_eq!(
            bump_version(&v("1.0.0"), &VersionBump::Explicit(v("9.9.9"))),
            v("9.9.9")
        );
    }

    #[test]
    fn from_str_parses_keywords_and_explicit_versions() {
        assert_eq!("patch".parse::<VersionBump>().unwrap(), VersionBump::Patch);
        assert_eq!(
            "1.2.3".parse::<VersionBump>().unwrap(),
            VersionBump::Explicit(v("1.2.3"))
        );
        assert!("not-a-version".parse::<VersionBump>().is_err());
    }

    fn write_manifest(dir: &Path, contents: &str) {
        std::fs::write(dir.join("manifest.json"), contents).unwrap();
    }

    fn minimal_manifest(version: &str) -> String {
        format!(
            r#"{{
                "spec": "1.0.0",
                "name": "test-pack",
                "version": "{version}",
                "domain": "testing",
                "audience": "internal",
                "intended_use": "unit tests",
                "known_limitations": "none",
                "update_date": "2026-01-01"
            }}"#
        )
    }

    #[test]
    fn bump_manifest_version_updates_only_version_field() {
        let tmp = TempDir::new().unwrap();
        write_manifest(tmp.path(), &minimal_manifest("1.0.0"));

        let (old, new) = bump_manifest_version(tmp.path(), &VersionBump::Patch, false).unwrap();
        assert_eq!(old, v("1.0.0"));
        assert_eq!(new, v("1.0.1"));

        let written = std::fs::read_to_string(tmp.path().join("manifest.json")).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&written).unwrap();
        assert_eq!(parsed["version"], "1.0.1");
        assert_eq!(parsed["name"], "test-pack");
        assert_eq!(parsed["update_date"], "2026-01-01");
    }

    #[test]
    fn bump_manifest_version_preserves_unknown_fields_and_order() {
        let tmp = TempDir::new().unwrap();
        let contents = r#"{
            "spec": "1.0.0",
            "name": "test-pack",
            "version": "1.0.0",
            "domain": "testing",
            "audience": "internal",
            "intended_use": "unit tests",
            "known_limitations": "none",
            "update_date": "2026-01-01",
            "future_field": {"nested": true}
        }"#;
        write_manifest(tmp.path(), contents);

        bump_manifest_version(tmp.path(), &VersionBump::Minor, false).unwrap();

        let written = std::fs::read_to_string(tmp.path().join("manifest.json")).unwrap();
        let keys: Vec<&str> = {
            let parsed: serde_json::Value = serde_json::from_str(&written).unwrap();
            parsed
                .as_object()
                .unwrap()
                .keys()
                .map(|k| Box::leak(k.clone().into_boxed_str()) as &str)
                .collect()
        };
        assert_eq!(
            keys,
            vec![
                "spec",
                "name",
                "version",
                "domain",
                "audience",
                "intended_use",
                "known_limitations",
                "update_date",
                "future_field",
            ]
        );
        let parsed: serde_json::Value = serde_json::from_str(&written).unwrap();
        assert_eq!(parsed["future_field"]["nested"], true);
        assert_eq!(parsed["version"], "1.1.0");
    }

    #[test]
    fn bump_manifest_version_explicit_same_version_errors_without_flag() {
        let tmp = TempDir::new().unwrap();
        write_manifest(tmp.path(), &minimal_manifest("2.0.0"));

        let err = bump_manifest_version(tmp.path(), &VersionBump::Explicit(v("2.0.0")), false)
            .unwrap_err();
        assert!(matches!(err, DkpError::VersionUnchanged { .. }));

        // Manifest must remain unchanged on disk.
        let written = std::fs::read_to_string(tmp.path().join("manifest.json")).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&written).unwrap();
        assert_eq!(parsed["version"], "2.0.0");
    }

    #[test]
    fn bump_manifest_version_explicit_same_version_succeeds_with_flag() {
        let tmp = TempDir::new().unwrap();
        write_manifest(tmp.path(), &minimal_manifest("2.0.0"));

        let (old, new) =
            bump_manifest_version(tmp.path(), &VersionBump::Explicit(v("2.0.0")), true).unwrap();
        assert_eq!(old, new);
    }

    #[test]
    fn bump_manifest_version_invalid_current_version_errors() {
        let tmp = TempDir::new().unwrap();
        write_manifest(tmp.path(), &minimal_manifest("not-a-semver"));

        let err = bump_manifest_version(tmp.path(), &VersionBump::Patch, false).unwrap_err();
        assert!(matches!(err, DkpError::VersionInvalid { .. }));
    }

    #[test]
    fn bump_manifest_version_missing_manifest_errors() {
        let tmp = TempDir::new().unwrap();
        let err = bump_manifest_version(tmp.path(), &VersionBump::Patch, false).unwrap_err();
        assert!(matches!(err, DkpError::ManifestMissing(_)));
    }
}
