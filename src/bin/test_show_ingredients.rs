extern crate rusty_nail;
extern crate diesel;

use self::rusty_nail::*;
use self::rusty_nail::recipe::*;
use self::rusty_nail::recipe::models::*;

use self::diesel::prelude::*;

fn main() {

    let connection = establish_connection();
    let results = Ingredient::all(&connection);
    println!("Displaying {} ingredients", results.len());
    for ingredient in results {
        println!("{:?}", ingredient);
    }
}
