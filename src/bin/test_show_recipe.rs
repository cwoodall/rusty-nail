#![feature(proc_macro)]
extern crate rusty_nail;

#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate diesel;

use self::rusty_nail::*;
use self::rusty_nail::recipe::*;
use self::rusty_nail::recipe::models::*;
use self::rusty_nail::recipe::schema::recipes::dsl as recipes;
use self::rusty_nail::recipe::schema::recipe_ingredients::dsl as recipe_ingredients;
use self::rusty_nail::recipe::schema::ingredients::dsl as ingredients;
use self::diesel::prelude::*;

#[derive(Debug, Queryable)]
struct RecipeIngredientComplete {
    pub name: String,
    pub description: String,
    pub available: i32,
    pub amount: f32,
}

fn main() {
    let connection = establish_connection();

    let results: Vec<Recipe> = recipes::recipes.load(&connection)
        .expect("Error loading recipes");


    println!("Displaying {} recipes", results.len());
    for recipe in results {
        println!("{:?}", recipe);

        let ings: Vec<RecipeIngredientComplete> =
            ingredients::ingredients.inner_join(recipe_ingredients::recipe_ingredients)
                .filter(recipe_ingredients::recipe_id.eq(recipe.id))
                .select((ingredients::name,
                         ingredients::description,
                         ingredients::available,
                         recipe_ingredients::amount))
                .load(&connection)
                .unwrap();
        for ingredient in ings {
            println!("\t- {:?}", ingredient);
        }
    }
}
