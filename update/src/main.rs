use itertools::Itertools;
use std::{collections::HashSet, error::Error, fs, io, path::Path, process::Command};

use quote::ToTokens;
use syn::{ForeignItem, Ident, Item};

fn main() {
    main_io().unwrap();
}

fn main_io() -> Result<(), Box<dyn Error>> {
    const USER: &str = "Trust-Machines";
    const REPO_NAME: &str = "secp256k1";
    const COMMIT_SHA: &str = "41b6073611725d2e12ac7a72d3da3d46fd43f932";

    let url = format!("https://github.com/{USER}/{REPO_NAME}/archive/{COMMIT_SHA}.zip");

    let output_dir = format!("../{REPO_NAME}");
    if Path::new(&output_dir).exists() {
        fs::remove_dir_all(&output_dir)?;
    }

    const ZIP: &str = "tmp.zip";
    Command::new("curl")
        .arg("-L")
        .arg("-o")
        .arg(&ZIP)
        .arg(&url)
        .status_ok()?;

    const TMP_DIR: &str = "tmp";
    Command::new("unzip")
        .arg("-d")
        .arg(&TMP_DIR)
        .arg(&ZIP)
        .status_ok()?;
    fs::remove_file(&ZIP)?;
    fs::rename(format!("{TMP_DIR}/{REPO_NAME}-{COMMIT_SHA}"), &output_dir)?;
    fs::remove_dir_all(&TMP_DIR)?;

    //
    const PREFIX_FILE: &str = "../_p256k1.h";
    const TMP_BINDINGS: &str = "./tmp_bindings.rs";

    fs::write(PREFIX_FILE, "")?;
    save_bindings(TMP_BINDINGS)?;

    let list = {
        let mut v = Vec::default();
        let mut push = |x: Ident| {
            let s = x.to_string();
            if s.starts_with("secp256k1_") {
                v.push(s);
            }
        };
        for i in read_syntax(TMP_BINDINGS)?.items {
            if let Item::ForeignMod(m) = i {
                for i in m.items {
                    match i {
                        ForeignItem::Fn(f) => push(f.sig.ident),
                        ForeignItem::Static(s) => push(s.ident),
                        _ => {}
                    }
                }
            }
        }
        v.sort();
        v
    };
    fs::remove_file(&TMP_BINDINGS)?;

    {
        let version = COMMIT_SHA;

        let prefix = |v| -> String { format!("s{version}_{v}") };

        write_file(
            PREFIX_FILE,
            &["#ifndef P256K1_H", "#define P256K1_H"],
            list.iter().map(|v| format!("#define {v} {}", prefix(v))),
            &["#endif", ""],
        )?;
        write_file(
            "../src/_rename.rs",
            &["pub use crate::_bindings::{"],
            list.iter().map(|v| format!("    {} as {v},", prefix(v))),
            &["};", ""],
        )?;

        fn write_file(
            path: &str,
            top: &[&str],
            content: impl Iterator<Item = String>,
            bottom: &[&str],
        ) -> io::Result<()> {
            fs::write(
                path,
                iter(top).chain(content).chain(iter(bottom)).join("\n"),
            )?;

            fn iter<'a>(a: &'a [&str]) -> impl Iterator<Item = String> + 'a {
                a.iter().map(|v| v.to_string())
            }

            Ok(())
        }
    }

    const BINDINGS_FILE: &str = "../src/_bindings.rs";
    save_bindings(BINDINGS_FILE)?;

    let serializable_types = ["secp256k1_scalar", "secp256k1_fe", "secp256k1_gej"];

    let add_serde_derive_attributes = |to_type_definitions: &[&str],
                                       syntax: syn::File|
     -> Result<(), Box<dyn Error>> {
        let type_identifiers: HashSet<_> = to_type_definitions
            .iter()
            .map(|name| proc_macro2::Ident::new(name, proc_macro2::Span::call_site()))
            .collect();

        let token_stream_with_added_derives = add_serde_derive_tokens(&type_identifiers, &syntax);

        let formatted_output = rustfmt_wrapper::rustfmt(proc_macro2::TokenStream::from_iter(
            token_stream_with_added_derives,
        ))?;

        std::fs::write(BINDINGS_FILE, formatted_output)?;

        Ok(())
    };

    add_serde_derive_attributes(&serializable_types, read_syntax(BINDINGS_FILE)?)?;

    Ok(())
}

trait CommandEx {
    fn status_ok(&mut self) -> io::Result<()>;
}

impl CommandEx for Command {
    fn status_ok(&mut self) -> io::Result<()> {
        if self.status()?.success() {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "failed"))
        }
    }
}

fn read_syntax(path: &str) -> Result<syn::File, Box<dyn Error>> {
    let file_content = std::fs::read_to_string(path)?;
    Ok(syn::parse_file(&file_content)?)
}

fn save_bindings(path: &str) -> Result<(), Box<dyn Error>> {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("../wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        // .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()?;

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings.write_to_file(path)?;
    Ok(())
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
