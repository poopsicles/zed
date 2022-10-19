use std::process::Command;

fn main() {
    println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.14");

    if let Ok(api_key) = std::env::var("ZED_MIXPANEL_TOKEN") {
        println!("cargo:rustc-env=ZED_MIXPANEL_TOKEN={api_key}");
    }

    let output = Command::new("npm")
        .current_dir("../../styles")
        .args(["install", "--no-save"])
        .output()
        .expect("failed to run npm");
    if !output.status.success() {
        panic!(
            "failed to install theme dependencies {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let output = Command::new("npm")
        .current_dir("../../styles")
        .args(["run", "build-themes"])
        .output()
        .expect("failed to run npm");
    if !output.status.success() {
        panic!(
            "build-themes script failed {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    println!("cargo:rerun-if-changed=../../styles/src");
}
