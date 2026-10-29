//! Minecraft's own game files, downloaded from Mojang's piston-meta and piston-data.
//!
//! Preparation used to stop when no launcher on the machine held the pinned version. It now
//! falls back to this module: the version JSON from piston-meta, the client jar, the client
//! libraries and every asset object from the asset index, each written whole and checked against
//! the sha1 Mojang lists. The files land in Scoria's own official-launcher layout
//! (`<data>/minecraft/versions/<v>/…`), which `sources::resolve` already reads first.
//!
//! Only the pinned version is downloaded: Scoria runs one Minecraft version, and a version JSON
//! is the only thing that says which jars, libraries and assets that version is made of.

use crate::mojang::File;
use crate::sources::{self, Pins};
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::AtomicBool;

/// Where the game files come from. The UI shows it as the source of an install.
pub const SOURCE: &str = "https://piston-meta.mojang.com";
/// How many files are fetched at once. Small jars dominate the count; the client jar is one file.
pub const JOBS: usize = 6;

/// Files done, of how many.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Count {
    pub done: usize,
    pub total: usize,
}

impl Count {
    pub fn complete(&self) -> bool {
        self.total > 0 && self.done == self.total
    }
}

/// What an install holds, read from the files already on disk.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Files {
    pub manifest: bool,
    pub client: bool,
    pub libraries: Count,
    pub assets: Count,
    /// Bytes still to fetch for the parts that are missing.
    pub missing_bytes: u64,
    pub installed: bool,
}

impl Files {
    fn finish(mut self) -> Files {
        self.installed =
            self.manifest && self.client && self.libraries.complete() && self.assets.complete();
        self
    }
}

/// The version folder in `dest`, the official launcher's layout.
pub fn version_dir(dest: &Path, version: &str) -> PathBuf {
    dest.join("versions").join(version)
}

pub fn client_jar(dest: &Path, version: &str) -> PathBuf {
    version_dir(dest, version).join(format!("{version}.jar"))
}

pub fn version_json(dest: &Path, version: &str) -> PathBuf {
    version_dir(dest, version).join(format!("{version}.json"))
}

/// The install's state, without touching the network: the version JSON (when it is one Mojang
/// published), the client jar against the pin, and every library and asset object the JSON lists.
pub fn status(pins: &Pins, dest: &Path) -> Files {
    let manifest_path = version_json(dest, &pins.version);
    let manifest = pins.manifest_accepted(&manifest_path);
    if !manifest {
        return Files::default();
    }
    let client = sources::sha1_file(&client_jar(dest, &pins.version)).as_deref()
        == Some(pins.client_sha1.as_str());
    let Ok(text) = fs::read_to_string(&manifest_path) else {
        return Files::default();
    };
    let Ok(json) = serde_json::from_str::<Value>(&text) else {
        return Files::default();
    };
    let (libraries, mut missing) = missing_libraries(&json, dest);
    let (assets, asset_bytes) = missing_assets(&json, dest);
    if !client {
        missing += json["downloads"]["client"]["size"].as_u64().unwrap_or(0);
    }
    Files {
        manifest,
        client,
        libraries,
        assets,
        // The version JSON itself is already here; the index and the files are counted above.
        missing_bytes: missing + asset_bytes,
        installed: false,
    }
    .finish()
}

/// The client libraries the official launcher installs here, and the bytes still missing.
fn missing_libraries(manifest: &Value, dest: &Path) -> (Count, u64) {
    let Some(libraries) = manifest["libraries"].as_array() else {
        return (Count::default(), 0);
    };
    let names: Vec<&str> = libraries
        .iter()
        .filter_map(|l| l.get("name").and_then(Value::as_str))
        .collect();
    let mut count = Count::default();
    let mut missing = 0;
    for library in libraries {
        if !library_allowed(library) || !native_library_matches(library, &names) {
            continue;
        }
        let Some(artifact) = library.get("downloads").and_then(|d| d.get("artifact")) else {
            continue;
        };
        let Some(path) = artifact["path"].as_str().and_then(safe_relative) else {
            continue;
        };
        count.total += 1;
        let file = dest.join("libraries").join(path);
        if whole(&file, artifact["size"].as_u64()) {
            count.done += 1;
        } else {
            missing += artifact["size"].as_u64().unwrap_or(0);
        }
    }
    (count, missing)
}

