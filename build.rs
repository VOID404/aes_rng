fn main() {
    // cc::Build::new().file("src/AES_RNG.hpp").compile("aes_rng");
    println!("cargo:rustc-link-lib=cryptopp");
    cpp_build::Config::new()
        .include("src/cpp")
        .build("src/lib.rs");
}
