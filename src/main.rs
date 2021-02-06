#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod players;
mod locations;

use rocket_contrib::{serve::StaticFiles, templates::Template};

// Initialize Server & Mount Routes
fn main() {
    rocket::ignite()
        .mount("/static", StaticFiles::from("static"))
        .mount("/", routes![players::index, players::login, players::name, locations::entry])
        .attach(Template::fairing())
        .launch();
}
