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

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

fn replace_url_codes(s: String) -> String {
    s.replace("%20", " ");
    s
}

#[get("/recipes")]
fn get_recipes() -> JSON<Vec<MixerRecipe>> {
    let connection = establish_connection();

    JSON(MixerRecipe::all(&connection))
}

#[get("/recipes/<name>")]
fn get_recipe_by_name(name: String) -> Result<JSON<MixerRecipe>> {
    let connection = establish_connection();
    let name = replace_url_codes(name);
    match MixerRecipe::find(&connection, &name) {
        Ok(recipe) => Ok(JSON(recipe)),
        Err(_) => Err(ErrorKind::RecipeNotFound(name.to_string()).into()),
    }
}

#[get("/ingredients/<name>")]
fn get_ingredient_by_name(name: String) -> Result<JSON<Ingredient>> {
    let connection = establish_connection();
    let name = replace_url_codes(name);
    match Ingredient::find(&connection, &name) {
        Ok(ingredient) => Ok(JSON(ingredient)),
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

#[post("/ingredients/<name>",  data = "<ingredient>")]
fn post_update_ingredients(name: String,
                           ingredient: JSON<UpdateIngredient>)
                           -> Result<JSON<Ingredient>> {
    let connection = establish_connection();
    let old_ingredient: Ingredient = try!(Ingredient::find(&connection, &name));
    Ok(JSON(try!(old_ingredient.update(&connection, ingredient.into_inner()))))
}


#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/api",
               routes![get_ingredients,
                       get_recipes,
                       get_recipe_by_name,
                       post_new_ingredients,
                       get_ingredient_by_name,
                       post_update_ingredients])
        .launch();
}
