use diesel::prelude::*;
use recipe::schema::*;
use diesel::sqlite::SqliteConnection;
use diesel;

use errors::*;

#[table_name="recipes"]
#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[has_many(recipe_ingredients, foreign_key="recipe_id")]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Insertable)]
#[table_name="recipes"]
pub struct NewRecipe<'a> {
    pub name: &'a str,
    pub description: &'a str,
}

#[table_name="recipe_ingredients"]
#[belongs_to(Recipe, foreign_key="recipe_id")]
#[belongs_to(Ingredient, foreign_key="ingredient_id")]
#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
pub struct RecipeIngredient {
    pub id: i32,
    pub recipe_id: i32,
    pub ingredient_id: i32,
    pub amount: f32,
}

#[derive(Insertable)]
#[table_name="recipe_ingredients"]
pub struct NewRecipeIngredient {
    pub recipe_id: i32,
    pub ingredient_id: i32,
    pub amount: f32,
}


#[derive(Deserialize, Serialize, Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[has_many(recipe_ingredients, foreign_key="ingredient_id")]
#[table_name="ingredients"]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub available: bool,
}

#[derive(Clone, Deserialize, Serialize, Debug, Identifiable,  AsChangeset)]
#[table_name="ingredients"]
pub struct UpdateIngredient {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub available: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Queryable)]
pub struct JoinIngredient {
    pub name: String,
    pub description: String,
    pub available: bool,
    pub amount: f32,
}


#[derive(Deserialize, Serialize, Insertable)]
#[table_name="ingredients"]
pub struct NewIngredient {
    pub name: String,
    pub description: String,
    pub available: bool,
}

impl Recipe {
    pub fn create<'a>(conn: &SqliteConnection,
                      name: &'a str,
                      description: &'a str)
                      -> Result<usize> {
        let new_recipe = NewRecipe {
            name: name,
            description: description,
        };

        let size = try!(diesel::insert(&new_recipe)
            .into(recipes::table)
            .execute(conn));
        Ok(size)
    }

    pub fn find<'a>(conn: &SqliteConnection, name: &'a str) -> Result<Recipe> {
        recipes::table.filter(recipes::dsl::name.eq(name))
            .first::<Recipe>(conn)
            .map_err(|e| ErrorKind::DatabaseError(e).into())
    }

    pub fn find_by_id<'a>(conn: &SqliteConnection, id: i32) -> Result<Recipe> {
        recipes::table.filter(recipes::dsl::id.eq(id))
            .first::<Recipe>(conn)
            .map_err(|e| ErrorKind::DatabaseError(e).into())
    }

    pub fn add_ingredient(&self, conn: &SqliteConnection, name: &str, amount: f32) -> Result<()> {
        let ingredient = try!(Ingredient::find(conn, name));

        let res = try!(RecipeIngredient::create(conn, self.id, ingredient.id, amount));

        Ok(())
    }

    pub fn add_ingredients(&self,
                           conn: &SqliteConnection,
                           ingredients: Vec<(&str, f32)>)
                           -> Result<()> {
        for (name, amount) in ingredients {
            try!(self.add_ingredient(conn, name, amount))
        }
        Ok(())
    }

    pub fn update_ingredients(&self,
                              conn: &SqliteConnection,
                              ingredients: &Vec<JoinIngredient>)
                              -> Result<()> {
        let recipe = try!(Recipe::find(conn, &self.name));
        for ingredient in ingredients {
            let ing = try!(Ingredient::find(conn, &ingredient.name));

            try!(RecipeIngredient::create(conn, recipe.id, ing.id, ingredient.amount));
        }
        Ok(())
    }

    pub fn all(conn: &SqliteConnection) -> Vec<Recipe> {
        recipes::table.load(conn).expect("Could not get recipes table")
    }

    pub fn get_ingredients(&self, conn: &SqliteConnection) -> Result<Vec<JoinIngredient>> {
        let a: Vec<JoinIngredient> = try!(ingredients::table.inner_join(recipe_ingredients::table)
            .filter(recipe_ingredients::dsl::recipe_id.eq(self.id))
            .select((ingredients::dsl::name,
                     ingredients::dsl::description,
                     ingredients::dsl::available,
                     recipe_ingredients::dsl::amount))
            .load(conn));
        Ok(a)
    }
}

impl RecipeIngredient {
    pub fn create<'a>(conn: &SqliteConnection,
                      recipe_id: i32,
                      ingredient_id: i32,
                      amount: f32)
                      -> Result<usize> {

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
}

impl Ingredient {
    pub fn create<'a>(conn: &SqliteConnection,
                      name: &'a str,
                      description: &'a str,
                      available: bool)
                      -> Result<usize> {

        let new_ingrdient = NewIngredient {
            name: name.to_string(),
            description: description.to_string(),
            available: available,
        };

        let size = try!(diesel::insert(&new_ingrdient)
            .into(ingredients::table)
            .execute(conn));
        Ok(size)
    }


    pub fn find_by_id(conn: &SqliteConnection, id: i32) -> Result<Ingredient> {
        ingredients::table.filter(ingredients::dsl::id.eq(id))
            .first::<Ingredient>(conn)
            .map_err(|e| ErrorKind::DatabaseError(e).into())
    }


    pub fn find<'a>(conn: &SqliteConnection, name: &'a str) -> Result<Ingredient> {
        ingredients::table.filter(ingredients::dsl::name.eq(name))
            .first::<Ingredient>(conn)
            .map_err(|e| ErrorKind::DatabaseError(e).into())
    }

    pub fn all(conn: &SqliteConnection) -> Vec<Ingredient> {
        ingredients::table.load(conn).expect("Could not get ingredients table")
    }

    pub fn update(self, conn: &SqliteConnection, x: UpdateIngredient) -> Result<Ingredient> {
        // What is the new name we are changing to? Default to keeping the name the same.
        let mut new_name = self.name;

        // Match against x.name which is an Option<T>
        match x.name {
            Some(name) => {
                // If the name is the same as the old name then we don't need to perform this
                // search.
                if name != new_name {
                    // Check if the named ingredient already exists. If it does return an Error
                    // code.
                    try!(match Ingredient::find(conn, &name) {
                        Err(_) => Ok(0 as usize),
                        Ok(_) => Err(format!("Ingredient {} already exists in the database", name)),
                    });
                }
                // To change the name we need to remove it and recreate it.
                try!(diesel::delete(ingredients::table.filter(ingredients::dsl::name.eq(new_name)))
                        .execute(conn));

                // Create and insert the new ingredient.
                let new_ingredient = NewIngredient {
                    name: name.to_string(),
                    description: x.description.unwrap_or(self.description),
                    available: x.available.unwrap_or(self.available),
                };
                new_name = name.to_string();
                try!(diesel::insert(&new_ingredient)
                    .into(ingredients::table)
                    .execute(conn))
            }

            None => try!(diesel::update(ingredients::table).set(&x).execute(conn)),
        };

        // Return the altered ingredient
        Ok(try!(Ingredient::find(conn, &new_name)))
    }

    pub fn get_recipes(&self, conn: &SqliteConnection) -> Vec<Recipe> {
        let found: Vec<RecipeIngredient> =
            recipe_ingredients::table.filter(recipe_ingredients::dsl::ingredient_id.eq(self.id))
                .load(conn)
                .expect("Could not load recipe_ingredients table");

        let mut result: Vec<Recipe> = vec![];
        for recipe_ingredient in found {
            let recipe: Recipe =
                recipes::table.find(recipe_ingredient.id).first(conn).expect("No Result Found");
            result.push(recipe);
        }

        result
    }
}
