#[macro_use]
extern crate backend;
extern crate diesel;

use backend::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What would you like the name to be?");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() -1)]; // Drop the newline character

    println!("\nOK! Let's write {} (Press {} when finished)\n", name, EOF);
    let mut acronym = String::new();
    stdin().read_line(&mut acronym).unwrap();
    let acronym = &acronym[..(acronym.len() -1)]; // Drop the newline character

    let game = create_game(&connection, name, acronym);
    println!("\nSaved game with name \'{}\' and acronym \'{}\'.", name, acronym);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";