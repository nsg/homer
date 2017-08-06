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

