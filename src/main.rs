#![feature(plugin)]
#![plugin(rocket_codegen)]

use std::env;
use std::env::vars_os;
use std::iter;

extern crate rocket;
use rocket::Request;

extern crate postgres;
use postgres::{Connection, TlsMode};


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

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() {
    let database_url = match env::var("DATABASE_URL") {
        Ok(value) => value,
        Err(why) => panic!(why)
    };
    println!("{}", database_url);

    let conn = Connection::connect(database_url, TlsMode::None).unwrap();
    conn.execute("CREATE TABLE person (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    data            BYTEA
                  )", &[]).unwrap();
    let me = Person {
        id: 0,
        name: "masashi".to_string(),
        data: None,
    };
    conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
                 &[&me.name, &me.data]).unwrap();

    for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2),
        };
        println!("Found person {}", person.name);
    }

    rocket::ignite()
        .mount("/hello", routes![index, world, name])
        .catch(errors![not_found])
        .launch();
}
