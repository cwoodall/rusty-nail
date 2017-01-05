use diesel::prelude::*;
use recipe::schema::*;

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


#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[has_many(recipe_ingredients, foreign_key="ingredient_id")]
#[table_name="ingredients"]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub available: bool,
}


#[derive(Insertable)]
#[table_name="ingredients"]
pub struct NewIngredient<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub available: bool,
}
