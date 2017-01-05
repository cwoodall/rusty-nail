extern crate rusty_nail;
extern crate diesel;

use self::rusty_nail::*;
use self::rusty_nail::recipe::*;
use self::rusty_nail::recipe::models::*;
use self::rusty_nail::recipe::schema;

use self::diesel::prelude::*;

fn main() {
    use self::rusty_nail::recipe::schema::ingredients::dsl::*;

    let connection = establish_connection();
    let results = ingredients.load::<Ingredient>(&connection)
        .expect("Error loading recipes");

    println!("Displaying {} ingredients", results.len());
    for ingredient in results {
        println!("{:?}", ingredient);
    }
}
