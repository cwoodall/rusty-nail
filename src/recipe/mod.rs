pub mod models;
pub mod schema;

use super::errors::*;
use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use self::models::{Recipe, NewRecipe};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn create_recipe<'a>(conn: &SqliteConnection,
                         name: &'a str,
                         description: &'a str)
                         -> Result<usize> {
    use self::schema::recipes;

    let new_recipe = NewRecipe {
        name: name,
        description: description,
    };

    let size = try!(diesel::insert(&new_recipe)
        .into(recipes::table)
        .execute(conn));
    Ok(size)
}
