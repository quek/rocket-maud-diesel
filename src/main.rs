#![feature(plugin)]
#![plugin(rocket_codegen)]
#![plugin(maud_macros)]

extern crate rocket;
extern crate maud;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use maud::Markup;

pub mod try_diesel;

#[get("/<name>")]
fn hello(name: &str) -> Markup {
    html! {
        h1 { "Hello, " (name) "!" }
        p "Nice to meet you!"
    }
}

fn main() {
    try_diesel::establish_connection();
    rocket::ignite().mount("/", routes![hello]).launch();
}
