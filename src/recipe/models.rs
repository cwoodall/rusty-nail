use recipe::schema::*;

#[derive(Queryable)]
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

#[derive(Queryable)]
pub struct RecipeIngredient {
    pub id: i32,
    pub recipe_id: i32,
    pub name: String,
    pub amount: f32,
}

#[derive(Insertable)]
#[table_name="recipe_ingredients"]
pub struct NewRecipeIngredient<'a> {
    pub recipe_id: i32,
    pub name: &'a str,
    pub amount: f32,
}
