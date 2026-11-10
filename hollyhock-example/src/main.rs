#![no_std]
#![no_main]

use hollyhock::{define_app_cargo, env::vars, prelude::*};

define_app_cargo!();

#[hollyhock::main]
fn main() {
    println!("Hello from Rust!");

    for (name, var) in vars() {
        println!("{name} = {var}");
    }
}
