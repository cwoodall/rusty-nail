pub mod models;
pub mod schema;

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

pub fn create_recipe_ingredients<'a>(conn: &SqliteConnection,
                                     recipe_id: i32,
                                     ingredient_id: i32,
                                     amount: f32)
                                     -> Result<usize> {
    use self::schema::recipe_ingredients;

    let new_ingrdient = NewRecipeIngredient {
        recipe_id: recipe_id,
        ingredient_id: ingredient_id,
        amount: amount,
    };

    let size = try!(diesel::insert(&new_ingrdient)
        .into(recipe_ingredients::table)
        .execute(conn));
    Ok(size)
}

pub fn create_ingredient<'a>(conn: &SqliteConnection,
                             name: &'a str,
                             description: &'a str,
                             available: bool)
                             -> Result<usize> {
    use self::schema::ingredients;

    let new_ingrdient = NewIngredient {
        name: name,
        description: description,
        available: available,
    };

    let size = try!(diesel::insert(&new_ingrdient)
        .into(ingredients::table)
        .execute(conn));
    Ok(size)
}
