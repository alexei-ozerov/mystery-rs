use rocket_contrib::templates::Template;
use std::collections::HashMap;

mod actions;

#[get("/entry")]
pub fn entry() -> Template {
    let entry_actions = actions::PlayerActions {
        describe: vec!["The hall is ornate and beautiful. Paintings hang side by side, as the shriveled faces of the aristocratic elite look down upon you."],
        examine: vec!["Test!!"],
        talk: vec!["Test!!"],
        walk: vec!["Test!!"],
    };

    let context: HashMap<&str, &str> = [("description", entry_actions.describe[0])].iter().cloned().collect();
    Template::render("entry", &context)
}