use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=assets/WebView2Loader.dll");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // out_dir is target/{profile}/build/vpack-gui-.../out
    // Navigate up to target/{profile}
    let target_dir = out_dir
        .parent().and_then(|p| p.parent()).and_then(|p| p.parent());

    if let Some(target_dir) = target_dir {
        let src = PathBuf::from("assets/WebView2Loader.dll");
        if src.exists() {
            let dest = target_dir.join("WebView2Loader.dll");
            let _ = fs::copy(&src, &dest);
        }
    }
}
