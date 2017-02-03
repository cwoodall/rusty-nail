#![feature(custom_derive, plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;

extern crate serde_json;

extern crate rocket;
extern crate rocket_contrib;

extern crate rusty_nail;
extern crate diesel;


use self::rusty_nail::recipe::*;
use self::rusty_nail::recipe::models::*;
use self::diesel::prelude::*;
use self::rusty_nail::errors::*;

use std::io::{stdin, Read};
use rocket_contrib::JSON;

#[get("/recipes")]
fn get_recipes() -> JSON<Vec<MixerRecipe>> {
    let connection = establish_connection();

    JSON(MixerRecipe::all(&connection))
}

#[get("/recipes/<name>")]
fn get_recipe_by_name(name: String) -> Result<JSON<MixerRecipe>> {
    let connection = establish_connection();
    let recipe_name = name.replace("%20", " ");
    match MixerRecipe::find(&connection, &recipe_name) {
        Ok(recipe) => Ok(JSON(recipe)),
        Err(_) => Err(ErrorKind::RecipeNotFound(name.to_string()).into()),
    }
}

#[get("/ingredients")]
fn get_ingredients() -> JSON<Vec<Ingredient>> {
    let connection = establish_connection();
    JSON(Ingredient::all(&connection))
}

#[post("/ingredients",  data = "<ingredient>")]
fn post_new_ingredients(ingredient: JSON<NewIngredient>) -> Result<JSON<Ingredient>> {
    let connection = establish_connection();
    try!(Ingredient::create(&connection,
                            &ingredient.name,
                            &ingredient.description,
                            ingredient.available));

    Ok(JSON(try!(Ingredient::find(&connection, &ingredient.name))))
}

fn main() {
    rocket::ignite()
        .mount("/api",
               routes![get_ingredients, get_recipes, get_recipe_by_name, post_new_ingredients])
        .launch();
}
