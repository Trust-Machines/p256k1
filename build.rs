fn main() {
    //
    println!("cargo:rustc-env=ECMULT_GEN_PREC_BITS=4");
    println!("cargo:rustc-env=ECMULT_WINDOW_SIZE=15");

    let mut base_config = cc::Build::new();
    base_config
        .include("secp256k1/")
        .include("secp256k1/include")
        .include("secp256k1/src")
        .flag_if_supported("-Wno-unused-function") // some ecmult stuff is defined but not used upstream
        .define("SECP256K1_API", Some(""))
        .define("ENABLE_MODULE_ECDH", Some("1"))
        .define("ENABLE_MODULE_SCHNORRSIG", Some("1"))
        .define("ENABLE_MODULE_EXTRAKEYS", Some("1"))
        .define("USE_NUM_NONE", Some("1"))
        .define("USE_FIELD_INV_BUILTIN", Some("1"))
        .define("USE_SCALAR_INV_BUILTIN", Some("1"))
        .define("ECMULT_GEN_PREC_BITS", Some("4"))
        .define("ECMULT_WINDOW_SIZE", Some("15"));

    base_config
        .file("secp256k1/src/secp256k1.c")
        .file("secp256k1/src/precomputed_ecmult.c")
        .file("secp256k1/src/precomputed_ecmult_gen.c")
        .compile("libsecp256k1.a");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-lib=secp256k1");
}
