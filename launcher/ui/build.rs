//! Per-OS build metadata for the launcher window.
//!
//! Windows gets the Scoria icon and a version resource on `scoria-ui.exe`; every target gets the
//! version, OS and architecture compiled in, so the About/rail line cannot drift from the build.
fn main() {
    println!("cargo:rerun-if-changed=../../packaging/icons/horizon.ico");
    println!("cargo:rerun-if-changed=assets/scoria-mark.png");
    println!("cargo:rerun-if-changed=assets/fonts");
    for (key, value) in [
        ("SCORIA_TARGET_OS", std::env::var("CARGO_CFG_TARGET_OS")),
        ("SCORIA_TARGET_ARCH", std::env::var("CARGO_CFG_TARGET_ARCH")),
        ("SCORIA_VERSION", std::env::var("CARGO_PKG_VERSION")),
    ] {
        if let Ok(value) = value {
            println!("cargo:rustc-env={key}={value}");
        }
    }

    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        #[cfg(windows)]
        {
            let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
            let icon = format!("{manifest}/../../packaging/icons/horizon.ico");
            let mut resource = winresource::WindowsResource::new();
            resource
                .set_icon(&icon)
                .set("ProductName", "Scoria")
                .set("FileDescription", "Scoria launcher")
                .set("OriginalFilename", "scoria-ui.exe")
                .set(
                    "ProductVersion",
                    std::env::var("CARGO_PKG_VERSION").unwrap_or_default(),
                )
                .set("LegalCopyright", "MIT");
            if let Err(error) = resource.compile() {
                panic!("embedding the Windows resources: {error}");
            }
        }
    }
}
