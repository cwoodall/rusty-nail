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
#[derive(Insertable)]
#[table_name="dispensers"]
pub struct NewDispenser<'a> {
    pub name: &'a str,
}

#[derive(Queryable)]
pub struct Dispenser {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name="recipe_dispensers"]
pub struct NewRecipeDispenser {
    pub recipe_id: i32,
    pub dispense_id: i32,
    pub amount: f32,
}
#[derive(Queryable)]
pub struct RecipeDispenser {
    pub id: i32,
    pub recipe_id: i32,
    pub dispense_id: String,
    pub amount: f32,
}
