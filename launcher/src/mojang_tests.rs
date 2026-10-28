//! A Java runtime download from a mock of Mojang's servers serving small fake files (never Mojang's).
use super::*;
use crate::sources::sha1_hex;
use std::collections::HashMap;

/// Never cancelled.
static GO: AtomicBool = AtomicBool::new(false);

struct Server {
    files: HashMap<String, Vec<u8>>,
    asked: Mutex<Vec<String>>,
}

impl Server {
    fn fetch(&self, url: &str) -> Result<Vec<u8>, String> {
        self.asked.lock().unwrap().push(url.to_string());
        self.files
            .get(url)
            .cloned()
            .ok_or_else(|| format!("{url} answered 404"))
    }

    fn asked(&self) -> Vec<String> {
        std::mem::take(&mut *self.asked.lock().unwrap())
    }
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("horizon-mojang-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

#[test]
fn version_file_paths_stay_in_the_folder() {
    assert!(relative("org/fake/a.jar").is_ok());
    for bad in ["../a.jar", "/etc/a", "a/../../b", "a\\b"] {
        assert!(relative(bad).is_err(), "{bad}");
    }
}

#[test]
fn links_stay_inside_the_runtime() {
    let at = |p: &str| PathBuf::from(p);
    assert!(link_stays_inside(
        &at("legal/java.compiler/LICENSE"),
        "../java.base/LICENSE"
    ));
    assert!(link_stays_inside(&at("lib/a"), "b"));
    assert!(!link_stays_inside(&at("lib/a"), "../../b"));
    assert!(!link_stays_inside(&at("a"), "/etc/passwd"));
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[test]
fn java_runtime_lays_out_mojangs_runtime_with_its_executables_and_links() {
    const JAVAC: &[u8] = b"#!/bin/sh\necho javac 25.0.1\n";
    const JAVA: &[u8] = b"#!/bin/sh\necho java 25.0.1\n";
    const LICENSE: &[u8] = b"a fake license";
    let manifest = serde_json::json!({"files": {
        "bin": {"type": "directory"},
        "bin/java": {"type": "file", "executable": true, "downloads": {"raw":
            {"sha1": sha1_hex(JAVA), "size": JAVA.len(), "url": "https://piston-data.mojang.com/java"}}},
        "bin/javac": {"type": "file", "executable": true, "downloads": {"raw":
            {"sha1": sha1_hex(JAVAC), "size": JAVAC.len(), "url": "https://piston-data.mojang.com/javac"}}},
        "legal/java.base/LICENSE": {"type": "file", "executable": false, "downloads": {"raw":
            {"sha1": sha1_hex(LICENSE), "size": LICENSE.len(), "url": "https://piston-data.mojang.com/license"}}},
        "legal/java.compiler/LICENSE": {"type": "link", "target": "../java.base/LICENSE"},
    }})
    .to_string();
    let index = serde_json::json!({"linux": {"java-runtime-epsilon": [{
        "manifest": {"sha1": sha1_hex(manifest.as_bytes()), "url": "https://piston-meta.mojang.com/m.json"},
        "version": {"name": "25.0.1"},
    }]}})
    .to_string();
    let server = Server {
        files: HashMap::from([
            (JAVA_RUNTIMES.to_string(), index.into_bytes()),
            (
                "https://piston-meta.mojang.com/m.json".into(),
                manifest.into_bytes(),
            ),
            ("https://piston-data.mojang.com/java".into(), JAVA.to_vec()),
            (
                "https://piston-data.mojang.com/javac".into(),
                JAVAC.to_vec(),
            ),
            (
                "https://piston-data.mojang.com/license".into(),
                LICENSE.to_vec(),
            ),
        ]),
        asked: Mutex::new(Vec::new()),
    };
    let dir = scratch("java");
    let home = java_runtime(
        "java-runtime-epsilon",
        &dir,
        2,
        &|url| server.fetch(url),
        &GO,
        &|_| {},
    )
    .unwrap();
    assert_eq!(
        home,
        dir.join("java-runtime-epsilon/linux/java-runtime-epsilon")
    );
    use std::os::unix::fs::PermissionsExt;
    let mode = fs::metadata(home.join("bin/javac"))
        .unwrap()
        .permissions()
        .mode();
    assert_eq!(mode & 0o111, 0o111);
    assert_eq!(
        fs::read(home.join("legal/java.compiler/LICENSE")).unwrap(),
        LICENSE
    );
    // horizon-prepare finds it where the official launcher would have put it.
    assert_eq!(horizon_prepare::java::minecraft_runtimes(&dir), [home]);
    // A second call keeps the files: only the index and manifest are asked again.
    server.asked();
    java_runtime(
        "java-runtime-epsilon",
        &dir,
        2,
        &|url| server.fetch(url),
        &GO,
        &|_| {},
    )
    .unwrap();
    assert_eq!(server.asked().len(), 2);
    assert!(java_runtime(
        "java-runtime-zeta",
        &dir,
        2,
        &|url| server.fetch(url),
        &GO,
        &|_| {}
    )
    .is_err());
    fs::remove_dir_all(&dir).unwrap();
}
