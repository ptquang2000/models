fn main() {
    println!("cargo::rustc-check-cfg=cfg(MODELS_INTERNAL)");
    println!("cargo::rustc-cfg=MODELS_INTERNAL");
    println!(
        "cargo::rustc-link-search=native={}/build",
        env!("CARGO_MANIFEST_DIR")
    );
}
