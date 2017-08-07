extern crate walkdir;

use std::env;
use std::io::Write;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;

use walkdir::WalkDir;

fn git_describe() {
    let git_dir = Path::new("../.git");

    println!("cargo:rerun-if-changed={}", git_dir.join("HEAD").display());
    for entry in WalkDir::new(git_dir.join("refs")) {
        let entry = entry.unwrap();
        println!("cargo:rerun-if-changed={}", entry.path().display());
    }

    let version;
    if git_dir.exists() {
        let git_describe =
            Command::new("git")
                    .arg("describe")
                    .arg("--tags")
                    .arg("--dirty")
                    .arg("--always")
                    .arg("--long")
                    .arg("--abbrev=8")
                    .output()
                    .ok()
                    .and_then(|o| String::from_utf8(o.stdout).ok())
                    .map(|mut s| {
                        let len = s.trim_right().len();
                        s.truncate(len);
                        s
                    })
                    .unwrap();
        let parts = git_describe.split("-").collect::<Vec<_>>();
        version = format!("{}+{}.{}", parts[0], parts[1], parts[2]);
    } else {
        version = "unknown".to_owned();
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut f = File::create(out_dir.join("git-describe")).unwrap();
    write!(f, "{}", version).unwrap();
}

fn linker_script() {
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=memory.x");
}

fn main() {
    linker_script();
    git_describe();
}
