## Adding tests!

I have found it useful to start writing unit tests. The Rust Book has a good
chapter on [unit testing](https://doc.rust-lang.org/book/testing.html) in rust.
For now I am going to write tests inline in the modules using the following
template:

```rust
#[cfg(test)]
mod tests {
  //  in
  use dispenser::TestDispenser;
  use mixer::Mixer;

  #[test]
  fn test_it_works() {
  }
}
```

There are a few other methods such as using the `tests` folder, but this is the
easiest for me to work with at the moment. Another interesting option is to add
doc-test code. Which allows example code in your documentation to be tested
and validated.

## Implementing the mixer struct

The `Mixers` are essentially a `HashMap` of `String`'s mapping to Dispensers.
Because the Dispensers are a trait object they must be `Box<Dispenser>`s. The
plan is to implement the following:

- [x] `clear`
- [x] `add`
- [x] `remove`
- [x] `get`
- [ ] `mix` (`make_recipe` in the previous recipe).
- [ ] `status`

## Setting Up Recipes

### Diesel with SQLite
I want the recipes to be installed in a database, for this section I am going
to use [diesel-rs](http://diesel.rs/guides/getting-started/) and sqlite3 (which
is great for embedded systems, but not webapps). This will require moving to
nightly builds of rust (which is easy with `rustup`):

```
$ rustup install nightly
$ rustup default nightly
```

Also changing the reccomended Cargo.toml addition a little:

```
diesel = "0.9.0"
diesel_codegen = { version = "0.9.0", features = ["sqlite"] }
dotenv = "0.8.0"
```

#### Make the database

First lets make the database and setup `dotenv` for diesel:

```
$ echo DATABASE_URL=$PWD/rusty_nail.db > .env
$ diesel setup
```

Then write the `up.sql` and `down.sql` for the migration:

up.sql:

```
CREATE TABLE recipes (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name VARCHAR NOT NULL,
  description TEXT NOT NULL
);

CREATE TABLE ingredients (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name VARCHAR NOT NULL,
  description TEXT NOT NULL,
  available BOOLEAN NOT NULL
);

CREATE TABLE recipe_ingredients (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  recipe_id INTEGER NOT NULL REFERENCES recipes(id) ,
  ingredient_id INTEGER NOT NULL REFERENCES ingredient(id) ,
  amount REAL NOT NULL
);
```

So we will have 3 tables in our database:

- `recipes`: Which is what we are really interested in and contains a representation
             of each individual recipe.
- `ingredients`: Which contains a representation of the available ingredients
                 (which can be mapped to dispensers).
- `recipe_ingredients`: Which will be used to create a list of ingredients for a
                        recipe and will be `JOIN`ed with `ingredients`.

So basically to get a list of ingredients for a recipe
and run the migration

```
$ diesel migration run
```

### Other Notes

- Use `use diesel::sqlite::SqliteConnection;` instead of `use diesel::pg::PgConnection;`
- `get_result()` is not supported for Sqlite insertions so:
  ```
  diesel::insert(&new_recipe).into(recipes::table)
    .get_result(conn)
    .expect("Error saving new recipe")
  ```

  From the demo does not work properly Instead you can use.

  ```
  diesel::insert(&new_recipe).into(recipe::table)
    .execute(conn)
    .expect("Error saving new recipe")
  ```

  Which returns `usize` instead of a Recipe.

### The right query for stitching ingredients to recipes

What we want to be able to do is for a given recipe look up the `Ingredient`
and `RecipeIngredient` for all ingredients in a given recipe creating a vector
of all relevant Ingrdient data ( not including primary and foreign keys):

```
+-------------+           +------------------+                +-------------+
| Recipe      |           | RecipeIngredient |                | Ingredient  |
+-------------+           +------------------+                +-------------+
| id          |<---+      | id               |        +------>| id          |
| name        |    +------| recipe_id        |        |       | name        |
| description |           | ingredient_id    |--------+       | description |
|             |           | amount           |                | available   |
+-------------+           +------------------+                +-------------+
```

To do this what we need to do is:

1. Perform a query to find the Recipe we are interested in.
2. Join the recipe_ingredients and ingrdients table using an inner join
3. Filter the join on `recipe_ingredients::recipe_id` to just get those belonging
   to our current recipe.
4. create a `select` statement to discard unneeded content.

We can then create a `Vec<String, String, i32, f32>` which can then correspond to
a  `Queryable` struct we can create which is a fusion of RecipeIngredient and
Ingredient in the following example called




... This may
change to `DispenserInstruction` or may become the public
representation of `Ingredient`. What would be ideal is to generate a
representation of a `Recipe` which had the following structure definition:

```
struct MixerIngredient {
    name: String,
    description: String,
    amount: f32
}

struct MixerRecipe {
  name: String,
  description: String,
  ingredients: Vec<MixerIngredient>
}
```

Here is an example query program:

```rust
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
struct MixerIngredient {
    pub name: String,
    pub description: String,
    pub available: bool,
    pub amount: f32,
}

fn main() {
    let connection = establish_connection();

    let results: Vec<Recipe> = recipes::recipes.load(&connection)
        .expect("Error loading recipes");


    println!("Displaying {} recipes", results.len());
    for recipe in results {
        println!("{:?}", recipe);

        let ings: Vec<MixerIngredient> =
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
```

Which can become:

```
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
                .select((ingredients::id, ingredients::name,
                         ingredients::description,
                         ingredients::available,
                         recipe_ingredients::amount))
                .load(&connection)
                .unwrap(),
        };
        println!("{:?}", test);
    }
}
```

Now the next big step is searching for ingredients which are availables and
encapsulating the states
