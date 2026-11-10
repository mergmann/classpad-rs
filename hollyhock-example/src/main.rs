#![no_std]
#![no_main]

use hollyhock::{env::vars, prelude::*};

#[hollyhock::main]
fn main() {
    println!("Hello from Rust!");

    for (name, var) in vars() {
        println!("{name} = {var}");
    }
}
