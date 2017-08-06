extern crate serde;
extern crate serde_json;

use rocket_contrib::Json;
use std::env;
use api;

struct HueConnectionData {
    token: String,
    url: String
}

fn setup() -> HueConnectionData {
    let token = match env::var("HUE_TOKEN") {
        Ok(v) => v,
        _ => panic!("Token not found")
    };

    let url = match env::var("HUE_URL") {
        Ok(v) => v,
        _ => panic!("Hue url not found")
    };

    HueConnectionData{token, url}
}

fn api(path: &str) -> serde_json::Value {
    let huecon = setup();
    let url = format!("{}/api/{}/{}", huecon.url, huecon.token, path);
    let data = api::get_http(url);
    serde_json::from_str(&data).unwrap()
}

#[get("/config")]
fn config() -> Json {
    Json(json!({
        "config": api("config")
    }))
}

#[get("/config/<val>")]
fn config_value(val: String) -> Json {
    Json(json!({
        "config": api("config")[val]
    }))
}
