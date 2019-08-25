use std::collections::HashMap;

use ptp_rs::config;
use reqwest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello world!");

    let resp: HashMap<String, String> = reqwest::get("https://httpbin.org/ip")?
        .json()?;

    println!("{:#?}", resp);

    let config = config::get_config();
    println!("{:?}", config);

    Ok(())
}