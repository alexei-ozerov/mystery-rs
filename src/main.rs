#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod players;

use rocket_contrib::{serve::StaticFiles, templates::Template};
use std::collections::HashMap;

#[get("/")]
fn index() -> Template {
    let context: HashMap<&str, &str> = [("name", "Alexei Ozerov")].iter().cloned().collect();
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
        .mount("/static", StaticFiles::from("static"))
        .mount("/", routes![index, players::name])
        .attach(Template::fairing())
        .launch();
}
