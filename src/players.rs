use rocket::http::RawStr;
use rocket::request::Form;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;

// Display Login Page
#[get("/")]
pub fn index() -> Template {
    let context: HashMap<&str, &str> = [("name", "Guest")].iter().cloned().collect();
    Template::render("login", &context)
}

// Get Session Code
#[derive(FromForm)]
pub struct Cypher<'f> {
    code: &'f RawStr,
}

// Create Session File & Start Game
#[post("/beginning", data = "<code>")]
pub fn login(code: Form<Cypher>) -> Template {
    // Get Code
    let req = code.code.as_str();

    // Write Code To File
    let mut file = File::create(req).unwrap();
    file.write_all(req.as_bytes()).unwrap();

    // Generate Context
    let context: HashMap<&str, &str> = [("code", req)]
        .iter()
        .cloned()
        .collect();
    Template::render("beginning", &context)
}

// Get Player Name
#[derive(FromForm)]
pub struct PlayerName<'f> {
    player_name: &'f RawStr,
}

// Get Player Name
#[post("/name", data = "<player_name>")]
pub fn name(player_name: Form<PlayerName>) -> Template {
    let req = player_name.player_name.as_str();
    let name_vec: Vec<&str> = req.split("+").collect();
    
    let player: String;
    if name_vec.len() > 1 {
        player = name_vec[0].to_owned() + " " + name_vec[1];
    } else {
        player = name_vec[0].to_string();
    }
    
    let context: HashMap<&str, &str> = [("name", player.as_str())]
        .iter()
        .cloned()
        .collect();
    Template::render("name", &context)
}