/// The asset index's objects, and the bytes still missing (the index included).
fn missing_assets(manifest: &Value, dest: &Path) -> (Count, u64) {
    let Some(index) = manifest.get("assetIndex") else {
        return (Count::default(), 0);
    };
    let Some(id) = index["id"].as_str() else {
        return (Count::default(), 0);
    };
    let path = dest
        .join("assets")
        .join("indexes")
        .join(format!("{id}.json"));
    let mut missing = 0;
    if sources::sha1_file(&path).as_deref() != index["sha1"].as_str() {
        missing += index["size"].as_u64().unwrap_or(0);
    }
    let Ok(text) = fs::read_to_string(&path) else {
        return (Count::default(), missing);
    };
    let Ok(json) = serde_json::from_str::<Value>(&text) else {
        return (Count::default(), missing);
    };
    let Some(objects) = json["objects"].as_object() else {
        return (Count::default(), missing);
    };
    let mut count = Count::default();
    for object in objects.values() {
        let Some(hash) = object["hash"].as_str().filter(|h| is_hash(h)) else {
            continue;
        };
        let size = object["size"].as_u64();
        count.total += 1;
        if whole(&object_path(dest, hash), size) {
            count.done += 1;
        } else {
            missing += size.unwrap_or(0);
        }
    }
    (count, missing)
}

/// Downloads every file the pinned version needs that is not already whole. Returns the state
/// afterwards; files that finished before a stop are kept, so the next run fetches only the rest.
pub fn download(
    pins: &Pins,
    dest: &Path,
    jobs: usize,
    fetch: sources::Fetch,
    cancel: &AtomicBool,
    log: &dyn Fn(String),
    progress: &dyn Fn(usize, usize),
) -> Result<Files, String> {
    let version = &pins.version;
    let manifest_path = version_json(dest, version);
    sources::fetch_manifest(pins, &manifest_path, fetch, log)?;
    let text = fs::read_to_string(&manifest_path)
        .map_err(|e| format!("{}: {e}", manifest_path.display()))?;
    let manifest: Value =
        serde_json::from_str(&text).map_err(|e| format!("Mojang's {version} version file: {e}"))?;
    let index = manifest.get("assetIndex").cloned().unwrap_or(Value::Null);
    let index_id = index["id"]
        .as_str()
        .ok_or_else(|| format!("Mojang's {version} version file has no asset index"))?;
    let index_path = dest
        .join("assets")
        .join("indexes")
        .join(format!("{index_id}.json"));
    let index_sha1 = index["sha1"]
        .as_str()
        .ok_or_else(|| format!("Mojang's {version} asset index has no sha1"))?;
    if sources::sha1_file(&index_path).as_deref() != Some(index_sha1) {
        let body = sources::fetch_checked(fetch, text_of(&index, "url")?, index_sha1)
            .map_err(|e| format!("Mojang's {version} asset index {e}"))?;
        sources::write_file(&index_path, &body)?;
    }
    let index_json: Value = serde_json::from_slice(
        &fs::read(&index_path).map_err(|e| format!("{}: {e}", index_path.display()))?,
    )
    .map_err(|e| format!("Mojang's {version} asset index: {e}"))?;

    let mut files = Vec::new();
    let client = &manifest["downloads"]["client"];
    let pinned_client = client["sha1"].as_str().unwrap_or_default();
    if pinned_client != pins.client_sha1 {
        return Err(format!(
            "Mojang's {version} version file lists client {pinned_client}, not the one Scoria pins"
        ));
    }
    files.push(File {
        url: text_of(client, "url")?.to_string(),
        sha1: pins.client_sha1.clone(),
        size: client["size"].as_u64(),
        path: client_jar(dest, version),
    });

    let Some(objects) = index_json["objects"].as_object() else {
        return Err(format!("Mojang's {version} asset index lists no objects"));
    };
    for object in objects.values() {
        let Some(hash) = object["hash"].as_str().filter(|h| is_hash(h)) else {
            continue;
        };
        files.push(File {
            url: format!(
                "https://resources.download.minecraft.net/{}/{hash}",
                &hash[..2]
            ),
            sha1: hash.to_string(),
            size: object["size"].as_u64(),
            path: object_path(dest, hash),
        });
    }

    let Some(libraries) = manifest["libraries"].as_array() else {
        return Err(format!("Mojang's {version} version file has no libraries"));
    };
    let names: Vec<&str> = libraries
        .iter()
        .filter_map(|l| l.get("name").and_then(Value::as_str))
        .collect();
    for library in libraries {
        if !library_allowed(library) || !native_library_matches(library, &names) {
            continue;
        }
        let Some(artifact) = library.get("downloads").and_then(|d| d.get("artifact")) else {
            continue;
        };
        let Some(path) = artifact["path"].as_str().and_then(safe_relative) else {
            continue;
        };
        files.push(File {
            url: text_of(artifact, "url")?.to_string(),
            sha1: text_of(artifact, "sha1")?.to_string(),
            size: artifact["size"].as_u64(),
            path: dest.join("libraries").join(path),
        });
    }

    let bytes: u64 = files.iter().filter_map(|f| f.size).sum();
    log(format!(
        "Downloading Minecraft {version} from Mojang: {} files, {}",
        files.len(),
        megabytes(bytes)
    ));
    crate::mojang::fetch_all_with(&files, jobs, fetch, cancel, log, progress)?;
    Ok(status(pins, dest))
}

