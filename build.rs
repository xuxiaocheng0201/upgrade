use std::{env, fs};
use std::path::Path;
use std::process::Command;
use anyhow::{anyhow, Context, Result};

fn main() {
    let windows_updater = compile_windows_updater().unwrap();
    println!("cargo:rustc-env=UPGRADER_WINDOWS_EXE={}", windows_updater);
}

const WINDOWS_UPDATER_CARGO: &str = r#"
[package]
name = "windows-upgrader"
version = "0.2.0"
edition = "2021"

[dependencies]
anyhow = "^1.0"
encoding_rs = "~0.8"
windows = { version = "~0.52", features = ["Win32_Foundation", "Win32_Security", "Win32_System_Threading", "Win32_System_Diagnostics_Debug", "Win32_System_Kernel", "Win32_System_Memory"] }
"#;

const WINDOWS_UPDATER: &str = r#"
#![windows_subsystem = "windows"]

use std::env::args;
use std::fs::{copy, remove_file, rename};

include!("./windows.rs");

fn main() {
    let mut args = args();
    args.next().unwrap();
    let source = args.next().unwrap();
    let target = args.next().unwrap();
    let runtime = args.next().unwrap();
    let delete = args.next().unwrap() == "1";
    if let Err(e) = remove_file(&source) {
        if e.kind() != ErrorKind::NotFound {
            panic!("{}", e)
        }
    }
    if delete {
        rename(&target, &source).unwrap();
    } else {
        copy(&target, &source).unwrap();
    }
    create_process(&Command::new(source).args(args), &runtime).unwrap();
}
"#;

fn compile_windows_updater() -> Result<String> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let upgrader = Path::new(&out_dir).join("windows-upgrader");
    let upgrader = upgrader.as_path();
    fs::create_dir_all(&upgrader.join("src"))?;
    fs::write(&upgrader.join("Cargo.toml"), WINDOWS_UPDATER_CARGO)?;
    fs::write(&upgrader.join("src/main.rs"), WINDOWS_UPDATER)?;
    fs::write(&upgrader.join("src/windows.rs"), include_bytes!("src/windows/windows.rs"))?;
    let status = Command::new("cargo").current_dir(upgrader).arg("build").arg("--release").status()
        .with_context(|| "Failed to execute cargo build.")?;
    if !status.success() {
        return Err(anyhow!("Failed to compile Windows updater."));
    }
    Ok(upgrader.join("target/release/windows-upgrader.exe").to_str().unwrap().to_string())
}
