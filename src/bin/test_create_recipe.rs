extern crate rusty_nail;
extern crate diesel;

use self::rusty_nail::recipe::*;
use self::rusty_nail::recipe::models::*;
use self::rusty_nail::recipe::schema;
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

    let size = create_recipe(&connection, my_name, &my_description)
        .expect("Could not create recipe");
    println!("\nSaved draft {} with size {}", my_name, size);

    use self::rusty_nail::recipe::schema::recipes::dsl::*;

    let mut results = recipes.filter(name.eq(my_name))
        .limit(1)
        .load::<Recipe>(&connection)
        .expect("Error loading recipes");

    let recipe_inserted = results.pop().expect("failed to get ingredient");

    {
        use self::rusty_nail::recipe::schema::ingredients::dsl::*;

        let results = ingredients.load::<Ingredient>(&connection)
            .expect("Error loading recipes");

        println!("Displaying {} ingredients", results.len());
        for ingredient in results {
            println!("{:?}", ingredient);
        }
    }

    let a = create_recipe_ingredients(&connection, recipe_inserted.id, 2, 1.0);

}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
