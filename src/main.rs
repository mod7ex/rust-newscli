#![allow(unused)]

/* https://youtu.be/4km2UijVC3M */

// 411cbce350f0425f984a82d3fd91600f

use std::error::Error;
use serde::Deserialize;
use serde_json;
use ureq;

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String
}

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;

    let articles: Articles = serde_json::from_str(&response)?;
    
    Ok(articles)
}

fn main() {
    let url = "https://newsapi.org/v2/top-headlines?country=us&category=business&apiKey=411cbce350f0425f984a82d3fd91600f";
    
    match get_articles(url) {
        Ok(v) => { println!("{:#?}", v); }
        Err(e) => { println!("{}", e); }
    }
}
