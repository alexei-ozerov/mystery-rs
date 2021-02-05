use rocket::http::RawStr;
use rocket::request::Form;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[derive(FromForm)]
pub struct PlayerName<'f> {
    // The raw, undecoded value. You _probably_ want `String` instead.
    player_name: &'f RawStr,
}

#[post("/name", data = "<player_name>")]
pub fn name(player_name: Form<PlayerName>) -> Template {
    let context: HashMap<&str, &str> = [("name", player_name.player_name.as_str())]
        .iter()
        .cloned()
        .collect();
    Template::render("name", &context)
}
