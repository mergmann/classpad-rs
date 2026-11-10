#![no_std]
#![no_main]

use hollyhock::{
    define_app_cargo,
    env::vars,
    fs::{BFileError, read_dir},
    prelude::*,
};

define_app_cargo!();

fn main_read() -> Result<(), BFileError> {
    let read = read_dir("\\fls0\\*")?;
    for entry in read {
        let entry = entry?;
        let name = entry.name_to_string().unwrap_or_default();
        println!(
            "name={name:?} size={} file={} dir={}",
            entry.size(),
            entry.is_file(),
            entry.is_dir()
        );
    }
    Ok(())
}

#[hollyhock::main]
fn main() {
    println!("Hello from Rust!");
    for (name, var) in vars() {
        println!("{name} = {var}");
    }

    if let Err(err) = main_read() {
        eprintln!("Error: {err}");
    }
}
