#![feature(proc_macro)]
extern crate rusty_nail;

#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate diesel;

use self::rusty_nail::recipe::*;
use self::rusty_nail::recipe::models::*;
use self::rusty_nail::recipe::schema::recipes::dsl as recipes;
use self::rusty_nail::recipe::schema::recipe_ingredients::dsl as recipe_ingredients;
use self::rusty_nail::recipe::schema::ingredients::dsl as ingredients;
use self::diesel::prelude::*;

#[derive(Debug, Queryable)]
struct MixerIngredient {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub available: bool,
    pub amount: f32,
}

#[derive(Debug)]
struct MixerRecipe {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub ingredients: Vec<MixerIngredient>,
}

fn main() {
    let connection = establish_connection();

    let results: Vec<Recipe> = recipes::recipes.load(&connection)
        .expect("Error loading recipes");


    println!("Displaying {} recipes", results.len());


    for recipe in results {
        let test: MixerRecipe = MixerRecipe {
            id: recipe.id,
            name: recipe.name,
            description: recipe.description,
            ingredients: ingredients::ingredients.inner_join(recipe_ingredients::recipe_ingredients)
                .filter(recipe_ingredients::recipe_id.eq(recipe.id))
                .select((ingredients::id,
                         ingredients::name,
                         ingredients::description,
                         ingredients::available,
                         recipe_ingredients::amount))
                .load(&connection)
                .unwrap(),
        };
        println!("{:?}", test);
    }
}
