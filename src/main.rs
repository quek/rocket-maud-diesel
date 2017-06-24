#![feature(plugin)]
#![plugin(rocket_codegen)]
#![plugin(maud_macros)]

extern crate rocket;
extern crate maud;

use maud::Markup;

#[get("/<name>")]
fn hello(name: &str) -> Markup {
    html! {
        h1 { "Hello, " (name) "!" }
        p "Nice to meet you!"
    }
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
