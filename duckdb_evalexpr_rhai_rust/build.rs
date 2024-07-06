#[cfg(target_os = "windows")]
fn main() {
    println!("cargo:rustc-link-lib=bcrypt");
}

#[cfg(not(target_os = "windows"))]
fn main() {
    // Do nothing on non-Windows platforms
}
