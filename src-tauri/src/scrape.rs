use std::collections::HashMap;

use reqwest;
use scraper::{Html, Selector};

fn parse_seasons(s_body: &str) -> Vec<String> {
    let fragment = Html::parse_fragment(&s_body);
    let season_selector = Selector::parse("a").unwrap();
    let season_elements = fragment.select(&season_selector);
    let mut seasons = Vec::<String>::new();
    for season in season_elements.clone().into_iter().skip(1) {
        seasons.push(format!("https://a.111477.xyz{}", season.value().attr("href").unwrap_or_default()));
    }
    seasons
}

async fn parse_episodes(season_url: &str) -> Result<(HashMap<String, String>), reqwest::Error> {
    let mut episode_links = HashMap::<String, String>::new();
    let s_body = reqwest::get(season_url).await?.text().await?
        .split("tbody").nth(1).map(|x|x.to_string()).unwrap_or_default();
    let fragment = Html::parse_fragment(&s_body);
    let episode_selector = Selector::parse("a").unwrap();
    for episode in fragment.select(&episode_selector).into_iter().skip(1) {
        episode_links.insert(episode.value().attr("href").unwrap_or_default().to_string(), episode.value().attr("title").unwrap_or_default().to_string());
    }
    
    Ok(episode_links)
}

pub async fn scrape(_show_name: String, url: String) -> Result<(), reqwest::Error> {
    let s_body = reqwest::get(url).await?.text().await?
        .split("tbody").nth(1).map(|x|x.to_string()).unwrap_or_default();
    println!("{s_body}");
    let seasons = parse_seasons(&s_body);
    let mut season_links = HashMap::<String, HashMap<String, String>>::new();
    for season in seasons {
        parse_episodes(&season).await?;
    }
    Ok(())
}