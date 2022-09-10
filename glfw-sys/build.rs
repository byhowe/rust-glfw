#![allow(unreachable_code)]

use std::path::Path;
use std::process::Command;

fn main() {
    // let vendored = cfg!(feature = "vendored");
    let vendored = false;

    if !vendored {
        let mut cfg = pkg_config::Config::new();
        if let Ok(_lib) = cfg.range_version("3.3".."3.4").probe("glfw3") {
            return;
        }
    }

    panic!("vendored is unsupported right now");

    println!("cargo:rustc-cfg=glfw_vendored");

    if !Path::new("glfw/.git").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init", "glfw"])
            .status();
    }
    println!("cargo:rerun-if-changed=glfw");
}
