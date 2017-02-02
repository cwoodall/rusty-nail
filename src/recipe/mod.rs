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
    pub name: String,
    pub description: String,
    pub ingredients: Vec<JoinIngredient>,
}

impl MixerRecipe {
    pub fn new(name: &str,
               description: &str,
               ingredients: Vec<JoinIngredient>)
               -> Result<MixerRecipe> {
        Ok(MixerRecipe {
            name: name.to_string(),
            description: description.to_string(),
            ingredients: ingredients,
        })
    }

    pub fn find(conn: &SqliteConnection, name: &str) -> Result<MixerRecipe> {
        let recipe = try!(Recipe::find(conn, name));
        MixerRecipe::new(name,
                         &recipe.description,
                         try!(recipe.get_ingredients(conn)))
    }

    pub fn all(conn: &SqliteConnection) -> Vec<MixerRecipe> {
        let recipes = Recipe::all(conn);
        // Get ingredients for all recipes and form into recipe ingredients.
        recipes.iter()
            .map(|r| {
                MixerRecipe::new(&r.name, &r.description, r.get_ingredients(conn).unwrap()).unwrap()
            })
            .collect::<Vec<_>>()
    }

    pub fn update(&mut self, conn: &SqliteConnection) -> Result<()> {
        let recipe = match Recipe::find(conn, &self.name) {
            Ok(x) => x,
            Err(e) => {
                try!(Recipe::create(conn, &self.name, &self.description));
                try!(Recipe::find(conn, &self.name))
            }
        };

        try!(recipe.update_ingredients(conn, &self.ingredients));
        Ok(())
    }
}
