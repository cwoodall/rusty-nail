extern crate rusty_nail;
extern crate diesel;

use self::rusty_nail::*;
use self::rusty_nail::recipe::*;
use self::rusty_nail::recipe::models::*;
use self::rusty_nail::recipe::schema;

use self::diesel::prelude::*;

fn main() {
    use self::rusty_nail::recipe::schema::recipes::dsl::*;

    let connection = establish_connection();
    let results = recipes.load::<Recipe>(&connection)
        .expect("Error loading recipes");

    println!("Displaying {} recipes", results.len());
    for recipe in results {
        println!("{}", recipe.name);
        println!("----------\n");
        println!("{}", recipe.description);
    }
}
