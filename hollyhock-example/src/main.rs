#![no_std]
#![no_main]

use hollyhock::{define_app_cargo, env::vars, prelude::*};

define_app_cargo!();

#[hollyhock::main]
fn main() {
    println!("Hello from Rust!");

    let mut buffer = String::new();
    for (name, var) in vars() {
        println!("{name} = {var}");
        buffer.push_str(name);
        buffer.push('=');
        buffer.push_str(var);
        buffer.push(',');
    }
    println!("{buffer}")
}
