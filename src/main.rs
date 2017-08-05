#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate rocket_contrib;
extern crate rocket;

use rocket_contrib::Template;
use rocket_contrib::Json;
use std::env;

#[get("/")]
fn index() -> Template {
    let context = {};
    Template::render("index", &context)
}

#[get("/<cmd>")]
fn api_hue(cmd: String) -> Json {
    let token = match env::var("HUE_TOKEN") {
        Ok(v) => v,
        _ => return Json(json!({"error": "Token not found"}))
    };

    Json(json!({
        "command": cmd,
        "token": token
    }))
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/api/hue/", routes![api_hue])
        .attach(Template::fairing())
        .launch();
}
