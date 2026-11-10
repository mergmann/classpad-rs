#![no_std]
#![no_main]

use hollyhock::{
    define_app_cargo,
    env::vars,
    fs::{File, FugueError},
    io::bufread::{BufReadExt, BufReader},
    prelude::*,
};

define_app_cargo!();

fn main_read() -> Result<(), FugueError> {
    let mut file = BufReader::new(File::open(c"\\fls0\\example.txt")?);
    for line in file.lines() {
        let line = line?;
        println!("line={line}");
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
