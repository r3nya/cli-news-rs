use std::error::Error;
use std::{env,process};

use serde::Deserialize;
use colour::{dark_green, yellow, dark_red_ln};

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let result = ureq::get(url).call()?.into_string()?;
    let articles: Articles = serde_json::from_str(&result)?;

    Ok(articles)
}

fn api_key() -> String {
    match env::var("API_KEY") {
        Ok(val) => val,
        Err(_) => {
            dark_red_ln!("Required the API_KEY environment variable");
            process::exit(1);
        }
    }
}

fn render_articles(articles: &Articles) {
    for item in &articles.articles {
        dark_green!("> {}\n", item.title);
        yellow!("  {}\n\n", item.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let api_key = api_key();
    let url = format!("https://newsapi.org/v2/top-headlines?country=us&apiKey={}", &api_key);
    let articles = get_articles(&url)?;

    render_articles(&articles);

    Ok(())
}
