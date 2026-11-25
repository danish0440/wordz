//! Windows only: embed the app icon and version info (VERSIONINFO) into `wordz.exe`, so it
//! shows in Explorer, the taskbar, the Start menu and Alt-Tab.
//!
//! The build channel (`WORDZ_BUILD_CHANNEL`) is decided here, before any code is compiled: a
//! nightly-v1 build gets its own cfg, its own icon and its exact feed version in VERSIONINFO.
//! On every other target this does nothing but that. A missing resource compiler is a warning, so
//! a cross-compile from macOS or Linux still links, unless `WORDZ_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

#[path = "src/build_channel.rs"]
mod build_channel;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=WORDZ_BUILD_CHANNEL");
    println!("cargo:rustc-check-cfg=cfg(wordz_nightly_v1)");
    let nightly = match std::env::var("WORDZ_BUILD_CHANNEL").as_deref() {
        Ok("nightly-v1") => {
            println!("cargo:rustc-cfg=wordz_nightly_v1");
            true
        }
        // An empty value (an unset CI input) means the default stable build.
        Ok("stable" | "") | Err(std::env::VarError::NotPresent) => false,
        _ => {
            eprintln!("unsupported WORDZ_BUILD_CHANNEL (expected stable or nightly-v1)");
            std::process::exit(1);
        }
    };
    println!("cargo:rerun-if-changed=../../assets/app-icon/wordz.ico");
    println!("cargo:rerun-if-changed=../../assets/app-icon/wordz-nightly.ico");
    println!("cargo:rerun-if-env-changed=WORDZ_REQUIRE_WINRES");
    println!("cargo:rerun-if-env-changed=WORDZ_VERSION");
    let built_version = if nightly {
        match std::env::var("WORDZ_VERSION") {
            Ok(version) if build_channel::is_nightly_version(&version) => version,
            _ => {
                eprintln!("nightly-v1 requires an explicit WORDZ_VERSION feed identity");
                std::process::exit(1);
            }
        }
    } else {
        std::env::var("CARGO_PKG_VERSION").unwrap_or_default()
    };
    println!("cargo:rustc-env=WORDZ_BUILT_VERSION={built_version}");

    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon(build_channel::windows_icon_path(nightly))
        .set("ProductName", if nightly { "WordZ Nightly" } else { "WordZ" })
        .set("FileDescription", if nightly { "WordZ Nightly word processor" } else { "WordZ word processor" })
        .set("CompanyName", "Learning Machines LLC")
        .set("LegalCopyright", "Copyright (c) the WordCraft authors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "wordz.exe")
        .set("InternalName", "wordz");
    if nightly {
        // The Launcher verifies the PE ProductVersion against the exact feed identity, not FileVersion.
        res.set("ProductVersion", &built_version);
    }
    if let Err(e) = res.compile() {
        if std::env::var_os("WORDZ_REQUIRE_WINRES").is_some() {
            eprintln!("embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=wordz.exe built without icon/version resources: {e}");
    }
}
