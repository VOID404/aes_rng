fn main() {
    let srcs = ["include/AES_RNG.cpp", "include/wrapper.cpp"];

    cc::Build::new()
        .files(srcs.iter())
        .cpp(true)
        .compile("wrapper");
    // cxx_build::bridge("src/lib.rs").compile("aes_rng");

    println!("cargo:rustc-link-lib=cryptopp");
}
