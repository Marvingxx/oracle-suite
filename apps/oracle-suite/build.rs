//! Windows only: embed the app icon and version info (VERSIONINFO) into `oracle-suite.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `ORACLE_SUITE_REQUIRE_WINRES=1` (set
//! by the release packaging) turns it into an error.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/oracle-suite.ico");
    println!("cargo:rerun-if-env-changed=ORACLE_SUITE_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/oracle-suite.ico")
        .set("ProductName", "Oracle Suite")
        .set("FileDescription", "Oracle Suite")
        .set("CompanyName", "Oracle")
        .set("LegalCopyright", "Copyright (c) the Oracle Suite authors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "oracle-suite.exe")
        .set("InternalName", "oracle-suite");
    if let Err(e) = res.compile() {
        if std::env::var_os("ORACLE_SUITE_REQUIRE_WINRES").is_some() {
            println!("cargo:warning=embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=oracle-suite.exe built without icon/version resources: {e}");
    }
}
