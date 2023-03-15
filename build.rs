use itertools::Itertools;
use syn::{Item, ForeignItem, Ident};
use std::collections::HashSet;

use std::iter::FromIterator;
use std::{env, fs};

use quote::ToTokens;

fn main() {
    let save_bindings = |path: &str| {
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
            .write_to_file(path)
            .expect("Couldn't write bindings!");
    };

    let read_syntax = |s: &str| {
        let file_content = std::fs::read_to_string(s)
            .map_err(Error::Io)
            .expect("Couldn't open write");
        syn::parse_file(&file_content)
            .map_err(Error::Syntax)
            .expect("Couldn't parse the bindings")
    };

    const TMP_BINDINGS: &str = "./_tmp_bindings.rs";
    const PREFIX_FILE: &str = "./_p256k1.h";

    fs::write(PREFIX_FILE, "").unwrap();
    save_bindings(TMP_BINDINGS);

    let list = {
        let mut v = Vec::default();
        let mut push = |x: Ident| {
            let s = x.to_string();
            if s.starts_with("secp256k1_") {
                v.push(s);
            }
        };
        for i in read_syntax(TMP_BINDINGS).items {
            if let Item::ForeignMod(m) = i {
                for i in m.items {
                    match i {
                        ForeignItem::Fn(f) => push(f.sig.ident),
                        ForeignItem::Static(s) => push(s.ident),
                        _ => {},
                    }
                }
            }
        }
        v.sort();
        v
    };

    {
        write_file(
            PREFIX_FILE,
            &["#ifndef P256K1_H", "#define P256K1_H"],
            list
                .iter()
                .map(|v| format!("#define {v} {}", prefix(&v))),
            &["#endif", ""],
        );
        write_file(
            "./src/_rename.rs",
            &["pub use crate::bindings::{"],
            list
                .iter()
                .map(|v| format!("    {} as {v},", prefix(&v))),
            &["};", ""],
        );

        fn prefix(v: &str) -> String {
            format!("p256k1_3_0_1_{v}")
        }

        fn write_file(
            path: &str,
            top: &[&str],
            content: impl Iterator<Item = String>,
            bottom: &[&str],
        ) {
            fs::write(
                path,
                iter(top).chain(content).chain(iter(bottom)).join("\n"),
            )
            .unwrap();

            fn iter<'a>(a: &'a [&str]) -> impl Iterator<Item = String> + 'a {
                a.into_iter().map(|v| v.to_string())
            }
        }
    }

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

    let bindings_file = &format!("{}/bindings.rs", env::var("OUT_DIR").unwrap());

    save_bindings(&bindings_file);

    let serializable_types = ["secp256k1_scalar", "secp256k1_fe", "secp256k1_gej"];

    let add_serde_derive_attributes = |to_type_definitions: &[&str],
                                       syntax: syn::File|
     -> Result<(), Error> {
        let type_identifiers: HashSet<_> = to_type_definitions
            .iter()
            .map(|name| proc_macro2::Ident::new(name, proc_macro2::Span::call_site()))
            .collect();

        let token_stream_with_added_derives = add_serde_derive_tokens(&type_identifiers, &syntax);

        let formatted_output = rustfmt_wrapper::rustfmt(proc_macro2::TokenStream::from_iter(
            token_stream_with_added_derives,
        ))
        .map_err(Error::Format)?;

        std::fs::write(bindings_file, formatted_output).map_err(Error::Io)?;

        Ok(())
    };

    add_serde_derive_attributes(&serializable_types, read_syntax(bindings_file))
        .expect("Failed to add serde derive to type definitions");
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
