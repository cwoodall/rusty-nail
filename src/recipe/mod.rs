pub mod models;
mod schema;

use super::errors::*;
use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use self::models::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Debug)]
pub struct MixerRecipe {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub ingredients: Vec<JoinIngredient>,
}
