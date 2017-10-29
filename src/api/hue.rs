extern crate serde;
extern crate serde_json;
extern crate palette;

use rocket_contrib::Json;
use self::palette::{Rgb, Hsv};
use std::env;
use api;

struct HueConnectionData {
    token: String,
    url: String,
    port: String
}

fn setup() -> HueConnectionData {
    let token = match env::var("HUE_TOKEN") {
        Ok(v) => v,
        _ => panic!("HUE_TOKEN not found")
    };

    let url = match env::var("HUE_IP") {
        Ok(v) => v,
        _ => panic!("HUE_IP not found")
    };

    let port = match env::var("HUE_PORT") {
        Ok(v) => v,
        _ => "80".to_string()
    };

    HueConnectionData{token, url, port}
}

fn api(path: &str) -> serde_json::Value {
    let huecon = setup();
    let url = format!("{}:{}/api/{}/{}", huecon.url, huecon.port, huecon.token, path);
    let data = api::get_http(url);
    serde_json::from_str(&data).unwrap()
}

fn api_post(path: &str, j: serde_json::Value) {
    let huecon = setup();
    let url = format!("{}:{}/api/{}/{}", huecon.url, huecon.port, huecon.token, path);
    api::put_http(url, format!("{}", j));
}

fn get_id_from_name_data(name: String) -> Result<String, &'static str> {
    let data = api("lights");
    get_id_from_name(data, name)
}

fn get_id_from_name(data: serde_json::Value, name: String) -> Result<String, &'static str> {
    let num_of_lamps = data.as_object().unwrap().len() + 1;

    for id in 1..num_of_lamps {
        let n = &data[id.to_string()]["name"];
        if n.as_str().unwrap() == name {
            return Ok(format!("{}",id));
        }
    }

    Err("Lamp not found")
}

fn hex_to_rgb(hex: String) -> (f32, f32, f32) {
    let r = u8::from_str_radix(&hex[..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();

    (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
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

#[get("/lights")]
fn lights() -> Json {
    Json(json!({
        "config": api("lights")
    }))
}

#[get("/lights/<id>")]
fn lights_id(id: u8) -> Json {
    Json(json!({
        "config": api(&format!("lights/{}", id))
    }))
}

#[get("/lights/<name>", rank = 2)]
fn lights_name(name: String) -> Json {
    let data = api("lights");
    let id = get_id_from_name(data.clone(), name);

    match id {
        Ok(v) => Json(json!({"lamp": data[v]})),
        Err(e) => Json(json!({"error": e})),
    }
}

#[get("/lights/version")]
fn lights_version() -> Json {
    let data = api("lights");
    let num_of_lamps = data.as_object().unwrap().len() + 1;
    let mut ret = Vec::new();

    for id in 1..num_of_lamps {
        ret.push(
            (
                &data[id.to_string()]["name"],
                &data[id.to_string()]["swversion"]
            )
        );

    }

    Json(json!({
        "config": ret
    }))
}

#[put("/lights/<id>/on/<state>")]
fn set_on(id: u8, state: bool) {
    let body = json!({"on": state});
    api_post(&format!("lights/{}/state", id), body);
}

#[put("/lights/<name>/on/<state>", rank = 2)]
fn set_on_name(name: String, state: bool) {
    let body = json!({"on": state});
    let id = get_id_from_name_data(name);
    match id {
        Ok(v) => api_post(&format!("lights/{}/state", v), body),
        Err(e) => print!("{}", e)
    }
}

#[put("/lights/<id>/brightness/<brightness>")]
fn set_brightnes(id: u8, brightness: u8) {
    let bri = if brightness > 254 {
        254
    } else {
        brightness
    };

    let body = json!({"bri": bri});
    api_post(&format!("lights/{}/state", id), body);
}

#[put("/lights/<name>/brightness/<brightness>", rank = 2)]
fn set_brightnes_name(name: String, brightness: u8) {
    let bri = if brightness > 254 {
        254
    } else {
        brightness
    };

    let body = json!({"bri": bri});
    let id = get_id_from_name_data(name);
    match id {
        Ok(v) => api_post(&format!("lights/{}/state", v), body),
        Err(e) => print!("{}", e)
    }
}

#[put("/lights/<id>/alert/<mode>")]
fn set_alert(id: u8, mode: u8) {
    let m = if mode == 10 {
        "lselect"
    } else {
        "select"
    };

    let body = json!({"alert": m});
    api_post(&format!("lights/{}/state", id), body);
}

#[put("/lights/<name>/alert/<mode>", rank = 2)]
fn set_alert_name(name: String, mode: u8) {
    let m = if mode == 10 {
        "lselect"
    } else {
        "select"
    };

    let body = json!({"alert": m});
    let id = get_id_from_name_data(name);
    match id {
        Ok(v) => api_post(&format!("lights/{}/state", v), body),
        Err(e) => print!("{}", e)
    }

}

#[put("/lights/<id>/color/<color>")]
fn set_color(id: u8, color: String) {
    // r, g and b are floats from 0.0 to 1.0
    let (r, g, b) = hex_to_rgb(color);

    let rgb = Rgb::new(r, g, b);
    let hsv: Hsv = Hsv::from(rgb).into();

    // to_positive_degrees() returns 0..360, hue uses
    // 0..65535 so I do a quick and dirty convert.
    let hue = hsv.hue.to_positive_degrees() * 182.0;

    // sat and bri also uses 0..254
    let body = json!({
        "hue": hue as u32,
        "sat": (hsv.saturation * 254.0) as u8,
        "bri": (hsv.value * 254.0) as u8
    });

    api_post(&format!("lights/{}/state", id), body)
}

#[put("/lights/<name>/color/<color>", rank = 2)]
fn set_color_name(name: String, color: String) {
    let id = get_id_from_name_data(name);

    match id {
        Ok(v) => set_color(v.parse::<u8>().unwrap(), color),
        Err(e) => print!("{}", e)
    }

}
