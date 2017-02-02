#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/hello/<name>")]
fn index(name: &str) -> String {
    format!("Hello, {0}!", name)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
