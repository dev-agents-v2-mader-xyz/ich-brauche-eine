fn main() {
    // tauri_build links against WebKitGTK on Linux; skip for WASM frontend builds.
    if std::env::var("CARGO_CFG_TARGET_ARCH").as_deref() != Ok("wasm32") {
        tauri_build::build()
    }
}
