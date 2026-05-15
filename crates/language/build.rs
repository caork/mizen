fn main() {
    if let Ok(bundled) = std::env::var("MIZEN_BUNDLE") {
        println!("cargo:rustc-env=MIZEN_BUNDLE={}", bundled);
    }
}
