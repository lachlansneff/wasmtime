
use std::path::PathBuf;

fn main() {
    let wasi_root = PathBuf::from("./spec").canonicalize().unwrap();
    println!("cargo:rustc-env=WASI_ROOT={}", wasi_root.display());
}
