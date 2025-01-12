use anyhow::Result;
use reqwest::{header, Client};
use retry::decorator;
use scraper::{Html, Selector};

async fn fetch_and_parse_html(url: &str) -> Result<Html> {
    let client = Client::new();

    let mut headers = header::HeaderMap::new();
    let res = client.get(url).send().await?;
    let html = res.text().await?;
    let document = Html::parse_document(&html);
    Ok(document)
}

macro_rules! headers {
    ( $( $key:expr => $value: literal ), * $(,)?) => {{
        let mut headers = header::HeaderMap::new();
        $(
            headers.insert($key, header::HeaderValue::from_static($value));
        )*

        headers
    }};
}

#[decorator]
async fn get_by_url(url: &str) -> Result<Html> {
    let client = reqwest::Client::new();
    let headers = headers!(header::USER_AGENT => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36");

    let response = client.get(url).send().await;

    todo!()
}
