#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::Request;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!!"
}

#[get("/<name>")]
fn name(name: &str) -> String {
    format!("Hello, {}", name)
}

#[error(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

fn main() {
    rocket::ignite()
        .mount("/hello", routes![index, world, name])
        .catch(errors![not_found])
        .launch();
}
