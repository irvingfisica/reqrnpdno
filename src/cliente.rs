use std::error::Error;
use reqwest::blocking;
use reqwest::header::{HeaderMap, CONTENT_LENGTH};
use crate::urls::url_base;

pub fn cliente_nuevo() -> Result<blocking::Client, Box<dyn Error>> {

    let mut def_head = HeaderMap::new();
    def_head.insert(CONTENT_LENGTH, "0".parse().unwrap());

    let client = blocking::Client::builder()
                    .cookie_store(true)
                    .default_headers(def_head)
                    .build()?;

    let _ = client.get(url_base()).send()?;

    Ok(client)
}

pub fn cliente_test() -> Result<blocking::Client, Box<dyn Error>> {

    let mut def_head = HeaderMap::new();
    def_head.insert(CONTENT_LENGTH, "0".parse().unwrap());

    let client = blocking::Client::builder()
                    .cookie_store(true)
                    .default_headers(def_head)
                    .build()?;

    // let _ = client.get(url_base()).send()?;

    Ok(client)
}