use reqwest;
use std::collections::HashMap;

fn main() -> Result<(), Box<std::error::Error>> {
    println!("Hello world!");

    let resp: HashMap<String, String> = reqwest::get("https://httpbin.org/ip")?
        .json()?;

    println!("{:#?}", resp);
    Ok(())
}