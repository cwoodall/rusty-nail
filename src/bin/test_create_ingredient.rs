extern crate rusty_nail;
extern crate diesel;

use self::rusty_nail::recipe::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What is the Recipe?");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)]; // Drop the newline character
    println!("\nDescribe it then press {}\n", EOF);
    let mut description = String::new();
    stdin().read_to_string(&mut description).unwrap();

    let size = create_ingredient(&connection, name, &description, 1)
        .expect("Could not create recipe");
    println!("\nSaved ingredient {} with size {}", name, size);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
