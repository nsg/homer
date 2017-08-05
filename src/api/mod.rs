use std::env;
use rocket_contrib::Json;
use std::io::{stdout, Write};
use curl::easy::Easy;

fn get_http(uri: String) {
    let mut easy = Easy::new();
    easy.url(uri.as_str()).unwrap();
    easy.write_function(|data| {
        Ok(stdout().write(data).unwrap())
    }).unwrap();
    easy.perform().unwrap();
}

#[get("/<cmd>")]
fn hue(cmd: String) -> Json {
    let token = match env::var("HUE_TOKEN") {
        Ok(v) => v,
        _ => return Json(json!({"error": "Token not found"}))
    };

    let hue_url = match env::var("HUE_URL") {
        Ok(v) => v,
        _ => return Json(json!({"error": "Hue URL not found"}))
    };

    get_http(format!("{}/api/{}", hue_url, token));

    Json(json!({
        "command": cmd,
        "token": token
    }))
}
