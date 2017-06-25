#![feature(plugin)]
#![plugin(rocket_codegen)]
#![plugin(maud_macros)]

extern crate rocket;
extern crate maud;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

use db_pool::DbPool;
use maud::Markup;

pub mod db_pool;

#[get("/<name>")]
fn hello(name: &str, db_pool: rocket::State<DbPool>) -> Markup {
    let connection = db_pool.get();
    assert!(connection.is_ok());

    html! {
        h1 { "Hello, " (name) "!" }
        p "Nice to meet you!"
    }
}

fn main() {
    rocket::ignite().manage(db_pool::create_db_pool()).mount("/", routes![hello]).launch();
}
