use std::env;
use rocket_contrib::Json;
use curl::easy::Easy;

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

    let r = get_http(format!("{}/api/{}", hue_url, token));

    Json(json!({
        "command": cmd,
        "token": token,
        "result": r
    }))
}
