use std::{env, fs, path::PathBuf};

const CONFIG_FILENAME: &str = "Config.toml";

fn main() {
    let to = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("target")
        .join(env::var("PROFILE").unwrap())
        .join(CONFIG_FILENAME);
    let file = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join(CONFIG_FILENAME);

    match fs::copy(file, to) {
        Ok(_) => (),
        Err(_) => {
            // panic!("unable to copy file/no env files")
        }
    }
}
