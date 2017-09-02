#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate rocket_contrib;

extern crate rocket;
extern crate curl;

mod api;

use rocket_contrib::Template;
use api::hue;

#[get("/")]
fn index() -> Template {
    let context = {};
    Template::render("index", &context)
}



fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/api/hue/", routes![
               hue::config,
               hue::config_value,
               hue::lights,
               hue::lights_id,
               hue::lights_name,
               hue::set_on,
               hue::set_brightnes
        ])
        .attach(Template::fairing())
        .launch();
}
