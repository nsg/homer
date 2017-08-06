use std::io::Read;
use curl::easy::Easy;

pub mod hue;

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

fn put_http(url: String, data: String) {
    println!("{} # {}", data, url);
    let mut buffer = (&data).as_bytes();
    let mut curl = Easy::new();
    curl.url(&url).unwrap();
    curl.put(true).unwrap();
    curl.post_field_size(buffer.len() as u64).unwrap();
    let mut tr = curl.transfer();
    tr.read_function(|b| {
        Ok(buffer.read(b).unwrap_or(0))
    }).unwrap();
    tr.perform().unwrap();
}