fn object_path(dest: &Path, hash: &str) -> PathBuf {
    dest.join("assets")
        .join("objects")
        .join(&hash[..2])
        .join(hash)
}

/// A file of the size Mojang lists, or any size when it lists none.
fn whole(path: &Path, size: Option<u64>) -> bool {
    fs::metadata(path)
        .map(|m| m.is_file() && size.is_none_or(|size| m.len() == size))
        .unwrap_or(false)
}

fn megabytes(bytes: u64) -> String {
    if bytes >= 1_000_000_000 {
        format!("{:.1} GB", bytes as f64 / 1e9)
    } else {
        format!("{:.0} MB", bytes as f64 / 1e6)
    }
}

fn text_of<'a>(value: &'a Value, key: &str) -> Result<&'a str, String> {
    value[key]
        .as_str()
        .ok_or_else(|| format!("Mojang's file has no {key}"))
}

fn is_hash(text: &str) -> bool {
    text.len() == 40 && text.bytes().all(|b| b.is_ascii_hexdigit())
}

/// A relative path from the version JSON, refused if it could leave the folder.
fn safe_relative(path: &str) -> Option<PathBuf> {
    let ok = !path.contains('\\')
        && !path.starts_with('/')
        && Path::new(path)
            .components()
            .all(|c| matches!(c, Component::Normal(_) | Component::CurDir));
    ok.then(|| PathBuf::from(path))
}

fn rule_system() -> &'static str {
    if cfg!(target_os = "macos") {
        "osx"
    } else if cfg!(windows) {
        "windows"
    } else {
        "linux"
    }
}

/// The official launcher's rules: the last matching rule decides, no rules means allowed.
fn library_allowed(library: &Value) -> bool {
    let system = rule_system();
    let rules = library
        .get("rules")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let mut allowed = rules.is_empty();
    for rule in rules {
        let os = rule.get("os");
        if os
            .and_then(|os| os["name"].as_str())
            .is_some_and(|name| name != system)
        {
            continue;
        }
        if let Some(arch) = os.and_then(|os| os["arch"].as_str()) {
            if !arch_matches(arch) {
                continue;
            }
        }
        // A rule that asks for a feature (demo mode, a custom resolution) never matches here.
        let features = rule.get("features");
        let asks = match features {
            Some(Value::Object(o)) => !o.is_empty(),
            Some(Value::Array(a)) => !a.is_empty(),
            Some(Value::String(s)) => !s.is_empty(),
            Some(Value::Bool(b)) => *b,
            _ => false,
        };
        if asks {
            continue;
        }
        allowed = rule.get("action").and_then(Value::as_str) == Some("allow");
    }
    allowed
}

/// The architecture patterns version JSONs use (`x86`, `x86_64`, `arm64`), against this machine.
fn arch_matches(pattern: &str) -> bool {
    let patterns: Vec<&str> = pattern.split('|').map(str::trim).collect();
    let aliases: &[&str] = match std::env::consts::ARCH {
        "x86_64" => &["x86_64", "amd64", "x64"],
        "aarch64" => &["arm64", "aarch64"],
        "x86" => &["i686", "x86"],
        other => &[other],
    };
    patterns
        .iter()
        .any(|pattern| aliases.iter().any(|alias| pattern == alias))
}

fn classifier_ends_with(classifier: &str, names: &[&str]) -> bool {
    names.iter().any(|name| {
        classifier == *name
            || classifier
                .strip_suffix(name)
                .is_some_and(|rest| rest.ends_with('-'))
    })
}

/// Modern manifests list x64 and ARM natives with OS-only rules; keep this machine's. A plain
/// `natives-<os>` jar is the x64 build when the manifest also lists an ARM variant of it.
fn native_library_matches(library: &Value, names: &[&str]) -> bool {
    let arm = matches!(std::env::consts::ARCH, "aarch64");
    let name = library["name"].as_str().unwrap_or("");
    let classifier = name.split(':').nth(3).unwrap_or("");
    if classifier_ends_with(classifier, &["arm64", "aarch64", "aarch_64"]) {
        return arm;
    }
    if classifier_ends_with(classifier, &["x86_64", "x64"]) {
        return !arm;
    }
    if arm && classifier.starts_with("natives-") {
        return !["arm64", "aarch64"]
            .iter()
            .any(|suffix| names.contains(&format!("{name}-{suffix}").as_str()));
    }
    true
}

#[cfg(test)]
#[path = "piston_tests.rs"]
mod tests;
