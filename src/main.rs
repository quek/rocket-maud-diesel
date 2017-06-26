#![feature(plugin)]
#![plugin(rocket_codegen)]
#![plugin(maud_macros)]

extern crate rocket;
extern crate maud;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

use maud::Markup;

pub mod schema;
pub mod models;
pub mod db;
pub mod post_controller;
mod layouts;

#[get("/")]
fn hello() -> Markup {
    html! {
        h1 { "Hello!" }
        p "Nice to meet you!"
    }
}

fn main() {
    rocket::ignite()
        .manage(db::create_db_pool())
        .mount("/", routes![hello])
        .mount("/posts",
               routes![post_controller::index, post_controller::new, post_controller::create])
        .launch();
}
