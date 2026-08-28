fn main() {
    println!("cargo:rerun-if-env-changed=BUILD_SHA");
    let sha = std::env::var("BUILD_SHA").unwrap_or_else(|_| "dev".to_string());
    println!("cargo:rustc-env=BUILD_SHA={sha}");
}
