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

CREATE TABLE recipe_ingredients (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  recipe_id INTEGER NOT NULL,
  name VARCHAR NOT NULL,
  amount REAL NOT NULL
);
```

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

- Can't do many to many with diesel at the moment ([see](https://github.com/diesel-rs/diesel/issues/398))
