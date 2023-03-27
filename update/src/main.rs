use std::{fs, io, process::Command, path::Path};

fn main() {
    main_io().unwrap();
}

fn main_io() -> io::Result<()> {
    const USER: &str = "Trust-Machines";
    const REPO_NAME: &str = "secp256k1";
    const COMMIT_SHA: &str = "41b6073611725d2e12ac7a72d3da3d46fd43f932";

    let url = format!("https://github.com/{USER}/{REPO_NAME}/archive/{COMMIT_SHA}.zip");

    let output_dir = format!("../{REPO_NAME}");
    if Path::new(&output_dir).exists() {
        fs::remove_dir_all(&output_dir)?;
    }

    const ZIP: &str = "_.zip";
    Command::new("curl")
        .arg("-L")
        .arg("-o")
        .arg("_.zip")
        .arg(&url)
        .status_ok()?;

    const TMP_DIR: &str = "_";
    Command::new("unzip")
        .arg("-d")
        .arg(&TMP_DIR)
        .arg(&ZIP)
        .status_ok()?;
    fs::remove_file(&ZIP)?;
    fs::rename(format!("{TMP_DIR}/{REPO_NAME}-{COMMIT_SHA}"), &output_dir)?;
    fs::remove_dir_all(&TMP_DIR)?;

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
