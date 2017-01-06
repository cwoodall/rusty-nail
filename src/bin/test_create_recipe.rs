extern crate rusty_nail;
extern crate diesel;

use self::rusty_nail::recipe::*;
use self::rusty_nail::recipe::models::*;
use self::diesel::prelude::*;

use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What is the Recipe?");
    let mut my_name = String::new();
    stdin().read_line(&mut my_name).unwrap();
    let my_name = &my_name[..(my_name.len() - 1)]; // Drop the newline character
    println!("\nDescribe it then press {}\n", EOF);
    let mut my_description = String::new();
    stdin().read_to_string(&mut my_description).unwrap();

    let size = Recipe::create(&connection, my_name, &my_description)
        .expect("Could not create recipe");
    println!("\nSaved draft {} with size {}", my_name, size);


    let mut recipe = Recipe::find(&connection, my_name).expect("Error loading recipes");

    let ingredients = Ingredient::all(&connection);
    println!("Displaying {} ingredients", ingredients.len());
    for (i, ingredient) in ingredients.iter().enumerate() {
        println!("{}. {:?}", i, ingredient);
    }

    println!("\nSelect one ingredient per line and then press {}\n", EOF);
    let mut my_ingredients_string = String::new();
    stdin().read_to_string(&mut my_ingredients_string).unwrap();

    let ingredients_indexes: Vec<usize> = my_ingredients_string.lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    for ingredients_index in ingredients_indexes {
        recipe.add_ingredient(&connection,
                              ingredients[ingredients_index].name.as_str(),
                              1.0);
    }
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
