use serde::Deserialize;
use std::error;

#[derive(Deserialize)]
struct Ip {
    origin: String,
}

pub fn get_my_ip() -> Result<String, Box<dyn error::Error>> {
    let response: Ip = reqwest::blocking::get("http://httpbin.org/ip")?.json()?;
    Ok(response.origin)
}
