//! Where Scoria's own builds come from: `sources.json` in the launcher's data folder.
//!
//! The default is this fork's GitHub repository. A server can be configured without rebuilding:
//!
//! ```json
//! { "builds": { "repo": "wshrx/scoria",
//!               "releases": "https://our.server/scoria/releases.json" } }
//! ```
//!
//! `releases` must answer with the GitHub releases array (tag_name, assets, body...). It is only
//! about our own builds; Minecraft files never come from here.

use serde::Deserialize;
use std::path::PathBuf;

pub const DEFAULT_REPO: &str = "wshrx/scoria";

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Sources {
    #[serde(default)]
    pub builds: Builds,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Builds {
    /// `owner/repo`, for the GitHub API and release assets.
    pub repo: Option<String>,
    /// A full URL that answers with a GitHub-compatible releases array.
    pub releases: Option<String>,
    /// A base URL for release files: `{tag}` and `{name}` are replaced. When unset, GitHub's
    /// `/releases/download/{tag}/{name}` under `repo` is used. `SHA256SUMS` and its `.sig`
    /// are fetched from the same place, so a mirror must carry them too.
    pub assets: Option<String>,
}

fn path() -> PathBuf {
    std::env::var_os("SCORIA_SOURCES")
        .filter(|p| !p.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| crate::backend::data_dir().join("sources.json"))
}

pub fn load() -> Sources {
    std::fs::read_to_string(path())
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default()
}

pub fn repo() -> String {
    load()
        .builds
        .repo
        .filter(|r| !r.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_REPO.to_string())
}

pub fn asset_url(tag: &str, name: &str) -> String {
    match load().builds.assets.filter(|u| !u.trim().is_empty()) {
        Some(template) => template
            .replace("{tag}", tag)
            .replace("{name}", name)
            .trim_end_matches('/')
            .to_string(),
        None => format!(
            "https://github.com/{}/releases/download/{tag}/{name}",
            repo()
        ),
    }
}

pub fn releases_url() -> String {
    load()
        .builds
        .releases
        .filter(|u| !u.trim().is_empty())
        .unwrap_or_else(|| {
            format!(
                "https://api.github.com/repos/{}/releases?per_page=100",
                repo()
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_repository_is_this_fork() {
        // No environment leak: the test only checks the fallback constant.
        assert_eq!(DEFAULT_REPO, "wshrx/scoria");
    }

    #[test]
    fn a_server_can_replace_both_the_repo_and_the_feed() {
        let sources: Sources = serde_json::from_str(
            r#"{"builds":{"repo":"example/scoria","releases":"https://example.test/releases.json"}}"#,
        )
        .unwrap();
        assert_eq!(sources.builds.repo.as_deref(), Some("example/scoria"));
        assert_eq!(
            sources.builds.releases.as_deref(),
            Some("https://example.test/releases.json")
        );
    }
}
