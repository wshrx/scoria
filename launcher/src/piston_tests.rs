//! The downloader against a fake Mojang: real checksums, fake bytes, no network.
use super::*;
use crate::sources::sha1_hex;
use serde_json::json;
use std::collections::HashMap;
use std::sync::atomic::Ordering;
use std::sync::Mutex;

/// A scratch folder, removed at the end.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Scratch {
        let dir = std::env::temp_dir().join(format!("scoria-piston-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        Scratch(dir)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Mojang's hosts, serving the bodies given and remembering every URL asked for.
struct Mojang {
    bodies: HashMap<String, Vec<u8>>,
    calls: Mutex<Vec<String>>,
}

impl Mojang {
    fn fetch(&self, url: &str) -> Result<Vec<u8>, String> {
        self.calls.lock().unwrap().push(url.to_string());
        self.bodies
            .get(url)
            .cloned()
            .ok_or_else(|| format!("{url} answered 404"))
    }
}

struct Game {
    mojang: Mojang,
    pins: Pins,
    client: Vec<u8>,
    asset: Vec<u8>,
    library: Vec<u8>,
}

/// Fake game files and the version JSON that lists them.
fn game() -> Game {
    let client = b"fake 26.3 client jar".to_vec();
    let library = b"fake lwjgl".to_vec();
    let asset = b"fake asset object".to_vec();
    let hash = sha1_hex(&asset);
    let index = serde_json::to_vec(&json!({
        "objects": {
            "minecraft/sounds/click.ogg": { "hash": hash, "size": asset.len() }
        }
    }))
    .unwrap();
    let index_sha1 = sha1_hex(&index);
    let client_sha1 = sha1_hex(&client);
    let library_sha1 = sha1_hex(&library);
    let manifest = serde_json::to_vec(&json!({
        "id": "26.3",
        "downloads": {
            "client": { "sha1": client_sha1, "size": client.len(),
                        "url": "https://piston-data.mojang.com/v1/objects/client.jar" }
        },
        "assetIndex": { "id": "34", "sha1": index_sha1, "size": index.len(),
                        "url": "https://piston-meta.mojang.com/v1/packages/34.json" },
        "libraries": [
            {
                "name": "org.lwjgl:lwjgl:3.4.3",
                "downloads": { "artifact": {
                    "path": "org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar",
                    "sha1": library_sha1, "size": library.len(),
                    "url": "https://libraries.minecraft.net/lwjgl-3.4.3.jar"
                }}
            },
            {
                "name": "org.lwjgl:lwjgl:3.4.3:natives-windows",
                "rules": [{ "action": "allow", "os": { "name": "windows" } }],
                "downloads": { "artifact": {
                    "path": "org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3-natives-windows.jar",
                    "sha1": library_sha1, "size": library.len(),
                    "url": "https://libraries.minecraft.net/lwjgl-natives-windows.jar"
                }}
            }
        ]
    }))
    .unwrap();
    let manifest_sha1 = sha1_hex(&manifest);
    let pins = Pins {
        version: "26.3".into(),
        client_sha1,
        manifests: vec![manifest_sha1.clone()],
    };
    let mut bodies = HashMap::new();
    let base = format!("https://piston-meta.mojang.com/v1/packages/{manifest_sha1}/26.3.json");
    bodies.insert(base, manifest);
    bodies.insert(
        "https://piston-meta.mojang.com/v1/packages/34.json".into(),
        index,
    );
    bodies.insert(
        "https://piston-data.mojang.com/v1/objects/client.jar".into(),
        client.clone(),
    );
    bodies.insert(
        "https://libraries.minecraft.net/lwjgl-3.4.3.jar".into(),
        library.clone(),
    );
    bodies.insert(
        format!(
            "https://resources.download.minecraft.net/{}/{}",
            &hash[..2],
            hash
        ),
        asset.clone(),
    );
    Game {
        mojang: Mojang {
            bodies,
            calls: Mutex::new(Vec::new()),
        },
        pins,
        client,
        asset,
        library,
    }
}

#[test]
fn downloads_the_version_json_client_libraries_and_assets() {
    let scratch = Scratch::new("download");
    let game = game();
    let dest = scratch.0.join("minecraft");
    let cancel = AtomicBool::new(false);
    let logs = Mutex::new(Vec::new());
    let files = download(
        &game.pins,
        &dest,
        3,
        &|url| game.mojang.fetch(url),
        &cancel,
        &|line| logs.lock().unwrap().push(line),
        &|_, _| {},
    )
    .unwrap();

    assert!(files.installed, "{files:?}");
    assert_eq!(files.client, true);
    assert!(files.manifest);
    assert_eq!(files.libraries.done, 1, "only this OS's library");
    assert_eq!(files.assets.done, 1);
    assert_eq!(fs::read(client_jar(&dest, "26.3")).unwrap(), game.client);
    assert_eq!(
        fs::read(dest.join("libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3.jar")).unwrap(),
        game.library
    );
    let hash = sha1_hex(&game.asset);
    assert_eq!(
        fs::read(dest.join("assets/objects").join(&hash[..2]).join(&hash)).unwrap(),
        game.asset
    );
    assert!(dest.join("assets/indexes/34.json").is_file());
    assert!(!dest
        .join("libraries/org/lwjgl/lwjgl/3.4.3/lwjgl-3.4.3-natives-windows.jar")
        .exists());
    assert!(logs
        .lock()
        .unwrap()
        .iter()
        .any(|line| line.contains("Downloading Minecraft 26.3 from Mojang")));
}

#[test]
fn a_second_download_asks_mojang_for_nothing() {
    let scratch = Scratch::new("again");
    let game = game();
    let dest = scratch.0.join("minecraft");
    let cancel = AtomicBool::new(false);
    let fetch = |url: &str| game.mojang.fetch(url);
    download(&game.pins, &dest, 2, &fetch, &cancel, &|_| {}, &|_, _| {}).unwrap();
    let before = game.mojang.calls.lock().unwrap().len();
    let files = download(&game.pins, &dest, 2, &fetch, &cancel, &|_| {}, &|_, _| {}).unwrap();
    assert!(files.installed);
    assert_eq!(game.mojang.calls.lock().unwrap().len(), before);
}

#[test]
fn a_half_written_asset_is_fetched_again() {
    let scratch = Scratch::new("partial");
    let game = game();
    let dest = scratch.0.join("minecraft");
    let cancel = AtomicBool::new(false);
    let fetch = |url: &str| game.mojang.fetch(url);
    download(&game.pins, &dest, 1, &fetch, &cancel, &|_| {}, &|_, _| {}).unwrap();
    let hash = sha1_hex(&game.asset);
    let object = dest.join("assets/objects").join(&hash[..2]).join(&hash);
    fs::write(&object, b"truncated").unwrap();
    let status = super::status(&game.pins, &dest);
    assert!(!status.installed);
    assert!(status.missing_bytes > 0);
    assert!(
        download(&game.pins, &dest, 1, &fetch, &cancel, &|_| {}, &|_, _| {})
            .unwrap()
            .installed
    );
    assert_eq!(fs::read(object).unwrap(), game.asset);
}

#[test]
fn status_without_a_manifest_is_empty() {
    let scratch = Scratch::new("empty");
    let game = game();
    let status = super::status(&game.pins, &scratch.0.join("minecraft"));
    assert_eq!(status, Files::default());
}

#[test]
fn the_platform_rules_pick_this_system() {
    let linux = json!({ "name": "l", "rules": [
        { "action": "allow" },
        { "action": "disallow", "os": { "name": "windows" } }
    ]});
    let windows_only = json!({ "name": "w", "rules": [
        { "action": "allow", "os": { "name": "windows" } }
    ]});
    assert_eq!(library_allowed(&linux), !cfg!(windows));
    assert_eq!(library_allowed(&windows_only), cfg!(windows));
    assert!(library_allowed(&json!({ "name": "no rules" })));
}

#[test]
fn a_feature_rule_never_matches() {
    let demo = json!({ "name": "demo", "rules": [
        { "action": "allow", "features": { "is_demo_user": true } }
    ]});
    assert!(!library_allowed(&demo));
}

#[test]
fn a_path_that_leaves_the_folder_is_refused() {
    assert!(safe_relative("org/lwjgl/lwjgl.jar").is_some());
    assert!(safe_relative("../outside.jar").is_none());
    assert!(safe_relative("/etc/passwd").is_none());
    assert!(safe_relative("org\\..\\outside.jar").is_none());
}

#[test]
fn cancelling_stops_before_the_next_file() {
    let scratch = Scratch::new("cancel");
    let game = game();
    let dest = scratch.0.join("minecraft");
    let cancel = AtomicBool::new(true);
    let error = download(
        &game.pins,
        &dest,
        1,
        &|url| game.mojang.fetch(url),
        &cancel,
        &|_| {},
        &|_, _| {},
    )
    .unwrap_err();
    assert!(error.contains("cancelled"), "{error}");
    cancel.store(false, Ordering::Relaxed);
}
