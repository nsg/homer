#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate rocket_contrib;

extern crate rocket;
extern crate curl;

use rocket_contrib::Template;

mod api;

#[get("/")]
fn index() -> Template {
    let context = {};
    Template::render("index", &context)
}



fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/api/hue/", routes![api::config, api::config_value])
        .attach(Template::fairing())
        .launch();
}
