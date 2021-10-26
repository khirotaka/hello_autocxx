fn main() {
    let path = std::path::PathBuf::from("csrc");
    let mut b = autocxx_build::Builder::new("src/main.rs", &[&path]).expect_build();
    b.flag_if_supported("-std=c++17")
        .file("csrc/hello.cpp")
        .compile("autcxx-hello");
    println!("cargo:rerun-if-changed=src/main.rs");
}
