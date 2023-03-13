use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;
use std::ffi::OsString;
use std::fs::{write, read_to_string};
use std::iter::FromIterator;
use std::path::Path;
use std::{env, fs};

use quote::ToTokens;

fn main() {
    // fix secp256k1 source code.
    /*
    {
        let r = Regex::new("SECP256K1_API").unwrap();
        let path = Path::new("./secp256k1");
        let src = read_dir_recursive(&path.join("src"));
        let include = read_dir_recursive(&path.join("include"));
        let all = src.into_iter().chain(include.into_iter()).collect::<Vec<_>>();
        write("_regex_report.txt", format!("{:?}", all)).unwrap();
        for i in all {
            let text = read_to_string(i).unwrap();
            let new_text = r.replace_all(&text, "p256k1v3_0_0_").to_string();
            // fs::write(i, new_text).unwrap();
        }
    }
    */

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

    let bindings_file = &format!("{}/bindings.rs", env::var("OUT_DIR").unwrap());

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(bindings_file)
        .expect("Couldn't write bindings!");

    let serializable_types = ["secp256k1_scalar", "secp256k1_fe", "secp256k1_gej"];

    add_serde_derive_attributes(&serializable_types, bindings_file)
        .expect("Failed to add serde derive to type definitions");
}

fn add_serde_derive_attributes<P: AsRef<std::path::Path>>(
    to_type_definitions: &[&str],
    file_path: P,
) -> Result<(), Error> {
    let file_content = std::fs::read_to_string(&file_path).map_err(Error::Io)?;
    let syntax = syn::parse_file(&file_content).map_err(Error::Syntax)?;
    let type_identifiers: HashSet<_> = to_type_definitions
        .iter()
        .map(|name| proc_macro2::Ident::new(name, proc_macro2::Span::call_site()))
        .collect();

    let token_stream_with_added_derives = add_serde_derive_tokens(&type_identifiers, &syntax);

    let formatted_output = rustfmt_wrapper::rustfmt(proc_macro2::TokenStream::from_iter(
        token_stream_with_added_derives,
    ))
    .map_err(Error::Format)?;

    std::fs::write(&file_path, formatted_output).map_err(Error::Io)?;

    Ok(())
}

#[derive(Debug)]
enum Error {
    Io(std::io::Error),
    Syntax(syn::Error),
    Format(rustfmt_wrapper::Error),
}

fn add_serde_derive_tokens<'a>(
    type_identifiers: &'a HashSet<proc_macro2::Ident>,
    syntax: &'a syn::File,
) -> impl Iterator<Item = proc_macro2::TokenTree> + 'a {
    syntax
        .into_token_stream()
        .into_iter()
        .collect::<Vec<_>>()
        .into_iter()
        .circular_tuple_windows()
        .flat_map(move |token_tuple| expand_tokens_if_matches(type_identifiers, token_tuple))
}

type TokenTuple = (
    proc_macro2::TokenTree,
    proc_macro2::TokenTree,
    proc_macro2::TokenTree,
);

fn expand_tokens_if_matches(
    type_identifiers: &HashSet<proc_macro2::Ident>,
    tokens: TokenTuple,
) -> Vec<proc_macro2::TokenTree> {
    match tokens {
        (token, _, proc_macro2::TokenTree::Ident(ident)) => {
            if type_identifiers.contains(&ident) {
                let tokens = quote::quote! {
                #[derive(serde::Serialize, serde::Deserialize)]
                };

                let mut expanded: Vec<_> = tokens.into_iter().collect();
                expanded.push(token);
                expanded
            } else {
                vec![token]
            }
        }
        (token, _, _) => vec![token],
    }
}

/*
fn read_dir_recursive(path: &Path) -> Vec<OsString> {
    fs::read_dir(path)
        .expect(path.to_str().unwrap())
        .flat_map(|r| {
            let de = r.unwrap();
            let file_name = path.join(de.file_name());
            if de.file_type().unwrap().is_dir() {
                read_dir_recursive(&file_name)
            } else {
                vec![file_name.as_os_str().to_os_string()]
            }
        })
        .collect()
}
*/
