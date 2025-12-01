use std::env;
use std::fs::File;
use std::io::{Result, Write};
use std::path::PathBuf;

fn main() -> Result<()> {
    println!("Copy .session file to target directory");

    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let session_path = PathBuf::from("../../.session");

    if session_path.exists() {
        File::create(out.join(".session"))?
            .write_all(include_bytes!("../../.session"))?;
    } else {
        println!("cargo::warning=.session not found, Create the file with the session key prior to execution");
    }
    Ok(())
}
