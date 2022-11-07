extern crate bindgen;

fn main() {
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
        .compile("libsecp256k1.a");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-lib=secp256k1");
    
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
