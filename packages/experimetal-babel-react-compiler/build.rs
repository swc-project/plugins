use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rerun-if-changed=src/index.ts");
    println!("cargo:rerun-if-changed=rspack.config.mjs");
    println!("cargo:rerun-if-changed=package.json");

    run_pnpm_build();
}

fn run_pnpm_build() {
    let status = Command::new("pnpm")
        .args(["run", "build"])
        .status()
        .expect("failed to execute process");

    if !status.success() {
        panic!("failed to build");
    }
}
