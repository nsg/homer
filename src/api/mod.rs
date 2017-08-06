extern crate serde;
extern crate serde_json;

use std::env;
use rocket_contrib::Json;
use curl::easy::Easy;

struct HueConnectionData {
    token: String,
    url: String
}

fn get_http(url: String) -> String {
    let mut curl = Easy::new();
    let mut dst = Vec::new();
    curl.url(&url).unwrap();

    {
        let mut tr = curl.transfer();
        tr.write_function(|d| {
            dst.extend_from_slice(d);
            Ok(d.len())
        }).unwrap();
        tr.perform().unwrap()
    }

    String::from_utf8(dst).unwrap()
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

fn api(path: &str) -> String {
    let huecon = setup();
    let url = format!("{}/api/{}/{}", huecon.url, huecon.token, path);
    get_http(url)
}

#[get("/config")]
fn config() -> Json {
    let data = api("config");
    let lr: serde_json::Value = serde_json::from_str(&data).unwrap();

    Json(json!({
        "config": lr
    }))
}

#[get("/config/<val>")]
fn config_value(val: String) -> Json {
    let data = api("config");
    let lr: serde_json::Value = serde_json::from_str(&data).unwrap();

    Json(json!({
        "config": lr[val]
    }))
}
