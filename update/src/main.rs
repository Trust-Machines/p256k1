use itertools::Itertools;
use std::{fs, path::Path, process::Command};

use syn::{ForeignItem, Ident, Item};

fn main() {
    const USER: &str = "bitcoin-core";
    const REPO_NAME: &str = "secp256k1";
    const COMMIT_SHA: &str = "2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480";

    let url = format!("https://github.com/{USER}/{REPO_NAME}/archive/{COMMIT_SHA}.zip");

    let output_dir = format!("./p256k1/_{REPO_NAME}");
    if Path::new(&output_dir).exists() {
        fs::remove_dir_all(&output_dir).unwrap();
    }

    {
        const ZIP: &str = "tmp.zip";
        Command::new("curl")
            .arg("-L")
            .arg("-o")
            .arg(ZIP)
            .arg(&url)
            .status_unwrap();

        const TMP_DIR: &str = "tmp";
        Command::new("unzip")
            .arg("-d")
            .arg(TMP_DIR)
            .arg(ZIP)
            .status_unwrap();
        fs::remove_file(ZIP).unwrap();
        fs::rename(format!("{TMP_DIR}/{REPO_NAME}-{COMMIT_SHA}"), &output_dir).unwrap();
        fs::remove_dir_all(TMP_DIR).unwrap();
    }

    //
    const PREFIX_FILE: &str = "./p256k1/_p256k1.h";
    let list = {
        const TMP_BINDINGS: &str = "./tmp_bindings.rs";

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
                            _ => {}
                        }
                    }
                }
            }
            v.sort();
            v
        };
        fs::remove_file(TMP_BINDINGS).unwrap();
        list
    };

    {
        let version = COMMIT_SHA;

        let prefix = |v| -> String { format!("s{version}_{v}") };

        write_file(
            PREFIX_FILE,
            &["#ifndef P256K1_H", "#define P256K1_H"],
            list.iter().map(|v| format!("#define {v} {}", prefix(v))),
            &["#endif", ""],
        );
        write_file(
            "./p256k1/src/_rename.rs",
            &["pub use crate::bindings::{"],
            list.iter().map(|v| format!("    {} as {v},", prefix(v))),
            &["};", ""],
        );

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
                a.iter().map(|v| v.to_string())
            }
        }
    }
}

trait CommandEx {
    fn status_unwrap(&mut self);
}

impl CommandEx for Command {
    fn status_unwrap(&mut self) {
        if !self.status().unwrap().success() {
            panic!("command failed.");
        }
    }
}

fn read_syntax(path: &str) -> syn::File {
    let file_content = std::fs::read_to_string(path).unwrap();
    syn::parse_file(&file_content).unwrap()
}

fn save_bindings(path: &str) {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("./p256k1/wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        // .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        .unwrap();

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings.write_to_file(path).unwrap();
}
