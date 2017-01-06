#![feature(proc_macro)]
extern crate rusty_nail;

#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate diesel;

use self::rusty_nail::recipe::*;
use self::rusty_nail::recipe::models::*;
use self::diesel::prelude::*;


fn main() {
    let connection = establish_connection();

    let results: Vec<Recipe> = Recipe::all(&connection);


    println!("Displaying {} recipes", results.len());


    for recipe in results {
        println!("{:?}", recipe.get_ingredients(&connection));
    }
}
